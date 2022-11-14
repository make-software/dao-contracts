use crate::common::DaoWorld;

use crate::common::helpers::{to_rep, value_to_bytes};
use casper_dao_contracts::voting::Choice;
use casper_dao_utils::TestContract;
use casper_types::U256;
use cucumber::gherkin::Step;
use cucumber::{given, then, when};

#[when(
    expr = "{word} starts slashing vote for {word} with {int} REP stake and {int}% slashing rate"
)]
fn start_vote(w: &mut DaoWorld, creator: String, va_name: String, stake: u64, slashing_rate: u32) {
    let slashing_rate = slashing_rate * 10;
    let creator = w.named_address(creator);
    let va = w.named_address(va_name);
    w.slashing_voter
        .as_account(creator)
        .create_voting(va, slashing_rate, U256::from(stake * 1_000_000_000))
        .unwrap();
}

#[when(expr = "slashing votes in voting {int} are")]
fn informal_voting(w: &mut DaoWorld, voting_id: u32, step: &Step) {
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

        w.slashing_voter
            .as_account(voter)
            .vote(voting_id, choice, stake)
            .unwrap();
    }
}

#[when(expr = "slashing voting {int} ends")]
fn voting_ends(w: &mut DaoWorld, voting_id: u32) {
    w.slashing_voter.advance_block_time_by(432000000u64);
    w.slashing_voter.finish_voting(voting_id).unwrap();
}
