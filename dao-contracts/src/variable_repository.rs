use casper_dao_modules::{Owner, Record, Repository, Whitelist};
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
};
use casper_types::bytesrepr::Bytes;

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
    pub owner: Owner,
    pub whitelist: Whitelist,
    pub repository: Repository,
}

impl VariableRepositoryContractInterface for VariableRepositoryContract {
    fn init(&mut self) {
        let deployer = caller();
        self.owner.init(deployer);
        self.whitelist.add_to_whitelist(deployer);
        self.repository.init();
    }

    fn change_ownership(&mut self, owner: Address) {
        self.owner.ensure_owner();
        self.owner.change_ownership(owner);
        self.whitelist.add_to_whitelist(owner);
    }

    fn add_to_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.add_to_whitelist(address);
    }

    fn remove_from_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.remove_from_whitelist(address);
    }

    fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>) {
        self.whitelist.ensure_whitelisted();
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

    fn get_owner(&self) -> Option<Address> {
        self.owner.get_owner()
    }

    fn is_whitelisted(&self, address: Address) -> bool {
        self.whitelist.is_whitelisted(&address)
    }
}
