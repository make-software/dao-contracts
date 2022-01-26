use std::{convert::TryInto, marker::PhantomData};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    system::CallStackElement,
    CLTyped, URef,
};

extern crate alloc;

mod address;
mod error;
pub mod owner;
pub mod token;
pub mod whitelist;

#[cfg(feature = "test-support")]
mod test_env;

pub use address::Address;
pub use error::Error;

#[cfg(feature = "test-support")]
pub use test_env::TestEnv;

pub struct Variable<T> {
    name: String,
    ty: PhantomData<T>,
}

impl<T: Default + FromBytes + ToBytes + CLTyped> Variable<T> {
    pub fn new(name: String) -> Self {
        Variable {
            name,
            ty: PhantomData::<T>::default(),
        }
    }

    pub fn get(&self) -> T {
        get_key(&self.name).unwrap_or_default()
    }

    pub fn set(&mut self, value: T) {
        set_key(&self.name, value);
    }

    pub fn path(&self) -> &str {
        &self.name
    }
}

pub struct Mapping<K, V> {
    name: String,
    key_ty: PhantomData<K>,
    value_ty: PhantomData<V>,
}

impl<K: ToBytes + CLTyped, V: ToBytes + FromBytes + CLTyped + Default> Mapping<K, V> {
    pub fn new(name: String) -> Self {
        Mapping {
            name,
            key_ty: PhantomData::<K>::default(),
            value_ty: PhantomData::<V>::default(),
        }
    }

    pub fn init(&self) {
        storage::new_dictionary(&self.name).unwrap_or_revert();
    }

    pub fn get(&self, key: &K) -> V {
        storage::dictionary_get(self.get_uref(), &to_dictionary_key(key))
            .unwrap_or_revert()
            .unwrap_or_default()
    }

    pub fn set(&self, key: &K, value: V) {
        storage::dictionary_put(self.get_uref(), &to_dictionary_key(key), value);
    }

    fn get_uref(&self) -> URef {
        let key = runtime::get_key(&self.name).unwrap_or_revert();
        *key.as_uref().unwrap_or_revert()
    }

    pub fn path(&self) -> &str {
        &self.name
    }
}

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

pub fn to_dictionary_key<T: ToBytes>(key: &T) -> String {
    let preimage = key.to_bytes().unwrap_or_revert();
    base64::encode(&preimage)
}

/// Returns address based on a [`CallStackElement`].
///
/// For `Session` and `StoredSession` variants it will return account hash, and for `StoredContract`
/// case it will use contract hash as the address.
fn call_stack_element_to_address(call_stack_element: CallStackElement) -> Address {
    match call_stack_element {
        CallStackElement::Session { account_hash } => Address::from(account_hash),
        CallStackElement::StoredSession { account_hash, .. } => {
            // Stored session code acts in account's context, so if stored session wants to interact
            // with an ERC20 token caller's address will be used.
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

// /// Gets the caller address which is stored on the top of the call stack.
// ///
// /// This is similar to what [`runtime::get_caller`] does but it also supports stored contracts.
// pub fn get_caller_address() -> Address {
//     let call_stack = runtime::get_call_stack();
//     let top_of_the_stack = take_call_stack_elem(0);
//     call_stack_element_to_address(top_of_the_stack)
// }
