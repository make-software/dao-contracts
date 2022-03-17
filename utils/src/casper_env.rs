use std::{collections::BTreeSet, convert::TryInto};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    contracts::NamedKeys,
    system::CallStackElement,
    CLTyped, ContractPackageHash, EntryPoints, URef,
};

use crate::{Address, Events};

/// Read value from the storage.
pub fn get_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            let value = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            Some(value)
        }
    }
}

/// Save value to the storage.
pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

/// Returns address based on a [`CallStackElement`].
///
/// For `Session` and `StoredSession` variants it will return account hash, and for `StoredContract`
/// case it will use contract hash as the address.
fn call_stack_element_to_address(call_stack_element: CallStackElement) -> Address {
    match call_stack_element {
        CallStackElement::Session { account_hash } => Address::from(account_hash),
        CallStackElement::StoredSession { account_hash, .. } => {
            // Stored session code acts in account's context, so if stored session
            // wants to interact, caller's address will be used.
            Address::from(account_hash)
        }
        CallStackElement::StoredContract {
            contract_package_hash,
            ..
        } => Address::from(contract_package_hash),
    }
}

fn take_call_stack_elem(n: usize) -> CallStackElement {
    runtime::get_call_stack()
        .into_iter()
        .nth_back(n)
        .unwrap_or_revert()
}

/// Gets the immediate session caller of the current execution.
///
/// This function ensures that only session code can execute this function, and disallows stored
/// session/stored contracts.
pub fn caller() -> Address {
    let second_elem = take_call_stack_elem(1);
    call_stack_element_to_address(second_elem)
}

/// Initialize events dictionary.
pub fn init_events() {
    Events::default().init();
}

/// Record event to the contract's storage.
pub fn emit<T: ToBytes>(event: T) {
    Events::default().emit(event);
}

/// Convert any key to base64.
pub fn to_dictionary_key<T: ToBytes>(key: &T) -> String {
    let preimage = key.to_bytes().unwrap_or_revert();
    base64::encode(&preimage)
}

pub fn install_contract(
    package_hash: &str,
    entry_points: EntryPoints,
    initializer: impl FnOnce(ContractPackageHash),
) {
    // Create a new contract package hash for the contract.
    let (contract_package_hash, _) = storage::create_contract_package_at_hash();
    runtime::put_key(package_hash, contract_package_hash.into());

    let init_access: URef =
        storage::create_contract_user_group(contract_package_hash, "init", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    storage::add_contract_version(contract_package_hash, entry_points, NamedKeys::new());

    // Call contrustor method.
    initializer(contract_package_hash);

    // Revoke access to init.
    let mut urefs = BTreeSet::new();
    urefs.insert(init_access);
    storage::remove_contract_user_group_urefs(contract_package_hash, "init", urefs)
        .unwrap_or_revert();
}
