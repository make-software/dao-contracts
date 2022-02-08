use std::marker::PhantomData;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped, URef,
};

use crate::casper_env::to_dictionary_key;

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
