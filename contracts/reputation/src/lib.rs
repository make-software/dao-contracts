use casper_contract::contract_api::{storage, runtime};
use casper_types::{
    CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, U256, contracts::NamedKeys, ContractPackageHash, runtime_args, RuntimeArgs
};

pub trait ReputationContractInterface {
    fn init(&mut self, initial_supply: U256);
    fn mint(&mut self, amount: U256);
}

pub struct ReputationContract {}

impl ReputationContract {
    pub fn install() {
        // Create a new contract package hash for the contract.
        let (contract_package_hash, _) = storage::create_contract_package_at_hash();
        runtime::put_key("reputation_contract_package_hash", contract_package_hash.into());
        storage::add_contract_version(contract_package_hash, ReputationContract::entry_points(), NamedKeys::new());

        // Read arguments for constructor.
        let initial_supply: U256 = runtime::get_named_arg("initial_supply");

        // Call contrustor method.
        let mut contract_instance = ReputationContractInstance::at(contract_package_hash);
        contract_instance.init(initial_supply);

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
        entry_points
    }
}

pub struct ReputationContractInstance {
    contract_package_hash: ContractPackageHash
}

impl ReputationContractInstance {
    pub fn at(contract_package_hash: ContractPackageHash) -> Self {
        ReputationContractInstance { contract_package_hash }
    }
}

impl ReputationContractInterface for ReputationContractInstance {
    fn init(&mut self, initial_supply: U256) {
        let _: () = runtime::call_versioned_contract(self.contract_package_hash, None, "init", runtime_args! {
            "initial_supply" => initial_supply
        });
    }

    fn mint(&mut self, amount: U256) {
        todo!()
    }
}

#[cfg(feature = "test-support")]
mod tests {
    use casper_types::{runtime_args, ContractPackageHash, RuntimeArgs, U256};
    use tests_utils::TestEnv;

    use crate::ReputationContractInterface;

    pub struct ReputationContractTest {
        env: TestEnv,
        package_hash: ContractPackageHash,
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
    }
}

#[cfg(feature = "test-support")]
pub use tests::ReputationContractTest;
