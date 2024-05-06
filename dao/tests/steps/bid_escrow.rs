use cucumber::{then, when};
use dao::bid_escrow::bid::BidStatus;
use dao::bid_escrow::contract::BidEscrowContractRef;
use dao::bid_escrow::job::JobStatus;
use dao::bid_escrow::job_offer::JobOfferStatus;
use dao::bid_escrow::types::JobId;
use dao::utils::types::DocumentHash;
use dao::utils::Error;
use odra::test_env;
use odra::types::{Balance, BlockTime};

use crate::common::params::ReputationBalance;
use crate::common::{
    helpers::{self, parse_bool},
    params::{Account, CsprBalance, TimeUnit},
    DaoWorld,
};
use crate::steps::suppress;

#[when(
    expr = "{account} posted a JobOffer with expected timeframe of {int} {time_unit}, maximum budget of {balance} CSPR and {balance} CSPR DOS Fee"
)]
fn post_job_offer(
    w: &mut DaoWorld,
    job_poster: Account,
    timeframe: BlockTime,
    time_unit: TimeUnit,
    maximum_budget: CsprBalance,
    dos_fee: CsprBalance,
) {
    let timeframe = helpers::to_milliseconds(timeframe, time_unit);
    suppress(|| w.post_offer(job_poster, timeframe, *maximum_budget, *dos_fee));
}

#[when(expr = "{account} cancels the JobOffer with id {int}")]
fn cancel_job_offer(w: &mut DaoWorld, caller: Account, offer_id: u32) {
    test_env::set_caller(w.get_address(&caller));
    suppress(|| w.bid_escrow.cancel_job_offer(offer_id));
}

#[when(expr = "{account} cancels the Job with id {int}")]
fn cancel_job(w: &mut DaoWorld, caller: Account, offer_id: u32) {
    test_env::set_caller(w.get_address(&caller));
    suppress(|| w.bid_escrow.cancel_job(offer_id));
}

#[when(
    expr = "{account} posted the Bid for JobOffer {int} with proposed timeframe of {int} {time_unit} and {balance} CSPR price and {reputation} REP stake"
)]
fn submit_bid_internal(
    w: &mut DaoWorld,
    worker: Account,
    job_offer_id: u32,
    timeframe: BlockTime,
    time_unit: TimeUnit,
    budget: CsprBalance,
    stake: ReputationBalance,
) {
    let timeframe = helpers::to_milliseconds(timeframe, time_unit);
    suppress(|| {
        w.post_bid(
            job_offer_id,
            worker,
            timeframe,
            *budget,
            *stake,
            false,
            None,
        )
    });
}

#[allow(clippy::too_many_arguments)]
#[when(
    expr = "{account} posted the Bid for JobOffer {int} with proposed timeframe of {int} {time_unit} and {balance} CSPR price and {balance} CSPR stake {word} onboarding"
)]
fn submit_bid_external(
    w: &mut DaoWorld,
    worker: Account,
    job_offer_id: u32,
    timeframe: BlockTime,
    time_unit: TimeUnit,
    budget: CsprBalance,
    stake: CsprBalance,
    onboarding: String,
) {
    let onboarding = parse_bool(onboarding);
    let timeframe = helpers::to_milliseconds(timeframe, time_unit);
    suppress(|| {
        w.post_bid(
            job_offer_id,
            worker,
            timeframe,
            *budget,
            Balance::zero(), // WON'T DO: Should be zero? - the interface is frozen
            onboarding,
            Some(*stake),
        )
    });
}

#[allow(clippy::too_many_arguments)]
#[when(
    expr = "InternalWorker posted the Bid for JobOffer {int} with {balance} CSPR and {reputation} REP staked"
)]
fn submit_bid_internal_cspr_stake(
    w: &mut DaoWorld,
    job_offer_id: u32,
    cspr_stake: CsprBalance,
    reputation_stake: ReputationBalance,
) {
    let worker = w.get_address(&Account::InternalWorker);
    test_env::set_caller(worker);
    suppress(|| {
        w.post_bid(
            job_offer_id,
            Account::InternalWorker,
            1000,
            100.into(),
            *reputation_stake,
            false,
            Some(*cspr_stake),
        )
    });
}

#[when(expr = "{account} picked the Bid of {account}")]
fn bid_picked(w: &mut DaoWorld, job_poster: Account, worker: Account) {
    w.pick_bid(job_poster, worker);
}

#[when(expr = "{account} picked the Bid without paying for {account}")]
fn bid_picked_without_paying(w: &mut DaoWorld, job_poster: Account, worker: Account) {
    w.pick_bid_without_enough_payment(job_poster, worker);
}

#[when(expr = "{account} submits the JobProof of Job {int}")]
fn submit_job_proof(w: &mut DaoWorld, worker: Account, job_id: JobId) {
    let worker = w.get_address(&worker);
    test_env::set_caller(worker);
    w.bid_escrow
        .submit_job_proof(job_id, DocumentHash::from("Job Proof"));
}

#[when(
    expr = "{account} submits the JobProof of Job {int} with {balance} CSPR stake {word} onboarding"
)]
fn submit_job_proof_during_grace_period_external(
    w: &mut DaoWorld,
    worker: Account,
    job_id: JobId,
    cspr_stake: CsprBalance,
    onboarding: String,
) {
    let onboarding = parse_bool(onboarding);
    let worker = w.get_address(&worker);
    test_env::set_caller(worker);
    w.bid_escrow
        .with_tokens(*cspr_stake)
        .submit_job_proof_during_grace_period(
            job_id,
            DocumentHash::from("Job Proof"),
            Balance::zero(),
            onboarding,
        );
}

#[when(expr = "{account} submits the JobProof of Job {int} with {reputation} REP stake")]
fn submit_job_proof_during_grace_period_internal(
    w: &mut DaoWorld,
    worker: Account,
    job_id: JobId,
    rep_stake: ReputationBalance,
) {
    let worker = w.get_address(&worker);
    test_env::set_caller(worker);
    w.bid_escrow.submit_job_proof_during_grace_period(
        job_id,
        DocumentHash::from("Job Proof"),
        *rep_stake,
        false,
    );
}

#[when(expr = "{account} cancels the Bid for {account}")]
fn cancel_bid(w: &mut DaoWorld, worker: Account, job_poster: Account) {
    suppress(|| {
        let job_offer_id = w.get_job_offer_id(&job_poster).unwrap();
        let bid = w.get_bid(*job_offer_id, worker).unwrap();
        w.cancel_bid(worker, *job_offer_id, bid.bid_id)
    });
}

// w
#[then(expr = "JobOffer with id {int} {word} cancelled")]
fn is_job_offer_cancelled(w: &mut DaoWorld, job_offer_id: u32, cancelled: String) {
    let cancelled = parse_bool(cancelled);
    let job_offer = w.bid_escrow.get_job_offer(job_offer_id).unwrap();
    match job_offer.status {
        JobOfferStatus::Cancelled => assert!(cancelled),
        _ => assert!(!cancelled),
    }
}

#[then(expr = "Job with id {int} {word} cancelled")]
fn is_job_cancelled(w: &mut DaoWorld, job_id: u32, cancelled: String) {
    let cancelled = parse_bool(cancelled);
    let job = w.bid_escrow.get_job(job_id).unwrap();
    match job.status() {
        JobStatus::Cancelled => assert!(cancelled),
        _ => assert!(!cancelled),
    }
}

#[when(expr = "{account} submits an onboarding request with the stake of {balance} CSPR")]
fn submit_onboarding_request(world: &mut DaoWorld, account: Account, cspr_stake: CsprBalance) {
    test_env::set_caller(world.get_address(&account));
    suppress(|| {
        world
            .onboarding
            .with_tokens(*cspr_stake)
            .create_voting(DocumentHash::default())
    });
}

#[then(expr = "the JobOffer by {account} {word} posted")]
fn assert_job_offer_status(world: &mut DaoWorld, job_poster: Account, job_offer_status: String) {
    let offer_id = world.get_job_offer_id(&job_poster);
    match parse_bool(job_offer_status) {
        true => assert!(offer_id.is_some()),
        false => assert!(offer_id.is_none()),
    };
}

#[then(expr = "the Bid of InternalWorker is in state Created")]
fn assert_bid_status(world: &mut DaoWorld) {
    let job_poster = Account::InternalWorker;
    let bid = world.get_bid(0, job_poster).unwrap();
    assert_eq!(bid.status, BidStatus::Created);
}

#[then(expr = "{account} cannot submit the JobProof of Job {int}")]
fn cannot_submit_job_proof(w: &mut DaoWorld, worker: Account, job_id: JobId) {
    let worker = w.get_address(&worker);
    test_env::set_caller(worker);
    test_env::assert_exception(Error::OnlyWorkerCanSubmitProof, || {
        let mut bid_escrow = BidEscrowContractRef::at(w.bid_escrow.address());
        bid_escrow.submit_job_proof(job_id, DocumentHash::from("Job Proof"))
    });
}

#[then(expr = "{account} cannot submit the JobProof of Job {int} for the second time")]
fn cannot_submit_job_proof_second_time(w: &mut DaoWorld, worker: Account, job_id: JobId) {
    let worker = w.get_address(&worker);
    test_env::set_caller(worker);
    test_env::assert_exception(Error::JobAlreadySubmitted, || {
        let mut bid_escrow = BidEscrowContractRef::at(w.bid_escrow.address());
        bid_escrow.submit_job_proof(job_id, DocumentHash::from("Job Proof"))
    });
}

#[then(expr = "{account} fails to pick the Bid of {account}")]
fn bid_pick_failed(w: &mut DaoWorld, job_poster: Account, worker: Account) {
    w.pick_bid_failed(job_poster, worker);
}

#[then(expr = "{account} fails submit the JobProof of outdated Job {int}")]
fn submit_outdated_job_proof(w: &mut DaoWorld, worker: Account, job_id: JobId) {
    let worker = w.get_address(&worker);
    test_env::set_caller(worker);
    test_env::assert_exception(Error::JobProofSubmittedAfterGracePeriod, || {
        w.bid_escrow
            .submit_job_proof(job_id, DocumentHash::from("Job Proof"));
    });
}
