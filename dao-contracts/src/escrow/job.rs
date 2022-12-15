use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    casper_env::revert,
    Address,
    BlockTime,
    DocumentHash,
    Error,
};
use casper_types::U512;

use super::types::{BidId, JobId, JobOfferId};
use crate::{
    escrow::{
        bid::Bid,
        job::JobStatus::Completed,
    },
    voting::types::VotingId,
};

#[derive(CLTyped, ToBytes, FromBytes, PartialEq, Eq, Clone, Copy, Debug)]
pub enum JobStatus {
    Created,
    Accepted,
    Cancelled,
    Submitted,
    Reclaimed,
    NotCompleted,
    Completed,
}

impl Default for JobStatus {
    fn default() -> Self {
        JobStatus::Created
    }
}

/// Struct holding Job
#[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
pub struct Job {
    job_id: JobId,
    bid_id: BidId,
    job_offer_id: JobOfferId,
    voting_id: Option<VotingId>,
    job_proof: Option<DocumentHash>,
    finish_time: BlockTime,
    status: JobStatus,
    worker: Address,
    worker_type: WorkerType,
    poster: Address,
    payment: U512,
    stake: U512,
    external_worker_cspr_stake: U512,
    followed_by: Option<JobId>,
}

impl Job {
    /// Job constructor
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        job_id: JobId,
        bid_id: BidId,
        job_offer_id: JobOfferId,
        finish_time: BlockTime,
        worker: Address,
        worker_type: WorkerType,
        poster: Address,
        payment: U512,
        stake: U512,
        external_worker_cspr_stake: U512,
    ) -> Self {
        Job {
            job_id,
            bid_id,
            job_offer_id,
            voting_id: None,
            job_proof: None,
            finish_time,
            status: JobStatus::Created,
            worker,
            worker_type,
            poster,
            payment,
            stake,
            external_worker_cspr_stake,
            followed_by: None,
        }
    }

    pub fn reclaim(&mut self, new_job_id: JobId, new_bid: &Bid) -> Job {
        self.status = Completed;
        self.followed_by = Some(new_job_id);

        let worker_type = match (new_bid.cspr_stake.is_some(), new_bid.onboard) {
            (_, true) => WorkerType::ExternalToVA,
            (true, false) => WorkerType::External,
            (false, false) => WorkerType::Internal,
        };

        let mut new_job = Job::new(
            new_job_id,
            new_bid.bid_id,
            new_bid.job_offer_id,
            new_bid.proposed_timeframe,
            new_bid.worker,
            worker_type,
            self.poster,
            self.payment,
            new_bid.reputation_stake,
            new_bid.cspr_stake.unwrap_or_default(),
        );

        new_job.status = JobStatus::Submitted;

        new_job
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

    pub fn is_grace_period(&self, _block_time: BlockTime) -> bool {
        // TODO: Implement
        false
    }

    pub fn submit_proof(&mut self, job_proof: DocumentHash) {
        if self.job_proof().is_some() {
            revert(Error::JobAlreadySubmitted);
        }

        self.job_proof = Some(job_proof);
        self.status = JobStatus::Submitted;
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
        self.job_proof.as_ref()
    }

    /// Get the job's bid id.
    pub fn bid_id(&self) -> BidId {
        self.bid_id
    }

    /// Get the job's offer id.
    pub fn job_offer_id(&self) -> JobOfferId {
        self.job_offer_id
    }

    /// Get the job's payment amount
    pub fn payment(&self) -> U512 {
        self.payment
    }

    /// Get the job's voting id.
    pub fn voting_id(&self) -> Option<VotingId> {
        self.voting_id
    }

    pub fn job_proof(&self) -> Option<&DocumentHash> {
        self.job_proof.as_ref()
    }

    /// Get the job's finish time
    pub fn finish_time(&self) -> BlockTime {
        self.finish_time
    }

    pub fn worker_type(&self) -> &WorkerType {
        &self.worker_type
    }

    pub fn stake(&self) -> U512 {
        self.stake
    }

    pub fn external_worker_cspr_stake(&self) -> U512 {
        self.external_worker_cspr_stake
    }

    pub fn set_voting_id(&mut self, voting_id: VotingId) {
        self.voting_id = Some(voting_id);
    }

    pub fn job_id(&self) -> JobId {
        self.job_id
    }
}

#[derive(CLTyped, ToBytes, FromBytes, Debug, PartialEq, Clone)]
pub enum WorkerType {
    Internal,
    ExternalToVA,
    External,
}
