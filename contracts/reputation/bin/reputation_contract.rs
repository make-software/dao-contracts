#![no_main]

use casper_contract::contract_api::runtime;
use casper_types::U256;
use reputation_contract::{ReputationContract, ReputationContractInterface};
use utils::Address;

#[no_mangle]
fn call() {
    ReputationContract::install();
}

#[no_mangle]
fn init() {
    ReputationContract::default().init();
}

#[no_mangle]
fn mint() {
    let recipient: Address = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    ReputationContract::default().mint(recipient, amount);
}

#[no_mangle]
fn burn() {
    let owner: Address = runtime::get_named_arg("owner");
    let amount: U256 = runtime::get_named_arg("amount");
    ReputationContract::default().burn(owner, amount);
}

#[no_mangle]
fn transfer_from() {
    let owner: Address = runtime::get_named_arg("owner");
    let recipient: Address = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    ReputationContract::default().transfer_from(owner, recipient, amount);
}

#[no_mangle]
fn change_ownership() {
    let owner: Address = runtime::get_named_arg("owner");
    ReputationContract::default().change_ownership(owner);
}

#[no_mangle]
fn add_to_whitelist() {
    let address: Address = runtime::get_named_arg("address");
    ReputationContract::default().add_to_whitelist(address);
}

#[no_mangle]
fn remove_from_whitelist() {
    let address: Address = runtime::get_named_arg("address");
    ReputationContract::default().remove_from_whitelist(address);
}

#[no_mangle]
fn stake() {
    let address: Address = runtime::get_named_arg("address");
    let amount: U256 = runtime::get_named_arg("amount");
    ReputationContract::default().stake(address, amount);
}

#[no_mangle]
fn unstake() {
    let address: Address = runtime::get_named_arg("address");
    let amount: U256 = runtime::get_named_arg("amount");
    ReputationContract::default().unstake(address, amount);
}
