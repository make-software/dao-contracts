use casper_dao_utils::{casper_dao_macros::Rule, Error};

use crate::rules::validation::Validation;

#[derive(Rule)]
pub struct IsUserKyced {
    pub user_kyced: bool,
}

impl Validation for IsUserKyced {
    fn validate(&self) -> Result<(), Error> {
        if self.user_kyced {
            Ok(())
        } else {
            Err(Error::NotKyced)
        }
    }
}
