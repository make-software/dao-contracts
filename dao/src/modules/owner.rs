//! The Owner module.
use crate::modules::owner::events::OwnerChanged;
use crate::utils::Error;
use odra::contract_env::{caller, revert};
use odra::types::event::OdraEvent;
use odra::types::Address;
use odra::Variable;

#[odra::module]
pub struct Owner {
    pub owner: Variable<Address>,
    pub proposed_owner: Variable<Option<Address>>,
}

/// Module entrypoints implementation.
#[odra::module]
impl Owner {
    /// Initialize the module.
    #[odra(init)]
    pub fn init(&mut self, owner: Address) {
        self.change_ownership(owner);
    }

    /// Sets a new owner proposition.
    pub fn propose_owner(&mut self, owner: Address) {
        self.proposed_owner.set(Some(owner));
    }

    /// Accepts the new owner proposition.
    pub fn accept_owner(&mut self, caller: Address) {
        if let Some(proposed_owner) = self.proposed_owner.get_or_default() {
            if proposed_owner != caller {
                revert(Error::NotAProposedOwner);
            }
            self.change_ownership(proposed_owner);
        } else {
            revert(Error::NoProposedOwner);
        }
    }

    /// Set the owner to the new address.
    pub fn change_ownership(&mut self, owner: Address) {
        self.owner.set(owner);
        self.proposed_owner.set(None);

        OwnerChanged { new_owner: owner }.emit();
    }

    /// Verify if the contract caller is the owner. Revert otherwise.
    pub fn ensure_owner(&self) {
        if let Some(owner) = self.owner.get() {
            if owner != caller() {
                revert(Error::NotAnOwner)
            }
        } else {
            revert(Error::OwnerIsNotInitialized) // Owner is not initialized.
        }
    }

    pub fn get_owner(&self) -> Option<Address> {
        self.owner.get()
    }
}
pub mod events {
    //! Events definitions.
    use odra::types::Address;
    use odra::Event;

    /// Event emitted when the owner change.
    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct OwnerChanged {
        pub new_owner: Address,
    }
}
