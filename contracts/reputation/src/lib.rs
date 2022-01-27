use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLTyped, ContractPackageHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, RuntimeArgs, U256,
};
use utils::{owner::Owner, token::Token, whitelist::Whitelist, Address};

pub trait ReputationContractInterface {
    fn init(&mut self);
    fn mint(&mut self, recipient: Address, amount: U256);
    fn burn(&mut self, owner: Address, amount: U256);
    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256);
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
}

#[derive(Default)]
pub struct ReputationContract {
    pub token: Token,
    pub owner: Owner,
    pub whitelist: Whitelist,
}

impl ReputationContractInterface for ReputationContract {
    fn init(&mut self) {
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
        self.owner.change_ownership(owner);
    }

    fn add_to_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.add_to_whitelist(address);
    }

    fn remove_from_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.remove_from_whitelist(address);
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
        storage::add_contract_version(
            contract_package_hash,
            ReputationContract::entry_points(),
            NamedKeys::new(),
        );

        // Call contrustor method.
        let mut contract_instance = ReputationContractCaller::at(contract_package_hash);
        contract_instance.init();

        // Revoke access to init.
    }

    pub fn entry_points() -> EntryPoints {
        let mut entry_points = EntryPoints::new();
        entry_points.add_entry_point(EntryPoint::new(
            "init",
            vec![],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));

        entry_points.add_entry_point(utils::owner::entry_points::change_ownership());
        entry_points.add_entry_point(utils::whitelist::entry_points::add_to_whitelist());
        entry_points.add_entry_point(utils::whitelist::entry_points::remove_from_whitelist());
        entry_points.add_entry_point(utils::token::entry_points::mint());
        entry_points.add_entry_point(utils::token::entry_points::burn());
        entry_points.add_entry_point(utils::token::entry_points::transfer_from());

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
            "init",
            runtime_args! {},
        );
    }

    fn mint(&mut self, recipient: Address, amount: U256) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            "mint",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount
            },
        )
    }

    fn burn(&mut self, owner: Address, amount: U256) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            "burn",
            runtime_args! {
                "owner" => owner,
                "amount" => amount
            },
        )
    }

    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            "transfer_from",
            runtime_args! {
                "owner" => owner,
                "recipient" => recipient,
                "amount" => amount
            },
        )
    }

    fn change_ownership(&mut self, owner: Address) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            "change_ownership",
            runtime_args! {
                "owner" => owner,
            },
        )
    }

    fn add_to_whitelist(&mut self, address: Address) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            "add_to_whitelist",
            runtime_args! {
                "address" => address,
            },
        )
    }

    fn remove_from_whitelist(&mut self, address: Address) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            "remove_from_whitelist",
            runtime_args! {
                "address" => address,
            },
        )
    }
}

#[cfg(feature = "test-support")]
mod tests {
    use casper_types::{runtime_args, ContractPackageHash, RuntimeArgs, U256};
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
                .get_value(self.package_hash, self.data.token.total_supply.path())
        }

        pub fn balance_of(&self, address: Address) -> U256 {
            self.env
                .get_dict_value(self.package_hash, self.data.token.balances.path(), address)
        }

        pub fn is_whitelisted(&self, address: Address) -> bool {
            self.env.get_dict_value(
                self.package_hash,
                self.data.whitelist.whitelist.path(),
                address,
            )
        }
    }

    impl ReputationContractInterface for ReputationContractTest {
        fn init(&mut self) {
            todo!()
        }

        fn mint(&mut self, recipient: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                "mint",
                runtime_args! {
                    "recipient" => recipient,
                    "amount" => amount
                },
            )
        }

        fn burn(&mut self, owner: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                "burn",
                runtime_args! {
                    "owner" => owner,
                    "amount" => amount
                },
            )
        }

        fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                "transfer_from",
                runtime_args! {
                    "owner" => owner,
                    "recipient" => recipient,
                    "amount" => amount
                },
            )
        }

        fn change_ownership(&mut self, owner: Address) {
            self.env.call_contract_package(
                self.package_hash,
                "change_ownership",
                runtime_args! {
                    "owner" => owner
                },
            )
        }

        fn add_to_whitelist(&mut self, address: Address) {
            self.env.call_contract_package(
                self.package_hash,
                "add_to_whitelist",
                runtime_args! {
                    "address" => address
                },
            )
        }

        fn remove_from_whitelist(&mut self, address: Address) {
            self.env.call_contract_package(
                self.package_hash,
                "remove_from_whitelist",
                runtime_args! {
                    "address" => address
                },
            )
        }
    }
}

#[cfg(feature = "test-support")]
pub use tests::ReputationContractTest;
