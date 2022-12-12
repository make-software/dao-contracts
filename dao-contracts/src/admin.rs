use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
    ContractCall,
};
use casper_types::{runtime_args, RuntimeArgs, U512};
use delegate::delegate;

use crate::{
    action::Action,
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
pub trait AdminContractInterface {
    /// see [VotingEngine](VotingEngine::init())
    fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);

    /// Creates new admin voting.
    ///
    /// `contract_to_update` is an [Address](Address) of a contract that will be updated
    ///
    /// `action` is an [Action](Action) that will be performed on given contract
    ///
    /// `address` is a parameter for given action - the [Address](Address) which permissions will be changed
    fn create_voting(
        &mut self,
        contract_to_update: Address,
        action: Action,
        address: Address,
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

/// Admin contract uses [VotingEngine](VotingEngine) to vote on changes of ownership and managing whitelists of other contracts.
///
/// Admin contract needs to have permissions to perform those actions.
///
/// For details see [AdminContractInterface](AdminContractInterface)
#[derive(Instance)]
pub struct AdminContract {
    voting: VotingEngine,
}

impl AdminContractInterface for AdminContract {
    delegate! {
        to self.voting {
            fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
            fn variable_repo_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
        }
    }

    fn create_voting(
        &mut self,
        contract_to_update: Address,
        action: Action,
        address: Address,
        stake: U512,
    ) {
        let voting_configuration = ConfigurationBuilder::new(
            self.voting.variable_repo_address(),
            self.voting.va_token_address(),
        )
        .contract_call(ContractCall {
            address: contract_to_update,
            entry_point: action.get_entry_point(),
            runtime_args: runtime_args! {
                action.get_arg() => address,
            },
        })
        .build();

        self.voting
            .create_voting(caller(), stake, voting_configuration);
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        self.voting.finish_voting(voting_id);
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

    fn cancel_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.voting.cancel_voter(voter, voting_id);
    }
}
