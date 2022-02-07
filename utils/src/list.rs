use std::fmt::Debug;

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped,
};

use crate::{consts, Error, Mapping, Variable};

pub struct List<T> {
    pub values: Mapping<u32, Option<T>>,
    pub length: Variable<u32>,
}

impl<T: ToBytes + FromBytes + CLTyped + Default + PartialEq + Debug> List<T> {
    pub fn new(name: &str) -> Self {
        Self {
            values: Mapping::new(name.to_string()),
            length: Variable::new(format!("{}{}", name, consts::LENGTH_SUFFIX)),
        }
    }

    pub fn init(&mut self) {
        self.values.init();
        self.length.set(0);
    }

    pub fn add(&mut self, item: T) {
        let length = self.length.get();
        self.values.set(&length, Some(item));
        self.length.set(length + 1);
    }

    pub fn add_or_update(&mut self, item: T) {
        if !self.exists(&item) {
            self.add(item);
        }
    }

    pub fn delete(&mut self, item: T) -> bool {
        let length = self.length.get();
        let (is_deleted, item_index) = self.delete_item(item);

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
        self.values.get(&index).unwrap_or_revert()
    }

    pub fn size(&self) -> u32 {
        self.length.get()
    }

    fn delete_item(&mut self, item: T) -> (bool, u32) {
        let length = self.length.get();
        for idx in 0..length {
            if let Some(value) = self.values.get(&idx) {
                if value == item {
                    self.values.set(&idx, None);
                    return (true, idx);
                }
            }
        }
        (false, length)
    }

    fn move_item(&mut self, from: u32, to: u32) {
        let value = self.values.get(&from);
        self.values.set(&to, value);
        self.values.set(&from, None);
    }

    fn exists(&self, item: &T) -> bool {
        let length = self.length.get();
        for idx in 0..length {
            if let Some(value) = self.values.get(&idx) {
                if value == *item {
                    return true;
                }
            }
        }
        false
    }
}
