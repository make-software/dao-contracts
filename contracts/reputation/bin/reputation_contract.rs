#![no_main]

use casper_contract::contract_api::runtime;
use casper_types::{ApiError, U256};
use contract_utils::Address;
use reputation_contract::{ReputationContract, ReputationContractInterface};

#[no_mangle]
fn call() {
    ReputationContract::install();
}

#[no_mangle]
fn init() {
    let initial_supply: U256 = runtime::get_named_arg("initial_supply");
    ReputationContract::default().init(initial_supply);
}

#[no_mangle]
fn mint() {

}

#[no_mangle]
fn transfer() {
    let recipient: Address = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    ReputationContract::default().transfer(recipient, amount);
}