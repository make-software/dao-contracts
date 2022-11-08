use casper_dao_utils::{BlockTime, DocumentHash, TestContract};
use casper_types::{U256, U512};

mod common;

use crate::common::helpers::value_to_bytes;
use crate::common::DaoWorld;
use casper_dao_contracts::voting::Choice;
use cucumber::gherkin::Step;
use cucumber::{given, then, when, World as _};

#[given(expr = "following balances")]
fn starting_balances(w: &mut DaoWorld, step: &Step) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let name = row[0].as_str();
        let cspr_balance = to_cspr(&row[1]);
        let rep_balance = to_rep(&row[2]);

        // set balances
        let address = w.named_address(name.to_string());
        w.set_cspr_balance(address, cspr_balance);
        w.set_rep_balance(address, rep_balance);

        // assert_eq!(
        //     w.get_cspr_balance(address),
        //     cspr_balance,
        //     "cspr set balance mismatch"
        // );
        // assert_eq!(
        //     w.get_rep_balance(address),
        //     rep_balance,
        //     "rep set balance mismatch"
        // );
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
    expr = "{word} posted the Bid with proposed timeframe of {int} days and {int} CSPR price and {int} {word} stake"
)]
fn post_bid(
    w: &mut DaoWorld,
    worker_name: String,
    timeframe: BlockTime,
    budget: u32,
    stake: u32,
    stake_type: String,
) {
    let worker = w.named_address(worker_name);
    match stake_type.as_str() {
        "CSPR" => {
            todo!();
        }
        "REP" => {
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
        _ => panic!("Unknown stake type"),
    }
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

#[then(expr = "balances are")]
fn balances(w: &mut DaoWorld, step: &Step) {
    let (total_rep_supply, all_rep_balances) = w.reputation_token.all_balances();
    dbg!(total_rep_supply);
    dbg!(all_rep_balances.balances);

    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let name = row.get(0).unwrap();
        let address = w.named_address(name.to_string());

        // Check REP balance.
        let expected_rep_balance = to_rep(&row[2]);
        let real_rep_balance = w.get_rep_balance(address);
        assert!(
            is_rep_close_enough(expected_rep_balance, real_rep_balance),
            "For account {} REP balance should be {:?} but is {:?}",
            name, expected_rep_balance, real_rep_balance
        );
        
        // Check CSPR balance
        let expected_cspr_balance = to_cspr(&row[1]);
        let real_cspr_balance = w.get_cspr_balance(address);
        assert!(
            is_cspr_close_enough(expected_cspr_balance, real_cspr_balance),
            "For account {} CSPR balance should be {:?} but is {:?}",
            name, expected_cspr_balance, real_cspr_balance
        );

        // Check staked REP balance.
        let expected_rep_stake = to_rep(&row[3]);
        let real_rep_stake = w.reputation_token.get_stake(address);
        assert!(
            is_rep_close_enough(expected_rep_stake, real_rep_stake),
            "For account {} REP stake should be {:?} but is {:?}",
            name, expected_rep_stake, real_rep_stake
        );
    }
}

fn to_rep(v: &str) -> U256 {
    U256::from((v.parse::<f32>().unwrap() * 1_000f32) as u32) * 1_000_000
}

fn to_cspr(v: &str) -> U512 {
    U512::from((v.parse::<f32>().unwrap() * 1_000f32) as u32) * 1_000_000
}

fn is_cspr_close_enough(a: U512, b: U512) -> bool {
    let diff = if a > b { a - b } else { b - a };
    diff < U512::from(10_000_000)
}
fn is_rep_close_enough(a: U256, b: U256) -> bool {
    let diff = if a > b { a - b } else { b - a };
    diff < U256::from(10_000_000)
}

fn main() {
    let runner = DaoWorld::cucumber().run_and_exit("tests/features/bid_escrow");
    futures::executor::block_on(runner);
}
