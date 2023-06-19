use crate::bid_escrow::job_offer::AuctionState;
use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;

/// Verifies if the worker can place a [`Bid`](crate::bid_escrow::bid::Bid) in the given state.
/// May return [Error::AuctionNotRunning], [Error::OnlyOnboardedWorkerCanBid]
/// or [Error::OnboardedWorkerCannotBid].
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
