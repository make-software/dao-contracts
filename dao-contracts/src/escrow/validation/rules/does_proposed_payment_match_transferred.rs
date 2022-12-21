use casper_dao_utils::{casper_dao_macros::Rule, Error};
use casper_types::U512;

use crate::rules::validation::Validation;

#[derive(Rule)]
pub struct DoesProposedPaymentMatchTransferred {
    pub proposed_payment: U512,
    pub transferred: U512,
}

impl Validation for DoesProposedPaymentMatchTransferred {
    fn validate(&self) -> Result<(), Error> {
        if self.proposed_payment != self.transferred {
            return Err(Error::PurseBalanceMismatch);
        }

        Ok(())
    }
}
