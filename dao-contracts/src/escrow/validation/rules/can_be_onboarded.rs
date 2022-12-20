use casper_dao_utils::{casper_dao_macros::Rule, Error};

use crate::rules::validation::Validation;

#[derive(Rule)]
pub struct CanBeOnboarded {
    is_va: bool,
    onboard: bool,
}

impl Validation for CanBeOnboarded {
    fn validate(&self) -> Result<(), Error> {
        if self.is_va && self.onboard {
            return Err(Error::VaOnboardedAlready);
        }
        Ok(())
    }
}
