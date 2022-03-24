//! Whitelist-based access control system.

use casper_contract::contract_api::runtime;

use crate::{
    casper_env::{caller, emit},
    consts, Address, Error, Mapping,
};

use self::events::{AddedToWhitelist, RemovedFromWhitelist};

/// The Whitelist module.
pub struct Whitelist {
    pub whitelist: Mapping<Address, bool>,
}

impl Default for Whitelist {
    fn default() -> Self {
        Self {
            whitelist: Mapping::from(consts::NAME_WHITELIST),
        }
    }
}

impl Whitelist {
    /// Initialize the module.
    pub fn init(&mut self) {
        self.whitelist.init();
    }

    /// Add new `address` to the whitelist.
    pub fn add_to_whitelist(&mut self, address: Address) {
        self.whitelist.set(&address, true);
        emit(AddedToWhitelist { address });
    }

    /// Remove an `address` from the whitelist.
    pub fn remove_from_whitelist(&mut self, address: Address) {
        self.whitelist.set(&address, false);
        emit(RemovedFromWhitelist { address });
    }

    /// Assert the caller is on the list. Revert otherwise.
    pub fn ensure_whitelisted(&self) {
        if !self.whitelist.get(&caller()) {
            runtime::revert(Error::NotWhitelisted);
        }
    }

    pub fn is_whitelisted(&self, address: &Address) -> bool {
        self.whitelist.get(address)
    }
}

pub mod events {
    //! Events definitions.
    use crate::Address;
    use casper_dao_macros::Event;

    /// Informs new address has been added to the whitelist.
    #[derive(Debug, PartialEq, Event)]
    pub struct AddedToWhitelist {
        pub address: Address,
    }

    /// Informs new address has been removed from the whitelist.
    #[derive(Debug, PartialEq, Event)]
    pub struct RemovedFromWhitelist {
        pub address: Address,
    }
}
