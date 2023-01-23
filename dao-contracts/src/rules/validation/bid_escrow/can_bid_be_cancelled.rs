use casper_dao_utils::{casper_dao_macros::Rule, BlockTime, Error};

use crate::{bid_escrow::job_offer::JobOfferStatus, rules::validation::Validation};

#[derive(Rule)]
pub struct CanBidBeCancelled {
    pub job_offer_status: JobOfferStatus,
    pub block_time: BlockTime,
    pub bid_timestamp: BlockTime,
    pub va_bid_acceptance_timeout: BlockTime,
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
