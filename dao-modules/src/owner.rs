//! Single-owner-based access control system.

// use casper_contract::contract_api::runtime;

use casper_dao_utils::{
    casper_dao_macros::Instance,
    casper_env::{self, caller, emit},
    Address, Error, Variable,
};

use self::events::OwnerChanged;

/// The Owner module.
#[derive(Instance)]
pub struct Owner {
    pub owner: Variable<Address>,
}

impl Owner {
    /// Initialize the module.
    pub fn init(&mut self, owner: Address) {
        self.change_ownership(owner);
    }

    /// Set the owner to the new address.
    pub fn change_ownership(&mut self, owner: Address) {
        self.owner.set(owner);
        emit(OwnerChanged { new_owner: owner });
    }

    /// Verify if the contract caller is the owner. Revert otherwise.
    pub fn ensure_owner(&self) {
        if let Some(owner) = self.owner.get() {
            if owner != caller() {
                casper_env::revert(Error::NotAnOwner) // User is not the owner.
            }
        } else {
            casper_env::revert(Error::OwnerIsNotInitialized) // Owner is not inicialized.
        }
    }

    pub fn get_owner(&self) -> Option<Address> {
        self.owner.get()
    }
}

pub mod events {
    //! Events definitions.
    use casper_dao_utils::{casper_dao_macros::Event, Address};

    /// Informs the owner change.
    #[derive(Debug, PartialEq, Event)]
    pub struct OwnerChanged {
        pub new_owner: Address,
    }
}
