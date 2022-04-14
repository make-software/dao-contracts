use casper_dao_modules::{AccessControl, Record, Repository};
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{caller, revert},
    Address, Error,
};
use casper_types::bytesrepr::{Bytes, FromBytes};
use delegate::delegate;

#[casper_contract_interface]
pub trait VariableRepositoryContractInterface {
    fn init(&mut self);
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>);
    fn get(&self, key: String) -> Option<Bytes>;
    fn get_full_value(&self, key: String) -> Option<Record>;
    fn get_key_at(&self, index: u32) -> Option<String>;
    fn keys_count(&self) -> u32;
    fn get_owner(&self) -> Option<Address>;
    fn is_whitelisted(&self, address: Address) -> bool;
}

#[derive(Instance)]
pub struct VariableRepositoryContract {
    pub access_control: AccessControl,
    pub repository: Repository,
}

impl VariableRepositoryContractInterface for VariableRepositoryContract {
    delegate! {
        to self.access_control {
            fn is_whitelisted(&self, address: Address) -> bool;
            fn get_owner(&self) -> Option<Address>;
            fn change_ownership(&mut self, owner: Address);
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
        }
    }

    fn init(&mut self) {
        let deployer = caller();
        self.access_control.init(deployer);
        self.repository.init();
    }

    fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>) {
        self.access_control.ensure_whitelisted();
        self.repository.update_at(key, value, activation_time);
    }

    fn get(&self, key: String) -> Option<Bytes> {
        self.repository.get(key)
    }

    fn get_full_value(&self, key: String) -> Option<Record> {
        self.repository.get_full_value(key)
    }

    fn get_key_at(&self, index: u32) -> Option<String> {
        self.repository.keys.get(index)
    }

    fn keys_count(&self) -> u32 {
        self.repository.keys.size()
    }
}

impl VariableRepositoryContractCaller {
    pub fn get_variable<T: FromBytes>(&self, key: &str) -> T {
        let variable = self.get(key.into()).unwrap_or_revert();
        let (variable, bytes) = <T>::from_bytes(&variable).unwrap_or_revert();
        if !bytes.is_empty() {
            revert(Error::ValueNotAvailable)
        }
        variable
    }
}
