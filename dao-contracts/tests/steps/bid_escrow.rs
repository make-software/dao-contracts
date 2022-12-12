use std::time::Duration;

use casper_dao_utils::{BlockTime, DocumentHash, TestContract};
use casper_types::bytesrepr::FromBytes;
use cucumber::{gherkin::Step, then, when};

use crate::common::{
    helpers::{self},
    params::{
        voting::{Choice, VotingType},
        Account,
        Balance,
        Result,
        TimeUnit,
    },
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
    let onboarding = match onboarding.as_str() {
        "with" => true,
        "without" => false,
        _ => {
            panic!("Unknown onboarding option");
        }
    };
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

#[when(expr = "{account} submits the JobProof")]
fn submit_job_proof(w: &mut DaoWorld, worker: Account) {
    // TODO: Use bid_ids from the storage.
    let worker = w.get_address(&worker);
    w.bid_escrow
        .as_account(worker)
        .submit_job_proof(0, DocumentHash::from(b"Job Proof".to_vec()))
        .unwrap();
}

#[when(expr = "Formal/Informal voting ends")]
fn voting_ends(w: &mut DaoWorld) {
    w.env.advance_block_time_by(Duration::from_secs(432005u64));
    w.bid_escrow.finish_voting(0).unwrap();
}

#[when(expr = "votes are")]
fn votes_are(w: &mut DaoWorld, step: &Step) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    let voting_type = w.bid_escrow.get_voting(0).unwrap().voting_type();
    for row in table {
        let voter = helpers::parse(row.get(0), "Couldn't parse account");
        let choice = helpers::parse::<Choice>(row.get(1), "Couldn't parse choice");
        let stake = helpers::parse::<Balance>(row.get(2), "Couldn't parse balance");

        let voter = w.get_address(&voter);
        w.bid_escrow
            .as_account(voter)
            .vote(0, voting_type, choice.into(), *stake)
            .unwrap();
    }
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

#[then(expr = "Formal voting does not start")]
fn formal_does_not_start(w: &mut DaoWorld) {
    let voting = w
        .bid_escrow
        .get_voting(0)
        .unwrap();
    assert_eq!(voting.voting_type(), VotingType::Informal.into());
}

#[then(expr = "ballot for {voting_type} voting {int} for {account} has {balance} unbounded tokens")]
fn ballot_is_unbounded(
    w: &mut DaoWorld,
    voting_type: VotingType,
    voting_id: u32,
    account: Account,
    amount: Balance,
) {
    let account = w.get_address(&account);
    let ballot = w
        .bid_escrow
        .get_ballot(voting_id, voting_type.into(), account);
    let ballot = ballot.unwrap_or_else(|| panic!("Ballot doesn't exists"));
    assert_eq!(
        ballot.choice,
        Choice::InFavor.into(),
        "Ballot choice not in favour"
    );
    assert!(ballot.unbounded, "Ballot is not unbounded");
    assert_eq!(
        ballot.stake, *amount,
        "Ballot has stake {:?}, but should be {:?}",
        ballot.stake, amount
    );
}

#[then(expr = "total unbounded stake for voting {int} is {balance} tokens")]
fn total_unbounded_stake_is(
    w: &mut DaoWorld,
    voting_id: u32,
    amount: Balance,
) {
    let total_unbounded_stake = w
        .bid_escrow
        .get_voting(voting_id)
        .unwrap()
        .total_unbounded_stake();
    assert_eq!(
        total_unbounded_stake, *amount,
        "Total unbounded stake is {:?}, but should be {:?}",
        total_unbounded_stake, amount
    );
}

#[then(expr = "{account} {choice} vote of {balance} REP {result}")]
fn cannot_vote(
    w: &mut DaoWorld,
    voter: Account,
    choice: Choice,
    stake: Balance,
    expected_result: Result,
) {
    let voter = w.get_address(&voter);
    let voting_type = w.bid_escrow.get_voting(0).unwrap().voting_type();
    let vote_result = w
        .bid_escrow
        .as_account(voter)
        .vote(0, voting_type, choice.into(), *stake);

    assert_eq!(*expected_result, vote_result.is_ok());
}

#[then(expr = "the JobOffer by {account} {word} posted")]
fn assert_job_offer_status(world: &mut DaoWorld, job_poster: Account, job_offer_status: String) {
    match job_offer_status.as_str() {
        "is" => assert!(world.get_job_offer_id(&job_poster).is_some()),
        "isn't" => assert!(world.get_job_offer_id(&job_poster).is_none()),
        _ => {
            panic!("Unknown job offer option");
        }
    };
}
