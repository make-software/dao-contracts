use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use casper_engine_test_support::{
    DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT,
    DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_GENESIS_CONFIG, DEFAULT_GENESIS_CONFIG_HASH,
    DEFAULT_PAYMENT,
};
use casper_execution_engine::core::engine_state::{
    run_genesis_request::RunGenesisRequest, GenesisAccount,
};
use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, Motes, PublicKey, RuntimeArgs,
    SecretKey, U512,
};

#[derive(Clone)]
pub struct TestEnv {
    state: Arc<Mutex<TestEnvState>>,
}

impl TestEnv {
    pub fn new() -> TestEnv {
        TestEnv {
            state: Arc::new(Mutex::new(TestEnvState::new())),
        }
    }

    pub fn deploy_wasm_file(&self, session_code: &str, session_args: RuntimeArgs) {
        self.state
            .lock()
            .unwrap()
            .deploy_wasm_file(session_code, session_args);
    }

    pub fn call_contract_package(
        &mut self,
        hash: ContractPackageHash,
        entry_point: &str,
        args: RuntimeArgs,
    ) {
        self.state
            .lock()
            .unwrap()
            .call_contract_package(hash, entry_point, args);
    }

    pub fn get_contract_package_hash(&self, name: &str) -> ContractPackageHash {
        self.state.lock().unwrap().get_contract_package_hash(name)
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        TestEnv::new()
    }
}

pub struct TestEnvState {
    account: AccountHash,
    context: InMemoryWasmTestBuilder,
}

impl TestEnvState {
    pub fn new() -> TestEnvState {
        // Create keypair.
        let secret_key = SecretKey::ed25519_from_bytes([1u8; 32]).unwrap();
        let public_key = PublicKey::from(&secret_key);

        // Create an AccountHash from a public key.
        let account_addr = AccountHash::from(&public_key);

        // Create a GenesisAccount.
        let account = GenesisAccount::account(
            public_key,
            Motes::new(U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)),
            None,
        );

        let mut genesis_config = DEFAULT_GENESIS_CONFIG.clone();
        genesis_config.ee_config_mut().push_account(account);

        let run_genesis_request = RunGenesisRequest::new(
            *DEFAULT_GENESIS_CONFIG_HASH,
            genesis_config.protocol_version(),
            genesis_config.take_ee_config(),
        );

        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&run_genesis_request).commit();

        TestEnvState {
            account: account_addr,
            context: builder,
        }
    }

    pub fn deploy_wasm_file(&mut self, wasm_path: &str, args: RuntimeArgs) {
        let session_code = PathBuf::from(wasm_path);

        let deploy_item = DeployItemBuilder::new()
            .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
            .with_authorization_keys(&[self.account])
            .with_address(self.account)
            .with_session_code(session_code, args)
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();
        self.context.exec(execute_request).commit().expect_success();
    }

    pub fn call_contract_package(
        &mut self,
        hash: ContractPackageHash,
        entry_point: &str,
        args: RuntimeArgs,
    ) {
        let deploy_item = DeployItemBuilder::new()
            .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
            .with_authorization_keys(&[self.account])
            .with_address(self.account)
            .with_stored_versioned_contract_by_hash(hash.value(), None, entry_point, args)
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();
        self.context.exec(execute_request).commit().expect_success();
    }

    pub fn get_contract_package_hash(&self, name: &str) -> ContractPackageHash {
        let account = self.context.get_account(self.account).unwrap();
        let key: &Key = account.named_keys().get(name).unwrap();
        ContractPackageHash::from(key.into_hash().unwrap())
    }
}
