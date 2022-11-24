use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
    ContractCall,
};
use casper_types::{runtime_args, RuntimeArgs, U256};
use delegate::delegate;

use crate::{
    action::Action,
    voting::{
        types::VotingId,
        voting::{Voting, VotingType},
        Ballot,
        Choice,
        GovernanceVoting,
    },
    VotingConfigurationBuilder,
};

#[casper_contract_interface]
pub trait AdminContractInterface {
    /// see [GovernanceVoting](GovernanceVoting::init())
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

/// Admin contract uses [GovernanceVoting](GovernanceVoting) to vote on changes of ownership and managing whitelists of other contracts.
///
/// Admin contract needs to have permissions to perform those actions.
///
/// For details see [AdminContractInterface](AdminContractInterface)
#[derive(Instance)]
pub struct AdminContract {
    voting: GovernanceVoting,
}

impl AdminContractInterface for AdminContract {
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
        contract_to_update: Address,
        action: Action,
        address: Address,
        stake: U256,
    ) {
        let voting_configuration = VotingConfigurationBuilder::defaults(self.voting.variable_repo_address(), self.voting.va_token_address())
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

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U256) {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.finish_voting(voting_id);
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
}
