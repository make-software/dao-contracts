//! Contains Dao Ids Contract definition and related abstractions.
//!
//! There is one continuous indexation of votes in the system.
//! Each new voting gets a unique across-the-system id generated by the contract.
use casper_dao_modules::{AccessControl, SequenceGenerator};
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
};
use delegate::delegate;

use crate::voting::VotingId;

#[casper_contract_interface]
pub trait DaoIdsContractInterface {
    ///  Initializes a contract.
    ///  Sets the deployer as the owner.
    ///
    ///  see [AccessControl](AccessControl::init())
    fn init(&mut self);
    /// Returns the next voting id in the system.
    ///
    /// # Errors
    /// Throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if the caller is not whitelisted.
    fn next_voting_id(&mut self) -> VotingId;
    /// Changes the ownership of the contract. Transfers the ownership to the `owner`.
    /// Only the current owner is permitted to call this method.
    ///
    /// [`Read more`](AccessControl::change_ownership())
    fn change_ownership(&mut self, owner: Address);
    /// Adds a new address to the whitelist.
    ///
    /// [`Read more`](AccessControl::add_to_whitelist())
    fn add_to_whitelist(&mut self, address: Address);
    /// Remove address from the whitelist.
    ///
    /// [`Read more`](AccessControl::remove_from_whitelist())
    fn remove_from_whitelist(&mut self, address: Address);
    /// Checks whether the given address is added to the whitelist.
    ///
    /// [`Read more`](AccessControl::is_whitelisted()).
    fn is_whitelisted(&self, address: Address) -> bool;
    /// Returns the address of the current owner.
    ///
    /// [`Read more`](AccessControl::get_owner()).
    fn get_owner(&self) -> Option<Address>;
}

/// Dao Ids contract manages voting ids in the system.
/// Only a whitelisted account is eligible to generate ids.
///
/// For details see [DaoIdsContractInterface](DaoIdsContractInterface).
#[derive(Instance)]
pub struct DaoIdsContract {
    access_control: AccessControl,
    voting_id_seq: SequenceGenerator<VotingId>,
}

impl DaoIdsContractInterface for DaoIdsContract {
    delegate! {
        to self.access_control {
            fn is_whitelisted(&self, address: Address) -> bool;
            fn get_owner(&self) -> Option<Address>;
            fn change_ownership(&mut self, owner: Address);
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
        }
    }

    fn init(&mut self) {
        let deployer = caller();
        self.access_control.init(deployer);
    }

    fn next_voting_id(&mut self) -> VotingId {
        self.access_control.ensure_whitelisted();
        self.voting_id_seq.next_value()
    }
}
