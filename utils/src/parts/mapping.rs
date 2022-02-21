use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

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

impl<K: ToBytes + CLTyped, V: ToBytes + FromBytes + CLTyped + Default> From<&str>
    for Mapping<K, V>
{
    fn from(name: &str) -> Self {
        Mapping::new(name.to_string())
    }
}

pub struct IndexedMapping<V> {
    mapping: Mapping<u32, Option<V>>,
    index: Index,
}

impl<V: ToBytes + FromBytes + CLTyped + Default + Hash> IndexedMapping<V> {
    pub fn new(name: String) -> Self {
        IndexedMapping {
            mapping: Mapping::new(name.to_string()),
            index: Index::new(name.to_string()),
        }
    }

    pub fn init(&self) {
        self.mapping.init();
        self.index.init();
    }

    pub fn set(&self, index: u32, value: V) {
        self.index.set(index, &value);
        self.mapping.set(&index, Some(value));
    }

    pub fn get(&self, index: u32) -> Option<V> {
        self.mapping.get(&index)
    }

    pub fn remove(&self, value: V) -> (bool, u32) {
        if let Some(item_index) = self.index.get(&value) {
            if self.mapping.get(&item_index).is_some() {
                self.index.unset(&value);
                self.mapping.set(&item_index, None);
                return (true, item_index);
            }
        }
        (false, 0)
    }

    pub fn unset(&self, index: u32) {
        self.mapping.set(&index, None);
    }

    pub fn index_of(&self, value: &V) -> Option<u32> {
        self.index.get(value)
    }

    pub fn contains(&self, value: &V) -> bool {
        matches!(self.index_of(value), Some(_))
    }

    pub fn path(&self) -> &str {
        self.mapping.path()
    }
}

pub struct Index {
    index: Mapping<u64, Option<u32>>,
}

impl Index {
    pub fn new(name: String) -> Self {
        Index {
            index: Mapping::new(format!("{}{}", name, "_idx")),
        }
    }

    pub fn init(&self) {
        self.index.init();
    }

    pub fn set<V: ToBytes + FromBytes + CLTyped + Default + Hash>(&self, index: u32, value: &V) {
        let value_hash = Index::calculate_hash(value);
        self.index.set(&value_hash, Some(index));
    }

    pub fn unset<V: ToBytes + FromBytes + CLTyped + Default + Hash>(&self, value: &V) {
        let value_hash = Index::calculate_hash(value);
        self.index.set(&value_hash, None);
    }

    pub fn get<V: ToBytes + FromBytes + CLTyped + Default + Hash>(&self, value: &V) -> Option<u32> {
        let value_hash = Index::calculate_hash(value);
        self.index.get(&value_hash)
    }

    fn calculate_hash<V: ToBytes + FromBytes + CLTyped + Default + Hash>(value: &V) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }
}
