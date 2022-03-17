use casper_contract::contract_api::runtime;

use crate::{
    casper_env::{caller, emit},
    consts, Address, Error, Mapping,
};

use self::events::{AddedToWhitelist, RemovedFromWhitelist};

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
        emit(AddedToWhitelist { address });
    }

    pub fn remove_from_whitelist(&mut self, address: Address) {
        self.whitelist.set(&address, false);
        emit(RemovedFromWhitelist { address });
    }

    pub fn ensure_whitelisted(&self) {
        if !self.whitelist.get(&caller()) {
            runtime::revert(Error::NotWhitelisted);
        }
    }
}

pub mod events {
    use crate::Address;
    use casper_dao_macros::Event;

    #[derive(Debug, PartialEq, Event)]
    pub struct AddedToWhitelist {
        pub address: Address,
    }

    #[derive(Debug, PartialEq, Event)]
    pub struct RemovedFromWhitelist {
        pub address: Address,
    }
}
