//! Whitelist module.
use crate::modules::whitelist::events::{AddedToWhitelist, RemovedFromWhitelist};
use crate::utils::Error;
use odra::contract_env::{caller, revert};
use odra::types::event::OdraEvent;
use odra::types::Address;
use odra::Mapping;

/// The Whitelist module.
#[odra::module]
pub struct Whitelist {
    pub whitelist: Mapping<Address, bool>,
}

#[odra::module]
impl Whitelist {
    /// Add new `address` to the whitelist.
    pub fn add_to_whitelist(&mut self, address: Address) {
        self.whitelist.set(&address, true);
        AddedToWhitelist { address }.emit();
    }

    /// Remove an `address` from the whitelist.
    pub fn remove_from_whitelist(&mut self, address: Address) {
        self.whitelist.set(&address, false);
        RemovedFromWhitelist { address }.emit();
    }

    /// Assert the caller is on the list. Revert otherwise.
    pub fn ensure_whitelisted(&self) {
        if !self.is_whitelisted(caller()) {
            revert(Error::NotWhitelisted);
        }
    }

    /// Returns true if the address is whitelisted.
    pub fn is_whitelisted(&self, address: Address) -> bool {
        self.whitelist.get(&address).unwrap_or(false)
    }
}

pub mod events {
    //! Events definitions.

    use odra::types::Address;
    use odra::Event;

    /// Event emitted when new address has been added to the whitelist.
    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct AddedToWhitelist {
        pub address: Address,
    }

    /// Event emitted when new address has been removed from the whitelist.
    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct RemovedFromWhitelist {
        pub address: Address,
    }
}
