use std::time::Duration;

use casper_dao_utils::Error;
use cucumber::{gherkin::Step, given, then, when};

use crate::common::{
    params::{
        voting::{BallotBuilder, Voting, VotingType},
        Account,
        Contract,
    },
    DaoWorld,
};

#[when(expr = "{account} starts voting with the following config")]
#[given(expr = "{account} starts voting with the following config")]
fn voting_setup(world: &mut DaoWorld, step: &Step, creator: Account) {
    let voting: &Vec<String> = step.table.as_ref().unwrap().rows.get(1).unwrap();
    let voting: Voting = voting.into();
    let _ = world.checked_create_voting(creator, voting);
}

#[when(expr = "voters vote in {contract}'s {voting_type} voting with id {int}")]
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
    .for_each(|ballot| world.vote(&contract, &ballot));
}

#[when(expr = "{voting_type} voting with id {int} ends in {contract} contract")]
fn end_voting(world: &mut DaoWorld, voting_type: VotingType, voting_id: u32, contract: Contract) {
    let voting_duration = Duration::from_secs(432000000u64);
    world.env.advance_block_time_by(voting_duration);
    world.finish_voting(&contract, voting_id, Some(voting_type));
}

#[then(expr = "formal voting with id {int} in {contract} contract does not start")]
fn assert_formal_voting_does_not_start(world: &mut DaoWorld, voting_id: u32, contract: Contract) {
    let voting = world.get_voting(&contract, voting_id, VotingType::Informal);

    assert_eq!(voting.formal_voting_id(), None);
}

#[then(expr = "informal voting with id {int} in {contract} contract does not start")]
fn assert_informal_voting_does_not_start(world: &mut DaoWorld, voting_id: u32, contract: Contract) {
    let voting = world.checked_get_voting(&contract, voting_id, VotingType::Informal);

    assert!(voting.is_none());
}

#[then(expr = "formal voting with id {int} in {contract} contract starts")]
fn assert_formal_voting_starts(world: &mut DaoWorld, voting_id: u32, contract: Contract) {
    let voting = world.get_voting(&contract, voting_id, VotingType::Informal);

    assert_eq!(voting.formal_voting_id(), Some(1));
}

#[then(expr = "votes in {contract}'s {voting_type} voting with id {int} fail")]
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
