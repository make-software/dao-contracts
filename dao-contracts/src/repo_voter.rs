use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
    ContractCall,
};
use casper_types::{bytesrepr::Bytes, runtime_args, RuntimeArgs, U256};
use delegate::delegate;

use crate::{
    voting::{
        types::VotingId,
        voting::{Voting, VotingType},
        Ballot,
        Choice,
        GovernanceVoting,
    },
    DaoConfigurationBuilder,
};

#[casper_contract_interface]
pub trait RepoVoterContractInterface {
    /// see [GovernanceVoting](GovernanceVoting::init())
    fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
    /// Creates new RepoVoter voting.
    ///
    /// `variable_repo_to_edit` takes an [Address](Address) of a [Variable Repo](crate::VariableRepositoryContract) instance that will be updated
    ///
    /// `key`, `value` and `activation_time` are parameters that will be passed to `update_at` method of a [Variable Repo](crate::VariableRepositoryContract)
    fn create_voting(
        &mut self,
        variable_repo_to_edit: Address,
        key: String,
        value: Bytes,
        activation_time: Option<u64>,
        stake: U256,
    );
    /// see [GovernanceVoting](GovernanceVoting::vote())
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U256);
    /// see [GovernanceVoting](GovernanceVoting::finish_voting())
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
    /// see [GovernanceVoting](GovernanceVoting::get_dust_amount())
    fn get_dust_amount(&self) -> U256;
    /// see [GovernanceVoting](GovernanceVoting::get_variable_repo_address())
    fn variable_repo_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting::get_reputation_token_address())
    fn reputation_token_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting::get_voting())
    fn get_voting(&self, voting_id: VotingId, voting_type: VotingType) -> Option<Voting>;
    /// see [GovernanceVoting](GovernanceVoting::get_ballot())
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// see [GovernanceVoting](GovernanceVoting::get_voter())
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
}

/// RepoVoterContract
///
/// It is responsible for managing variables held in [Variable Repo](crate::VariableRepositoryContract).
///
/// Each change to the variable is being voted on, and when the voting passes, a change is made at given time.
#[derive(Instance)]
pub struct RepoVoterContract {
    voting: GovernanceVoting,
}

impl RepoVoterContractInterface for RepoVoterContract {
    delegate! {
        to self.voting {
            fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
            fn get_dust_amount(&self) -> U256;
            fn variable_repo_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
        }
    }

    fn create_voting(
        &mut self,
        variable_repo_to_edit: Address,
        key: String,
        value: Bytes,
        activation_time: Option<u64>,
        stake: U256,
    ) {
        let voting_configuration = DaoConfigurationBuilder::new(
            self.voting.variable_repo_address(),
            self.voting.va_token_address(),
        )
        .contract_call(ContractCall {
            address: variable_repo_to_edit,
            entry_point: "update_at".into(),
            runtime_args: runtime_args! {
                "key" => key,
                "value" => value,
                "activation_time" => activation_time,
            },
        })
        .build();

        self.voting
            .create_voting(caller(), stake, voting_configuration);
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U256) {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    fn get_voting(&self, voting_id: VotingId, voting_type: VotingType) -> Option<Voting> {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.get_voting(voting_id)
    }

    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot> {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.get_ballot(voting_id, address)
    }

    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address> {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.get_voter(voting_id, at)
    }

    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.finish_voting(voting_id);
    }
}
