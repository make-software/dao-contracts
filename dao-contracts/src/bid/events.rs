use casper_dao_utils::{casper_dao_macros::Event, Address, BlockTime};
use casper_types::U512;

use crate::voting::ReputationAmount;

use super::types::{BidId, Description};

#[derive(Debug, PartialEq, Event)]
pub struct JobCreated {
    pub bid_id: BidId,
    pub job_poster: Address,
    pub worker: Address,
    pub description: Description,
    pub finish_time: BlockTime,
    pub required_stake: Option<ReputationAmount>,
    pub cspr_amount: U512,
}

#[derive(Debug, PartialEq, Event)]
pub struct JobAccepted {
    pub bid_id: BidId,
    pub job_poster: Address,
    pub worker: Address,
}

#[derive(Debug, PartialEq, Event)]
pub struct JobSubmitted {
    pub bid_id: BidId,
    pub job_poster: Address,
    pub worker: Address,
    pub result: Description,
}
