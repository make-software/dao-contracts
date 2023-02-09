//! Whitelist-based access control system.

use casper_dao_utils::{
    casper_dao_macros::Instance,
    casper_env::{self, caller, emit},
    Address,
    Error,
    Mapping,
};
use casper_event_standard::Schemas;

use self::events::{AddedToWhitelist, RemovedFromWhitelist};

/// The Whitelist module.
#[derive(Instance)]
pub struct Whitelist {
    pub whitelist: Mapping<Address, bool>,
}

impl Whitelist {
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
        if !self.is_whitelisted(&caller()) {
            casper_env::revert(Error::NotWhitelisted);
        }
    }

    pub fn is_whitelisted(&self, address: &Address) -> bool {
        self.whitelist.get(address).unwrap_or(false)
    }
}

pub mod events {
    //! Events definitions.

    use casper_dao_utils::Address;
    use casper_event_standard::Event;

    /// Informs new address has been added to the whitelist.
    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct AddedToWhitelist {
        pub address: Address,
    }

    /// Informs new address has been removed from the whitelist.
    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct RemovedFromWhitelist {
        pub address: Address,
    }
}

pub fn add_event_schemas(schemas: &mut Schemas) {
    schemas.add::<AddedToWhitelist>();
    schemas.add::<RemovedFromWhitelist>();
}
