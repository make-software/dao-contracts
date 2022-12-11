use casper_dao_utils::Error;
use cucumber::{gherkin::Step, given, then, when};

use crate::common::{
    params::{
        voting::{BallotBuilder, Voting, VotingType},
        Account,
        Balance,
        Contract,
    },
    DaoWorld,
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

#[when(expr = "{account} creates random voting in {contract} with {balance} stake")]
fn create_random_voting(
    world: &mut DaoWorld,
    creator: Account,
    contract: Contract,
    stake: Balance,
) {
    world.create_random_voting(contract, creator, stake);
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

#[then(expr = "votes in {contract}'s     {voting_type} voting with id {int} fail")]
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
