use casper_dao_utils::{
    casper_contract::{
        contract_api::{
            runtime::revert,
            system::{self, get_purse_balance},
        },
        unwrap_or_revert::UnwrapOrRevert,
    },
    casper_dao_macros::{CLTyped, FromBytes, Instance, ToBytes},
    casper_env::{self, caller},
    Address,
    DocumentHash,
    Error,
    Mapping,
};
use casper_types::{URef, U512};

use crate::{
    voting::{Choice, VotingEngine, VotingId},
    Configuration,
    ConfigurationBuilder,
};

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub struct Request {
    reason: DocumentHash,
    stake: U512,
    voting_id: VotingId,
}

#[derive(Instance)]
pub struct Onboarding {
    requests: Mapping<Address, Request>,
    #[scoped = "contract"]
    voting: VotingEngine,
}

impl Onboarding {
    pub fn submit_request(&mut self, reason: DocumentHash, purse: URef) {
        let candidate = caller();

        // Check if a request already exists.
        if self.requests.get(&candidate).is_some() {
            revert(0) // request already submitted.
        }

        let configuration = self.build_configuration();

        let cspr_amount = Self::stake_cspr(purse);
        let stake = configuration.apply_reputation_conversion_rate_to(cspr_amount);

        // Create voting and cast creator's ballot
        let voting_id = self.init_voting(candidate, configuration, stake);

        self.store_request(&candidate, reason, stake, voting_id);
        // TODO: emit event
    }
}

impl Onboarding {
    fn stake_cspr(cargo_purse: URef) -> U512 {
        let main_purse = casper_env::contract_main_purse();
        let amount = get_purse_balance(cargo_purse).unwrap_or_revert();

        system::transfer_from_purse_to_purse(cargo_purse, main_purse, amount, None)
            .unwrap_or_revert();

        amount
    }

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

    fn store_request(&mut self, candidate: &Address, reason: DocumentHash, stake: U512, voting_id: VotingId) {
        let request = Request {
            reason,
            stake,
            voting_id,
        };

        self.requests.set(candidate, request);
    }
}
