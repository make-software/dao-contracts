use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;

/// Verifies if `Voting` can be created. May return [Error::NotOnboarded].
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
