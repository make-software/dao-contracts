use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped,
};
use num_traits::{Num, One, Zero};

use crate::{instance::Instanced, Variable};

pub struct SequenceGenerator<T>
where
    T: Num + One + CLTyped + ToBytes + FromBytes,
{
    value: Variable<T>,
}

impl<T> SequenceGenerator<T>
where
    T: Num + One + Zero + Default + Copy + CLTyped + ToBytes + FromBytes,
{
    pub fn get_current_value(&self) -> T {
        self.value.get().unwrap_or_default()
    }

    pub fn next_value(&mut self) -> T {
        match self.value.get() {
            None => {
                self.value.set(T::zero());
                T::zero()
            }
            Some(value) => {
                let next = value + T::one();
                self.value.set(next);
                next
            }
        }
    }
}

impl<T> Instanced for SequenceGenerator<T>
where
    T: Num + One + CLTyped + ToBytes + FromBytes,
{
    fn instance(namespace: &str) -> Self {
        Self {
            value: Instanced::instance(format!("{}_{}", "value", namespace).as_str()),
        }
    }
}
