use casper_dao_utils::{casper_dao_macros::Rule, Error};

use crate::{bid_escrow::job_offer::AuctionState, rules::validation::Validation};

#[derive(Rule)]
pub struct CanBidOnAuctionState {
    auction_state: AuctionState,
    is_worker_va: bool,
    va_can_bid_on_public_auction: bool,
}

impl Validation for CanBidOnAuctionState {
    fn validate(&self) -> Result<(), Error> {
        match self.auction_state {
            AuctionState::None => {
                return Err(Error::AuctionNotRunning);
            }
            AuctionState::Internal => {
                if !self.is_worker_va {
                    return Err(Error::OnlyOnboardedWorkerCanBid);
                }
            }
            AuctionState::Public => {
                if self.is_worker_va && !self.va_can_bid_on_public_auction {
                    return Err(Error::OnboardedWorkerCannotBid);
                }
            }
        }
        Ok(())
    }
}
