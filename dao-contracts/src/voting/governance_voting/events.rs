use casper_dao_utils::{casper_dao_macros::Event, Address};
use casper_types::U256;

use crate::voting::ballot::{Choice, VotingId};

/// Event thrown after voting contract is created
#[derive(Debug, PartialEq, Event)]
pub struct VotingContractCreated {
    pub voter_contract: Address,
    pub variable_repo: Address,
    pub reputation_token: Address,
}

/// Event thrown after ballot is cast
#[derive(Debug, PartialEq, Event)]
pub struct BallotCast {
    pub voter: Address,
    pub voting_id: VotingId,
    pub choice: Choice,
    pub stake: U256,
}

/// Event thrown after voting is created
#[derive(Debug, PartialEq, Event)]
pub struct VotingCreated {
    pub creator: Address,
    pub voting_id: VotingId,
    pub stake: U256,
}

/// Event thrown when voting ends
#[derive(Debug, PartialEq, Event)]
pub struct VotingEnded {
    pub voting_id: U256,
    pub result: String,
    pub votes_count: U256,
    pub stake_in_favor: U256,
    pub stake_against: U256,
    pub informal_voting_id: VotingId,
    pub formal_voting_id: Option<VotingId>,
}
