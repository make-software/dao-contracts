use casper_dao_utils::{casper_dao_macros::{CLTyped, ToBytes, FromBytes}, Address};

use crate::voting::ReputationAmount;

use super::types::{BidId, Description};

#[derive(CLTyped, ToBytes, FromBytes, PartialEq)]
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
    required_stake_for_va: Option<ReputationAmount>,
    poster: Option<Address>,
    worker: Option<Address>,
    status: JobStatus,
}

impl Job {
    pub fn new(bid_id: BidId, description: Description, poster: Address, worker: Address, required_stake_for_va: Option<ReputationAmount>) -> Self {
        Job {
            bid_id,
            description,
            result: None,
            required_stake_for_va,
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
    pub fn status(&self) -> &JobStatus {
        &self.status
    }

    /// Get the job's worker.
    pub fn worker(&self) -> &Option<Address> {
        &self.worker
    }

    /// Set the job's result.
    pub fn set_result(&mut self, result: Description) {
        self.result = Some(result);
    }

    /// Get the job's result.
    pub fn result(&self) -> &Option<Description> {
        &self.result
    }

    /// Get the job's job poster.
    #[must_use]
    pub fn poster(&self) -> Option<Address> {
        self.poster
    }
}