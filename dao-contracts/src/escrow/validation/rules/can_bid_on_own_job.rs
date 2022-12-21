use casper_dao_utils::{casper_dao_macros::Rule, Address, Error};

use crate::rules::validation::Validation;

#[derive(Rule)]
pub struct CanBidOnOwnJob {
    worker: Address,
    job_poster: Address,
}

impl Validation for CanBidOnOwnJob {
    fn validate(&self) -> Result<(), Error> {
        if self.worker == self.job_poster {
            return Err(Error::CannotBidOnOwnJob);
        }
        Ok(())
    }
}
