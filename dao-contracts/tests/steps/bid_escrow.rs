use std::time::Duration;

use casper_dao_contracts::voting::{voting::VotingType, Choice};
use casper_dao_utils::{BlockTime, DocumentHash, TestContract};
use casper_types::U256;
use cucumber::{gherkin::Step, then, when};

use crate::common::{
    helpers::{match_choice, match_result, to_rep, to_voting_type, self},
    DaoWorld, params::Account,
};

#[when(
    expr = "{account} posted a JobOffer with expected timeframe of {int} days, maximum budget of {int} CSPR and {int} CSPR DOS Fee"
)]
fn post_job_offer(
    w: &mut DaoWorld,
    job_poster: Account,
    timeframe: BlockTime,
    maximum_budget: u64,
    dos_fee: u64,
) {
    let job_poster = w.get_address(&job_poster);
    w.post_offer(job_poster, timeframe, maximum_budget, dos_fee);
}

#[when(
    expr = "{account} posted the Bid with proposed timeframe of {int} days and {int} CSPR price and {int} REP stake"
)]
fn submit_bid_internal(
    w: &mut DaoWorld,
    worker: Account,
    timeframe: BlockTime,
    budget: u64,
    stake: u64,
) {
    let worker = w.get_address(&worker);
    w.post_bid(0, worker, timeframe, budget, stake, false, None);
}

#[when(
    expr = "{account} posted the Bid with proposed timeframe of {int} days and {int} CSPR price and {int} CSPR stake {word} onboarding"
)]
fn submit_bid_external(
    w: &mut DaoWorld,
    worker: Account,
    timeframe: BlockTime,
    budget: u64,
    stake: u64,
    onboarding: String,
) {
    let worker = w.get_address(&worker);
    let onboarding = match onboarding.as_str() {
        "with" => true,
        "without" => false,
        _ => {
            panic!("Unknown onboarding option");
        }
    };

    w.post_bid(0, worker, timeframe, budget, 0, onboarding, Some(stake));
}

#[when(expr = "{account} picked the Bid of {account}")]
fn bid_picked(w: &mut DaoWorld, job_poster: Account, worker: Account) {
    let job_poster = w.get_address(&job_poster);
    let worker = w.get_address(&worker);
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
fn informal_voting(w: &mut DaoWorld, step: &Step) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let voter = helpers::parse(row.get(0), "Couldn't parse account");
        let choice = match row.get(1).unwrap().as_str() {
            "Yes" => Choice::InFavor,
            "No" => Choice::Against,
            _ => panic!("Unknown choice"),
        };
        let stake = to_rep(&row[2]);

        let voter = w.get_address(&voter);

        w.bid_escrow
            .as_account(voter)
            .vote(0, choice, stake)
            .unwrap();
    }
}

#[when(expr = "{account} cancels the Bid for {account}")]
fn cancel_bid(w: &mut DaoWorld, worker: Account, job_poster: Account) {
    let worker = w.get_address(&worker);
    let job_poster = w.get_address(&job_poster);
    let job_offer_id = w.get_job_offer_id(&job_poster).unwrap();
    let bid = w.get_bid(*job_offer_id, worker).unwrap();

    w.cancel_bid(worker, *job_offer_id, bid.bid_id);
}

#[then(expr = "Formal voting does not start")]
fn formal_does_not_start(w: &mut DaoWorld) {
    let voting = w.bid_escrow.get_voting(0, VotingType::Informal).unwrap();
    assert_eq!(voting.formal_voting_id(), None);
}

#[then(expr = "ballot for {word} voting {int} for {account} has {int} unbounded tokens")]
fn ballot_is_unbounded(
    w: &mut DaoWorld,
    voting_type: String,
    voting_id: u32,
    account: Account,
    amount: u32,
) {
    let account = w.get_address(&account);
    let voting_type = to_voting_type(&voting_type);
    let ballot = w.bid_escrow.get_ballot(voting_id, voting_type, account);
    let ballot = ballot.unwrap_or_else(|| panic!("Ballot doesn't exists"));
    let amount = U256::from(amount) * 1_000_000_000;
    assert_eq!(
        ballot.choice,
        Choice::InFavor,
        "Ballot choice not in favour"
    );
    assert!(ballot.unbounded, "Ballot is not unbounded");
    assert_eq!(
        ballot.stake, amount,
        "Ballot has stake {:?}, but should be {:?}",
        ballot.stake, amount
    );
}

#[then(expr = "total unbounded stake for {word} voting {int} is {int} tokens")]
fn total_unbounded_stake_is(w: &mut DaoWorld, voting_type: String, voting_id: u32, amount: u32) {
    let voting_type = to_voting_type(&voting_type);
    let total_unbounded_stake = w
        .bid_escrow
        .get_voting(voting_id, voting_type)
        .unwrap()
        .total_unbounded_stake();
    let amount = U256::from(amount) * 1_000_000_000;
    assert_eq!(
        total_unbounded_stake, amount,
        "Total unbounded stake is {:?}, but should be {:?}",
        total_unbounded_stake, amount
    );
}

#[then(expr = "{account} {word} vote of {int} REP {word}")]
fn cannot_vote(w: &mut DaoWorld, voter: Account, choice: String, stake: u64, result: String) {
    let voter = w.get_address(&voter);
    let stake = U256::from(stake * 1_000_000_000);
    let choice = match_choice(choice);
    let expected_result = match_result(result);

    let vote_result = w.bid_escrow.as_account(voter).vote(0, choice, stake);

    assert_eq!(expected_result, vote_result.is_ok());
}
