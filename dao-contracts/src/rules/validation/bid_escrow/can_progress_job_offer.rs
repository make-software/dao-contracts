use casper_dao_utils::{casper_dao_macros::Rule, Address, Error};

use crate::rules::validation::Validation;

/// Makes sure the job poster is the one who progresses the [`Job`](crate::bid_escrow::job::Job).
/// May return [Error::OnlyJobPosterCanModifyJobOffer].
#[derive(Rule)]
pub struct CanProgressJobOffer {
    address: Address,
    job_poster: Address,
}

impl Validation for CanProgressJobOffer {
    fn validate(&self) -> Result<(), Error> {
        if self.job_poster != self.address {
            return Err(Error::OnlyJobPosterCanModifyJobOffer);
        }
        Ok(())
    }
}
