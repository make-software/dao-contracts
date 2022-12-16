use casper_dao_utils::Error;

use crate::rules::validation::Validation;

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

impl IsUserKyced {
    pub fn create(user_kyced: bool) -> Box<IsUserKyced> {
        Box::new(Self { user_kyced })
    }
}
