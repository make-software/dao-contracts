use casper_dao_utils::Error;

use crate::rules::validation::Validation;

pub struct IsNotVa {
    pub is_va: bool,
}

impl Validation for IsNotVa {
    fn validate(&self) -> Result<(), Error> {
        if self.is_va {
            return Err(Error::VaOnboardedAlready);
        };

        Ok(())
    }
}

impl IsNotVa {
    pub fn create(is_va: bool) -> Box<IsNotVa> {
        Box::new(Self { is_va })
    }
}
