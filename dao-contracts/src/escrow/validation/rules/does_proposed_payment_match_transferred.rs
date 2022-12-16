use casper_dao_utils::Error;
use casper_types::U512;

use crate::rules::validation::Validation;

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

impl DoesProposedPaymentMatchTransferred {
    pub fn create(
        proposed_payment: U512,
        transferred: U512,
    ) -> Box<DoesProposedPaymentMatchTransferred> {
        Box::new(Self {
            proposed_payment,
            transferred,
        })
    }
}
