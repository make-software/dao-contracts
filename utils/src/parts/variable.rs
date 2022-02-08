use std::marker::PhantomData;

use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped,
};

use crate::casper_env::{get_key, set_key};

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
