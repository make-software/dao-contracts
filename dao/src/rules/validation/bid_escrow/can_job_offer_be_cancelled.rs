use crate::bid_escrow::job_offer::AuctionState;
use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;

/// Verifies if the [`Job Offer`](crate::bid_escrow::job_offer::JobOffer) can be canceled
/// in the given state. May return [Error::JobOfferCannotBeYetCanceled].
#[derive(Rule)]
pub struct CanJobOfferBeCancelled {
    auction_state: AuctionState,
}

impl Validation for CanJobOfferBeCancelled {
    fn validate(&self) -> Result<(), Error> {
        if self.auction_state != AuctionState::None {
            return Err(Error::JobOfferCannotBeYetCanceled);
        }

        Ok(())
    }
}
