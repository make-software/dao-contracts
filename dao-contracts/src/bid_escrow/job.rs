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
    rules::{
        RulesBuilder,
        validation::bid_escrow::{CanPickBid, DoesProposedPaymentMatchTransferred}
    },
    voting::VotingId,
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

pub struct PickBidRequest {
    pub job_id: JobId,
    pub job_offer_id: JobOfferId,
    pub bid_id: BidId,
    pub caller: Address,
    pub poster: Address,
    pub worker: Address,
    pub is_worker_va: bool,
    pub onboard: bool,
    pub block_time: BlockTime,
    pub timeframe: BlockTime,
    pub payment: U512,
    pub transferred_cspr: U512,
    pub stake: U512,
    pub external_worker_cspr_stake: U512,
}

pub struct ReclaimJobRequest {
    pub new_job_id: JobId,
    pub new_bid_id: BidId,
    pub proposed_timeframe: BlockTime,
    pub worker: Address,
    pub cspr_stake: Option<U512>,
    pub reputation_stake: U512,
    pub onboard: bool,
    pub block_time: BlockTime,
}

pub struct SubmitJobProofRequest {
    pub proof: DocumentHash,
}

/// Struct holding Job
#[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
pub struct Job {
    job_id: JobId,
    bid_id: BidId,
    job_offer_id: JobOfferId,
    voting_id: Option<VotingId>,
    job_proof: Option<DocumentHash>,
    start_time: BlockTime,
    time_for_job: BlockTime,
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
    pub fn new(request: &PickBidRequest) -> Self {
        RulesBuilder::new()
            .add_validation(CanPickBid::create(request.caller, request.poster))
            .add_validation(DoesProposedPaymentMatchTransferred::create(
                request.payment,
                request.transferred_cspr,
            ))
            .build()
            .validate_generic_validations();

        let worker_type = if request.is_worker_va {
            WorkerType::Internal
        } else if request.onboard {
            WorkerType::ExternalToVA
        } else {
            WorkerType::External
        };

        Job {
            job_id: request.job_id,
            bid_id: request.bid_id,
            job_offer_id: request.job_offer_id,
            voting_id: None,
            job_proof: None,
            start_time: request.block_time,
            time_for_job: request.timeframe,
            status: JobStatus::Created,
            worker: request.worker,
            worker_type,
            poster: request.poster,
            payment: request.payment,
            stake: request.stake,
            external_worker_cspr_stake: request.external_worker_cspr_stake,
            followed_by: None,
        }
    }

    pub fn reclaim(&mut self, request: ReclaimJobRequest) -> Job {
        self.status = JobStatus::Completed;
        self.followed_by = Some(request.new_job_id);

        let worker_type = match (request.cspr_stake.is_some(), request.onboard) {
            (_, true) => WorkerType::ExternalToVA,
            (true, false) => WorkerType::External,
            (false, false) => WorkerType::Internal,
        };

        Job {
            job_id: request.new_job_id,
            bid_id: request.new_bid_id,
            job_offer_id: self.job_offer_id,
            voting_id: None,
            job_proof: None,
            start_time: request.block_time,
            time_for_job: request.proposed_timeframe,
            status: JobStatus::Submitted,
            worker: request.worker,
            worker_type,
            poster: self.poster,
            payment: self.payment,
            stake: request.reputation_stake,
            external_worker_cspr_stake: request.cspr_stake.unwrap_or_default(),
            followed_by: None,
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

    pub fn validate_cancel(&self, block_time: BlockTime) -> Result<(), Error> {
        if self.status() != JobStatus::Created {
            return Err(Error::CannotCancelJob);
        }

        if self.finish_time() + self.grace_period() >= block_time {
            return Err(Error::JobCannotBeYetCanceled);
        }

        Ok(())
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
        self.start_time + self.time_for_job <= block_time
    }

    pub fn submit_proof(&mut self, request: SubmitJobProofRequest) {
        if self.job_proof().is_some() {
            revert(Error::JobAlreadySubmitted);
        }

        self.job_proof = Some(request.proof);
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
        self.start_time + self.time_for_job
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

    fn grace_period(&self) -> BlockTime {
        self.time_for_job
    }
}

#[derive(CLTyped, ToBytes, FromBytes, Debug, PartialEq, Clone)]
pub enum WorkerType {
    Internal,
    ExternalToVA,
    External,
}
