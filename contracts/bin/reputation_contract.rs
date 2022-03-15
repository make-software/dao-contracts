#![no_main]

use casper_contract::contract_api::runtime;
use casper_dao_contracts::{ReputationContract, ReputationContractInterface};
use casper_dao_utils::{consts, Address};
use casper_types::U256;

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
    let recipient: Address = runtime::get_named_arg(consts::PARAM_RECIPIENT);
    let amount: U256 = runtime::get_named_arg(consts::PARAM_AMOUNT);
    ReputationContract::default().mint(recipient, amount);
}

#[no_mangle]
fn burn() {
    let owner: Address = runtime::get_named_arg(consts::PARAM_OWNER);
    let amount: U256 = runtime::get_named_arg(consts::PARAM_AMOUNT);
    ReputationContract::default().burn(owner, amount);
}

#[no_mangle]
fn transfer_from() {
    let owner: Address = runtime::get_named_arg(consts::PARAM_OWNER);
    let recipient: Address = runtime::get_named_arg(consts::PARAM_RECIPIENT);
    let amount: U256 = runtime::get_named_arg(consts::PARAM_AMOUNT);
    ReputationContract::default().transfer_from(owner, recipient, amount);
}

#[no_mangle]
fn change_ownership() {
    let owner: Address = runtime::get_named_arg(consts::PARAM_OWNER);
    ReputationContract::default().change_ownership(owner);
}

#[no_mangle]
fn add_to_whitelist() {
    let address: Address = runtime::get_named_arg(consts::PARAM_ADDRESS);
    ReputationContract::default().add_to_whitelist(address);
}

#[no_mangle]
fn remove_from_whitelist() {
    let address: Address = runtime::get_named_arg(consts::PARAM_ADDRESS);
    ReputationContract::default().remove_from_whitelist(address);
}

#[no_mangle]
fn stake() {
    let address: Address = runtime::get_named_arg(consts::PARAM_ADDRESS);
    let amount: U256 = runtime::get_named_arg(consts::PARAM_AMOUNT);
    ReputationContract::default().stake(address, amount);
}

#[no_mangle]
fn unstake() {
    let address: Address = runtime::get_named_arg(consts::PARAM_ADDRESS);
    let amount: U256 = runtime::get_named_arg(consts::PARAM_AMOUNT);
    ReputationContract::default().unstake(address, amount);
}
