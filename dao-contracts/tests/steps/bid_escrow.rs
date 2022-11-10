use crate::common::helpers::{to_rep, value_to_bytes};
use crate::common::DaoWorld;
use casper_dao_contracts::voting::Choice;
use casper_dao_utils::{BlockTime, DocumentHash, TestContract};
use casper_types::{U256, U512};
use cucumber::gherkin::Step;
use cucumber::{given, when};

#[given(expr = "following configuration")]
fn configuration(w: &mut DaoWorld, step: &Step) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let variable = row[0].as_str();
        let value = row[1].as_str();
        w.set_variable(variable.to_string(), value_to_bytes(value));
        assert_eq!(
            w.get_variable(variable.to_string()),
            value_to_bytes(value),
            "variable mismatch"
        );
    }
}

#[given(
    expr = "{word} posted a JobOffer with expected timeframe of {int} days, maximum budget of {int} CSPR and {int} CSPR DOS Fee"
)]
fn post_job_offer(
    w: &mut DaoWorld,
    job_poster_name: String,
    timeframe: BlockTime,
    maximum_budget: u32,
    dos_fee: u32,
) {
    let job_poster = w.named_address(job_poster_name);
    w.bid_escrow
        .as_account(job_poster)
        .post_job_offer_with_cspr_amount(
            timeframe,
            U512::from(maximum_budget) * 1_000_000_000,
            U512::from(dos_fee) * 1_000_000_000,
        );
}

#[given(
    expr = "{word} posted the Bid with proposed timeframe of {int} days and {int} CSPR price and {int} REP stake"
)]
fn submit_bid_internal(
    w: &mut DaoWorld,
    worker_name: String,
    timeframe: BlockTime,
    budget: u32,
    stake: u32,
) {
    let worker = w.named_address(worker_name);
    w.bid_escrow
        .as_account(worker)
        .submit_bid(
            0,
            timeframe,
            U512::from(budget) * 1_000_000_000,
            U256::from(stake) * 1_000_000_000,
            false,
            None,
        )
        .unwrap();
}

#[given(
    expr = "{word} posted the Bid with proposed timeframe of {int} days and {int} CSPR price and {int} CSPR stake {word} onboarding"
)]
fn submit_bid_external(
    w: &mut DaoWorld,
    worker_name: String,
    timeframe: BlockTime,
    budget: u32,
    stake: u32,
    onboarding: String,
) {
    let worker = w.named_address(worker_name);
    let onboarding = match onboarding.as_str() {
        "with" => {
            true
        },
        "without" => {
            false
        },
        _ => {
            panic!("Unknown onboarding option");
        }
    };

    w.bid_escrow
        .as_account(worker)
        .submit_bid_with_cspr_amount(
            0,
            timeframe,
            U512::from(budget) * 1_000_000_000,
            U256::from(0),
            onboarding,
            U512::from(stake) * 1_000_000_000,
        );
}

#[given(expr = "{word} picked the Bid of {word}")]
fn bid_picked(w: &mut DaoWorld, job_poster_name: String, worker_name: String) {
    let job_poster = w.named_address(job_poster_name);
    let _worker = w.named_address(worker_name);
    let required_budget = w.bid_escrow.get_bid(0).unwrap().proposed_payment;
    // TODO: Use bid_ids from the storage.
    w.bid_escrow
        .as_account(job_poster)
        .pick_bid_with_cspr_amount(0, 0, required_budget);
}

#[when(expr = "{word} submits the JobProof")]
fn submit_job_proof(w: &mut DaoWorld, worker_name: String) {
    // TODO: Use bid_ids from the storage.
    let worker = w.named_address(worker_name);
    w.bid_escrow
        .as_account(worker)
        .submit_job_proof(0, DocumentHash::from(b"Job Proof".to_vec()))
        .unwrap();
}

#[when(expr = "Formal/Informal voting ends with votes")]
fn informal_voting(w: &mut DaoWorld, step: &Step) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let name = row.get(0).unwrap();
        let choice = match row.get(1).unwrap().as_str() {
            "Yes" => Choice::InFavor,
            "No" => Choice::Against,
            _ => panic!("Unknown choice"),
        };
        let stake = to_rep(&row[2]);

        let voter = w.named_address(name.clone());

        w.bid_escrow
            .as_account(voter)
            .vote(0, choice, stake)
            .unwrap();
    }

    w.bid_escrow.advance_block_time_by(432000000u64);
    w.bid_escrow.finish_voting(0).unwrap();
}
