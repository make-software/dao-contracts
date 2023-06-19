use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;
use odra::types::Balance;

/// Verifies if the actual payment matches the proposed payment. May return [Error::PurseBalanceMismatch].
#[derive(Rule)]
pub struct DoesProposedPaymentMatchTransferred {
    proposed_payment: Balance,
    transferred: Balance,
}

impl Validation for DoesProposedPaymentMatchTransferred {
    fn validate(&self) -> Result<(), Error> {
        if self.proposed_payment != self.transferred {
            return Err(Error::PurseBalanceMismatch);
        }

        Ok(())
    }
}
