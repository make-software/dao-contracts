use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;
use odra::types::Address;

/// Makes sure the job poster is the one who picks the [`Job Offer`](crate::bid_escrow::job_offer::JobOffer).
/// May return [Error::OnlyJobPosterCanPickABid].
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
