#[cfg(feature = "test-support")]
mod tests {
    use casper_types::{runtime_args, ContractPackageHash, RuntimeArgs};
    use utils::TestEnv;

    pub struct VariableRepositoryContractTest {
        env: TestEnv,
        package_hash: ContractPackageHash,
    }

    impl VariableRepositoryContractTest {
        pub fn new(env: &TestEnv) -> VariableRepositoryContractTest {
            env.deploy_wasm_file("variable_repository.wasm", runtime_args! {});
            let package_hash = env.get_contract_package_hash("variable_repository_package_hash");
            VariableRepositoryContractTest {
                env: env.clone(),
                package_hash,
            }
        }
    }
}

#[cfg(feature = "test-support")]
pub use tests::VariableRepositoryContractTest;
