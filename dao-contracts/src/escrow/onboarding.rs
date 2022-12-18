use casper_dao_utils::{
    casper_contract::{
        contract_api::{
            runtime::revert,
        },
        unwrap_or_revert::UnwrapOrRevert,
    },
    casper_dao_macros::{CLTyped, FromBytes, Instance, ToBytes},
    casper_env::caller,
    Address,
    DocumentHash,
    Error,
    Mapping, transfer,
};
use casper_types::{URef, U512};

use crate::{
    voting::{Choice, VotingEngine, VotingId, voting_state_machine::{VotingType, VotingResult}},
    Configuration,
    ConfigurationBuilder, VaNftContractInterface, ReputationContractCaller, ReputationContractInterface,
};

use super::{redistribution::Redistribution, storage::JobStorage};

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub struct Request {
    creator: Address,
    reason: DocumentHash,
    stake: U512,
    cspr_stake: U512,
    voting_id: VotingId,
}

#[derive(Instance)]
pub struct Onboarding {
    requests: Mapping<VotingId, Request>,
    configurations: Mapping<VotingId, Configuration>,
    #[scoped = "contract"]
    voting: VotingEngine,
    #[scoped = "contract"]
    job_storage: JobStorage,
    redistribution: Redistribution
}

impl Onboarding {
    pub fn get_request(&self, voting_id: VotingId) -> Option<Request> {
        self.requests.get_or_none(&voting_id)
    }

    pub fn submit_request(&mut self, reason: DocumentHash, purse: URef) {
        let candidate = caller();

        // Check if is not a VA

        // Check if a request already exists.
        // if self.requests.get(&candidate).is_some() {
        //     revert(0) // request already submitted.
        // }

        let configuration = self.build_configuration();

        let cspr_amount = transfer::deposit_cspr(purse);
        let stake = configuration.apply_reputation_conversion_rate_to(cspr_amount);

        // Create voting and cast creator's ballot
        let voting_id = self.init_voting(candidate, configuration.clone(), stake);

        self.store_configuration(voting_id, configuration);
        self.store_request(candidate, reason, stake, cspr_amount, voting_id);
        // TODO: emit event
    }

    pub fn finish_voting(&mut self, voting_id: VotingId) {
        let request = self.requests.get_or_revert(&voting_id);
        let voting_summary = self
            .voting
            .finish_voting_without_token_redistribution(voting_id);

        let configuration = self.configurations.get_or_revert(&voting_id);
        match voting_summary.voting_type() {
            VotingType::Informal => match voting_summary.result() {
                VotingResult::InFavor => {
                    self.create_formal_voting(voting_id);
                }
                VotingResult::Against => {
                    self.create_formal_voting(voting_id);
                }
                VotingResult::QuorumNotReached => {
                    if configuration.informal_stake_reputation() {
                        self.voting
                            .return_reputation_of_yes_voters(voting_id, VotingType::Informal);
                        self.voting
                            .return_reputation_of_no_voters(voting_id, VotingType::Informal);
                    }
                    transfer::withdraw_cspr(request.creator, request.stake);
                }
                VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
            },
            VotingType::Formal => {
                match voting_summary.result() {
                    VotingResult::InFavor => {
                        // Make user VA.
                        self.voting.va_token().mint(request.creator);

                        // Bound ballot for worker.
                        self.voting
                            .bound_ballot(voting_id, request.creator, VotingType::Formal);

                        self.voting
                            .return_reputation_of_yes_voters(voting_id, VotingType::Formal);
                        self.voting.redistribute_reputation_of_no_voters(
                            voting_id,
                            VotingType::Formal,
                        );
                        let configuration = self.configurations.get_or_revert(&voting_id);
                        self.burn_external_worker_reputation(&configuration, &request);
                        self.mint_and_redistribute_reputation_for_requestor(voting_id, &request);
                        self.redistribute_cspr(&configuration, request.cspr_stake);
                    },
                    VotingResult::Against =>  {
                        self.voting
                            .return_reputation_of_no_voters(voting_id, VotingType::Formal);
                        self.voting.redistribute_reputation_of_yes_voters(
                            voting_id,
                            VotingType::Formal,
                        );
                        transfer::withdraw_cspr(request.creator, request.stake);

                        // self.redistribute_cspr_external_worker_failed(&job);
                    },
                    VotingResult::QuorumNotReached => {
                        self.voting
                            .return_reputation_of_yes_voters(voting_id, VotingType::Formal);
                        self.voting
                            .return_reputation_of_no_voters(voting_id, VotingType::Formal);
                        transfer::withdraw_cspr(request.creator, request.stake);
                    }
                    VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
                }
            }
        }
    }

    fn create_formal_voting(&mut self, voting_id: VotingId) {
        let voting = self
            .voting
            .get_voting(voting_id)
            .unwrap_or_revert_with(Error::VotingDoesNotExist);
        if voting.voting_configuration().informal_stake_reputation() {
            self.voting
                .unstake_all_reputation(voting_id, VotingType::Informal);
        }
        self.voting
            .recast_creators_ballot_from_informal_to_formal(voting_id);
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
        let voting_id = self
            .voting
            .create_voting(creator, U512::zero(), configuration);

        self.voting.cast_ballot(
            creator,
            voting_id,
            Choice::InFavor,
            stake,
            true,
            self.voting
                .get_voting(voting_id)
                .unwrap_or_revert_with(Error::VotingDoesNotExist),
        );

        voting_id
    }

    fn store_request(&mut self, candidate: Address, reason: DocumentHash, stake: U512, cspr_stake: U512, voting_id: VotingId) {
        let request = Request {
            creator: candidate,
            reason,
            stake,
            cspr_stake,
            voting_id,
        };

        self.requests.set(&voting_id, request);
    }

    fn store_configuration(&mut self, voting_id: VotingId, configuration: Configuration) {
        self.configurations.set(&voting_id, configuration);
    }

    fn mint_and_redistribute_reputation_for_requestor(&mut self, voting_id: VotingId, request: &Request) {
        let configuration = self.configurations.get_or_revert(&voting_id);

        let reputation_to_mint = request.stake;
        let reputation_to_redistribute =
            configuration.apply_default_policing_rate_to(reputation_to_mint);

        // Worker
        ReputationContractCaller::at(self.voting.reputation_token_address()).mint(
            request.creator,
            reputation_to_mint - reputation_to_redistribute,
        );

        // Voters
        self.mint_reputation_for_voters(voting_id, reputation_to_redistribute);
    }

    fn mint_reputation_for_voters(&mut self, voting_id: VotingId, amount: U512) {
        let voting = self
            .voting
            .get_voting(voting_id)
            .unwrap_or_revert();

        let mut reputation = ReputationContractCaller::at(self.voting.reputation_token_address());
        for i in 0..self
            .voting
            .voters()
            .len((voting.voting_id(), VotingType::Formal))
        {
            let ballot = self
                .voting
                .get_ballot_at(voting.voting_id(), VotingType::Formal, i);
            if ballot.unbounded {
                continue;
            }
            let to_transfer = ballot.stake * amount / voting.total_bounded_stake();
            reputation.mint(ballot.voter, to_transfer);
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
        let token = ReputationContractCaller::at(self.voting.reputation_token_address());
        let (total_supply, balances) = token.all_balances();
        for (address, balance) in balances.balances {
            let amount = to_redistribute * balance / total_supply;
            casper_dao_utils::transfer::withdraw_cspr(address, amount);
        }
    }

    fn burn_external_worker_reputation(&self, configuration: &Configuration, request: &Request) {
        let mut token = ReputationContractCaller::at(self.voting.reputation_token_address());
        token.burn(request.creator, request.stake);
    }
}
