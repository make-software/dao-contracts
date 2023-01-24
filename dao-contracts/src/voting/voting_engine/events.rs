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
    config::Configuration,
    voting::{ballot::Choice, types::VotingId, voting_state_machine::VotingStateMachine, Ballot},
};
/// Represents an explanation for a particular action (mint, burn, stake).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Reason {
    /// Informal voting finished.
    InformalFinished = 1,
    /// Voting process finished.
    FormalFinished = 2,
    /// Voting process finished, voters voted in favor.
    FormalWon = 3,
    /// Voting process finished, voters voted against.
    FormalLost = 4,
}

impl ToBytes for Reason {
    fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
        (*self as u32).to_bytes()
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
    /// The voter's address.
    pub voter: Address,
    /// A unique voting id.
    pub voting_id: VotingId,
    /// Voting type (Formal/Informal).
    pub voting_type: VotingType,
    /// Selected option.
    pub choice: Choice,
    /// Vote power.
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
    /// The creator's address.
    pub creator: Address,
    /// The amount of tokens staked by the creator
    pub stake: Option<U512>,
    /// A unique voting id.
    pub voting_id: VotingId,
    /// Configuration value - [informal voting quorum](crate::config::Configuration::informal_voting_quorum()).
    pub config_informal_quorum: u32,
    /// Configuration value - [informal voting time](crate::config::Configuration::informal_voting_time()).
    pub config_informal_voting_time: u64,
    /// Configuration value - [formal voting quorum](crate::config::Configuration::formal_voting_quorum()).
    pub config_formal_quorum: u32,
    /// Configuration value - [formal voting time](crate::config::Configuration::formal_voting_time()).
    pub config_formal_voting_time: u64,
    /// Configuration value - [total number of onboarded users](crate::config::Configuration::total_onboarded()).
    pub config_total_onboarded: U512,
    /// Configuration value - [is the time between votes doubled](crate::config::Configuration::should_double_time_between_votings()).
    pub config_double_time_between_votings: bool,
    /// Configuration value - [voting clearness delta](crate::config::Configuration::voting_clearness_delta()).
    pub config_voting_clearness_delta: U512,
    /// Configuration value - [the time between informal/formal voting](crate::config::Configuration::time_between_informal_and_formal_voting()).
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
        voting: &VotingStateMachine,
        voting_result: VotingResult,
        stats: &Stats,
        unstakes: BTreeMap<(Address, Reason), U512>,
        stakes: BTreeMap<(Address, Reason), U512>,
        burns: BTreeMap<(Address, Reason), U512>,
        mints: BTreeMap<(Address, Reason), U512>,
    ) -> Self {
        Self {
            voting_id: voting.voting_id(),
            voting_type: voting.voting_type(),
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
    /// The voter's address.
    pub voter: Address,
    /// A unique voting id.
    pub voting_id: VotingId,
    /// Voting type (Formal/Informal).
    pub voting_type: VotingType,
    /// Selected option.
    pub choice: Choice,
    /// Vote power.
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
    /// A unique voting id.
    pub voting_id: VotingId,
    /// Voting type (Formal/Informal).
    pub voting_type: VotingType,
    /// Map of voters' addresses to their canceled stakes.
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
