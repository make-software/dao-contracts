use crate::bid_escrow::job_offer::JobOfferStatus;
use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;
use odra::types::BlockTime;

/// Verifies if the [`Bid`](crate::bid_escrow::bid::Bid) can be canceled.
/// May return [Error::CannotCancelBidOnCompletedJobOffer] or [Error::CannotCancelBidBeforeAcceptanceTimeout].
#[derive(Rule)]
pub struct CanBidBeCancelled {
    job_offer_status: JobOfferStatus,
    block_time: BlockTime,
    bid_timestamp: BlockTime,
    va_bid_acceptance_timeout: BlockTime,
}

impl Validation for CanBidBeCancelled {
    fn validate(&self) -> Result<(), Error> {
        if self.job_offer_status != JobOfferStatus::Created {
            return Err(Error::CannotCancelBidOnCompletedJobOffer);
        }

        if self.block_time < self.bid_timestamp + self.va_bid_acceptance_timeout {
            return Err(Error::CannotCancelBidBeforeAcceptanceTimeout);
        }

        Ok(())
    }
}
