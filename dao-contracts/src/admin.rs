use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address, ContractCall,
};
use casper_types::{runtime_args, RuntimeArgs, U256};

use crate::{
    action::Action,
    voting::{voting::Voting, Ballot, Choice, GovernanceVoting},
    VotingConfigurationBuilder,
};

use crate::voting::types::VotingId;
use delegate::delegate;

#[casper_contract_interface]
pub trait AdminContractInterface {
    /// see [GovernanceVoting](GovernanceVoting)
    fn init(&mut self, variable_repo: Address, reputation_token: Address);

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
    /// see [GovernanceVoting](GovernanceVoting)
    fn vote(&mut self, voting_id: VotingId, choice: Choice, stake: U256);
    /// see [GovernanceVoting](GovernanceVoting)
    fn finish_voting(&mut self, voting_id: VotingId);
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_dust_amount(&self) -> U256;
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_variable_repo_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_reputation_token_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_voting(&self, voting_id: U256) -> Option<Voting>;
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_ballot(&self, voting_id: U256, address: Address) -> Option<Ballot>;
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_voter(&self, voting_id: U256, at: u32) -> Option<Address>;
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
    fn create_voting(
        &mut self,
        contract_to_update: Address,
        action: Action,
        address: Address,
        stake: U256,
    ) {
        let voting_configuration = VotingConfigurationBuilder::with_defaults(&self.voting)
            .with_contract_call(ContractCall {
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

    fn vote(&mut self, voting_id: VotingId, choice: Choice, stake: U256) {
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    delegate! {
        to self.voting {
            fn init(&mut self, variable_repo: Address, reputation_token: Address);
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
