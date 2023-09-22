use cucumber::{gherkin::Step, given, then, when};
use dao::{utils::Error as DaoError, voting::voting_engine::voting_state_machine::VotingState};
use odra::test_env;

use crate::common::{
    helpers::{self, to_milliseconds},
    params::{
        voting::{Ballot, BallotBuilder, Choice, Voting, VotingType},
        Account, Contract, Error, ReputationBalance, Result, TimeUnit,
    },
    DaoWorld,
};
use crate::steps::suppress;

#[when(expr = "{account} starts voting with the following config")]
#[given(expr = "{account} starts voting with the following config")]
fn voting_setup(world: &mut DaoWorld, step: &Step, creator: Account) {
    let rows = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in rows {
        let voting: Voting = row.as_slice().into();
        world.create_voting(creator, voting);
    }
}

#[then(expr = "{account} can't start voting with the following config")]
fn voting_creation_fails(world: &mut DaoWorld, step: &Step, creator: Account) {
    let rows = step.table.as_ref().unwrap().rows.iter().skip(1);

    for row in rows {
        if let Some((error, row)) = row.split_last() {
            let error = error.parse::<Error>().expect("Valid error expected");
            let voting: Voting = row.into();
            test_env::assert_exception(*error, || world.create_voting(creator, voting));
        }
    }
}

#[when(expr = "voters vote in {account} {voting_type} voting with id {int}")]
fn voting(
    world: &mut DaoWorld,
    step: &Step,
    contract: Account,
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
        suppress(|| world.vote(&contract, &ballot));
    });
}

#[when(expr = "{account} creates test voting in {contract} with {reputation} stake")]
fn create_test_voting(
    world: &mut DaoWorld,
    creator: Account,
    contract: Contract,
    stake: ReputationBalance,
) {
    world.create_test_voting(contract, creator, stake);
}

#[when(expr = "{voting_type} voting with id {int} ends in {account} contract")]
fn end_voting(world: &mut DaoWorld, voting_type: VotingType, voting_id: u32, contract: Account) {
    world.finish_voting(&contract, voting_id, Some(voting_type));
}

#[when(expr = "{account} calls {account} to slash {account}")]
fn slash_voter(world: &mut DaoWorld, caller: Account, contract: Account, voter: Account) {
    world.slash_voter(caller, contract, voter);
}

#[then(expr = "formal voting with id {int} in {account} contract does not start")]
fn assert_formal_voting_does_not_start(world: &mut DaoWorld, voting_id: u32, contract: Account) {
    let voting_exists = world.voting_exists(&contract, voting_id, VotingType::Formal);
    assert!(!voting_exists);
}

#[then(expr = "informal voting with id {int} in {account} contract does not start")]
fn assert_informal_voting_does_not_start(world: &mut DaoWorld, voting_id: u32, contract: Account) {
    let voting_exists = world.voting_exists(&contract, voting_id, VotingType::Informal);
    assert!(!voting_exists);
}

#[then(expr = "formal voting with id {int} in {account} contract starts")]
fn assert_formal_voting_starts(world: &mut DaoWorld, voting_id: u32, contract: Account) {
    let voting_exists = world.voting_exists(&contract, voting_id, VotingType::Formal);
    assert!(voting_exists);
}

#[then(expr = "voting with id {int} in {account} contract starts")]
fn assert_voting_(world: &mut DaoWorld, voting_id: u32, contract: Account) {
    assert!(world.voting_exists(&contract, voting_id, VotingType::Informal));
}

#[then(expr = "votes in {account} {voting_type} voting with id {int} fail")]
fn assert_vote_fails(
    world: &mut DaoWorld,
    step: &Step,
    contract: Account,
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
    .for_each(|(error, ballot)| {
        match *(error.parse::<crate::common::params::Error>().unwrap()) {
            DaoError::CannotVoteTwice => {
                world.failing_vote(&contract, &ballot, DaoError::CannotVoteTwice)
            }
            DaoError::InsufficientBalance => {
                world.failing_vote(&contract, &ballot, DaoError::InsufficientBalance)
            }
            DaoError::InsufficientBalanceForStake => {
                world.failing_vote(&contract, &ballot, DaoError::InsufficientBalanceForStake)
            }
            DaoError::ZeroStake => world.failing_vote(&contract, &ballot, DaoError::ZeroStake),
            _ => panic!("Unknown error"),
        }
    });
}

#[then(expr = "{account} {choice} vote of {reputation} REP {result}")]
fn assert_vote(
    world: &mut DaoWorld,
    step: &Step,
    voter: Account,
    choice: Choice,
    stake: ReputationBalance,
    expected_result: Result,
) {
    let voting = step.table.as_ref().unwrap().rows.first().unwrap();
    let contract = helpers::parse::<Account>(voting.get(0), "Couldn't parse contract");
    let voting_id = helpers::parse_or_default::<u32>(voting.get(1));
    let voting_type = helpers::parse_or_default::<VotingType>(voting.get(2));

    let ballot = Ballot {
        voting_id,
        voting_type,
        voter,
        choice,
        stake,
    };
    suppress(|| world.vote(&contract, &ballot));

    assert_eq!(
        *expected_result,
        world
            .get_ballot(&contract, &voter, voting_id, voting_type.into())
            .is_some()
    );
}

#[then(expr = "{account} ballot for voting {int} for {account} has {reputation} unbounded tokens")]
fn assert_ballot_is_unbounded(
    w: &mut DaoWorld,
    contract: Account,
    voting_id: u32,
    account: Account,
    amount: ReputationBalance,
) {
    let voting = w.get_voting(&contract, voting_id);
    let voting_type = voting.voting_type();

    let ballot = w
        .get_ballot(&contract, &account, voting_id, voting_type)
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

#[then(expr = "{account} total unbounded stake for voting {int} is {reputation} tokens")]
fn assert_unbounded_stake(
    w: &mut DaoWorld,
    contract: Account,
    voting_id: u32,
    amount: ReputationBalance,
) {
    let voting = w.get_voting(&contract, voting_id);
    let total_unbounded_stake = voting.total_unbound_stake();
    assert_eq!(
        total_unbounded_stake, *amount,
        "Total unbounded stake is {:?}, but should be {:?}",
        total_unbounded_stake, amount
    );
}

#[when(expr = "{account} voting with id {int} created by {account} passes")]
fn voting_passes(
    world: &mut DaoWorld,
    step: &Step,
    contract: Account,
    voting_id: u32,
    creator: Account,
) {
    conduct_voting(world, step, contract, voting_id, creator, Choice::InFavor);
}

#[when(expr = "{account} voting with id {int} created by {account} fails")]
fn voting_fails(
    world: &mut DaoWorld,
    step: &Step,
    contract: Account,
    voting_id: u32,
    creator: Account,
) {
    conduct_voting(world, step, contract, voting_id, creator, Choice::Against);
}

#[then(expr = "{account} voting with id {int} is canceled")]
fn assert_voting_is_cancelled(world: &mut DaoWorld, contract: Account, voting_id: u32) {
    let voting = world.get_voting(&contract, voting_id);
    let state = voting.state();
    assert_eq!(
        state,
        &VotingState::Canceled,
        "Voting status is {:?}, but should be canceled",
        state,
    );
}

fn conduct_voting(
    world: &mut DaoWorld,
    step: &Step,
    contract: Account,
    voting_id: u32,
    creator: Account,
    choice: Choice,
) {
    // creator starts voting
    let rows = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in rows {
        let voting: Voting = row.as_slice().into();
        world.create_voting(creator, voting);
    }
    let stake = "500".parse().unwrap();
    let voting_type = VotingType::Informal;

    // voters vote in favor
    (2..4)
        .map(Account::VA)
        .map(|voter| Ballot {
            voter,
            stake,
            choice,
            voting_id,
            voting_type,
        })
        .for_each(|ballot| {
            world.vote(&contract, &ballot);
        });

    // 5 days passed
    world.advance_time(to_milliseconds(5, TimeUnit::Days));

    // informal voting ends
    world.finish_voting(&contract, voting_id, Some(voting_type));

    // 2 days passed
    world.advance_time(to_milliseconds(2, TimeUnit::Days));

    // voters vote in favor
    let voting_type = VotingType::Formal;
    (2..4)
        .map(Account::VA)
        .map(|voter| Ballot {
            voter,
            stake,
            choice,
            voting_id,
            voting_type,
        })
        .for_each(|ballot| world.vote(&contract, &ballot));

    // 5 days passed
    world.advance_time(to_milliseconds(5, TimeUnit::Days));

    // formal voting ends
    world.finish_voting(&contract, voting_id, Some(voting_type));
}
