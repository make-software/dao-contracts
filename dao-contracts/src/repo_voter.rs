use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
    ContractCall,
};
use casper_types::{bytesrepr::Bytes, runtime_args, RuntimeArgs, U512};
use delegate::delegate;

use crate::{
    voting::{
        types::VotingId,
        voting_state_machine::{VotingStateMachine, VotingType},
        Ballot,
        Choice,
        VotingEngine,
    },
    ConfigurationBuilder,
};

#[casper_contract_interface]
pub trait RepoVoterContractInterface {
    /// see [VotingEngine](VotingEngine::init())
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
        stake: U512,
    );
    /// see [VotingEngine](VotingEngine::vote())
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
    /// see [VotingEngine](VotingEngine::finish_voting())
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
    /// see [VotingEngine](VotingEngine::get_variable_repo_address())
    fn variable_repo_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine::get_reputation_token_address())
    fn reputation_token_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine::get_voting())
    fn get_voting(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
    ) -> Option<VotingStateMachine>;
    /// see [VotingEngine](VotingEngine::get_ballot())
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// see [VotingEngine](VotingEngine::get_voter())
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    fn cancel_voter(&mut self, voter: Address, voting_id: VotingId);
}

/// RepoVoterContract
///
/// It is responsible for managing variables held in [Variable Repo](crate::VariableRepositoryContract).
///
/// Each change to the variable is being voted on, and when the voting passes, a change is made at given time.
#[derive(Instance)]
pub struct RepoVoterContract {
    voting: VotingEngine,
}

impl RepoVoterContractInterface for RepoVoterContract {
    delegate! {
        to self.voting {
            fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
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
        stake: U512,
    ) {
        let voting_configuration = ConfigurationBuilder::new(
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

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    fn get_voting(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
    ) -> Option<VotingStateMachine> {
        self.voting.get_voting(voting_id)
    }

    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot> {
        self.voting.get_ballot(voting_id, voting_type, address)
    }

    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address> {
        self.voting.get_voter(voting_id, voting_type, at)
    }

    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        self.voting.finish_voting(voting_id);
    }

    fn cancel_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.voting.cancel_voter(voter, voting_id);
    }
}
