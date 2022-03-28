use casper_dao_modules::{GovernanceVoting, VotingId, voting::Voting};
use casper_dao_utils::{casper_dao_macros::casper_contract_interface, Address, consts};
use casper_types::{U256, bytesrepr::{Bytes}, RuntimeArgs, runtime_args};

use crate::{VariableRepositoryContract};

#[casper_contract_interface]
pub trait RepoVoterContractInterface {
    fn init(&mut self, variable_repo: Address, reputation_token: Address);
    fn create_voting(&mut self, variable_repo_to_edit: Address, key: String, value: Bytes, activation_time: Option<u64>, stake: U256);
    fn vote(&mut self, voting_id: VotingId, choice: bool, stake: U256);
    fn finish_voting(&mut self, voting_id: VotingId);    
}

#[derive(Default)]
pub struct RepoVoterContract {
    voting: GovernanceVoting,
}

impl RepoVoterContractInterface for RepoVoterContract {
    fn init(&mut self, variable_repo: Address, reputation_token: Address) {
        self.voting.init(variable_repo, reputation_token);
    }

    fn create_voting(&mut self, variable_repo_to_edit: Address, key: String, value: Bytes, activation_time: Option<u64>, stake: U256) {
        let voting = Voting {
            voting_id: self.voting.votings_count.get(),
            informal_voting_id: self.voting.votings_count.get(),
            formal_voting_id: None,
            formal_voting_quorum: VariableRepositoryContract::get_variable(self.voting.get_variable_repo_address(), consts::FORMAL_VOTING_TIME),
            formal_voting_time: VariableRepositoryContract::get_variable(self.voting.get_variable_repo_address(), consts::FORMAL_VOTING_TIME),
            informal_voting_quorum: VariableRepositoryContract::get_variable(self.voting.get_variable_repo_address(), consts::INFORMAL_VOTING_QUORUM),
            informal_voting_time: VariableRepositoryContract::get_variable(self.voting.get_variable_repo_address(), consts::INFORMAL_VOTING_TIME),
            stake_in_favor: U256::from(0),
            stake_against: U256::from(0),
            completed: false,
            contract_to_call: Some(variable_repo_to_edit),
            entry_point: "update_at".into(),
            runtime_args: runtime_args! {
                "key" => key,
                "value" => value,
                "activation_time" => activation_time,
            },    
            minimum_governance_reputation: VariableRepositoryContract::get_variable(self.voting.get_variable_repo_address(), consts::MINIMUM_GOVERNANCE_REPUTATION),
        };
        self.voting.create_voting(&voting, stake);
    }

    fn vote(&mut self, voting_id: VotingId, choice: bool, stake: U256) {
        self.voting.vote(voting_id, choice, stake);
    }

    fn finish_voting(&mut self, voting_id: VotingId) {
        self.voting.finish_voting(voting_id);
    }
}

#[cfg(feature = "test-support")]
use casper_dao_modules::vote::Vote;
#[cfg(feature = "test-support")]
impl RepoVoterContractTest {
    
    pub fn get_variable_repo_address(&self) -> Address {
        let address: Option<Address> = self
            .env
            .get_value(self.package_hash, self.data.voting.variable_repo.path());
        address.unwrap()
    }

    pub fn get_reputation_token_address(&self) -> Address {
        let address: Option<Address> = self
            .env
            .get_value(self.package_hash, self.data.voting.reputation_token.path());
        address.unwrap()
    }

    pub fn get_voting(&self, voting_id: U256) -> Voting {
        let voting: Voting = self.env.get_dict_value(self.package_hash, self.data.voting.votings.path(), voting_id);
        voting
    }

    pub fn get_vote(&self, voting_id: U256, address: Address) -> Vote {
        self.env.get_dict_value(self.package_hash, self.data.voting.votes.path(), (voting_id, address))
    }

    pub fn get_voters(&self, voting_id: U256) -> Vec<Option<Address>> {
        self.env.get_dict_value(self.package_hash, self.data.voting.voters.path(), voting_id)
    }
}
