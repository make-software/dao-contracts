//! Set of voting events.
use crate::configuration::Configuration;
use crate::voting::ballot::{Ballot, Choice};
use crate::voting::types::VotingId;
use crate::voting::voting_engine::voting_state_machine::{
    Stats, VotingResult, VotingStateMachine, VotingType,
};
use odra::types::{Address, Balance, BlockTime};
use odra::{Event, OdraType};
use std::collections::BTreeMap;

/// Represents an explanation for a particular action (mint, burn, stake).
#[derive(OdraType, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
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

/// Event thrown after ballot is cast.
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
    pub stake: Balance,
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

/// Event thrown after voting is created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct VotingCreatedInfo {
    /// The creator's address.
    pub creator: Address,
    /// The amount of tokens staked by the creator
    pub stake: Option<Balance>,
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
    pub config_total_onboarded: Balance,
    /// Configuration value - [is the time between votes doubled](crate::config::Configuration::should_double_time_between_votings()).
    pub config_double_time_between_votings: bool,
    /// Configuration value - [voting clearness delta](crate::config::Configuration::voting_clearness_delta()).
    pub config_voting_clearness_delta: Balance,
    /// Configuration value - [the time between informal/formal voting](crate::config::Configuration::time_between_informal_and_formal_voting()).
    pub config_time_between_informal_and_formal_voting: BlockTime,
}

impl VotingCreatedInfo {
    pub fn new(
        creator: Address,
        voting_id: VotingId,
        stake: Option<Balance>,
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

/// Event thrown when voting ends.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct VotingEnded {
    pub voting_id: VotingId,
    pub voting_type: VotingType,
    pub voting_result: VotingResult,
    pub stake_in_favor: Balance,
    pub stake_against: Balance,
    pub unbound_stake_in_favor: Balance,
    pub unbound_stake_against: Balance,
    pub votes_in_favor: u32,
    pub votes_against: u32,
    pub unstakes: BTreeMap<(Address, Reason), Balance>,
    pub stakes: BTreeMap<(Address, Reason), Balance>,
    pub burns: BTreeMap<(Address, Reason), Balance>,
    pub mints: BTreeMap<(Address, Reason), Balance>,
}

impl VotingEnded {
    pub fn new(
        voting: &VotingStateMachine,
        voting_result: VotingResult,
        stats: &Stats,
        unstakes: BTreeMap<(Address, Reason), Balance>,
        stakes: BTreeMap<(Address, Reason), Balance>,
        burns: BTreeMap<(Address, Reason), Balance>,
        mints: BTreeMap<(Address, Reason), Balance>,
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
    pub stake: Balance,
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
    pub unstakes: BTreeMap<Address, Balance>,
}

impl VotingCanceled {
    pub fn new(
        voting_id: VotingId,
        voting_type: VotingType,
        unstakes: BTreeMap<Address, Balance>,
    ) -> Self {
        Self {
            voting_id,
            voting_type,
            unstakes,
        }
    }
}
