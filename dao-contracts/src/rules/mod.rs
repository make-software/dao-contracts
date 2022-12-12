use casper_dao_utils::casper_env::revert;

use crate::rules::validation::Validation;

pub mod action;
pub mod builder;
pub mod validation;

pub struct Rules {
    pub validations: Vec<Box<dyn Validation>>,
}

impl Rules {
    pub fn validate(&self) {
        for validation in &self.validations {
            let result = validation.validate();
            if result.is_err() {
                revert(result.err().unwrap());
            }
        }
    }
}
