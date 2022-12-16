use casper_dao_utils::Error;

use crate::{escrow::job_offer::AuctionState, rules::validation::Validation};

pub struct CanBidOnAuctionState {
    pub auction_state: AuctionState,
    pub is_worker_va: bool,
    pub va_can_bid_on_public_auction: bool,
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

impl CanBidOnAuctionState {
    pub fn create(
        auction_state: AuctionState,
        is_va: bool,
        va_can_bid_on_public_auction: bool,
    ) -> Box<CanBidOnAuctionState> {
        Box::new(Self {
            auction_state,
            is_worker_va: is_va,
            va_can_bid_on_public_auction,
        })
    }
}
