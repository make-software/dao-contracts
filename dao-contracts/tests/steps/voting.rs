use casper_dao_utils::Error;
use cucumber::{gherkin::Step, given, then, when};

use crate::{
    common::{
        helpers::{self, to_seconds},
        params::{
            voting::{Ballot, BallotBuilder, Choice, Voting, VotingType},
            Account,
            Balance,
            Contract,
            Result,
            TimeUnit,
        },
        DaoWorld,
    },
    on_voting_contract,
};

#[when(expr = "{account} starts voting with the following config")]
#[given(expr = "{account} starts voting with the following config")]
fn voting_setup(world: &mut DaoWorld, step: &Step, creator: Account) {
    let rows = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in rows {
        let voting: Voting = row.into();
        let _ = world.checked_create_voting(creator, voting);
    }
}

#[when(expr = "voters vote in {contract} {voting_type} voting with id {int}")]
fn voting(
    world: &mut DaoWorld,
    step: &Step,
    contract: Contract,
    voting_type: VotingType,
    voting_id: u32,
) {
    let rows = step.table.as_ref().unwrap().rows.iter().skip(1);

    rows.map(|row| {
        BallotBuilder::default()
            .with_voting_id(voting_id)
            .with_voting_type(voting_type)
            .build(row)
    })
    .filter(|ballot| !ballot.stake.is_zero())
    .for_each(|ballot| {
        let _ = world.checked_vote(&contract, &ballot);
    });
}

#[when(expr = "{account} creates test voting in {contract} with {balance} stake")]
fn create_test_voting(world: &mut DaoWorld, creator: Account, contract: Contract, stake: Balance) {
    world.create_test_voting(contract, creator, stake);
}

#[when(expr = "{voting_type} voting with id {int} ends in {contract} contract")]
fn end_voting(world: &mut DaoWorld, voting_type: VotingType, voting_id: u32, contract: Contract) {
    world.finish_voting(&contract, voting_id, Some(voting_type));
}

#[when(expr = "{contract} slashes {account} in voting with id {int}")]
fn slash_voter(world: &mut DaoWorld, contract: Contract, voter: Account, voting_id: u32) {
    world.checked_slash_voter(contract, voter, voting_id);
}

#[then(expr = "formal voting with id {int} in {contract} contract does not start")]
fn assert_formal_voting_does_not_start(world: &mut DaoWorld, voting_id: u32, contract: Contract) {
    let voting_exists = world.voting_exists(&contract, voting_id, VotingType::Formal);
    assert!(!voting_exists);
}

#[then(expr = "informal voting with id {int} in {contract} contract does not start")]
fn assert_informal_voting_does_not_start(world: &mut DaoWorld, voting_id: u32, contract: Contract) {
    let voting_exists = world.voting_exists(&contract, voting_id, VotingType::Informal);
    assert!(!voting_exists);
}

#[then(expr = "formal voting with id {int} in {contract} contract starts")]
fn assert_formal_voting_starts(world: &mut DaoWorld, voting_id: u32, contract: Contract) {
    let voting_exists = world.voting_exists(&contract, voting_id, VotingType::Formal);
    assert!(voting_exists);
}

#[then(expr = "votes in {contract} {voting_type} voting with id {int} fail")]
fn assert_vote_fails(
    world: &mut DaoWorld,
    step: &Step,
    contract: Contract,
    voting_type: VotingType,
    voting_id: u32,
) {
    let rows = step.table.as_ref().unwrap().rows.iter().skip(1);

    rows.map(|row| {
        let expected_error = row.get(3).unwrap().to_owned();
        (
            expected_error,
            BallotBuilder::default()
                .with_voting_id(voting_id)
                .with_voting_type(voting_type)
                .build(row),
        )
    })
    .map(|(error, ballot)| (error, world.checked_vote(&contract, &ballot)))
    .for_each(|(error, result)| match error.as_str() {
        "CannotVoteTwice" => assert_eq!(Error::CannotVoteTwice, result.unwrap_err()),
        "InsufficientBalance" => assert_eq!(Error::InsufficientBalance, result.unwrap_err()),
        "ZeroStake" => assert_eq!(Error::ZeroStake, result.unwrap_err()),
        unknown => panic!("Unknown error {}", unknown),
    });
}

#[then(expr = "{account} {choice} vote of {balance} REP {result}")]
fn assert_vote(
    world: &mut DaoWorld,
    step: &Step,
    voter: Account,
    choice: Choice,
    stake: Balance,
    expected_result: Result,
) {
    let voting = step.table.as_ref().unwrap().rows.first().unwrap();
    let contract = helpers::parse::<Contract>(voting.get(0), "Couldn't parse contract");
    let voting_id = helpers::parse_or_default::<u32>(voting.get(1));
    let voting_type = helpers::parse_or_default::<VotingType>(voting.get(2));

    let ballot = Ballot {
        voting_id,
        voting_type,
        voter,
        choice,
        stake,
    };

    assert_eq!(
        *expected_result,
        world.checked_vote(&contract, &ballot).is_ok()
    );
}

#[then(expr = "{contract} ballot for voting {int} for {account} has {balance} unbounded tokens")]
fn assert_ballot_is_unbounded(
    w: &mut DaoWorld,
    contract: Contract,
    voting_id: u32,
    account: Account,
    amount: Balance,
) {
    let voting = on_voting_contract!(w, contract, get_voting(voting_id)).unwrap();
    let voting_type = voting.voting_type();
    let account = w.get_address(&account);
    let ballot = on_voting_contract!(w, contract, get_ballot(voting_id, voting_type, account))
        .unwrap_or_else(|| panic!("Ballot doesn't exists"));
    assert_eq!(
        ballot.choice,
        Choice::InFavor.into(),
        "Ballot choice not in favor"
    );
    assert!(ballot.unbound, "Ballot is not unbounded");
    assert_eq!(
        ballot.stake, *amount,
        "Ballot has stake {:?}, but should be {:?}",
        ballot.stake, amount
    );
}

#[then(expr = "{contract} total unbounded stake for voting {int} is {balance} tokens")]
fn assert_unbounded_stake(w: &mut DaoWorld, contract: Contract, voting_id: u32, amount: Balance) {
    let voting = on_voting_contract!(w, contract, get_voting(voting_id)).unwrap();
    let total_unbounded_stake = voting.total_unbound_stake();
    assert_eq!(
        total_unbounded_stake, *amount,
        "Total unbounded stake is {:?}, but should be {:?}",
        total_unbounded_stake, amount
    );
}

#[when(expr = "{contract} voting with id {int} created by {account} passes")]
fn voting_passes(
    world: &mut DaoWorld,
    step: &Step,
    contract: Contract,
    voting_id: u32,
    creator: Account,
) {
    conduct_voting(world, step, contract, voting_id, creator, Choice::InFavor);
}

#[when(expr = "{contract} voting with id {int} created by {account} fails")]
fn voting_fails(
    world: &mut DaoWorld,
    step: &Step,
    contract: Contract,
    voting_id: u32,
    creator: Account,
) {
    conduct_voting(world, step, contract, voting_id, creator, Choice::Against);
}

fn conduct_voting(
    world: &mut DaoWorld,
    step: &Step,
    contract: Contract,
    voting_id: u32,
    creator: Account,
    choice: Choice,
) {
    // creator starts voting
    let rows = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in rows {
        let voting: Voting = row.into();
        let _ = world.checked_create_voting(creator, voting);
    }
    let stake = "500".parse().unwrap();
    let voting_type = VotingType::Informal;

    // voters vote in favor
    (1..8)
        .into_iter()
        .map(|n| Account::VA(n))
        .map(|voter| Ballot {
            voter,
            stake,
            choice,
            voting_id,
            voting_type,
        })
        .for_each(|ballot| {
            let _ = world.checked_vote(&contract, &ballot);
        });

    // 5 days passed
    world.advance_time(to_seconds(5, TimeUnit::Days));
    // informal voting ends
    world.finish_voting(&contract, voting_id, Some(voting_type));
    // 2 days passed
    world.advance_time(to_seconds(2, TimeUnit::Days));

    // voters vote in favor
    let voting_type = VotingType::Formal;
    (1..8)
        .into_iter()
        .map(|n| Account::VA(n))
        .map(|voter| Ballot {
            voter,
            stake,
            choice,
            voting_id,
            voting_type,
        })
        .for_each(|ballot| {
            let _ = world.checked_vote(&contract, &ballot);
        });

    // 5 days passed
    world.advance_time(to_seconds(5, TimeUnit::Days));
    // formal voting ends
    world.finish_voting(&contract, voting_id, Some(voting_type));
}
