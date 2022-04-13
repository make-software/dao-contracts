use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
};
use casper_types::{bytesrepr::Bytes, runtime_args, RuntimeArgs, U256};

use crate::voting::{voting::Voting, GovernanceVoting, Vote, VotingId};

use delegate::delegate;

#[casper_contract_interface]
pub trait RepoVoterContractInterface {
    fn init(&mut self, variable_repo: Address, reputation_token: Address);
    fn create_voting(
        &mut self,
        variable_repo_to_edit: Address,
        key: String,
        value: Bytes,
        activation_time: Option<u64>,
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
pub struct RepoVoterContract {
    voting: GovernanceVoting,
}

impl RepoVoterContractInterface for RepoVoterContract {
    fn create_voting(
        &mut self,
        variable_repo_to_edit: Address,
        key: String,
        value: Bytes,
        activation_time: Option<u64>,
        stake: U256,
    ) {
        self.voting.create_voting(
            caller(),
            stake,
            variable_repo_to_edit,
            "update_at".into(),
            runtime_args! {
                "key" => key,
                "value" => value,
                "activation_time" => activation_time,
            },
        );
    }

    fn vote(&mut self, voting_id: VotingId, choice: bool, stake: U256) {
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    delegate! {
        to self.voting {
            fn init(&mut self, variable_repo: Address, reputation_token: Address);
            fn finish_voting(&mut self, voting_id: VotingId);
            fn get_dust_amount(&self) -> U256;
            fn get_variable_repo_address(&self) -> Address;
            fn get_reputation_token_address(&self) -> Address;
            fn get_voting(&self, voting_id: U256) -> Voting;
            fn get_vote(&self, voting_id: U256, address: Address) -> Vote;
            fn get_voter(&self, voting_id: U256, at: u32) -> Address;
        }
    }
}
