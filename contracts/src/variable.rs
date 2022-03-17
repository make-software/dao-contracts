use casper_types::bytesrepr::Bytes;

use casper_dao_utils::{
    casper_dao_macros::casper_contract_interface,
    casper_env::{caller, init_events},
    owner::Owner,
    repository::{Repository, RepositoryDefaults},
    whitelist::Whitelist,
    Address,
};

#[casper_contract_interface]
pub trait VariableRepositoryContractInterface {
    fn init(&mut self);
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn set_or_update(&mut self, key: String, value: Bytes);
    fn get(&mut self, key: String) -> Bytes;
}

#[derive(Default)]
pub struct VariableRepositoryContract {
    pub owner: Owner,
    pub whitelist: Whitelist,
    pub repository: Repository,
}

impl VariableRepositoryContractInterface for VariableRepositoryContract {
    fn init(&mut self) {
        init_events();
        let deployer = caller();
        self.owner.init(deployer);
        self.whitelist.init();
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

    fn set_or_update(&mut self, key: String, value: Bytes) {
        self.whitelist.ensure_whitelisted();
        self.repository.set_or_update(key, value);
    }

    fn get(&mut self, key: String) -> Bytes {
        self.repository.get(key)
    }
}

#[cfg(feature = "test-support")]
impl VariableRepositoryContractTest {
    pub fn is_whitelisted(&self, address: Address) -> bool {
        self.env.get_dict_value(
            self.package_hash,
            self.data.whitelist.whitelist.path(),
            address,
        )
    }

    pub fn get_owner(&self) -> Option<Address> {
        self.env
            .get_value(self.package_hash, self.data.owner.owner.path())
    }

    pub fn get_value(&self, key: String) -> Bytes {
        let result: Option<Bytes> =
            self.env
                .get_dict_value(self.package_hash, self.data.repository.storage.path(), key);
        result.unwrap()
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
