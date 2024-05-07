//! AccessControl module.
use crate::modules::{Owner, Whitelist};
use odra::contract_env::caller;
use odra::types::Address;

/// A AccessControl module storage definition.
#[odra::module]
pub struct AccessControl {
    pub owner: Owner,
    pub whitelist: Whitelist,
}

/// Module entrypoints implementation.
#[odra::module]
impl AccessControl {
    /// Module constructor.
    ///
    /// Initializes submodules.
    ///
    /// See [`Owner`] and [`Whitelist`].
    #[odra(init)]
    pub fn init(&mut self, address: Address) {
        self.owner.init(address);
        self.whitelist.add_to_whitelist(address);
    }

    /// Proposes a change of ownership of the contract. Owner will be changed if accepted by propsed
    /// new owner. Only the current owner is permited to call this method.
    ///
    /// # Errors
    /// Throws [`NotAnOwner`](crate::utils::Error::NotAnOwner) if caller
    /// is not the current owner.
    ///
    pub fn propose_new_owner(&mut self, owner: Address) {
        self.owner.ensure_owner();
        self.owner.propose_owner(owner);
    }

    /// Accepts the new owner proposition. This can be called only by the proposed owner.
    ///
    /// # Events
    /// Emits [`OwnerChanged`](crate::modules::owner::events::OwnerChanged),
    /// [`AddedToWhitelist`](crate::modules::whitelist::events::AddedToWhitelist) events.
    pub fn accept_new_owner(&mut self) {
        let caller = caller();
        self.owner.accept_owner(caller);
        self.whitelist.add_to_whitelist(caller);
    }

    /// Changes the ownership of the contract to the new address.
    ///
    /// # Events
    /// Emits [`OwnerChanged`](crate::modules::owner::events::OwnerChanged) event.
    ///
    /// # Errors
    /// Throws [`NotAnOwner`](crate::utils::Error::NotAnOwner) if caller
    /// is not the current owner.
    pub fn change_ownership(&mut self, owner: Address) {
        self.owner.ensure_owner();
        self.owner.change_ownership(owner);
        self.whitelist.add_to_whitelist(owner);
    }

    /// Adds a new address to the whitelist.
    ///
    /// # Errors
    /// Throws [`NotAnOwner`](crate::utils::Error::NotAnOwner) if the caller
    /// is not the current owner.
    ///
    /// # Events
    /// Emits [`AddedToWhitelist`](crate::modules::whitelist::events::AddedToWhitelist) event.
    pub fn add_to_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.add_to_whitelist(address);
    }

    /// Removes the `address` from the whitelist.
    ///
    /// # Errors
    /// Throws [`NotAnOwner`](crate::utils::Error::NotAnOwner) if caller
    /// is not the current owner.
    ///
    /// # Events
    /// It emits [`RemovedFromWhitelist`](crate::modules::whitelist::events::RemovedFromWhitelist)
    pub fn remove_from_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.remove_from_whitelist(address);
    }

    /// Checks whether the given address is added to the whitelist.
    /// See [`Whitelist`].
    pub fn is_whitelisted(&self, address: Address) -> bool {
        self.whitelist.is_whitelisted(address)
    }

    /// Verifies whether the current caller address is added to the whitelist.
    pub fn ensure_whitelisted(&self) {
        self.whitelist.ensure_whitelisted();
    }

    /// Returns the address of the current owner.
    ///
    /// See [`Owner`].
    pub fn get_owner(&self) -> Option<Address> {
        self.owner.get_owner()
    }
}
