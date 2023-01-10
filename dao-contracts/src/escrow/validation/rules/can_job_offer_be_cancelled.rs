use casper_dao_utils::{casper_dao_macros::Rule, Error};

use crate::{escrow::job_offer::AuctionState, rules::validation::Validation};

#[derive(Rule)]
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
