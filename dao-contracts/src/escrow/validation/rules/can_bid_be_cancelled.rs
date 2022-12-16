use casper_dao_utils::{BlockTime, Error};

use crate::{escrow::job_offer::JobOfferStatus, rules::validation::Validation};

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

impl CanBidBeCancelled {
    pub fn create(
        job_offer_status: JobOfferStatus,
        block_time: BlockTime,
        bid_timestamp: BlockTime,
        va_bid_acceptance_timeout: BlockTime,
    ) -> Box<CanBidBeCancelled> {
        Box::new(Self {
            job_offer_status,
            block_time,
            bid_timestamp,
            va_bid_acceptance_timeout,
        })
    }
}
