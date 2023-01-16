use std::collections::BTreeMap;

use casper_dao_utils::{
    casper_dao_macros::{CLTyped, Event, FromBytes, ToBytes},
    Address,
    BlockTime,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes, U32_SERIALIZED_LENGTH},
    CLTyped,
    U512,
};

use super::voting_state_machine::{Stats, VotingResult, VotingType};
use crate::{
    voting::{ballot::Choice, types::VotingId, Ballot},
    Configuration,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Reason {
    InformalFinished = 1,
    FormalFinished = 2,
    FormalWon = 3,
    FormalLost = 4,
}

impl ToBytes for Reason {
    fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
        (self.clone() as u32).to_bytes()
    }

    fn serialized_length(&self) -> usize {
        U32_SERIALIZED_LENGTH
    }
}

impl FromBytes for Reason {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
        let (num, bytes) = u32::from_bytes(bytes)?;
        let reason = match num {
            1 => Self::InformalFinished,
            2 => Self::FormalFinished,
            3 => Self::FormalWon,
            4 => Self::FormalLost,
            _ => return Err(casper_types::bytesrepr::Error::Formatting),
        };
        Ok((reason, bytes))
    }
}

impl CLTyped for Reason {
    fn cl_type() -> casper_types::CLType {
        <u32>::cl_type()
    }
}

/// Event thrown after ballot is cast
#[derive(Debug, PartialEq, Eq, Event)]
pub struct BallotCast {
    pub voter: Address,
    pub voting_id: VotingId,
    pub voting_type: VotingType,
    pub choice: Choice,
    pub stake: U512,
}

impl BallotCast {
    pub fn new(ballot: &Ballot) -> Self {
        BallotCast {
            voter: ballot.voter,
            voting_id: ballot.voting_id,
            voting_type: ballot.voting_type,
            choice: ballot.choice,
            stake: ballot.stake,
        }
    }
}

/// Event thrown after voting is created
#[derive(Debug, PartialEq, Eq, ToBytes, FromBytes, CLTyped)]
pub struct VotingCreatedInfo {
    pub creator: Address,
    pub stake: Option<U512>,
    pub voting_id: VotingId,
    pub config_informal_quorum: u32,
    pub config_informal_voting_time: u64,
    pub config_formal_quorum: u32,
    pub config_formal_voting_time: u64,
    pub config_total_onboarded: U512,
    pub config_double_time_between_votings: bool,
    pub config_voting_clearness_delta: U512,
    pub config_time_between_informal_and_formal_voting: BlockTime,
}

impl VotingCreatedInfo {
    pub fn new(
        creator: Address,
        voting_id: VotingId,
        stake: Option<U512>,
        config: &Configuration,
    ) -> Self {
        Self {
            creator,
            stake,
            voting_id,
            config_informal_quorum: config.informal_voting_quorum(),
            config_informal_voting_time: config.informal_voting_time(),
            config_formal_quorum: config.formal_voting_quorum(),
            config_formal_voting_time: config.formal_voting_time(),
            config_total_onboarded: config.total_onboarded(),
            config_voting_clearness_delta: config.voting_clearness_delta(),
            config_double_time_between_votings: config.should_double_time_between_votings(),
            config_time_between_informal_and_formal_voting: config
                .time_between_informal_and_formal_voting(),
        }
    }
}

/// Event thrown when voting ends
#[derive(Debug, PartialEq, Eq, Event)]
pub struct VotingEnded {
    pub voting_id: VotingId,
    pub voting_type: VotingType,
    pub voting_result: VotingResult,
    pub stake_in_favor: U512,
    pub stake_against: U512,
    pub unbound_stake_in_favor: U512,
    pub unbound_stake_against: U512,
    pub votes_in_favor: u32,
    pub votes_against: u32,
    pub unstakes: BTreeMap<(Address, Reason), U512>,
    pub stakes: BTreeMap<(Address, Reason), U512>,
    pub burns: BTreeMap<(Address, Reason), U512>,
    pub mints: BTreeMap<(Address, Reason), U512>,
}

impl VotingEnded {
    pub fn new(
        voting_id: VotingId,
        voting_type: VotingType,
        voting_result: VotingResult,
        stats: &Stats,
        unstakes: BTreeMap<(Address, Reason), U512>,
        stakes: BTreeMap<(Address, Reason), U512>,
        burns: BTreeMap<(Address, Reason), U512>,
        mints: BTreeMap<(Address, Reason), U512>,
    ) -> Self {
        Self {
            voting_id,
            voting_type,
            voting_result,
            stake_in_favor: stats.stake_in_favor,
            stake_against: stats.stake_against,
            unbound_stake_in_favor: stats.unbound_stake_in_favor,
            unbound_stake_against: stats.unbound_stake_against,
            votes_in_favor: stats.votes_in_favor,
            votes_against: stats.votes_against,
            unstakes,
            stakes,
            burns,
            mints,
        }
    }
}

/// Event thrown after ballot is canceled during full slashing.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct BallotCanceled {
    pub voter: Address,
    pub voting_id: VotingId,
    pub voting_type: VotingType,
    pub choice: Choice,
    pub stake: U512,
}

impl BallotCanceled {
    pub fn new(ballot: &Ballot) -> Self {
        Self {
            voter: ballot.voter,
            voting_id: ballot.voting_id,
            voting_type: ballot.voting_type,
            choice: ballot.choice,
            stake: ballot.stake,
        }
    }
}

/// Event thrown after voting is canceled during full slashing.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct VotingCanceled {
    pub voting_id: VotingId,
    pub voting_type: VotingType,
    pub unstakes: BTreeMap<Address, U512>,
}

impl VotingCanceled {
    pub fn new(
        voting_id: VotingId,
        voting_type: VotingType,
        unstakes: BTreeMap<Address, U512>,
    ) -> Self {
        Self {
            voting_id,
            voting_type,
            unstakes,
        }
    }
}
