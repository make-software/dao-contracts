use casper_types::U256;

use crate::{instance::Instanced, Variable};

pub struct SequenceGenerator {
    value: Variable<U256>,
}

impl SequenceGenerator {
    pub fn get_current_value(&self) -> U256 {
        self.value.get()
    }

    pub fn next_value(&mut self) -> U256 {
        let next = self.value.get() + U256::one();
        self.value.set(next);
        next
    }
}

impl Instanced for SequenceGenerator {
    fn instance(namespace: &str) -> Self {
        Self {
            value: Instanced::instance(format!("{}_{}", "value", namespace).as_str()),
        }
    }
}
