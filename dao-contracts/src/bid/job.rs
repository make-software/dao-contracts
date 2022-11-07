use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address, BlockTime, DocumentHash, Error,
};
use casper_types::{U256, U512};

use crate::voting::types::VotingId;

use super::types::{BidId, JobOfferId, JobId};

#[derive(CLTyped, ToBytes, FromBytes, PartialEq, Eq, Clone, Copy, Debug)]
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

/// Struct holding Job
#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub struct Job {
    job_id: JobId,
    bid_id: BidId,
    job_offer_id: JobOfferId,
    informal_voting_id: Option<VotingId>,
    formal_voting_id: Option<VotingId>,
    result: Option<DocumentHash>,
    finish_time: BlockTime,
    status: JobStatus,
}

impl Job {
    /// Job constructor
    pub fn new(
        job_id: JobId,
        bid_id: BidId,
        job_offer_id: JobOfferId,
        finish_time: BlockTime
    ) -> Self {
        Job {
            job_id,
            bid_id,
            job_offer_id,
            informal_voting_id: None,
            formal_voting_id: None,
            result: None,
            finish_time,
            status: JobStatus::Created
        }
    }

    /// Changes status to the Accepted
    pub fn accept(&mut self, caller: Address, block_time: BlockTime) -> Result<(), Error> {
        if !self.can_be_accepted(caller, block_time) {
            return Err(Error::CannotAcceptJob);
        }

        self.status = JobStatus::Accepted;
        Ok(())
    }

    fn can_be_accepted(&self, caller: Address, block_time: BlockTime) -> bool {
        if self.status() != JobStatus::Created {
            return false;
        }

        if self.worker() == caller && !self.has_time_ended(block_time) {
            return true;
        }

        false
    }

    /// Changes status to the Cancelled
    pub fn cancel(&mut self, caller: Address) -> Result<(), Error> {
        if self.status() != JobStatus::Created || self.poster() != caller {
            return Err(Error::CannotCancelJob);
        }

        self.status = JobStatus::Cancelled;
        Ok(())
    }

    /// Changes status to the Completed
    pub fn complete(&mut self) {
        self.status = JobStatus::Completed;
    }

    /// Changes status to the NotCompleted
    pub fn not_completed(&mut self) {
        self.status = JobStatus::NotCompleted;
    }

    pub fn has_time_ended(&self, block_time: BlockTime) -> bool {
        self.finish_time <= block_time
    }

    fn can_submit(&self, caller: Address, block_time: BlockTime) -> bool {
        if self.has_time_ended(block_time) {
            if caller == self.worker() || caller == self.poster() {
                return true;
            }
        } else if caller == self.worker() && self.status() == JobStatus::Accepted {
            return true;
        }

        false
    }

    pub fn submit(
        &mut self,
        caller: Address,
        block_time: BlockTime,
        result: DocumentHash,
    ) -> Result<(), Error> {
        if !self.can_submit(caller, block_time) {
            return Err(Error::NotAuthorizedToSubmitResult);
        }

        if self.result().is_some() {
            return Err(Error::JobAlreadySubmitted);
        }

        self.result = Some(result);
        self.status = JobStatus::Submitted;
        Ok(())
    }

    /// Get the job's status.
    pub fn status(&self) -> JobStatus {
        self.status
    }

    /// Get the job's worker.
    pub fn worker(&self) -> Address {
        self.worker
    }

    /// Get the job's poster.    
    pub fn poster(&self) -> Address {
        self.poster
    }

    /// Get the job's result.
    pub fn result(&self) -> Option<&DocumentHash> {
        self.result.as_ref()
    }

    /// Get the job's bid id.
    pub fn bid_id(&self) -> BidId {
        self.bid_id
    }

    /// Get the job's required stake for va.
    pub fn required_stake(&self) -> Option<U256> {
        self.required_stake
    }

    /// Get a reference to the job's description.
    pub fn document_hash(&self) -> &DocumentHash {
        &self.document_hash
    }

    /// Get the job's cspr amount.
    pub fn cspr_amount(&self) -> U512 {
        self.cspr_amount
    }

    /// Get the job's informal voting id.
    pub fn informal_voting_id(&self) -> Option<VotingId> {
        self.informal_voting_id
    }

    /// Get the job's formal voting id.
    pub fn formal_voting_id(&self) -> Option<VotingId> {
        self.formal_voting_id
    }

    /// Set the job's informal voting id.
    pub fn set_informal_voting_id(&mut self, informal_voting_id: Option<VotingId>) {
        self.informal_voting_id = informal_voting_id;
    }

    /// Set the job's formal voting id.
    pub fn set_formal_voting_id(&mut self, formal_voting_id: Option<VotingId>) {
        self.formal_voting_id = formal_voting_id;
    }

    /// Get the current voting Id
    pub fn current_voting_id(&self) -> Option<VotingId> {
        if self.formal_voting_id.is_some() {
            return self.formal_voting_id;
        } else if self.informal_voting_id.is_some() {
            return self.informal_voting_id;
        }

        None
    }

    /// Get the job's finish time
    pub fn finish_time(&self) -> BlockTime {
        self.finish_time
    }
}
