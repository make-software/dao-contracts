use std::ops::Add;

use casper_dao_utils::{casper_dao_macros::{CLTyped, ToBytes, FromBytes}, Address, BlockTime};

use crate::voting::ReputationAmount;

use super::types::{BidId, Description};

#[derive(CLTyped, ToBytes, FromBytes, PartialEq, Clone, Copy, Debug)]
pub enum JobStatus {
    Created,
    Accepted,
    Cancelled,
    Submitted,
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
    finish_time: BlockTime,
    required_stake: Option<ReputationAmount>,
    poster: Option<Address>,
    worker: Option<Address>,
    status: JobStatus,
}

impl Job {
    pub fn new(bid_id: BidId, description: Description, poster: Address, worker: Address, finish_time: BlockTime, required_stake: Option<ReputationAmount>) -> Self {
        Job {
            bid_id,
            description,
            result: None,
            finish_time,
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

    pub fn can_submit(&self, caller: Address, block_time: BlockTime) -> bool {
        if self.time_ended(block_time) {
            if caller == self.worker() || caller == self.poster() {
                return true
            }
        } else if caller == self.worker() && self.status() == JobStatus::Accepted {
            return true
        }

        false
    }

    pub fn can_cancel(&self, caller: Address) -> bool {
        if self.status() == JobStatus::Created && self.poster() == caller {
            return true
        }
        false
    }

    pub fn can_accept(&self, caller: Address, block_time: BlockTime) -> bool {
        if self.status() != JobStatus::Created {
            return false
        }

        if self.worker() == caller && !self.time_ended(block_time) {
            return true
        }
        
        false
    }

    pub fn time_ended(&self, block_time: BlockTime) -> bool {
        self.finish_time <= block_time
    }

    pub fn submit(&mut self, result: Description) {
        self.result = Some(result);
        self.status = JobStatus::Submitted;
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