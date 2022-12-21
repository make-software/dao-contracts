use casper_dao_utils::{Address, Error};

use crate::rules::validation::Validation;

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

impl HasPermissionsToCancelJobOffer {
    pub fn create(
        canceller: Address,
        job_offer_poster: Address,
    ) -> Box<HasPermissionsToCancelJobOffer> {
        Box::new(Self {
            canceller,
            job_offer_poster,
        })
    }
}
