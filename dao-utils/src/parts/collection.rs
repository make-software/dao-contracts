use std::{fmt::Debug, hash::Hash};

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped,
};

use crate::{consts, Error, Variable};

use super::mapping::IndexedMapping;

pub struct OrderedCollection<T> {
    pub values: IndexedMapping<T>,
    pub length: Variable<u32>,
}

impl<T: ToBytes + FromBytes + CLTyped + Default + PartialEq + Debug + Hash> OrderedCollection<T> {
    pub fn new(name: &str) -> Self {
        Self {
            values: IndexedMapping::new(name.to_string()),
            length: Variable::new(format!("{}{}", name, consts::LENGTH_SUFFIX)),
        }
    }

    pub fn delete(&mut self, item: T) -> bool {
        let length = self.length.get();
        let (is_deleted, item_index) = self.values.remove(item);

        if !is_deleted {
            return false;
        }

        self.length.set(length - 1);
        let last_index = length - 1;
        // if the last item was removed, we are done here, no need to reindex
        if item_index == last_index {
            return true;
        }

        self.move_item(last_index, item_index);
        true
    }

    pub fn get(&self, index: u32) -> T {
        if index > self.length.get() - 1 {
            runtime::revert(Error::ValueNotAvailable);
        }
        self.values.get(index).unwrap_or_revert()
    }

    pub fn size(&self) -> u32 {
        self.length.get()
    }

    fn move_item(&mut self, from: u32, to: u32) {
        let value = self.values.get(from).unwrap();
        self.values.set(to, value);
        self.values.unset(from);
    }

    fn _add(&mut self, item: T) {
        let length = self.length.get();
        self.values.set(length, item);
        self.length.set(length + 1);
    }
}

pub trait Set<T> {
    fn add(&mut self, item: T);
}

impl<T: ToBytes + FromBytes + CLTyped + Default + PartialEq + Debug + Hash> Set<T>
    for OrderedCollection<T>
{
    fn add(&mut self, item: T) {
        if !self.values.contains(&item) {
            self._add(item);
        }
    }
}

pub trait List<T> {
    fn add(&mut self, item: T);
}

impl<T: ToBytes + FromBytes + CLTyped + Default + PartialEq + Debug + Hash> List<T>
    for OrderedCollection<T>
{
    fn add(&mut self, item: T) {
        self._add(item);
    }
}
