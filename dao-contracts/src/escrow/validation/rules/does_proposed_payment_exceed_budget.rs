use casper_dao_utils::Error;
use casper_types::U512;

use crate::rules::validation::Validation;

pub struct DoesProposedPaymentExceedBudget {
    pub proposed_payment: U512,
    pub max_budget: U512,
}

impl Validation for DoesProposedPaymentExceedBudget {
    fn validate(&self) -> Result<(), Error> {
        if self.proposed_payment > self.max_budget {
            return Err(Error::PaymentExceedsMaxBudget);
        }

        Ok(())
    }
}

impl DoesProposedPaymentExceedBudget {
    pub fn create(
        proposed_payment: U512,
        max_budget: U512,
    ) -> Box<DoesProposedPaymentExceedBudget> {
        Box::new(Self {
            proposed_payment,
            max_budget,
        })
    }
}
