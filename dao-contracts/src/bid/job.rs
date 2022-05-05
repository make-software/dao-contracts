use casper_dao_utils::{casper_dao_macros::{CLTyped, ToBytes, FromBytes}, Address, casper_contract::unwrap_or_revert::UnwrapOrRevert};

use crate::voting::ReputationAmount;

use super::types::{BidId, Description};

#[derive(CLTyped, ToBytes, FromBytes, PartialEq, Clone, Copy, Debug)]
pub enum JobStatus {
    Created,
    Accepted,
    Cancelled,
    NotCompleted,
    Completed,
}

impl Default for JobStatus {
    fn default() -> Self {
        JobStatus::Created
    }
}

#[derive(CLTyped, ToBytes, FromBytes, Default)]
pub struct Job {
    bid_id: BidId,
    description: Description,
    result: Option<Description>,
    required_stake: Option<ReputationAmount>,
    poster: Option<Address>,
    worker: Option<Address>,
    status: JobStatus,
}

impl Job {
    pub fn new(bid_id: BidId, description: Description, poster: Address, worker: Address, required_stake: Option<ReputationAmount>) -> Self {
        Job {
            bid_id,
            description,
            result: None,
            required_stake,
            poster: Some(poster),
            worker: Some(worker),
            status: JobStatus::default()
        }
    }

    pub fn accept(&mut self) {
        self.status = JobStatus::Accepted;
    }

    pub fn cancel(&mut self) {
        self.status = JobStatus::Cancelled;
    }

    /// Get the job's status.
    pub fn status(&self) -> JobStatus {
        self.status
    }

    /// Get the job's worker.
    pub fn worker(&self) -> Address {
        self.worker.unwrap()
    }
    
    /// Get the job's poster.
    #[must_use]
    pub fn poster(&self) -> Address {
        self.poster.unwrap()
    }

    /// Set the job's result.
    pub fn set_result(&mut self, result: Description) {
        self.result = Some(result);
    }

    /// Get the job's result.
    pub fn result(&self) -> Option<&Description> {
        self.result.as_ref()
    }

    /// Get the job's bid id.
    pub fn bid_id(&self) -> u32 {
        self.bid_id
    }

    /// Get the job's required stake for va.
    #[must_use]
    pub fn required_stake(&self) -> Option<u32> {
        self.required_stake
    }

    /// Get a reference to the job's description.
    #[must_use]
    pub fn description(&self) -> &String {
        &self.description
    }
}