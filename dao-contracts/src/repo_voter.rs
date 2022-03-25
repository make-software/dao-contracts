use casper_dao_modules::{vote::Vote, GovernanceVoting, VotingId, voting::Voting};
use casper_dao_utils::{casper_dao_macros::casper_contract_interface, Address, consts, casper_contract::unwrap_or_revert::UnwrapOrRevert};
use casper_types::{U256, bytesrepr::{Bytes, FromBytes}, RuntimeArgs};

use crate::{VariableRepositoryContractCaller, VariableRepositoryContractInterface};

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
        let variable_repo_caller = VariableRepositoryContractCaller::at(self.voting.get_variable_repo_address().as_contract_package_hash().unwrap_or_revert().clone());
        let informal_voting_quorum = variable_repo_caller.get(consts::INFORMAL_VOTING_QUORUM.into()).unwrap_or_revert();
        let (informal_voting_quorum, _bytes) = U256::from_bytes(&informal_voting_quorum).unwrap_or_revert();
        let voting = Voting {
            voting_id: self.voting.votings_count.get(),
            informal_voting_id: self.voting.votings_count.get(),
            formal_voting_id: None,
            formal_voting_quorum: U256::from(2),
            formal_voting_time: U256::from(2),
            informal_voting_quorum,
            //informal_voting_quorum: U256::from(2),
            informal_voting_time: U256::from(2),
            stake_in_favor: U256::from(0),
            stake_against: U256::from(0),
            completed: false,
            entry_point: "update_variable_at".into(),
            runtime_args: RuntimeArgs::new(),
            minimum_governance_reputation: U256::from(2),
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
