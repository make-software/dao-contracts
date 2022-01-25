use casper_contract::contract_api::{storage, runtime};
use casper_types::{
    CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, U256, contracts::NamedKeys, ContractPackageHash, runtime_args, RuntimeArgs
};
use contract_utils::{ERC20Token, Address};

pub trait ReputationContractInterface {
    fn init(&mut self, initial_supply: U256);
    fn mint(&mut self, amount: U256);
    fn balance_of(&self, address: Address) -> U256;
    fn total_supply(&self) -> U256;
    fn transfer(&mut self, recipient: Address, amount: U256);
}

#[derive(Default)]
pub struct ReputationContract {
    pub erc20: ERC20Token,
}

impl ReputationContractInterface for ReputationContract {
    fn init(&mut self, initial_supply: U256) {
        self.erc20.init();
        self.erc20.mint(contract_utils::get_immediate_caller_address(), initial_supply);
    }

    fn mint(&mut self, amount: U256) {
        todo!()
    }

    fn total_supply(&self) -> U256 {
        self.erc20.total_supply.get()
    }

    fn balance_of(&self, address: Address) -> U256 {
        self.erc20.balances.get(&address)
    }

    fn transfer(&mut self, recipient: Address, amount: U256) {
        self.erc20.transfer(contract_utils::get_immediate_caller_address(), recipient, amount);
    }
}

impl ReputationContract {
    pub fn install() {
        // Create a new contract package hash for the contract.
        let (contract_package_hash, _) = storage::create_contract_package_at_hash();
        runtime::put_key("reputation_contract_package_hash", contract_package_hash.into());
        storage::add_contract_version(contract_package_hash, ReputationContract::entry_points(), NamedKeys::new());

        // Read arguments for constructor.
        let initial_supply: U256 = runtime::get_named_arg("initial_supply");

        // Call contrustor method.
        let mut contract_instance = ReputationContractCaller::at(contract_package_hash);
        contract_instance.init(initial_supply);

        // Revoke access to init.
        
        // Hash of the installed contract will be reachable through named keys.
        // runtime::put_key(contract_key_name, Key::from(contract_hash));
    }

    pub fn entry_points() -> EntryPoints {
        let mut entry_points = EntryPoints::new();
        entry_points.add_entry_point(EntryPoint::new(
            "mint",
            vec![Parameter::new("amount", U256::cl_type())],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "init",
            vec![Parameter::new("initial_supply", U256::cl_type())],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "transfer",
            vec![
                Parameter::new("recipient", Address::cl_type()),
                Parameter::new("amount", U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));

        entry_points
    }
}

pub struct ReputationContractCaller {
    contract_package_hash: ContractPackageHash
}

impl ReputationContractCaller {
    pub fn at(contract_package_hash: ContractPackageHash) -> Self {
        ReputationContractCaller { contract_package_hash }
    }
}

impl ReputationContractInterface for ReputationContractCaller {
    fn init(&mut self, initial_supply: U256) {
        let _: () = runtime::call_versioned_contract(self.contract_package_hash, None, "init", runtime_args! {
            "initial_supply" => initial_supply
        });
    }

    fn mint(&mut self, amount: U256) {
        todo!()
    }

    fn total_supply(&self) -> U256 {
        runtime::call_versioned_contract(self.contract_package_hash, None, "total_supply", runtime_args! {})
    }
    
    fn balance_of(&self, address: Address) -> U256 {
        runtime::call_versioned_contract(self.contract_package_hash, None, "balance_of", runtime_args! {
            "address" => address
        })
    }

    fn transfer(&mut self, recipient: Address, amount: U256) {
        runtime::call_versioned_contract(self.contract_package_hash, None, "transfer", runtime_args! {
            "recipient" => recipient,
            "amount" => amount
        })
    }
}

#[cfg(feature = "test-support")]
mod tests {
    use casper_types::{runtime_args, ContractPackageHash, RuntimeArgs, U256};
    use contract_utils::Address;
    use test_utils::TestEnv;

    use crate::{ReputationContractInterface, ReputationContract};

    pub struct ReputationContractTest {
        env: TestEnv,
        package_hash: ContractPackageHash,
        data: ReputationContract
    }

    impl ReputationContractTest {
        pub fn new(env: &TestEnv, initial_supply: U256) -> ReputationContractTest {
            env.deploy_wasm_file("reputation_contract.wasm", runtime_args! {
                "initial_supply" => initial_supply
            });
            let package_hash = env.get_contract_package_hash("reputation_contract_package_hash");
            ReputationContractTest {
                env: env.clone(),
                package_hash,
                data: ReputationContract::default()
            }
        }
    }

    impl ReputationContractInterface for ReputationContractTest {
        fn mint(&mut self, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                "mint",
                runtime_args! {
                    "amount" => amount
                },
            )
        }

        fn init(&mut self, initial_supply: U256) {
            todo!()
        }

        fn total_supply(&self) -> U256 {
            self.env.get_value(self.package_hash, self.data.erc20.total_supply.path())
        }
        
        fn balance_of(&self, address: Address) -> U256 {
            self.env.get_dict_value(self.package_hash, self.data.erc20.balances.path(), address)
        }

        fn transfer(&mut self, recipient: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                "transfer",
                runtime_args! {
                    "recipient" => recipient,
                    "amount" => amount
                },
            )
        }
    }
}

#[cfg(feature = "test-support")]
pub use tests::ReputationContractTest;
