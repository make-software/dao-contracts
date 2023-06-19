use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;

/// Verifies if the user is not onboarded. May return [Error::VaOnboardedAlready].
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
