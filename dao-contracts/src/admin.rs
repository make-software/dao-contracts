use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
};
use casper_types::{runtime_args, RuntimeArgs, U256};

use crate::{voting::{voting::Voting, GovernanceVoting, Vote, VotingId}, action::Action};

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
    fn vote(&mut self, voting_id: VotingId, choice: bool, stake: U256);
    fn finish_voting(&mut self, voting_id: VotingId);
    fn get_dust_amount(&self) -> U256;
    fn get_variable_repo_address(&self) -> Address;
    fn get_reputation_token_address(&self) -> Address;
    fn get_voting(&self, voting_id: U256) -> Voting;
    fn get_vote(&self, voting_id: U256, address: Address) -> Vote;
    fn get_voter(&self, voting_id: U256, at: u32) -> Address;
}

#[derive(Instance)]
pub struct AdminContract {
    voting: GovernanceVoting,
}

impl AdminContractInterface for AdminContract {
    fn init(&mut self, variable_repo: Address, reputation_token: Address) {
        self.voting.init(variable_repo, reputation_token);
    }

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

    fn vote(&mut self, voting_id: VotingId, choice: bool, stake: U256) {
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    fn finish_voting(&mut self, voting_id: VotingId) {
        self.voting.finish_voting(voting_id);
    }

    fn get_dust_amount(&self) -> U256 {
        self.voting.get_dust_amount()
    }

    fn get_variable_repo_address(&self) -> Address {
        self.voting.get_variable_repo_address()
    }

    fn get_reputation_token_address(&self) -> Address {
        self.voting.get_reputation_token_address()
    }

    fn get_voting(&self, voting_id: VotingId) -> Voting {
        self.voting.get_voting(voting_id)
    }

    fn get_vote(&self, voting_id: U256, address: Address) -> Vote {
        self.voting.get_vote(voting_id, address)
    }

    fn get_voter(&self, voting_id: U256, at: u32) -> Address {
        self.voting.get_voter(voting_id, at)
    }
}