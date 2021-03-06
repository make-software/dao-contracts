use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{caller, self_address},
    Address, ContractCall, Variable,
};
use casper_types::{runtime_args, RuntimeArgs, U256};

use crate::{
    voting::{types::VotingId, voting::Voting, Ballot, Choice, GovernanceVoting},
    VotingConfigurationBuilder,
};

use delegate::delegate;

#[casper_contract_interface]
pub trait MockVoterContractInterface {
    fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
    fn create_voting(&mut self, value: String, stake: U256);
    fn vote(&mut self, voting_id: VotingId, choice: Choice, stake: U256);
    fn finish_voting(&mut self, voting_id: VotingId);
    fn get_dust_amount(&self) -> U256;
    fn get_variable_repo_address(&self) -> Address;
    fn get_reputation_token_address(&self) -> Address;
    fn get_voting(&self, voting_id: U256) -> Option<Voting>;
    fn get_ballot(&self, voting_id: U256, address: Address) -> Option<Ballot>;
    fn get_voter(&self, voting_id: U256, at: u32) -> Option<Address>;
    fn set_variable(&mut self, variable: String);
    fn get_variable(&self) -> String;
}

#[doc(hidden)]
#[derive(Instance)]
pub struct MockVoterContract {
    voting: GovernanceVoting,
    variable: Variable<String>,
}

impl MockVoterContractInterface for MockVoterContract {
    fn create_voting(&mut self, value: String, stake: U256) {
        let voting_configuration = VotingConfigurationBuilder::defaults(&self.voting)
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

    fn vote(&mut self, voting_id: VotingId, choice: Choice, stake: U256) {
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    fn set_variable(&mut self, variable: String) {
        self.variable.set(variable);
    }

    fn get_variable(&self) -> String {
        self.variable.get().unwrap_or_default()
    }

    delegate! {
        to self.voting {
            fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
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
