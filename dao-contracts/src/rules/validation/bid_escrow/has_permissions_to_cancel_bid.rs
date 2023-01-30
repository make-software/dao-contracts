use casper_dao_utils::{casper_dao_macros::Rule, Address, Error};

use crate::rules::validation::Validation;

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
