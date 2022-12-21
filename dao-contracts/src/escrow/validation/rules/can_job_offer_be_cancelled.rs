use casper_dao_utils::Error;

use crate::{escrow::job_offer::AuctionState, rules::validation::Validation};

pub struct CanJobOfferBeCancelled {
    pub auction_state: AuctionState,
}

impl Validation for CanJobOfferBeCancelled {
    fn validate(&self) -> Result<(), Error> {
        if self.auction_state != AuctionState::None {
            return Err(Error::JobOfferCannotBeYetCanceled);
        }

        Ok(())
    }
}

impl CanJobOfferBeCancelled {
    pub fn create(auction_state: AuctionState) -> Box<CanJobOfferBeCancelled> {
        Box::new(Self { auction_state })
    }
}
