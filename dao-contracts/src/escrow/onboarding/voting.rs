use casper_dao_utils::{
    casper_contract::contract_api::runtime::revert,
    casper_dao_macros::Instance,
    casper_env::caller,
    cspr,
    Address,
    DocumentHash,
    Error,
    Mapping,
};
use casper_types::U512;

use super::request::{OnboardingRequest, Request};
use crate::{
    refs::{ContractRefs, ContractRefsWithKycStorage},
    voting::{
        kyc_info::KycInfo,
        onboarding_info::OnboardingInfo,
        voting_state_machine::{VotingResult, VotingStateMachine, VotingSummary, VotingType},
        Choice,
        VotingCreatedInfo,
        VotingEngine,
        VotingId,
    },
    Configuration,
    ConfigurationBuilder,
    ReputationContractInterface,
    VaNftContractInterface,
};

#[derive(Instance)]
pub struct Onboarding {
    requests: Mapping<VotingId, Request>,
    configurations: Mapping<VotingId, Configuration>,
    ids: Mapping<Address, VotingId>,
    #[scoped = "contract"]
    refs: ContractRefsWithKycStorage,
    #[scoped = "contract"]
    voting: VotingEngine,
    kyc: KycInfo,
    info: OnboardingInfo,
}

impl Onboarding {
    pub fn get_request(&self, voting_id: VotingId) -> Option<Request> {
        self.requests.get_or_none(&voting_id)
    }

    pub fn submit_request(
        &mut self,
        reason: DocumentHash,
        cspr_deposit: U512,
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
            is_va: self.info.is_onboarded(&requestor),
            exists_ongoing_voting,
            is_kyced: self.kyc.is_kycd(&requestor),
        };

        // Create voting and cast creator's ballot
        let voting_info = self.init_voting(requestor, configuration.clone(), rep_stake);
        let voting_id = voting_info.voting_id;

        self.store_request(request, voting_id);
        self.store_configuration(voting_id, configuration);

        voting_info
    }

    pub fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let request = self.requests.get_or_revert(&voting_id);
        let summary = self.voting.finish_voting(voting_id, voting_type);

        match summary.voting_type() {
            VotingType::Informal => self.finish_informal_voting(voting_id, &request, &summary),
            VotingType::Formal => self.finish_formal_voting(voting_id, &request, &summary),
        }
    }

    pub fn vote(
        &mut self,
        voting_id: VotingId,
        voting_type: VotingType,
        choice: Choice,
        stake: U512,
    ) {
        self.voting
            .vote(caller(), voting_id, voting_type, choice, stake);
    }

    fn get_user_voting(&self, address: &Address) -> Option<VotingStateMachine> {
        self.ids
            .get_or_none(address)
            .and_then(|voting_id| self.voting.get_voting(voting_id))
    }
}

impl Onboarding {
    fn build_configuration(&self, requestor: Address) -> Configuration {
        ConfigurationBuilder::new(&self.refs)
            .only_va_can_create(false)
            .is_bid_escrow(true)
            .bound_ballot_for_successful_voting(requestor)
            .build()
    }

    fn init_voting(
        &mut self,
        creator: Address,
        configuration: Configuration,
        stake: U512,
    ) -> VotingCreatedInfo {
        let voting_info = self.voting.create_voting(creator, stake, configuration);
        let voting_id = voting_info.voting_id;
        let mut voting = self.voting.get_voting_or_revert(voting_id);

        // passed config disables casting first votes, must be casted manually.
        self.voting.cast_ballot(
            creator,
            voting_id,
            Choice::InFavor,
            stake,
            true,
            &mut voting,
        );
        self.ids.set(&creator, voting_id);
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
        cspr::withdraw_cspr(request.creator(), request.cspr_deposit());
    }

    fn on_informal_voting_finished(&mut self, _voting_id: VotingId) {}

    fn on_formal_voting_in_favor(&mut self, voting_id: VotingId, request: &Request) {
        let configuration = self.configurations.get_or_revert(&voting_id);
        let voting = self.voting.get_voting_or_revert(voting_id);
        // Make the user VA.
        self.refs.va_token().mint(request.creator());
        // Burn temporary reputation.
        self.burn_requestor_reputation(request);
        self.mint_and_redistribute_reputation_for_requestor(&voting, request);
        self.redistribute_cspr(&configuration, request.cspr_deposit());
    }

    fn on_formal_voting_against(&mut self, voting_id: VotingId, request: &Request) {
        let configuration = self.configurations.get_or_revert(&voting_id);
        let amount = self.redistribute_to_governance(&configuration, request.cspr_deposit());
        cspr::withdraw_cspr(request.creator(), amount);
    }
}

// redistribution
impl Onboarding {
    fn mint_and_redistribute_reputation_for_requestor(
        &mut self,
        voting: &VotingStateMachine,
        request: &Request,
    ) {
        let configuration = self.configurations.get_or_revert(&voting.voting_id());

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

    fn mint_reputation_for_voters(&mut self, voting: &VotingStateMachine, amount: U512) {
        let voting_id = voting.voting_id();

        for i in 0..self.voting.voters_count(voting_id, VotingType::Formal) {
            let ballot = self.voting.get_ballot_at(voting_id, VotingType::Formal, i);
            if ballot.unbounded {
                continue;
            }
            let to_transfer = ballot.stake * amount / voting.total_bounded_stake();
            self.refs.reputation_token().mint(ballot.voter, to_transfer);
        }
    }

    fn redistribute_cspr(&mut self, configuration: &Configuration, amount: U512) {
        let to_redistribute = self.redistribute_to_governance(configuration, amount);
        self.redistribute_cspr_to_all_vas(to_redistribute);
    }

    fn redistribute_to_governance(&mut self, configuration: &Configuration, amount: U512) -> U512 {
        let governance_wallet: Address = configuration.bid_escrow_wallet_address();
        let governance_wallet_payment = configuration.apply_bid_escrow_payment_ratio_to(amount);
        cspr::withdraw_cspr(governance_wallet, governance_wallet_payment);

        amount - governance_wallet_payment
    }

    fn redistribute_cspr_to_all_vas(&mut self, to_redistribute: U512) {
        let all_balances = self.refs.reputation_token().all_balances();
        let total_supply = all_balances.total_supply();

        for (address, balance) in all_balances.balances() {
            let amount = to_redistribute * balance / total_supply;
            cspr::withdraw_cspr(*address, amount);
        }
    }

    fn burn_requestor_reputation(&self, request: &Request) {
        self.refs
            .reputation_token()
            .burn(request.creator(), request.rep_stake());
    }
}
