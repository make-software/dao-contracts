use casper_dao_utils::{casper_dao_macros::Rule, Error};

use crate::rules::validation::Validation;

#[derive(Rule)]
pub struct IsNotVa {
    is_va: bool,
}

impl Validation for IsNotVa {
    fn validate(&self) -> Result<(), Error> {
        if self.is_va {
            return Err(Error::VaOnboardedAlready);
        };

        Ok(())
    }
}
