use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
};
use casper_types::{bytesrepr::Bytes, runtime_args, RuntimeArgs, U256};

use crate::voting::{voting::Voting, GovernanceVoting, Vote, VotingId};

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
    fn get_voters(&self, voting_id: U256) -> Vec<Address>;
}

#[derive(Instance)]
pub struct RepoVoterContract {
    voting: GovernanceVoting,
}

impl RepoVoterContractInterface for RepoVoterContract {
    fn init(&mut self, variable_repo: Address, reputation_token: Address) {
        self.voting.init(variable_repo, reputation_token);
    }

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

    fn get_voters(&self, voting_id: U256) -> Vec<Address> {
        self.voting.get_voters(voting_id)
    }
}
