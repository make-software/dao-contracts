use std::marker::PhantomData;

use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped,
};

use crate::{
    casper_env::{get_key, set_key},
    instance::Instanced,
};

/// Data structure for storing a single value.
pub struct Variable<T> {
    name: String,
    ty: PhantomData<T>,
}

impl<T: Default + FromBytes + ToBytes + CLTyped> Variable<T> {
    /// Create a new Variable instance.
    pub fn new(name: String) -> Self {
        Variable {
            name,
            ty: PhantomData::<T>::default(),
        }
    }

    /// Read from the storage or return default value.
    pub fn get(&self) -> T {
        get_key(&self.name).unwrap_or_default()
    }

    /// Store `value` to the storage.
    pub fn set(&mut self, value: T) {
        set_key(&self.name, value);
    }

    /// Return the named key path to the variable's URef.
    pub fn path(&self) -> &str {
        &self.name
    }
}

impl<T: Default + FromBytes + ToBytes + CLTyped> From<&str> for Variable<T> {
    fn from(name: &str) -> Self {
        Variable::new(name.to_string())
    }
}

impl<T: Default + FromBytes + ToBytes + CLTyped> Instanced for Variable<T> {
    fn instance(namespace: &str) -> Self {
        namespace.into()
    }
}
