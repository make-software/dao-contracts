use casper_dao_contracts::escrow::{job::JobStatus, job_offer::JobOfferStatus, types::JobId};
use casper_dao_utils::{BlockTime, DocumentHash, TestContract};
use casper_types::U512;
use cucumber::{then, when};

use crate::common::{
    helpers::{self, parse_bool},
    params::{Account, Balance, TimeUnit},
    DaoWorld,
};

#[when(
    expr = "{account} posted a JobOffer with expected timeframe of {int} {time_unit}, maximum budget of {balance} CSPR and {balance} CSPR DOS Fee"
)]
fn post_job_offer(
    w: &mut DaoWorld,
    job_poster: Account,
    timeframe: BlockTime,
    time_unit: TimeUnit,
    maximum_budget: Balance,
    dos_fee: Balance,
) {
    let timeframe = helpers::to_seconds(timeframe, time_unit);
    let _ = w.post_offer(job_poster, timeframe, maximum_budget, dos_fee);
}

#[when(expr = "{account} cancels the JobOffer with id {int}")]
fn cancel_job_offer(w: &mut DaoWorld, caller: Account, offer_id: u32) {
    let _ = w
        .bid_escrow
        .as_account(w.get_address(&caller))
        .cancel_job_offer(offer_id);
}

#[when(expr = "{account} cancels the Job with id {int}")]
fn cancel_job(w: &mut DaoWorld, caller: Account, offer_id: u32) {
    let _ = w
        .bid_escrow
        .as_account(w.get_address(&caller))
        .cancel_job(offer_id);
}

#[when(
    expr = "{account} posted the Bid for JobOffer {int} with proposed timeframe of {int} {time_unit} and {balance} CSPR price and {balance} REP stake"
)]
fn submit_bid_internal(
    w: &mut DaoWorld,
    worker: Account,
    job_offer_id: u32,
    timeframe: BlockTime,
    time_unit: TimeUnit,
    budget: Balance,
    stake: Balance,
) {
    let timeframe = helpers::to_seconds(timeframe, time_unit);
    w.post_bid(job_offer_id, worker, timeframe, budget, stake, false, None);
}

#[when(
    expr = "{account} posted the Bid for JobOffer {int} with proposed timeframe of {int} {time_unit} and {balance} CSPR price and {balance} CSPR stake {word} onboarding"
)]
fn submit_bid_external(
    w: &mut DaoWorld,
    worker: Account,
    job_offer_id: u32,
    timeframe: BlockTime,
    time_unit: TimeUnit,
    budget: Balance,
    stake: Balance,
    onboarding: String,
) {
    let onboarding = parse_bool(onboarding);
    let timeframe = helpers::to_seconds(timeframe, time_unit);
    w.post_bid(
        job_offer_id,
        worker,
        timeframe,
        budget,
        Balance::zero(),
        onboarding,
        Some(stake),
    );
}

#[when(expr = "{account} picked the Bid of {account}")]
fn bid_picked(w: &mut DaoWorld, job_poster: Account, worker: Account) {
    w.pick_bid(job_poster, worker);
}

#[when(expr = "{account} submits the JobProof of Job {int}")]
fn submit_job_proof(w: &mut DaoWorld, worker: Account, job_id: JobId) {
    let worker = w.get_address(&worker);
    w.bid_escrow
        .as_account(worker)
        .submit_job_proof(job_id, DocumentHash::from(b"Job Proof".to_vec()))
        .unwrap();
}

#[when(
    expr = "{account} submits the JobProof of Job {int} with {balance} CSPR stake {word} onboarding"
)]
fn submit_job_proof_during_grace_period_external(
    w: &mut DaoWorld,
    worker: Account,
    job_id: JobId,
    cspr_stake: Balance,
    onboarding: String,
) {
    let onboarding = parse_bool(onboarding);
    let worker = w.get_address(&worker);
    w.bid_escrow
        .as_account(worker)
        .submit_job_proof_during_grace_period_with_cspr_amount(
            job_id,
            DocumentHash::from(b"Job Proof".to_vec()),
            U512::zero(),
            onboarding,
            *cspr_stake,
        )
        .unwrap();
}

#[when(expr = "{account} submits the JobProof of Job {int} with {balance} REP stake")]
fn submit_job_proof_during_grace_period_internal(
    w: &mut DaoWorld,
    worker: Account,
    job_id: JobId,
    rep_stake: Balance,
) {
    let worker = w.get_address(&worker);
    w.bid_escrow
        .as_account(worker)
        .submit_job_proof_during_grace_period(
            job_id,
            DocumentHash::from(b"Job Proof".to_vec()),
            *rep_stake,
            false,
            None,
        )
        .unwrap();
}

#[when(expr = "{account} cancels the Bid for {account}")]
fn cancel_bid(w: &mut DaoWorld, worker: Account, job_poster: Account) {
    let job_offer_id = w.get_job_offer_id(&job_poster).unwrap();
    let bid = w.get_bid(*job_offer_id, worker).unwrap();

    w.cancel_bid(worker, *job_offer_id, bid.bid_id);
}

#[when(expr = "{account} got his active job offers slashed")]
fn slash_all_active_job_offers(w: &mut DaoWorld, bidder: Account) {
    w.slash_all_active_job_offers(bidder);
}

#[when(expr = "bid with id {int} is slashed")]
fn slash_bid(w: &mut DaoWorld, bid_id: u32) {
    w.slash_bid(bid_id);
}

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
fn submit_onboarding_request(world: &mut DaoWorld, account: Account, cspr_stake: Balance) {
    let account = world.get_address(&account);
    let _ = world
        .onboarding
        .as_account(account)
        .submit_onboarding_request_with_cspr_amount(DocumentHash::default(), *cspr_stake);
}

#[then(expr = "the JobOffer by {account} {word} posted")]
fn assert_job_offer_status(world: &mut DaoWorld, job_poster: Account, job_offer_status: String) {
    let offer_id = world.get_job_offer_id(&job_poster);
    match parse_bool(job_offer_status) {
        true => assert!(offer_id.is_some()),
        false => assert!(offer_id.is_none()),
    };
}
