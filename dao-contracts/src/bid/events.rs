use casper_dao_utils::{Address, casper_dao_macros::Event, BlockTime};

use crate::voting::ReputationAmount;

use super::{types::{Description, BidId}};

#[derive(Debug, PartialEq, Event)]
pub struct JobCreated {
    pub bid_id: BidId,
    pub job_poster: Address,
    pub worker: Address,
    pub description: Description,
    pub finish_time: BlockTime,
    pub required_stake: Option<ReputationAmount>
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
