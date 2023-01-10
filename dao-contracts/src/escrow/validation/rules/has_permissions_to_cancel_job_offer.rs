use casper_dao_utils::{casper_dao_macros::Rule, Address, Error};

use crate::rules::validation::Validation;

#[derive(Rule)]
pub struct HasPermissionsToCancelJobOffer {
    pub canceller: Address,
    pub job_offer_poster: Address,
}

impl Validation for HasPermissionsToCancelJobOffer {
    fn validate(&self) -> Result<(), Error> {
        if self.canceller != self.job_offer_poster {
            return Err(Error::CannotCancelNotOwnedJobOffer);
        }
        Ok(())
    }
}
