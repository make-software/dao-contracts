use casper_dao_utils::{BlockTime, DocumentHash, TestContract};
use casper_types::{U256, U512};

mod common;

use crate::common::helpers::value_to_bytes;
use crate::common::DaoWorld;
use cucumber::gherkin::Step;
use cucumber::{given, then, when, World as _};

#[given(expr = "following balances")]
fn starting_balances(w: &mut DaoWorld, step: &Step) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let name = row[0].as_str();
        let cspr_balance = U512::from(row[1].parse::<u32>().unwrap()) * 1_000_000_000;
        let rep_balance = U256::from(row[2].parse::<u32>().unwrap());

        // set balances
        let address = w.named_address(name.to_string());
        w.set_cspr_balance(address, cspr_balance);
        w.set_rep_balance(address, rep_balance);

        assert_eq!(
            w.get_cspr_balance(address),
            cspr_balance,
            "cspr set balance mismatch"
        );
        assert_eq!(
            w.get_rep_balance(address),
            rep_balance,
            "rep set balance mismatch"
        );
    }
}

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

#[given(expr = "{word} picked a bid with {int} CSPR and {int} Reputation for {word}")]
fn pick_bid(
    w: &mut DaoWorld,
    job_poster_name: String,
    cspr_amount: u32,
    rep_amount: u32,
    worker_name: String,
) {
    let job_poster = w.named_address(job_poster_name);
    let worker = w.named_address(worker_name);
    w.bid_escrow
        .as_account(job_poster)
        .pick_bid_with_cspr_amount(
            worker,
            DocumentHash::from(b"Job Description".to_vec()),
            60,
            Some(U256::from(rep_amount)),
            U512::from(cspr_amount * 1_000_000_000),
        );
}

#[given(expr = "{word} posted a JobOffer with expected timeframe of {int} days and maximum budget of {int} CSPR")]
fn post_job_offer(
    w: &mut DaoWorld,
    job_poster_name: String,
    timeframe: u32,
    maximum_budget: u32
) {
    let job_poster = w.named_address(job_poster_name);
    w.bid_escrow
        .as_account(job_poster)
        .post_job_offer(
            BlockTime::from(timeframe * 24 * 60 * 60),

        );
}

#[when(expr = "{word} accepts the job")]
fn accept_job(w: &mut DaoWorld, worker_name: String) {
    let worker = w.named_address(worker_name);
    w.bid_escrow.as_account(worker).accept_job(0).unwrap();
}

#[then(expr = "balances are")]
fn balances(w: &mut DaoWorld, step: &Step) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let name = row.get(0).unwrap();
        let cspr_balance = U512::from(row[1].parse::<u32>().unwrap()) * 1_000_000_000;
        let rep_balance = U256::from(row[2].parse::<u32>().unwrap());

        let address = w.named_address(name.to_string());

        assert_eq!(
            w.get_cspr_balance(address),
            cspr_balance,
            "cspr balance mismatch"
        );
        assert_eq!(
            w.get_rep_balance(address),
            rep_balance,
            "rep balance mismatches"
        );
    }
}

#[tokio::main]
async fn main() {
    DaoWorld::run("tests/features/bid_escrow").await;
}
