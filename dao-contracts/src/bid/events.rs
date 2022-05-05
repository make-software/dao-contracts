use casper_dao_utils::{Address, casper_dao_macros::Event};

use crate::voting::ReputationAmount;

use super::{types::{Description, BidId}};

#[derive(Debug, PartialEq, Event)]
pub struct JobCreated {
    pub bid_id: BidId,
    pub job_poster: Address,
    pub worker: Address,
    pub description: Description,
    pub required_stake: Option<ReputationAmount>
}