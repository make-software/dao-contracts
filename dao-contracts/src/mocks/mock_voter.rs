use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{caller, self_address},
    Address,
    ContractCall,
    Variable,
};
use casper_types::{runtime_args, RuntimeArgs, U512};
use delegate::delegate;

use crate::{
    refs::ContractRefsStorage,
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
pub trait MockVoterContractInterface {
    fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
    fn create_voting(&mut self, value: String, stake: U512);
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
    fn variable_repository_address(&self) -> Address;
    fn reputation_token_address(&self) -> Address;
    fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    fn set_variable(&mut self, variable: String);
    fn get_variable(&self) -> String;
}

#[doc(hidden)]
#[derive(Instance)]
pub struct MockVoterContract {
    refs: ContractRefsStorage,
    voting: VotingEngine,
    variable: Variable<String>,
}

impl MockVoterContractInterface for MockVoterContract {
    delegate! {
        to self.voting {
            fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
            fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
            fn get_ballot(&self, voting_id: VotingId, voting_type: VotingType, address: Address) -> Option<Ballot>;
            fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
        }

        to self.refs {
            fn init(&mut self, variable_repository: Address, reputation_token: Address, va_token: Address);
            fn variable_repository_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
        }
    }

    fn create_voting(&mut self, value: String, stake: U512) {
        let voting_configuration = ConfigurationBuilder::new(&self.refs)
            .contract_call(ContractCall {
                address: self_address(),
                entry_point: "set_variable".into(),
                runtime_args: runtime_args! {
                    "variable" => value,
                },
            })
            .build();

        self.voting
            .create_voting(caller(), stake, voting_configuration);
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        self.voting
            .vote(caller(), voting_id, voting_type, choice, stake);
    }

    fn set_variable(&mut self, variable: String) {
        self.variable.set(variable);
    }

    fn get_variable(&self) -> String {
        self.variable.get().unwrap_or_default()
    }
}
