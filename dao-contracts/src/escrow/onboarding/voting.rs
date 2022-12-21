use casper_dao_utils::{
    casper_contract::contract_api::runtime::revert,
    casper_dao_macros::Instance,
    casper_env::caller,
    transfer,
    Address,
    DocumentHash,
    Error,
    Mapping,
};
use casper_types::{URef, U512};

use super::request::{OnboardingRequest, Request};
use crate::{
    voting::{
        kyc_info::KycInfo,
        onboarding_info::OnboardingInfo,
        voting_state_machine::{VotingResult, VotingStateMachine, VotingSummary, VotingType},
        Choice,
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
    voting: VotingEngine,
    kyc: KycInfo,
    info: OnboardingInfo,
}

impl Onboarding {
    pub fn init(&mut self, va_token: Address, kyc_token: Address) {
        self.info.init(va_token);
        self.kyc.init(kyc_token);
    }

    pub fn get_request(&self, voting_id: VotingId) -> Option<Request> {
        self.requests.get_or_none(&voting_id)
    }

    pub fn submit_request(&mut self, reason: DocumentHash, purse: URef) {
        let requestor = caller();

        let configuration = self.build_configuration();
        let cspr_deposit = transfer::deposit_cspr(purse);
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
        let voting_id = self.init_voting(requestor, configuration.clone(), rep_stake);

        self.store_request(request, voting_id);
        self.store_configuration(voting_id, configuration);
        // TODO: emit event
    }

    pub fn finish_voting(&mut self, voting_id: VotingId) {
        let request = self.requests.get_or_revert(&voting_id);
        let summary = self
            .voting
            .finish_voting_without_token_redistribution(voting_id);

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
    fn build_configuration(&self) -> Configuration {
        ConfigurationBuilder::new(
            self.voting.variable_repo_address(),
            self.voting.va_token_address(),
        )
        .only_va_can_create(false)
        .is_bid_escrow(true)
        .build()
    }

    fn init_voting(
        &mut self,
        creator: Address,
        configuration: Configuration,
        stake: U512,
    ) -> VotingId {
        let voting_id = self.voting.create_voting(creator, stake, configuration);

        // passed config disables casting first votes, must be casted manually.
        self.voting.cast_ballot(
            creator,
            voting_id,
            Choice::InFavor,
            stake,
            true,
            self.voting.get_voting_or_revert(voting_id),
        );
        self.ids.set(&creator, voting_id);

        voting_id
    }

    fn store_request(&mut self, request: OnboardingRequest, voting_id: VotingId) {
        let request = Request::new(request);
        self.requests.set(&voting_id, request);
    }

    fn store_configuration(&mut self, voting_id: VotingId, configuration: Configuration) {
        self.configurations.set(&voting_id, configuration);
    }

    fn create_formal_voting(&mut self, voting_id: VotingId) {
        let voting = self.voting.get_voting_or_revert(voting_id);
        if voting.voting_configuration().informal_stake_reputation() {
            self.voting
                .unstake_all_reputation(voting_id, VotingType::Informal);
        }
        self.voting
            .recast_creators_ballot_from_informal_to_formal(voting_id);
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
                self.on_informal_voting_finished(voting_id)
            }
            VotingResult::QuorumNotReached => {
                self.on_quorum_not_reached(voting_id, VotingType::Informal, request)
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
            VotingResult::QuorumNotReached => {
                self.on_quorum_not_reached(voting_id, VotingType::Formal, request)
            }
            VotingResult::Canceled => Self::on_voting_canceled(),
        }
    }

    fn on_voting_canceled() {
        revert(Error::VotingAlreadyCanceled)
    }

    fn on_quorum_not_reached(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        request: &Request,
    ) {
        let configuration = self.configurations.get_or_revert(&voting_id);

        if (configuration.informal_stake_reputation() && voting_type == VotingType::Informal)
            || voting_type == VotingType::Formal
        {
            self.voting
                .return_reputation_of_yes_voters(voting_id, voting_type);
            self.voting
                .return_reputation_of_no_voters(voting_id, voting_type);
        }

        transfer::withdraw_cspr(request.creator(), request.cspr_deposit());
    }

    fn on_informal_voting_finished(&mut self, voting_id: VotingId) {
        self.create_formal_voting(voting_id);
    }

    fn on_formal_voting_in_favor(&mut self, voting_id: VotingId, request: &Request) {
        let configuration = self.configurations.get_or_revert(&voting_id);

        // Make the user VA.
        self.voting.va_token().mint(request.creator());

        // Bound ballot for the requester - mint temporary reputation.
        self.voting
            .bound_ballot(voting_id, request.creator(), VotingType::Formal);
        self.voting
            .return_reputation_of_yes_voters(voting_id, VotingType::Formal);
        self.voting
            .redistribute_reputation_of_no_voters(voting_id, VotingType::Formal);
        // Burn temporary reputation.
        self.burn_requestor_reputation(&request);
        self.mint_and_redistribute_reputation_for_requestor(voting_id, &request);
        self.redistribute_cspr(&configuration, request.cspr_deposit());
    }

    fn on_formal_voting_against(&mut self, voting_id: VotingId, request: &Request) {
        let configuration = self.configurations.get_or_revert(&voting_id);

        self.voting
            .return_reputation_of_no_voters(voting_id, VotingType::Formal);
        self.voting
            .redistribute_reputation_of_yes_voters(voting_id, VotingType::Formal);
        let amount = self.redistribute_to_governance(&configuration, request.cspr_deposit());
        transfer::withdraw_cspr(request.creator(), amount);
    }
}

// redistribution
impl Onboarding {
    fn mint_and_redistribute_reputation_for_requestor(
        &mut self,
        voting_id: VotingId,
        request: &Request,
    ) {
        let configuration = self.configurations.get_or_revert(&voting_id);

        let reputation_to_mint = request.rep_stake();
        let reputation_to_redistribute =
            configuration.apply_default_policing_rate_to(reputation_to_mint);

        // Worker
        self.voting.reputation_token().mint(
            request.creator(),
            reputation_to_mint - reputation_to_redistribute,
        );

        // Voters
        self.mint_reputation_for_voters(voting_id, reputation_to_redistribute);
    }

    fn mint_reputation_for_voters(&mut self, voting_id: VotingId, amount: U512) {
        let voting = self.voting.get_voting_or_revert(voting_id);

        for i in 0..self.voting.voters_count(voting_id, VotingType::Formal) {
            let ballot = self.voting.get_ballot_at(voting_id, VotingType::Formal, i);
            if ballot.unbounded {
                continue;
            }
            let to_transfer = ballot.stake * amount / voting.total_bounded_stake();
            self.voting
                .reputation_token()
                .mint(ballot.voter, to_transfer);
        }
    }

    fn redistribute_cspr(&mut self, configuration: &Configuration, amount: U512) {
        let to_redistribute = self.redistribute_to_governance(configuration, amount);
        self.redistribute_cspr_to_all_vas(to_redistribute);
    }

    fn redistribute_to_governance(&mut self, configuration: &Configuration, amount: U512) -> U512 {
        let governance_wallet: Address = configuration.bid_escrow_wallet_address();
        let governance_wallet_payment = configuration.apply_bid_escrow_payment_ratio_to(amount);
        casper_dao_utils::transfer::withdraw_cspr(governance_wallet, governance_wallet_payment);

        amount - governance_wallet_payment
    }

    fn redistribute_cspr_to_all_vas(&mut self, to_redistribute: U512) {
        let (total_supply, balances) = self.voting.reputation_token().all_balances();
        for (address, balance) in balances.balances {
            let amount = to_redistribute * balance / total_supply;
            casper_dao_utils::transfer::withdraw_cspr(address, amount);
        }
    }

    fn burn_requestor_reputation(&self, request: &Request) {
        self.voting
            .reputation_token()
            .burn(request.creator(), request.rep_stake());
    }
}
