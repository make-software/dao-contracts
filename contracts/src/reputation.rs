use casper_contract::contract_api::runtime::{self};
use casper_dao_utils::{
    casper_env::{caller, init_events, install_contract},
    consts,
    owner::Owner,
    staking::TokenWithStaking,
    whitelist::Whitelist,
    Address,
};
use casper_types::{
    runtime_args, CLTyped, ContractPackageHash, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Group, RuntimeArgs, U256,
};

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
        init_events();
        let deployer = caller();
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
        install_contract(
            "reputation_contract_package_hash",
            ReputationContract::entry_points(),
            |contract_package_hash| {
                let mut contract_instance = ReputationContractCaller::at(contract_package_hash);
                contract_instance.init()
            },
        );
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

        entry_points.add_entry_point(casper_dao_utils::owner::entry_points::change_ownership());
        entry_points.add_entry_point(casper_dao_utils::whitelist::entry_points::add_to_whitelist());
        entry_points
            .add_entry_point(casper_dao_utils::whitelist::entry_points::remove_from_whitelist());
        entry_points.add_entry_point(casper_dao_utils::staking::entry_points::mint());
        entry_points.add_entry_point(casper_dao_utils::staking::entry_points::burn());
        entry_points.add_entry_point(casper_dao_utils::staking::entry_points::transfer_from());
        entry_points.add_entry_point(casper_dao_utils::staking::entry_points::stake());
        entry_points.add_entry_point(casper_dao_utils::staking::entry_points::unstake());

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
    use std::fmt::Debug;

    use casper_dao_utils::{consts, Address, TestEnv};
    use casper_types::bytesrepr::{Bytes, FromBytes};
    use casper_types::{runtime_args, ContractPackageHash, RuntimeArgs, U256};

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
            let raw_event: Option<Bytes> =
                self.env
                    .get_dict_value(self.package_hash, consts::NAME_EVENTS, index);
            let raw_event = raw_event.unwrap();
            let (event, bytes) = T::from_bytes(&raw_event).unwrap();
            assert!(bytes.is_empty());
            event
        }

        pub fn assert_event_at<T: FromBytes + PartialEq + Debug>(&self, index: u32, event: T) {
            assert_eq!(self.event::<T>(index), event);
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
