use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;
use odra::types::Address;

/// Verifies if the worker attempts to bid on his own [`Job`](crate::bid_escrow::job::Job).
/// May return [Error::CannotBidOnOwnJob],
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
