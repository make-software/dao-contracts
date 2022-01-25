use casper_contract::contract_api::runtime;
use casper_types::ApiError;

use crate::{caller, Address, Variable};

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
        self.ensure_owner();
        self.owner.set(Some(owner));
    }

    pub fn ensure_owner(&self) {
        if let Some(owner) = self.owner.get() {
            if owner != caller() {
                runtime::revert(ApiError::User(1000)) // User is not the owner.
            }
        } else {
            runtime::revert(ApiError::User(1001)) // Owner is not inicialized.
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
