use casper_dao_utils::Error;

use crate::rules::validation::Validation;

pub struct CanBeOnboarded {
    pub is_va: bool,
    pub onboard: bool,
}

impl Validation for CanBeOnboarded {
    fn validate(&self) -> Result<(), Error> {
        if self.is_va && self.onboard {
            return Err(Error::VaOnboardedAlready);
        }
        Ok(())
    }
}

impl CanBeOnboarded {
    pub fn create(is_va: bool, onboard: bool) -> Box<CanBeOnboarded> {
        Box::new(Self { is_va, onboard })
    }
}
