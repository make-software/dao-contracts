use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
};
use casper_types::{runtime_args, RuntimeArgs, U256};

use crate::{
    action::Action,
    voting::{voting::Voting, Ballot, Choice, GovernanceVoting, VotingId},
};

use delegate::delegate;

#[casper_contract_interface]
pub trait AdminContractInterface {
    fn init(&mut self, variable_repo: Address, reputation_token: Address);
    fn create_voting(
        &mut self,
        contract_to_update: Address,
        action: Action,
        address: Address,
        stake: U256,
    );
    fn vote(&mut self, voting_id: VotingId, choice: Choice, stake: U256);
    fn finish_voting(&mut self, voting_id: VotingId);
    fn get_dust_amount(&self) -> U256;
    fn get_variable_repo_address(&self) -> Address;
    fn get_reputation_token_address(&self) -> Address;
    fn get_voting(&self, voting_id: U256) -> Option<Voting>;
    fn get_ballot(&self, voting_id: U256, address: Address) -> Option<Ballot>;
    fn get_voter(&self, voting_id: U256, at: u32) -> Option<Address>;
}

#[derive(Instance)]
pub struct AdminContract {
    voting: GovernanceVoting,
}

impl AdminContractInterface for AdminContract {
    fn create_voting(
        &mut self,
        contract_to_update: Address,
        action: Action,
        address: Address,
        stake: U256,
    ) {
        self.voting.create_voting(
            caller(),
            stake,
            contract_to_update,
            action.get_entry_point(),
            runtime_args! {
                action.get_arg() => address,
            },
        );
    }

    fn vote(&mut self, voting_id: VotingId, choice: Choice, stake: U256) {
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    delegate! {
        to self.voting {
            fn init(&mut self, variable_repo: Address, reputation_token: Address);
            fn finish_voting(&mut self, voting_id: VotingId);
            fn get_dust_amount(&self) -> U256;
            fn get_variable_repo_address(&self) -> Address;
            fn get_reputation_token_address(&self) -> Address;
            fn get_voting(&self, voting_id: U256) -> Option<Voting>;
            fn get_ballot(&self, voting_id: U256, address: Address) -> Option<Ballot>;
            fn get_voter(&self, voting_id: U256, at: u32) -> Option<Address>;
        }
    }
}
