use casper_dao_utils::{casper_dao_macros::Rule, Error};
use casper_types::U512;

use crate::rules::validation::Validation;

#[derive(Rule)]
pub struct DoesProposedPaymentExceedBudget {
    proposed_payment: U512,
    max_budget: U512,
}

impl Validation for DoesProposedPaymentExceedBudget {
    fn validate(&self) -> Result<(), Error> {
        if self.proposed_payment > self.max_budget {
            return Err(Error::PaymentExceedsMaxBudget);
        }

        Ok(())
    }
}
