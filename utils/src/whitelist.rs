use casper_contract::contract_api::runtime;

use crate::{caller, consts, Address, Error, Mapping};

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

    use crate::{consts, Address};

    pub fn add_to_whitelist() -> EntryPoint {
        EntryPoint::new(
            consts::EP_ADD_TO_WHITELIST,
            vec![Parameter::new(consts::PARAM_ADDRESS, Address::cl_type())],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }

    pub fn remove_from_whitelist() -> EntryPoint {
        EntryPoint::new(
            consts::EP_REMOVE_FROM_WHITELIST,
            vec![Parameter::new(consts::PARAM_ADDRESS, Address::cl_type())],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }
}
