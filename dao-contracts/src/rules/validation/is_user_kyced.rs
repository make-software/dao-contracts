use casper_dao_utils::{Error, casper_dao_macros::Rule};

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
