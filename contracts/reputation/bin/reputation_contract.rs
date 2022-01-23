#![no_main]

use casper_contract::contract_api::runtime;
use casper_types::ApiError;
use reputation_contract::ReputationContract;

#[no_mangle]
fn call() {
    ReputationContract::install();
}

#[no_mangle]
fn init() {}

#[no_mangle]
fn mint() {
    runtime::revert(ApiError::None);
}

