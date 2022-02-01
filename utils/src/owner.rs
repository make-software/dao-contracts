use casper_contract::contract_api::runtime;

use crate::{caller, Address, Error, Variable};

pub struct Owner {
    pub owner: Variable<Option<Address>>,
}

impl Default for Owner {
    fn default() -> Self {
        Self {
            owner: Variable::new(format!("owner")),
        }
    }
}

impl Owner {
    pub fn init(&mut self, owner: Address) {
        self.owner.set(Some(owner));
    }

    pub fn change_ownership(&mut self, owner: Address) {
        self.owner.set(Some(owner));
    }

    pub fn ensure_owner(&self) {
        if let Some(owner) = self.owner.get() {
            if owner != caller() {
                runtime::revert(Error::NotAnOwner) // User is not the owner.
            }
        } else {
            runtime::revert(Error::OwnerIsNotInitialized) // Owner is not inicialized.
        }
    }
}

pub mod entry_points {
    use casper_types::{CLTyped, EntryPoint, EntryPointAccess, EntryPointType, Parameter};

    use crate::Address;

    pub fn change_ownership() -> EntryPoint {
        EntryPoint::new(
            "change_ownership",
            vec![Parameter::new("owner", Address::cl_type())],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }
}
