#![no_main]

use casper_contract::contract_api::runtime;
use casper_types::bytesrepr::Bytes;
use utils::{consts, Address};
use variable_repository::{VariableRepositoryContract, VariableRepositoryContractInterface};

#[no_mangle]
fn call() {
    VariableRepositoryContract::install();
}

#[no_mangle]
fn init() {
    VariableRepositoryContract::default().init();
}

#[no_mangle]
fn change_ownership() {
    let owner: Address = runtime::get_named_arg(consts::PARAM_OWNER);
    VariableRepositoryContract::default().change_ownership(owner);
}

#[no_mangle]
fn add_to_whitelist() {
    let address: Address = runtime::get_named_arg(consts::PARAM_ADDRESS);
    VariableRepositoryContract::default().add_to_whitelist(address);
}

#[no_mangle]
fn remove_from_whitelist() {
    let address: Address = runtime::get_named_arg(consts::PARAM_ADDRESS);
    VariableRepositoryContract::default().remove_from_whitelist(address);
}

#[no_mangle]
fn set_or_update() {
    let key: String = runtime::get_named_arg(consts::PARAM_KEY);
    let value: Bytes = runtime::get_named_arg(consts::PARAM_VALUE);
    VariableRepositoryContract::default().set_or_update(key, value);
}

#[no_mangle]
fn get() {
    let key: String = runtime::get_named_arg(consts::PARAM_KEY);
    VariableRepositoryContract::default().get(key);
}

#[no_mangle]
fn delete() {
    let key: String = runtime::get_named_arg(consts::PARAM_KEY);
    VariableRepositoryContract::default().delete(key);
}
