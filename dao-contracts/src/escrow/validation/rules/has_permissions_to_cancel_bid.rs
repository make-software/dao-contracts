use casper_dao_utils::{casper_dao_macros::Rule, Address, Error};

use crate::rules::validation::Validation;

#[derive(Rule)]
pub struct HasPermissionsToCancelBid {
    pub canceller: Address,
    pub bidder: Address,
}

impl Validation for HasPermissionsToCancelBid {
    fn validate(&self) -> Result<(), Error> {
        if self.canceller != self.bidder {
            return Err(Error::CannotCancelNotOwnedBid);
        }
        Ok(())
    }
}
