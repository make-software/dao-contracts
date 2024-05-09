use crate::bid_escrow::bid::BidStatus;
use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;

/// Makes sure the job poster is the one who picks the [`Bid`](crate::bid_escrow::bid::Bid).
/// May return [Error::OnlyJobPosterCanPickABid].
#[derive(Rule)]
pub struct CanBidBePicked {
    bid_status: BidStatus,
}

impl Validation for CanBidBePicked {
    fn validate(&self) -> Result<(), Error> {
        match self.bid_status {
            BidStatus::Created | BidStatus::Reclaimed => Ok(()),
            BidStatus::Picked => Err(Error::BidAlreadyPicked),
            BidStatus::Canceled => Err(Error::BidCanceled),
            BidStatus::Rejected => Err(Error::BidRejected),
        }
    }
}
