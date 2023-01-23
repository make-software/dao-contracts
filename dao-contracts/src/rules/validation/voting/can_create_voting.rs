use casper_dao_utils::{casper_dao_macros::Rule, Error};

use crate::rules::validation::Validation;

#[derive(Rule)]
pub struct CanCreateVoting {
    is_va: bool,
    only_va_can_create: bool,
}

impl Validation for CanCreateVoting {
    fn validate(&self) -> Result<(), Error> {
        if self.only_va_can_create && !self.is_va {
            return Err(Error::NotOnboarded);
        }

        Ok(())
    }
}
