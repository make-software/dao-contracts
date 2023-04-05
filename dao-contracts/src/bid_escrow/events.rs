//! BidEscrow-related events.
use casper_dao_utils::{Address, BlockTime, DocumentHash};
use casper_event_standard::{Event, Schemas};
use casper_types::U512;

use super::types::BidId;
use crate::{
    bid_escrow::{
        job::Job,
        job_offer::JobOffer,
        types::{JobId, JobOfferId},
    },
    config::Configuration,
    voting::VotingId,
};

/// Informs a new [Job Offer](crate::bid_escrow::job_offer::JobOffer) has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobOfferCreated {
    /// The offer id.
    job_offer_id: JobOfferId,
    /// The address of an account that created the offer.
    job_poster: Address,
    /// Max CSPR amount to be paid to the `Worker`.
    max_budget: U512,
    /// Offer validity time.
    expected_timeframe: BlockTime,
}

impl JobOfferCreated {
    /// Creates a new event.
    pub fn new(job_offer: &JobOffer) -> Self {
        JobOfferCreated {
            job_offer_id: job_offer.job_offer_id,
            job_poster: job_offer.job_poster,
            max_budget: job_offer.max_budget,
            expected_timeframe: job_offer.expected_timeframe,
        }
    }
}

/// Informs a new [Bid](crate::bid_escrow::bid::Bid) has been placed.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct BidSubmitted {
    bid_id: BidId,
    job_offer_id: JobOfferId,
    worker: Address,
    onboard: bool,
    proposed_timeframe: BlockTime,
    proposed_payment: U512,
    reputation_stake: Option<U512>,
    cspr_stake: Option<U512>,
}

impl BidSubmitted {
    /// Creates a new event.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bid_id: BidId,
        job_offer_id: JobOfferId,
        worker: Address,
        onboard: bool,
        proposed_timeframe: BlockTime,
        proposed_payment: U512,
        reputation_stake: Option<U512>,
        cspr_stake: Option<U512>,
    ) -> Self {
        BidSubmitted {
            bid_id,
            job_offer_id,
            worker,
            onboard,
            proposed_timeframe,
            proposed_payment,
            reputation_stake,
            cspr_stake,
        }
    }
}

/// Informs that a [Bid](crate::bid_escrow::bid::Bid) has been cancelled.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct BidCancelled {
    bid_id: BidId,
    caller: Address,
    job_offer_id: JobOfferId,
}

impl BidCancelled {
    /// Creates a new event.
    pub fn new(bid_id: BidId, caller: Address, job_offer_id: JobOfferId) -> Self {
        BidCancelled {
            bid_id,
            caller,
            job_offer_id,
        }
    }
}

/// Informs a new [Job](crate::bid_escrow::job::Job) has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobCreated {
    bid_id: BidId,
    job_poster: Address,
    worker: Address,
    finish_time: BlockTime,
    payment: U512,
}

impl JobCreated {
    /// Creates a new event.
    pub fn new(job: &Job) -> JobCreated {
        JobCreated {
            bid_id: job.bid_id(),
            job_poster: job.poster(),
            worker: job.worker(),
            finish_time: job.finish_time(),
            payment: job.payment(),
        }
    }
}

/// Informs the [Job](crate::bid_escrow::job::Job) proof has been submitted by the `Worker`.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobSubmitted {
    bid_id: BidId,
    job_poster: Address,
    worker: Address,
    result: DocumentHash,
}

impl JobSubmitted {
    /// Creates a new event.
    pub fn new(job: &Job) -> JobSubmitted {
        let result = match job.result() {
            None => DocumentHash::default(),
            Some(res) => res.clone(),
        };

        JobSubmitted {
            bid_id: job.bid_id(),
            job_poster: job.poster(),
            worker: job.worker(),
            result,
        }
    }
}

/// Informs the [Job](crate::bid_escrow::job::Job) has been canceled.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobCancelled {
    bid_id: BidId,
    caller: Address,
    job_poster: Address,
    worker: Address,
    cspr_amount: U512,
}

impl JobCancelled {
    /// Creates a new event.
    pub fn new(job: &Job, caller: Address) -> JobCancelled {
        JobCancelled {
            bid_id: job.bid_id(),
            caller,
            job_poster: job.poster(),
            worker: job.worker(),
            cspr_amount: Default::default(),
        }
    }
}

/// Informs `Voting` on the [Job](crate::bid_escrow::job::Job) passed.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobDone {
    bid_id: BidId,
    caller: Address,
    job_poster: Address,
    worker: Address,
    cspr_amount: U512,
}

impl JobDone {
    /// Creates a new event.
    pub fn new(job: &Job, caller: Address) -> JobDone {
        JobDone {
            bid_id: job.bid_id(),
            caller,
            job_poster: job.poster(),
            worker: job.worker(),
            cspr_amount: job.payment(),
        }
    }
}

/// Informs `Voting` on the [Job](crate::bid_escrow::job::Job) failed.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobRejected {
    bid_id: BidId,
    caller: Address,
    job_poster: Address,
    worker: Address,
    cspr_amount: U512,
}

impl JobRejected {
    /// Creates a new event.
    pub fn new(job: &Job, caller: Address) -> JobRejected {
        JobRejected {
            bid_id: job.bid_id(),
            caller,
            job_poster: job.poster(),
            worker: job.worker(),
            cspr_amount: job.payment(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Event)]
pub struct BidEscrowVotingCreated {
    bid_id: BidId,
    job_id: JobId,
    job_offer_id: JobOfferId,
    job_poster: Address,
    worker: Address,
    creator: Address,
    voting_id: VotingId,
    config_informal_quorum: u32,
    config_informal_voting_time: u64,
    config_formal_quorum: u32,
    config_formal_voting_time: u64,
    config_total_onboarded: U512,
    config_double_time_between_votings: bool,
    config_voting_clearness_delta: U512,
    config_time_between_informal_and_formal_voting: BlockTime,
}

impl BidEscrowVotingCreated {
    pub fn new(
        job: &Job,
        creator: Address,
        voting_id: VotingId,
        configuration: &Configuration,
    ) -> BidEscrowVotingCreated {
        BidEscrowVotingCreated {
            bid_id: job.bid_id(),
            job_id: job.job_id(),
            job_offer_id: job.job_offer_id(),
            job_poster: job.poster(),
            worker: job.worker(),
            creator,
            voting_id,
            config_informal_quorum: configuration.informal_voting_quorum(),
            config_informal_voting_time: configuration.informal_voting_time(),
            config_formal_quorum: configuration.formal_voting_quorum(),
            config_formal_voting_time: configuration.formal_voting_time(),
            config_total_onboarded: configuration.total_onboarded(),
            config_double_time_between_votings: configuration.should_double_time_between_votings(),
            config_voting_clearness_delta: configuration.voting_clearness_delta(),
            config_time_between_informal_and_formal_voting: configuration
                .time_between_informal_and_formal_voting(),
        }
    }
}

pub enum TransferReason {
    JobPayment,
    JobPaymentReturn,
    JobPayout,
    BidStake,
    BidStakeReturn,
    DOSFeeReturn,
    JobPaymentAndDOSFeeReturn,
    Redistribution,
}

impl ToString for TransferReason {
    fn to_string(&self) -> String {
        match self {
            TransferReason::JobPayment => "JobPayment".to_string(),
            TransferReason::JobPaymentReturn => "JobPaymentReturn".to_string(),
            TransferReason::JobPayout => "JobPayout".to_string(),
            TransferReason::BidStake => "BidStake".to_string(),
            TransferReason::BidStakeReturn => "BidStakeReturn".to_string(),
            TransferReason::DOSFeeReturn => "DOSFeeReturn".to_string(),
            TransferReason::JobPaymentAndDOSFeeReturn => "JobPaymentAndDOSFeeReturn".to_string(),
            TransferReason::Redistribution => "Redistribution".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Event)]
pub struct CSPRTransfer {
    pub from: Address,
    pub to: Address,
    pub amount: U512,
    pub reason: String,
}

pub fn add_event_schemas(schemas: &mut Schemas) {
    schemas.add::<BidSubmitted>();
    schemas.add::<BidCancelled>();
    schemas.add::<JobCreated>();
    schemas.add::<JobOfferCreated>();
    schemas.add::<JobSubmitted>();
    schemas.add::<JobCancelled>();
    schemas.add::<JobDone>();
    schemas.add::<JobRejected>();
    schemas.add::<BidEscrowVotingCreated>();
    schemas.add::<CSPRTransfer>();
}
