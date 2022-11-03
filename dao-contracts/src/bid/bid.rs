use casper_types::U512;
use casper_dao_utils::BlockTime;
use casper_dao_utils::casper_dao_macros::{CLTyped, FromBytes, ToBytes};
use crate::bid::types::{BidId, JobOfferId};

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub struct Bid {
    bid_id: BidId,
    job_offer_id: JobOfferId,
    proposed_timeframe: BlockTime,
    proposed_payment: U512,
    reputation_stake: Option<U512>,
    cspr_stake: Option<U512>,
}