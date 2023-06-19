use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;
use odra::types::Balance;

/// Verifies if the proposed payment does not exceeds the budget.
/// May return [Error::PaymentExceedsMaxBudget].
#[derive(Rule)]
pub struct DoesProposedPaymentExceedBudget {
    proposed_payment: Balance,
    max_budget: Balance,
}

impl Validation for DoesProposedPaymentExceedBudget {
    fn validate(&self) -> Result<(), Error> {
        if self.proposed_payment > self.max_budget {
            return Err(Error::PaymentExceedsMaxBudget);
        }

        Ok(())
    }
}
