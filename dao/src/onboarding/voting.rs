//! Voting module for onboarding requests.

use crate::bid_escrow::events::TransferReason;
use crate::configuration::{Configuration, ConfigurationBuilder};
use crate::modules::refs::ContractRefs;
use crate::onboarding::request::{OnboardingRequest, Request};
use crate::utils::types::DocumentHash;
use crate::utils::{withdraw, Error};
use crate::voting::ballot::Choice;
use crate::voting::cspr_redistribution::{
    redistribute_cspr_to_all_vas, redistribute_to_governance,
};
use crate::voting::types::VotingId;
use crate::voting::voting_engine::events::VotingCreatedInfo;
use crate::voting::voting_engine::voting_state_machine::{
    VotingResult, VotingStateMachine, VotingSummary, VotingType,
};
use crate::voting::voting_engine::VotingEngine;
use odra::contract_env::{caller, revert};
use odra::types::{Address, Balance};
use odra::{Mapping, UnwrapOrRevert};

/// Onboarding voting module.
#[odra::module]
pub struct Onboarding {
    requests: Mapping<VotingId, Request>,
    configurations: Mapping<VotingId, Configuration>,
    ids: Mapping<Address, VotingId>,
    refs: ContractRefs,
    voting: VotingEngine,
}

impl Onboarding {
    pub fn get_request(&self, voting_id: VotingId) -> Option<Request> {
        self.requests.get(&voting_id)
    }

    pub fn submit_request(
        &mut self,
        reason: DocumentHash,
        cspr_deposit: Balance,
    ) -> VotingCreatedInfo {
        let requestor = caller();

        let configuration = self.build_configuration(requestor);
        let rep_stake = configuration.apply_reputation_conversion_rate_to(cspr_deposit);
        let exists_ongoing_voting = self
            .get_user_voting(&requestor)
            .map(|v| !v.completed())
            .unwrap_or_default();

        let request = OnboardingRequest {
            requestor,
            reason,
            rep_stake,
            cspr_deposit,
            is_va: !self.refs.va_token().balance_of(&requestor).is_zero(),
            exists_ongoing_voting,
            is_kyced: !self.refs.kyc_token().balance_of(&requestor).is_zero(),
        };

        // Create voting and cast creator's ballot
        let voting_info = self.init_voting(requestor, configuration.clone(), rep_stake);
        let voting_id = voting_info.voting_id;

        self.store_request(request, voting_id);
        self.store_configuration(voting_id, configuration);

        voting_info
    }

    pub fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) -> VotingSummary {
        let request = self
            .requests
            .get(&voting_id)
            .unwrap_or_revert_with(Error::OnboardingRequestNotFound);
        let summary = self.voting.finish_voting(voting_id, voting_type);

        match summary.voting_type() {
            VotingType::Informal => self.finish_informal_voting(voting_id, &request, &summary),
            VotingType::Formal => self.finish_formal_voting(voting_id, &request, &summary),
        }

        summary
    }

    pub fn vote(
        &mut self,
        voting_id: VotingId,
        voting_type: VotingType,
        choice: Choice,
        stake: Balance,
    ) {
        self.voting
            .vote(caller(), voting_id, voting_type, choice, stake);
    }

    fn get_user_voting(&self, address: &Address) -> Option<VotingStateMachine> {
        self.ids
            .get(address)
            .and_then(|voting_id| self.voting.get_voting(voting_id))
    }
}

impl Onboarding {
    fn build_configuration(&self, requestor: Address) -> Configuration {
        ConfigurationBuilder::new(
            self.refs.va_token().total_supply(),
            &self.refs.variable_repository().all_variables(),
        )
        .only_va_can_create(false)
        .set_is_bid_escrow(true)
        .bind_ballot_for_successful_voting(requestor)
        .build()
    }

    fn init_voting(
        &mut self,
        creator: Address,
        configuration: Configuration,
        stake: Balance,
    ) -> VotingCreatedInfo {
        let (voting_info, mut voting) =
            self.voting
                .create_voting(creator, stake, configuration.clone());

        // passed config disables casting first votes, must be casted manually.
        self.voting.cast_ballot(
            creator,
            Choice::InFavor,
            stake,
            true,
            &mut voting,
            &configuration,
        );
        self.ids.set(&creator, voting.voting_id());
        self.voting.set_voting(voting);
        voting_info
    }

    fn store_request(&mut self, request: OnboardingRequest, voting_id: VotingId) {
        let request = Request::new(request);
        self.requests.set(&voting_id, request);
    }

    fn store_configuration(&mut self, voting_id: VotingId, configuration: Configuration) {
        self.configurations.set(&voting_id, configuration);
    }
}

// handling voting result
impl Onboarding {
    fn finish_informal_voting(
        &mut self,
        voting_id: VotingId,
        request: &Request,
        summary: &VotingSummary,
    ) {
        match summary.result() {
            VotingResult::InFavor | VotingResult::Against => {
                self.on_informal_voting_finished(voting_id);
            }
            VotingResult::QuorumNotReached => {
                self.on_quorum_not_reached(request);
            }
            VotingResult::Canceled => Self::on_voting_canceled(),
        }
    }

    fn finish_formal_voting(
        &mut self,
        voting_id: VotingId,
        request: &Request,
        summary: &VotingSummary,
    ) {
        match summary.result() {
            VotingResult::InFavor => self.on_formal_voting_in_favor(voting_id, request),
            VotingResult::Against => self.on_formal_voting_against(voting_id, request),
            VotingResult::QuorumNotReached => self.on_quorum_not_reached(request),
            VotingResult::Canceled => Self::on_voting_canceled(),
        }
    }

    fn on_voting_canceled() {
        revert(Error::VotingAlreadyCanceled)
    }

    fn on_quorum_not_reached(&self, request: &Request) {
        withdraw(
            &request.creator(),
            request.cspr_deposit(),
            TransferReason::OnboardingStakeReturn,
        );
    }

    fn on_informal_voting_finished(&mut self, _voting_id: VotingId) {}

    fn on_formal_voting_in_favor(&mut self, voting_id: VotingId, request: &Request) {
        let configuration = self
            .configurations
            .get(&voting_id)
            .unwrap_or_revert_with(Error::OnboardingConfigurationNotFound);
        let voting = self.voting.get_voting_or_revert(voting_id);
        // Make the user VA.
        self.refs.va_token().mint(request.creator());
        // Burn temporary reputation.
        self.burn_requestor_reputation(request);
        self.mint_and_redistribute_reputation_for_requestor(&voting, request);
        self.redistribute_cspr(&configuration, request.cspr_deposit());
    }

    fn on_formal_voting_against(&mut self, voting_id: VotingId, request: &Request) {
        let configuration = self
            .configurations
            .get(&voting_id)
            .unwrap_or_revert_with(Error::OnboardingConfigurationNotFound);
        let amount = redistribute_to_governance(request.cspr_deposit(), &configuration);
        withdraw(&request.creator(), amount, TransferReason::BidStakeReturn);
    }
}

// redistribution
impl Onboarding {
    fn mint_and_redistribute_reputation_for_requestor(
        &mut self,
        voting: &VotingStateMachine,
        request: &Request,
    ) {
        let configuration = self
            .configurations
            .get(&voting.voting_id())
            .unwrap_or_revert_with(Error::OnboardingConfigurationNotFound);

        let reputation_to_mint = request.rep_stake();
        let reputation_to_redistribute =
            configuration.apply_default_policing_rate_to(reputation_to_mint);

        // Worker
        self.refs.reputation_token().mint(
            request.creator(),
            reputation_to_mint - reputation_to_redistribute,
        );

        // Voters
        self.mint_reputation_for_voters(voting, reputation_to_redistribute);
    }

    fn mint_reputation_for_voters(&mut self, voting: &VotingStateMachine, amount: Balance) {
        let voting_id = voting.voting_id();

        for i in 0..self.voting.voters_count(voting_id, VotingType::Formal) {
            let ballot = self.voting.get_ballot_at(voting_id, VotingType::Formal, i);
            if ballot.unbound {
                continue;
            }
            let to_transfer = ballot.stake * amount / voting.total_bound_stake();
            self.refs.reputation_token().mint(ballot.voter, to_transfer);
        }
    }

    fn redistribute_cspr(&mut self, configuration: &Configuration, amount: Balance) {
        let to_redistribute = redistribute_to_governance(amount, configuration);
        redistribute_cspr_to_all_vas(to_redistribute, &self.refs);
    }

    fn burn_requestor_reputation(&self, request: &Request) {
        self.refs
            .reputation_token()
            .burn(request.creator(), request.rep_stake());
    }
}
