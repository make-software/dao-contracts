use casper_dao_modules::{vote::Vote, voting::Voting, GovernanceVoting, VotingId};
use casper_dao_utils::{
    casper_dao_macros::casper_contract_interface, casper_env::get_block_time, consts, Address,
};
use casper_types::{bytesrepr::Bytes, runtime_args, RuntimeArgs, U256};

use crate::VariableRepositoryContractCaller;

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
    fn get_voters(&self, voting_id: U256) -> Vec<Option<Address>>;
}

#[derive(Default)]
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
        let variable_repo_package_hash = *self
            .voting
            .get_variable_repo_address()
            .as_contract_package_hash()
            .unwrap();
        let repo_caller = VariableRepositoryContractCaller::at(variable_repo_package_hash);
        let informal_voting_time = repo_caller.get_variable(consts::INFORMAL_VOTING_TIME);
        let informal_voting_quorum = repo_caller.get_variable(consts::INFORMAL_VOTING_QUORUM);
        let formal_voting_time = repo_caller.get_variable(consts::FORMAL_VOTING_TIME);
        let formal_voting_quorum = repo_caller.get_variable(consts::FORMAL_VOTING_QUORUM);
        let minimum_governance_reputation =
            repo_caller.get_variable(consts::MINIMUM_GOVERNANCE_REPUTATION);

        let voting = Voting {
            voting_id: self.voting.votings_count.get(),
            completed: false,
            stake_in_favor: U256::zero(),
            stake_against: U256::zero(),
            finish_time: get_block_time() + informal_voting_time,
            informal_voting_id: self.voting.votings_count.get(),
            formal_voting_id: None,
            formal_voting_quorum,
            formal_voting_time,
            informal_voting_quorum,
            informal_voting_time,
            contract_to_call: Some(variable_repo_to_edit),
            entry_point: "update_at".into(),
            runtime_args: runtime_args! {
                "key" => key,
                "value" => value,
                "activation_time" => activation_time,
            },
            minimum_governance_reputation,
        };
        self.voting.create_voting(&voting, stake);
    }

    fn vote(&mut self, voting_id: VotingId, choice: bool, stake: U256) {
        self.voting.vote(voting_id, choice, stake);
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
        self.voting.votings.get(&voting_id)
    }

    fn get_vote(&self, voting_id: U256, address: Address) -> Vote {
        self.voting.votes.get(&(voting_id, address))
    }

    fn get_voters(&self, voting_id: U256) -> Vec<Option<Address>> {
        self.voting.voters.get(&voting_id)
    }
}
