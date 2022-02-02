use casper_contract::contract_api::runtime;

use crate::{caller, Address, Error, Mapping};

pub struct Whitelist {
    pub whitelist: Mapping<Address, bool>,
}

impl Default for Whitelist {
    fn default() -> Self {
        Self {
            whitelist: Mapping::new(String::from("whitelist")),
        }
    }
}

impl Whitelist {
    pub fn init(&mut self) {
        self.whitelist.init();
    }

    pub fn add_to_whitelist(&mut self, address: Address) {
        self.whitelist.set(&address, true);
    }

    pub fn remove_from_whitelist(&mut self, address: Address) {
        self.whitelist.set(&address, false);
    }

    pub fn ensure_whitelisted(&self) {
        if !self.whitelist.get(&caller()) {
            runtime::revert(Error::NotWhitelisted);
        }
    }
}

pub mod entry_points {
    use casper_types::{CLTyped, EntryPoint, EntryPointAccess, EntryPointType, Parameter};

    use crate::Address;

    pub fn add_to_whitelist() -> EntryPoint {
        EntryPoint::new(
            "add_to_whitelist",
            vec![Parameter::new("address", Address::cl_type())],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }

    pub fn remove_from_whitelist() -> EntryPoint {
        EntryPoint::new(
            "remove_from_whitelist",
            vec![Parameter::new("address", Address::cl_type())],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }
}
