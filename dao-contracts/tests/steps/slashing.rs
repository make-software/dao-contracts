use std::str::FromStr;

use casper_dao_utils::TestContract;
use cucumber::{gherkin::Step, when};

use crate::common::{
    helpers,
    params::{
        voting::{Choice, VotingType},
        Account,
        Balance,
    },
    DaoWorld,
};

#[when(
    expr = "{account} starts slashing vote for {account} with {balance} REP stake and {int}% slashing rate"
)]
fn start_vote(w: &mut DaoWorld, creator: Account, va: Account, stake: Balance, slashing_rate: u32) {
    let slashing_rate = slashing_rate * 10;
    let creator = w.get_address(&creator);
    let va = w.get_address(&va);
    w.slashing_voter
        .as_account(creator)
        .create_voting(va, slashing_rate, *stake)
        .unwrap();
}

#[when(expr = "slashing votes in {voting_type} voting {int} are")]
fn informal_voting(w: &mut DaoWorld, voting_type: VotingType, voting_id: u32, step: &Step) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let voter = helpers::parse(row.first(), "Couldn't parse account");
        let choice = helpers::parse::<Choice>(row.get(1), "Couldn't parse choice");
        let stake = Balance::from_str(&row[2]).unwrap();

        let voter = w.get_address(&voter);

        w.slashing_voter
            .as_account(voter)
            .vote(voting_id, voting_type.into(), choice.into(), *stake)
            .unwrap();
    }
}

#[when(expr = "slashing {voting_type} voting {int} ends")]
fn voting_ends(w: &mut DaoWorld, voting_type: VotingType, voting_id: u32) {
    w.slashing_voter.advance_block_time_by(432000u64);
    w.slashing_voter
        .finish_voting(voting_id, voting_type.into())
        .unwrap();
}
