use casper_dao_utils::{Address, Error};

use crate::rules::validation::Validation;

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

impl HasPermissionsToCancelBid {
    pub fn create(canceller: Address, bidder: Address) -> Box<HasPermissionsToCancelBid> {
        Box::new(Self { canceller, bidder })
    }
}
