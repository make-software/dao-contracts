use casper_contract::contract_api::runtime;

use crate::{consts, list::List, Error, Mapping};
use casper_types::bytesrepr::Bytes;

pub struct Repository {
    pub storage: Mapping<String, Option<Bytes>>,
    pub keys: List<String>,
}

impl Default for Repository {
    fn default() -> Self {
        Self {
            storage: Mapping::from(consts::NAME_STORAGE),
            keys: List::new(consts::NAME_KEYS),
        }
    }
}

impl Repository {
    pub fn init(&mut self) {
        self.storage.init();
        self.keys.init();
    }

    pub fn set_or_update(&mut self, key: String, value: Bytes) {
        self.storage.set(&key, Some(value));
        self.keys.add_or_update(key);
    }

    pub fn get(&self, key: String) -> Bytes {
        match self.storage.get(&key) {
            Some(value) => value,
            None => runtime::revert(Error::ValueNotAvailable),
        }
    }

    pub fn delete(&mut self, key: String) {
        let deletion_success = self.keys.delete(key.clone());

        if deletion_success {
            self.storage.set(&key, None);
        } else {
            runtime::revert(Error::ValueNotAvailable);
        }
    }

    pub fn get_key_at(&self, index: u32) -> String {
        self.keys.get(index)
    }
}

pub mod entry_points {
    use casper_types::{
        bytesrepr::Bytes, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, Parameter,
    };

    use crate::consts;

    pub fn set_or_update() -> EntryPoint {
        EntryPoint::new(
            consts::EP_SET_OR_UPDATE,
            vec![
                Parameter::new(consts::PARAM_KEY, String::cl_type()),
                Parameter::new(consts::PARAM_VALUE, Bytes::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }

    pub fn get() -> EntryPoint {
        EntryPoint::new(
            consts::EP_GET,
            vec![Parameter::new(consts::PARAM_KEY, String::cl_type())],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }

    pub fn delete() -> EntryPoint {
        EntryPoint::new(
            consts::EP_DELETE,
            vec![Parameter::new(consts::PARAM_KEY, String::cl_type())],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }
}
