use casper_dao_utils::{casper_dao_macros::Rule, Address, Error};

use crate::rules::validation::Validation;

#[derive(Rule)]
pub struct CanPickBid {
    pub caller: Address,
    pub job_poster: Address,
}

impl Validation for CanPickBid {
    fn validate(&self) -> Result<(), Error> {
        if self.job_poster != self.caller {
            return Err(Error::OnlyJobPosterCanPickABid);
        }
        Ok(())
    }
}