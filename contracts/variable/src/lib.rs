use std::collections::BTreeSet;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::Bytes, contracts::NamedKeys, runtime_args, CLTyped, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, RuntimeArgs, URef,
};

use utils::{consts, owner::Owner, repository::Repository, whitelist::Whitelist, Address};

const PACKAGE_HASH_KEY: &str = "variable_repository_package_hash";

pub trait VariableRepositoryContractInterface {
    fn init(&mut self);
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn set_or_update(&mut self, key: String, value: Bytes);
    fn get(&mut self, key: String) -> Bytes;
    fn delete(&mut self, key: String);
}

#[derive(Default)]
pub struct VariableRepositoryContract {
    pub owner: Owner,
    pub whitelist: Whitelist,
    pub repository: Repository,
}

impl VariableRepositoryContractInterface for VariableRepositoryContract {
    fn init(&mut self) {
        utils::init_events();
        let deployer = utils::caller();
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
        self.repository.set_or_update(key, value);
    }

    fn get(&mut self, key: String) -> Bytes {
        self.repository.get(key)
    }

    fn delete(&mut self, key: String) {
        self.repository.delete(key);
    }
}

impl VariableRepositoryContract {
    pub fn install() {
        // Create a new contract package hash for the contract.
        let (contract_package_hash, _) = storage::create_contract_package_at_hash();
        runtime::put_key(PACKAGE_HASH_KEY, contract_package_hash.into());

        let init_access: URef = storage::create_contract_user_group(
            contract_package_hash,
            "init",
            1,
            Default::default(),
        )
        .unwrap_or_revert()
        .pop()
        .unwrap_or_revert();

        storage::add_contract_version(
            contract_package_hash,
            VariableRepositoryContract::entry_points(),
            NamedKeys::new(),
        );

        // Call contrustor method.
        let mut contract_instance = VariableRepositoryContractCaller::at(contract_package_hash);
        contract_instance.init();

        // Revoke access to init.
        let mut urefs = BTreeSet::new();
        urefs.insert(init_access);
        storage::remove_contract_user_group_urefs(contract_package_hash, "init", urefs)
            .unwrap_or_revert();
    }

    pub fn entry_points() -> EntryPoints {
        let mut entry_points = EntryPoints::new();
        entry_points.add_entry_point(EntryPoint::new(
            "init",
            vec![],
            <()>::cl_type(),
            EntryPointAccess::Groups(vec![Group::new("init")]),
            EntryPointType::Contract,
        ));

        entry_points.add_entry_point(utils::owner::entry_points::change_ownership());
        entry_points.add_entry_point(utils::whitelist::entry_points::add_to_whitelist());
        entry_points.add_entry_point(utils::whitelist::entry_points::remove_from_whitelist());
        entry_points.add_entry_point(utils::repository::entry_points::set_or_update());
        entry_points.add_entry_point(utils::repository::entry_points::get());
        entry_points.add_entry_point(utils::repository::entry_points::delete());
        entry_points
    }
}

pub struct VariableRepositoryContractCaller {
    contract_package_hash: ContractPackageHash,
}

impl VariableRepositoryContractCaller {
    pub fn at(contract_package_hash: ContractPackageHash) -> Self {
        VariableRepositoryContractCaller {
            contract_package_hash,
        }
    }
}

impl VariableRepositoryContractInterface for VariableRepositoryContractCaller {
    fn init(&mut self) {
        let _: () = runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_INIT,
            runtime_args! {},
        );
    }

    fn change_ownership(&mut self, owner: Address) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_CHANGE_OWNERSHIP,
            runtime_args! {
                consts::PARAM_OWNER => owner,
            },
        )
    }

    fn add_to_whitelist(&mut self, address: Address) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_ADD_TO_WHITELIST,
            runtime_args! {
                consts::PARAM_ADDRESS => address,
            },
        )
    }

    fn remove_from_whitelist(&mut self, address: Address) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_REMOVE_FROM_WHITELIST,
            runtime_args! {
                consts::PARAM_ADDRESS => address,
            },
        )
    }

    fn set_or_update(&mut self, key: String, value: Bytes) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_SET_OR_UPDATE,
            runtime_args! {
                consts::PARAM_KEY => key,
                consts::PARAM_VALUE => value,
            },
        )
    }

    fn get(&mut self, key: String) -> Bytes {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_GET,
            runtime_args! {
                consts::PARAM_KEY => key,
            },
        )
    }

    fn delete(&mut self, key: String) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_DELETE,
            runtime_args! {
                consts::PARAM_KEY => key,
            },
        )
    }
}

#[cfg(feature = "test-support")]
mod tests {
    use casper_types::{bytesrepr::Bytes, runtime_args, ContractPackageHash, RuntimeArgs};
    use utils::{consts, Address, TestEnv};

    use crate::{
        VariableRepositoryContract, VariableRepositoryContractInterface, PACKAGE_HASH_KEY,
    };

    const WASM_FILE_NAME: &str = "variable_repository.wasm";

    pub struct VariableRepositoryContractTest {
        env: TestEnv,
        package_hash: ContractPackageHash,
        data: VariableRepositoryContract,
    }

    impl VariableRepositoryContractTest {
        pub fn new(env: &TestEnv) -> VariableRepositoryContractTest {
            env.deploy_wasm_file(WASM_FILE_NAME, runtime_args! {});
            let package_hash = env.get_contract_package_hash(PACKAGE_HASH_KEY);
            VariableRepositoryContractTest {
                env: env.clone(),
                package_hash,
                data: VariableRepositoryContract::default(),
            }
        }

        pub fn is_whitelisted(&self, address: Address) -> bool {
            self.env.get_dict_value(
                self.package_hash,
                self.data.whitelist.whitelist.path(),
                address,
            )
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
    }

    impl VariableRepositoryContractInterface for VariableRepositoryContractTest {
        fn init(&mut self) {
            todo!()
        }

        fn change_ownership(&mut self, owner: utils::Address) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_CHANGE_OWNERSHIP,
                runtime_args! {
                    consts::PARAM_OWNER => owner
                },
            )
        }

        fn add_to_whitelist(&mut self, address: Address) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_ADD_TO_WHITELIST,
                runtime_args! {
                    consts::PARAM_ADDRESS => address
                },
            )
        }

        fn remove_from_whitelist(&mut self, address: Address) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_REMOVE_FROM_WHITELIST,
                runtime_args! {
                    consts::PARAM_ADDRESS => address
                },
            )
        }

        fn set_or_update(&mut self, key: String, value: Bytes) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_SET_OR_UPDATE,
                runtime_args! {
                    consts::PARAM_KEY => key,
                    consts::PARAM_VALUE => value,
                },
            )
        }

        fn get(&mut self, key: String) -> Bytes {
            self.env
                .get_dict_value(self.package_hash, self.data.repository.storage.path(), key)
        }

        fn delete(&mut self, key: String) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_DELETE,
                runtime_args! {
                    consts::PARAM_KEY => key
                },
            );
        }
    }
}

#[cfg(feature = "test-support")]
pub use tests::VariableRepositoryContractTest;
