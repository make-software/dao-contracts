use odra::types::{Bytes, OdraType};

use crate::common::DaoWorld;

impl DaoWorld {
    // sets variable value
    pub fn set_variable(&mut self, name: String, value: Bytes) {
        self.variable_repository.update_at(name, value, None);
    }

    // gets variable value
    pub fn get_variable_or_none<T: OdraType>(&self, name: &str) -> Option<T> {
        let bytes = self.variable_repository.get(name.to_string()).unwrap();
        T::deserialize(bytes.as_slice())
    }
}
