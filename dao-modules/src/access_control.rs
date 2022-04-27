use casper_dao_utils::{casper_dao_macros::Instance, Address};

use crate::{Owner, Whitelist};

/// The Access control module.
///
/// Aggregates the typical applications of [`Owner`] and [`Whitelist`] modules.
#[derive(Instance)]
pub struct AccessControl {
    pub owner: Owner,
    pub whitelist: Whitelist,
}

impl AccessControl {
    pub fn init(&mut self, address: Address) {
        self.owner.init(address);
        self.whitelist.add_to_whitelist(address);
    }

    pub fn change_ownership(&mut self, owner: Address) {
        self.owner.ensure_owner();
        self.owner.change_ownership(owner);
        self.whitelist.add_to_whitelist(owner);
    }

    pub fn add_to_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.add_to_whitelist(address);
    }

    pub fn remove_from_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.remove_from_whitelist(address);
    }

    pub fn is_whitelisted(&self, address: Address) -> bool {
        self.whitelist.is_whitelisted(&address)
    }

    pub fn ensure_whitelisted(&self) {
        self.whitelist.ensure_whitelisted();
    }

    pub fn get_owner(&self) -> Option<Address> {
        self.owner.get_owner()
    }
}
