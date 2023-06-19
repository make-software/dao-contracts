use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;
use odra::types::Address;

/// Makes sure the bidder is the one who cancels the [`Bid`](crate::bid_escrow::bid::Bid).
/// May return [Error::CannotCancelNotOwnedBid].
#[derive(Rule)]
pub struct HasPermissionsToCancelBid {
    canceller: Address,
    bidder: Address,
}

impl Validation for HasPermissionsToCancelBid {
    fn validate(&self) -> Result<(), Error> {
        if self.canceller != self.bidder {
            return Err(Error::CannotCancelNotOwnedBid);
        }
        Ok(())
    }
}
