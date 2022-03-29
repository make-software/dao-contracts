use casper_dao_modules::{Owner, Repository, Whitelist};
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

    fn is_whitelisted(&self, address: Address) -> bool {
        self.whitelist.is_whitelisted(&address)
    }
}

#[cfg(feature = "test-support")]
use casper_dao_utils::BytesConversion;

#[cfg(feature = "test-support")]
use casper_dao_modules::RepositoryDefaults;

#[cfg(feature = "test-support")]
impl VariableRepositoryContractTest {
    pub fn get_owner(&self) -> Option<Address> {
        self.env
            .get_value(self.package_hash, self.data.owner.owner.path())
    }

    pub fn get_value<K: ToString, V: BytesConversion>(&self, key: K) -> V {
        let (current, future) = self.get_full_value(key);
        assert!(future.is_none());
        current
    }

    pub fn get_full_value<K: ToString, V: BytesConversion>(&self, key: K) -> (V, Option<(V, u64)>) {
        let result: (Bytes, Option<(Bytes, u64)>) = self.env.get_dict_value(
            self.package_hash,
            self.data.repository.storage.path(),
            key.to_string(),
        );
        let current: V = V::convert_from_bytes(result.0);
        let future = result
            .1
            .map(|(future, time)| (V::convert_from_bytes(future), time));
        (current, future)
    }

    pub fn get_key_at(&self, index: u32) -> Option<String> {
        self.env.get_dict_value(
            self.package_hash,
            self.data.repository.keys.values.path(),
            index,
        )
    }

    pub fn get_keys_length(&self) -> u32 {
        self.env
            .get_value(self.package_hash, self.data.repository.keys.length.path())
    }

    pub fn get_non_default_key_at(&self, index: u32) -> Option<String> {
        self.env.get_dict_value(
            self.package_hash,
            self.data.repository.keys.values.path(),
            RepositoryDefaults::len() + index,
        )
    }

    pub fn get_non_default_keys_length(&self) -> u32 {
        let count: u32 = self
            .env
            .get_value(self.package_hash, self.data.repository.keys.length.path());
        count - RepositoryDefaults::len()
    }
}
