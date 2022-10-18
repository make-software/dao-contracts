use casper_dao_macros::{FromBytes, ToBytes};
use casper_types::RuntimeArgs;

use crate::{casper_env::call_contract, Address};

#[derive(Debug, ToBytes, FromBytes, Clone, PartialEq, Eq)]
pub struct ContractCall {
    pub address: Address,
    pub entry_point: String,
    pub runtime_args: RuntimeArgs,
}

impl ContractCall {
    /// Get the contract call's address.
    pub fn address(&self) -> Address {
        self.address
    }

    /// Get the contract call's entry point.
    pub fn entry_point(&self) -> String {
        self.entry_point.clone()
    }

    /// Get a reference to the contract call's runtime args.
    pub fn runtime_args(&self) -> RuntimeArgs {
        self.runtime_args.clone()
    }

    pub fn call(&self) {
        call_contract(
            self.address(),
            self.entry_point().as_str(),
            self.runtime_args(),
        )
    }
}
