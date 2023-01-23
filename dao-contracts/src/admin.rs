//! Contains Admin Contract definition and related abstractions.
//!
//! # General
//! The contract is used to manage the other contracts' [`access control`].
//!
//! Three types of voting can be created:
//! * add an [`Address`] to the whitelist.
//! * remove an [`Address`] from the whitelist.
//! * sets an [`Address`] as a new contract owner.
//!
//! # Voting
//! The Voting process is managed by [`VotingEngine`].
//!
//! [`access control`]: casper_dao_modules::AccessControl
//! [VotingEngine]: crate::voting::VotingEngine
use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, CLTyped, Event, FromBytes, Instance, ToBytes},
    casper_env::caller,
    Address,
    BlockTime,
    ContractCall,
};
use casper_types::{runtime_args, RuntimeArgs, U512};
use delegate::delegate;

use crate::{
    config::ConfigurationBuilder,
    voting::{
        refs::ContractRefsStorage,
        voting_state_machine::{VotingStateMachine, VotingType},
        Ballot,
        Choice,
        VotingCreatedInfo,
        VotingEngine,
        VotingId,
    },
};

#[casper_contract_interface]
pub trait AdminContractInterface {
    /// Constructor function.
    ///
    /// # Note
    /// Initializes contract elements:
    /// * Sets up [`ContractRefsStorage`] by writing addresses of [`Variable Repository`](crate::variable_repository::VariableRepositoryContract),
    /// [`Reputation Token`](crate::reputation::ReputationContract), [`VA Token`](crate::va_nft::VaNftContract).
    /// * Sets [`caller`] as the owner of the contract.
    /// * Adds [`caller`] to the whitelist.
    ///
    /// # Events
    /// Emits:
    /// * [`OwnerChanged`](casper_dao_modules::events::OwnerChanged),
    /// * [`AddedToWhitelist`](casper_dao_modules::events::AddedToWhitelist),
    fn init(&mut self, variable_repository: Address, reputation_token: Address, va_token: Address);

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
    /// Returns the address of [Variable Repository](crate::variable_repository::VariableRepositoryContract) contract.
    fn variable_repository_address(&self) -> Address;
    /// Returns the address of [Reputation Token](crate::reputation::ReputationContract) contract.
    fn reputation_token_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine::get_voting())
    fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
    /// see [VotingEngine](VotingEngine::get_ballot())
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// see [VotingEngine](VotingEngine::get_voter())
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;

    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn get_owner(&self) -> Option<Address>;
    fn is_whitelisted(&self, address: Address) -> bool;
}

/// Admin contract uses [VotingEngine](VotingEngine) to vote on changes of ownership and managing whitelists of other contracts.
///
/// Admin contract needs to have permissions to perform those actions.
///
/// For details see [AdminContractInterface](AdminContractInterface).
#[derive(Instance)]
pub struct AdminContract {
    refs: ContractRefsStorage,
    voting_engine: VotingEngine,
    access_control: AccessControl,
}

impl AdminContractInterface for AdminContract {
    delegate! {
        to self.voting_engine {
            fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
            fn get_voting(
                &self,
                voting_id: VotingId,
            ) -> Option<VotingStateMachine>;
            fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                address: Address,
            ) -> Option<Ballot>;
             fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
        }

        to self.access_control {
            fn change_ownership(&mut self, owner: Address);
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
            fn is_whitelisted(&self, address: Address) -> bool;
            fn get_owner(&self) -> Option<Address>;
        }

        to self.refs {
            fn variable_repository_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
        }
    }

    fn init(&mut self, variable_repository: Address, reputation_token: Address, va_token: Address) {
        self.refs
            .init(variable_repository, reputation_token, va_token);
        self.access_control.init(caller());
    }

    fn create_voting(
        &mut self,
        contract_to_update: Address,
        action: Action,
        address: Address,
        stake: U512,
    ) {
        let voting_configuration = ConfigurationBuilder::new(&self.refs)
            .contract_call(ContractCall {
                address: contract_to_update,
                entry_point: action.get_entry_point(),
                runtime_args: runtime_args! {
                    action.get_arg() => address,
                },
            })
            .build();

        let (info, _) = self
            .voting_engine
            .create_voting(caller(), stake, voting_configuration);

        AdminVotingCreated::new(contract_to_update, action, address, info).emit();
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        self.voting_engine
            .vote(caller(), voting_id, voting_type, choice, stake);
    }

    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        self.voting_engine.finish_voting(voting_id, voting_type);
    }

    fn slash_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.voting_engine.slash_voter(voter, voting_id);
    }
}

/// Event emitted once voting is created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct AdminVotingCreated {
    contract_to_update: Address,
    action: Action,
    address: Address,
    creator: Address,
    stake: Option<U512>,
    voting_id: VotingId,
    config_informal_quorum: u32,
    config_informal_voting_time: u64,
    config_formal_quorum: u32,
    config_formal_voting_time: u64,
    config_total_onboarded: U512,
    config_double_time_between_votings: bool,
    config_voting_clearness_delta: U512,
    config_time_between_informal_and_formal_voting: BlockTime,
}

impl AdminVotingCreated {
    pub fn new(
        contract_to_update: Address,
        action: Action,
        address: Address,
        info: VotingCreatedInfo,
    ) -> Self {
        Self {
            contract_to_update,
            action,
            address,
            creator: info.creator,
            stake: info.stake,
            voting_id: info.voting_id,
            config_informal_quorum: info.config_informal_quorum,
            config_informal_voting_time: info.config_informal_voting_time,
            config_formal_quorum: info.config_formal_quorum,
            config_formal_voting_time: info.config_formal_voting_time,
            config_total_onboarded: info.config_total_onboarded,
            config_double_time_between_votings: info.config_double_time_between_votings,
            config_voting_clearness_delta: info.config_voting_clearness_delta,
            config_time_between_informal_and_formal_voting: info
                .config_time_between_informal_and_formal_voting,
        }
    }
}

/// Enum for actions that [AdminContract] can perform
///
/// - `AddToWhitelist` - calls `add_to_whitelist` method
/// - `RemoveFromWhitelist` - calls `remove_from_whitelist` method
/// - `ChangeOwner` - calls `change_ownership` method
#[derive(CLTyped, PartialEq, Eq, Debug, FromBytes, ToBytes)]
pub enum Action {
    AddToWhitelist,
    RemoveFromWhitelist,
    ChangeOwner,
}

impl Action {
    pub(crate) fn get_entry_point(&self) -> String {
        match self {
            Action::AddToWhitelist => "add_to_whitelist",
            Action::RemoveFromWhitelist => "remove_from_whitelist",
            Action::ChangeOwner => "change_ownership",
        }
        .to_string()
    }

    pub(crate) fn get_arg(&self) -> &str {
        match self {
            Action::AddToWhitelist => "address",
            Action::RemoveFromWhitelist => "address",
            Action::ChangeOwner => "owner",
        }
    }
}

#[test]
fn test_action() {
    use casper_types::bytesrepr::{FromBytes, ToBytes};
    let action = Action::ChangeOwner;
    let (deserialized_action, _) = Action::from_bytes(&action.to_bytes().unwrap()).unwrap();

    assert_eq!(action, deserialized_action);
    assert_eq!(deserialized_action.get_arg(), "owner");
    assert_eq!(
        deserialized_action.get_entry_point(),
        "change_ownership".to_string()
    );
}
