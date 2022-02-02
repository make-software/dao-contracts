use std::collections::BTreeSet;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLTyped, ContractPackageHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Group, RuntimeArgs, URef, U256,
};
use utils::{consts, owner::Owner, staking::TokenWithStaking, whitelist::Whitelist, Address};

pub trait ReputationContractInterface {
    fn init(&mut self);
    fn mint(&mut self, recipient: Address, amount: U256);
    fn burn(&mut self, owner: Address, amount: U256);
    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256);
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn stake(&mut self, address: Address, amount: U256);
    fn unstake(&mut self, address: Address, amount: U256);
}

#[derive(Default)]
pub struct ReputationContract {
    pub token: TokenWithStaking,
    pub owner: Owner,
    pub whitelist: Whitelist,
}

impl ReputationContractInterface for ReputationContract {
    fn init(&mut self) {
        utils::init_events();
        let deployer = utils::caller();
        self.owner.init(deployer);
        self.whitelist.init();
        self.whitelist.add_to_whitelist(deployer);
        self.token.init();
    }

    fn mint(&mut self, recipient: Address, amount: U256) {
        self.whitelist.ensure_whitelisted();
        self.token.mint(recipient, amount);
    }

    fn burn(&mut self, owner: Address, amount: U256) {
        self.whitelist.ensure_whitelisted();
        self.token.burn(owner, amount);
    }

    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256) {
        self.whitelist.ensure_whitelisted();
        self.token.raw_transfer(owner, recipient, amount);
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

    fn stake(&mut self, address: Address, amount: U256) {
        self.whitelist.ensure_whitelisted();
        self.token.stake(address, amount);
    }

    fn unstake(&mut self, address: Address, amount: U256) {
        self.whitelist.ensure_whitelisted();
        self.token.unstake(address, amount);
    }
}

impl ReputationContract {
    pub fn install() {
        // Create a new contract package hash for the contract.
        let (contract_package_hash, _) = storage::create_contract_package_at_hash();
        runtime::put_key(
            "reputation_contract_package_hash",
            contract_package_hash.into(),
        );

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
            ReputationContract::entry_points(),
            NamedKeys::new(),
        );

        // Call contrustor method.
        let mut contract_instance = ReputationContractCaller::at(contract_package_hash);
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
        entry_points.add_entry_point(utils::staking::entry_points::mint());
        entry_points.add_entry_point(utils::staking::entry_points::burn());
        entry_points.add_entry_point(utils::staking::entry_points::transfer_from());
        entry_points.add_entry_point(utils::staking::entry_points::stake());
        entry_points.add_entry_point(utils::staking::entry_points::unstake());

        entry_points
    }
}

pub struct ReputationContractCaller {
    contract_package_hash: ContractPackageHash,
}

impl ReputationContractCaller {
    pub fn at(contract_package_hash: ContractPackageHash) -> Self {
        ReputationContractCaller {
            contract_package_hash,
        }
    }
}

impl ReputationContractInterface for ReputationContractCaller {
    fn init(&mut self) {
        let _: () = runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_INIT,
            runtime_args! {},
        );
    }

    fn mint(&mut self, recipient: Address, amount: U256) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_MINT,
            runtime_args! {
                consts::PARAM_RECIPIENT => recipient,
                consts::PARAM_AMOUNT => amount
            },
        )
    }

    fn burn(&mut self, owner: Address, amount: U256) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_BURN,
            runtime_args! {
                consts::PARAM_OWNER => owner,
                consts::PARAM_AMOUNT => amount
            },
        )
    }

    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_TRANSFER_FROM,
            runtime_args! {
                consts::PARAM_OWNER => owner,
                consts::PARAM_RECIPIENT => recipient,
                consts::PARAM_AMOUNT => amount
            },
        )
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

    fn stake(&mut self, address: Address, amount: U256) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_STAKE,
            runtime_args! {
                consts::PARAM_ADDRESS => address,
                consts::PARAM_AMOUNT => amount
            },
        )
    }

    fn unstake(&mut self, address: Address, amount: U256) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_UNSTAKE,
            runtime_args! {
                consts::PARAM_ADDRESS => address,
                consts::PARAM_AMOUNT => amount
            },
        )
    }
}

#[cfg(feature = "test-support")]
mod tests {
    use casper_types::bytesrepr::{Bytes, FromBytes};
    use casper_types::{runtime_args, ContractPackageHash, RuntimeArgs, U256};
    use utils::consts;
    use utils::Address;
    use utils::TestEnv;

    use crate::{ReputationContract, ReputationContractInterface};

    pub struct ReputationContractTest {
        env: TestEnv,
        package_hash: ContractPackageHash,
        data: ReputationContract,
    }

    impl ReputationContractTest {
        pub fn new(env: &TestEnv) -> ReputationContractTest {
            env.deploy_wasm_file("reputation_contract.wasm", runtime_args! {});
            let package_hash = env.get_contract_package_hash("reputation_contract_package_hash");
            ReputationContractTest {
                env: env.clone(),
                package_hash,
                data: ReputationContract::default(),
            }
        }

        pub fn as_account(&mut self, account: Address) -> &mut Self {
            self.env.as_account(account);
            self
        }

        pub fn get_owner(&self) -> Option<Address> {
            self.env
                .get_value(self.package_hash, self.data.owner.owner.path())
        }

        pub fn total_supply(&self) -> U256 {
            self.env
                .get_value(self.package_hash, self.data.token.token.total_supply.path())
        }

        pub fn balance_of(&self, address: Address) -> U256 {
            self.env.get_dict_value(
                self.package_hash,
                self.data.token.token.balances.path(),
                address,
            )
        }

        pub fn is_whitelisted(&self, address: Address) -> bool {
            self.env.get_dict_value(
                self.package_hash,
                self.data.whitelist.whitelist.path(),
                address,
            )
        }

        pub fn get_staked_balance_of(&self, address: Address) -> U256 {
            self.env
                .get_dict_value(self.package_hash, self.data.token.stakes.path(), address)
        }

        pub fn event<T: FromBytes>(&self, index: u32) -> T {
            let raw_event: Bytes = self.env.get_dict_value(self.package_hash, "events", index);
            let (event, bytes) = T::from_bytes(&raw_event).unwrap();
            assert!(bytes.is_empty());
            event
        }
    }

    impl ReputationContractInterface for ReputationContractTest {
        fn init(&mut self) {
            self.env
                .call_contract_package(self.package_hash, "init", runtime_args! {})
        }

        fn mint(&mut self, recipient: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_MINT,
                runtime_args! {
                    consts::PARAM_RECIPIENT => recipient,
                    consts::PARAM_AMOUNT => amount
                },
            )
        }

        fn burn(&mut self, owner: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_BURN,
                runtime_args! {
                    consts::PARAM_OWNER => owner,
                    consts::PARAM_AMOUNT => amount
                },
            )
        }

        fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_TRANSFER_FROM,
                runtime_args! {
                    consts::PARAM_OWNER => owner,
                    consts::PARAM_RECIPIENT => recipient,
                    consts::PARAM_AMOUNT => amount
                },
            )
        }

        fn change_ownership(&mut self, owner: Address) {
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

        fn stake(&mut self, address: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_STAKE,
                runtime_args! {
                    consts::PARAM_ADDRESS => address,
                    consts::PARAM_AMOUNT => amount
                },
            )
        }

        fn unstake(&mut self, address: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_UNSTAKE,
                runtime_args! {
                    consts::PARAM_ADDRESS => address,
                    consts::PARAM_AMOUNT => amount
                },
            )
        }
    }
}

#[cfg(feature = "test-support")]
pub use tests::ReputationContractTest;
