//! The Owner module.
use crate::modules::owner::events::OwnerChanged;
use crate::utils::Error;
use odra::contract_env::{caller, revert, self};
use odra::types::event::OdraEvent;
use odra::types::Address;

const KEY_OWNER: &[u8] = b"__odra_owner";

#[odra::module]
pub struct Owner;

/// Module entrypoints implementation.
#[odra::module]
impl Owner {
    /// Initialize the module.
    #[odra(init)]
    pub fn init(&mut self, owner: Address) {
        self.change_ownership(owner);
    }

    /// Set the owner to the new address.
    pub fn change_ownership(&mut self, owner: Address) {
        contract_env::set_var(KEY_OWNER, owner);

        OwnerChanged { new_owner: owner }.emit();
    }

    /// Verify if the contract caller is the owner. Revert otherwise.
    pub fn ensure_owner(&self) {
        if let Some(owner) = contract_env::get_var::<Address>(KEY_OWNER) {
            if owner != caller() {
                revert(Error::NotAnOwner)
            }
        } else {
            revert(Error::OwnerIsNotInitialized) // Owner is not initialized.
        }
    }

    pub fn get_owner(&self) -> Option<Address> {
        contract_env::get_var(KEY_OWNER)
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
