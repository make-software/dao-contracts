use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;

/// Verifies if the user can be onboarded. May return [Error::VaOnboardedAlready].
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
