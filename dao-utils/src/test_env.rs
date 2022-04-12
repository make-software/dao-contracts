use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{Address, Error};
use casper_engine_test_support::{
    DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT,
    DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_GENESIS_CONFIG, DEFAULT_GENESIS_CONFIG_HASH,
    DEFAULT_PAYMENT,
};
use casper_execution_engine::core::engine_state::{
    self, run_genesis_request::RunGenesisRequest, GenesisAccount,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{self, Bytes, FromBytes, ToBytes},
    runtime_args, ApiError, CLTyped, ContractPackageHash, Key, Motes, PublicKey, RuntimeArgs,
    SecretKey, URef, U512,
};

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

pub use casper_execution_engine::core::execution::Error as ExecutionError;

/// CasperVM based testing environment.
#[derive(Clone)]
pub struct TestEnv {
    state: Arc<Mutex<TestEnvState>>,
}

impl TestEnv {
    /// Create new TestEnv.
    pub fn new() -> TestEnv {
        TestEnv {
            state: Arc::new(Mutex::new(TestEnvState::new())),
        }
    }

    /// Deploy new wasm file.
    pub fn deploy_wasm_file(&self, session_code: &str, session_args: RuntimeArgs) {
        self.state
            .lock()
            .unwrap()
            .deploy_wasm_file(session_code, session_args);
    }

    /// Call contract and return a value.
    pub fn call<T: FromBytes>(
        &self,
        hash: ContractPackageHash,
        entry_point: &str,
        args: RuntimeArgs,
        has_return: bool,
    ) -> Result<Option<T>, Error> {
        self.state
            .lock()
            .unwrap()
            .call(hash, entry_point, args, has_return)
    }

    /// Read [`ContractPackageHash`] from the active user's named keys.
    pub fn get_contract_package_hash(&self, name: &str) -> ContractPackageHash {
        self.state.lock().unwrap().get_contract_package_hash(name)
    }

    /// Read [`casper_types::CLValue`] from the contract's named keys.
    pub fn get_value<T: FromBytes + CLTyped>(&self, hash: ContractPackageHash, name: &str) -> T {
        self.state.lock().unwrap().get_value(hash, name)
    }

    /// Read [`casper_types::CLValue`] from the contract's dictionary.
    pub fn get_dict_value<K: ToBytes + CLTyped, V: FromBytes + CLTyped + Default>(
        &self,
        hash: ContractPackageHash,
        name: &str,
        key: K,
    ) -> V {
        self.state.lock().unwrap().get_dict_value(hash, name, key)
    }

    /// Get account by index.
    pub fn get_account(&self, n: usize) -> Address {
        self.state.lock().unwrap().get_account(n)
    }

    /// Set the account context.
    pub fn as_account(&self, account: Address) {
        self.state.lock().unwrap().as_account(account);
    }

    /// Increases the current value of block_time
    pub fn advance_block_time_by(&self, seconds: Duration) {
        self.state.lock().unwrap().block_time += seconds.as_secs();
    }

    pub fn get_block_time(&self) -> u64 {
        self.state.lock().unwrap().block_time
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        TestEnv::new()
    }
}

pub struct TestEnvState {
    accounts: Vec<Address>,
    active_account: Address,
    context: InMemoryWasmTestBuilder,
    block_time: u64,
    calls_counter: u32,
}

impl TestEnvState {
    pub fn new() -> TestEnvState {
        let mut genesis_config = DEFAULT_GENESIS_CONFIG.clone();
        let mut accounts: Vec<Address> = Vec::new();
        for i in 0..10 {
            // Create keypair.
            let secret_key = SecretKey::ed25519_from_bytes([i; 32]).unwrap();
            let public_key = PublicKey::from(&secret_key);

            // Create an AccountHash from a public key.
            let account_addr = AccountHash::from(&public_key);

            // Create a GenesisAccount.
            let account = GenesisAccount::account(
                public_key,
                Motes::new(U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)),
                None,
            );
            genesis_config.ee_config_mut().push_account(account);

            accounts.push(account_addr.into());
        }

        let run_genesis_request = RunGenesisRequest::new(
            *DEFAULT_GENESIS_CONFIG_HASH,
            genesis_config.protocol_version(),
            genesis_config.take_ee_config(),
        );

        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&run_genesis_request).commit();

        TestEnvState {
            active_account: accounts[0],
            context: builder,
            accounts,
            block_time: 0,
            calls_counter: 0,
        }
    }

    pub fn deploy_wasm_file(&mut self, wasm_path: &str, args: RuntimeArgs) {
        let session_code = PathBuf::from(wasm_path);
        let deploy_item = DeployItemBuilder::new()
            .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
            .with_authorization_keys(&[self.active_account_hash()])
            .with_address(self.active_account_hash())
            .with_session_code(session_code, args)
            .with_deploy_hash(self.next_hash())
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item)
            .with_block_time(self.block_time)
            .build();
        self.context.exec(execute_request).commit().expect_success();
    }

    pub fn call<T: FromBytes>(
        &mut self,
        hash: ContractPackageHash,
        entry_point: &str,
        args: RuntimeArgs,
        has_return: bool,
    ) -> Result<Option<T>, Error> {
        let session_code = PathBuf::from("getter_proxy.wasm");

        let args_bytes: Vec<u8> = args.to_bytes().unwrap();
        let args = runtime_args! {
            "contract_package_hash" => hash,
            "entry_point" => entry_point,
            "args" => Bytes::from(args_bytes),
            "has_return" => has_return
        };

        let deploy_item = DeployItemBuilder::new()
            .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
            .with_authorization_keys(&[self.active_account_hash()])
            .with_address(self.active_account_hash())
            .with_session_code(session_code, args)
            .with_deploy_hash(self.next_hash())
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item)
            .with_block_time(self.block_time)
            .build();
        self.context.exec(execute_request).commit();

        let active_account = self.active_account_hash();

        let result = if self.context.is_error() {
            Err(parse_error(self.context.get_error().unwrap()))
        } else if has_return {
            let result: Bytes = self.get_account_value(active_account, "result");
            Ok(Some(bytesrepr::deserialize(result.to_vec()).unwrap()))
        } else {
            Ok(None)
        };
        self.active_account = self.get_account(0);
        result
    }

    pub fn get_contract_package_hash(&self, name: &str) -> ContractPackageHash {
        let account = self
            .context
            .get_account(self.active_account_hash())
            .unwrap();
        let key: &Key = account.named_keys().get(name).unwrap();
        ContractPackageHash::from(key.into_hash().unwrap())
    }

    pub fn get_value<T: FromBytes + CLTyped>(&self, hash: ContractPackageHash, name: &str) -> T {
        let contract_hash = self
            .context
            .get_contract_package(hash)
            .unwrap()
            .current_contract_hash()
            .unwrap();

        self.context
            .query(None, Key::Hash(contract_hash.value()), &[name.to_string()])
            .unwrap()
            .as_cl_value()
            .unwrap()
            .clone()
            .into_t()
            .unwrap()
    }

    pub fn get_account_value<T: FromBytes + CLTyped>(&self, hash: AccountHash, name: &str) -> T {
        self.context
            .query(None, Key::Account(hash), &[name.to_string()])
            .unwrap()
            .as_cl_value()
            .unwrap()
            .clone()
            .into_t()
            .unwrap()
    }

    pub fn get_dict_value<K: ToBytes + CLTyped, V: FromBytes + CLTyped + Default>(
        &self,
        hash: ContractPackageHash,
        name: &str,
        key: K,
    ) -> V {
        let contract_hash = self
            .context
            .get_contract_package(hash)
            .unwrap()
            .current_contract_hash()
            .unwrap();

        let dictionary_seed_uref: URef = *self
            .context
            .get_contract(contract_hash)
            .unwrap()
            .named_keys()
            .get(name)
            .unwrap()
            .as_uref()
            .unwrap();

        match self.context.query_dictionary_item(
            None,
            dictionary_seed_uref,
            &to_dictionary_key(&key),
        ) {
            Ok(val) => {
                let value: V = val.as_cl_value().unwrap().clone().into_t::<V>().unwrap();
                value
            }
            Err(_) => V::default(),
        }
    }

    fn active_account_hash(&self) -> AccountHash {
        *self.active_account.as_account_hash().unwrap()
    }

    pub fn get_account(&self, n: usize) -> Address {
        *self.accounts.get(n).unwrap()
    }

    pub fn as_account(&mut self, account: Address) {
        self.active_account = account;
    }

    fn next_hash(&mut self) -> [u8; 32] {
        let seed = self.calls_counter;
        self.calls_counter += 1;
        let mut hash = [0u8; 32];
        hash[0] = seed as u8;
        hash[1] = (seed >> 8) as u8;
        hash
    }
}

fn to_dictionary_key<T: ToBytes>(key: &T) -> String {
    let preimage = key.to_bytes().unwrap();
    let hash = blake2b(preimage);
    hex::encode(hash)
}

fn blake2b<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let mut result = [0; 32];
    let mut hasher = VarBlake2b::new(32).expect("should create hasher");

    hasher.update(data);
    hasher.finalize_variable(|slice| {
        result.copy_from_slice(slice);
    });
    result
}

fn parse_error(err: engine_state::Error) -> Error {
    if let engine_state::Error::Exec(exec_err) = err {
        match exec_err {
            ExecutionError::Revert(ApiError::User(id)) => Error::from(id),
            ExecutionError::InvalidContext => Error::InvalidContext,
            ExecutionError::NoSuchMethod(name) => Error::NoSuchMethod(name),
            _ => panic!("{}", exec_err.to_string()),
        }
    } else {
        panic!("{}", err.to_string())
    }
}
