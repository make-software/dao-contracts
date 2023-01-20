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
    /// Change ownership of the contract. Transfer the ownership to the `owner`. Only current owner
    /// is permitted to call this method.
    ///
    /// See [AccessControl](AccessControl::change_ownership())
    fn change_ownership(&mut self, owner: Address);
    /// Add new address to the whitelist.
    ///
    /// See [AccessControl](AccessControl::add_to_whitelist())
    fn add_to_whitelist(&mut self, address: Address);
    /// Remove address from the whitelist.
    ///
    /// See [AccessControl](AccessControl::remove_from_whitelist())
    fn remove_from_whitelist(&mut self, address: Address);
    /// Returns the address of the current owner.
    fn get_owner(&self) -> Option<Address>;
    /// Checks whether the given address is added to the whitelist.
    fn is_whitelisted(&self, address: Address) -> bool;
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
