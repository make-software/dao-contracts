use cucumber::{gherkin::Step, given, then};

use crate::common::{
    params::{
        voting::{Ballot, BallotBuilder, Voting, VotingType},
        Account,
        Contract,
    },
    DaoWorld,
};

#[given(expr = "{account} starts voting with the following config")]
fn voting_setup(world: &mut DaoWorld, step: &Step, creator: Account) {
    let voting: &Vec<String> = step.table.as_ref().unwrap().rows.get(1).unwrap();
    let voting: Voting = voting.into();

    world.create_voting(creator, voting);
}

#[then(expr = "voters vote in {contract}'s {voting_type} voting with id {int}")]
fn voting(
    world: &mut DaoWorld,
    step: &Step,
    contract: Contract,
    voting_type: VotingType,
    voting_id: u32,
) {
    let rows = step.table.as_ref().unwrap().rows.iter().skip(1);

    for row in rows {
        let ballot: Ballot = BallotBuilder::default()
            .with_voting_id(voting_id)
            .with_voting_type(voting_type)
            .build(row);

        world.vote(&contract, ballot);
    }
}
