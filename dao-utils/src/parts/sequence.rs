use casper_types::U256;

use crate::{instance::Instanced, Variable};

pub struct SequenceGenerator {
    value: Variable<U256>,
}

impl SequenceGenerator {
    pub fn get_current_value(&self) -> U256 {
        self.value.get().unwrap_or_default()
    }

    pub fn next_value(&mut self) -> U256 {
        let next = self.get_current_value() + U256::one();
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
