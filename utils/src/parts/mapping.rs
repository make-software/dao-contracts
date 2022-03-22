use std::{
    collections::{hash_map::DefaultHasher, BTreeMap},
    hash::{Hash, Hasher},
    marker::PhantomData,
    sync::Mutex,
};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped, Key, URef,
};
use lazy_static::lazy_static;

use crate::{casper_env::to_dictionary_key, Error};

/// Data structure for storing key-value pairs.
///
/// It's is a wrapper on top of Casper's dictionary.
/// The main difference is that Mapping returns default value, if the value doesn't exists
/// and it stores dictionary's URef for later use.
pub struct Mapping<K, V> {
    name: String,
    key_ty: PhantomData<K>,
    value_ty: PhantomData<V>,
}

lazy_static! {
    static ref SEEDS: Mutex<BTreeMap<String, URef>> = Mutex::new(BTreeMap::new());
}

impl<K: ToBytes + CLTyped, V: ToBytes + FromBytes + CLTyped + Default> Mapping<K, V> {
    /// Create new Mapping instance.
    pub fn new(name: String) -> Self {
        Mapping {
            name,
            key_ty: PhantomData::<K>::default(),
            value_ty: PhantomData::<V>::default(),
        }
    }

    /// Create dictionary's URef.
    pub fn init(&self) {
        storage::new_dictionary(&self.name).unwrap_or_revert();
    }

    /// Read `key` from the storage or return default value.
    pub fn get(&self, key: &K) -> V {
        storage::dictionary_get(self.get_uref(), &to_dictionary_key(key))
            .unwrap_or_revert()
            .unwrap_or_default()
    }

    /// Read `key` from the storage or revert if the key stores no value.
    pub fn get_or_revert(&self, key: &K) -> V {
        storage::dictionary_get(self.get_uref(), &to_dictionary_key(key))
            .unwrap_or_revert()
            .unwrap_or_revert_with(Error::ValueNotAvailable)
    }

    /// Set `value` under `key` to the storage. It overrides by default.
    pub fn set(&self, key: &K, value: V) {
        storage::dictionary_put(self.get_uref(), &to_dictionary_key(key), value);
    }

    /// Return the named key path to the dictionarie's URef.
    pub fn path(&self) -> &str {
        &self.name
    }

    fn get_uref(&self) -> URef {
        let mut seeds = SEEDS.lock().unwrap();
        match seeds.get(&self.name) {
            Some(seed) => *seed,
            None => {
                let key: Key = runtime::get_key(&self.name).unwrap_or_revert();
                let seed: URef = *key.as_uref().unwrap_or_revert();
                seeds.insert(self.name.clone(), seed);
                seed
            }
        }
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
            mapping: Mapping::new(name.clone()),
            index: Index::new(name),
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
