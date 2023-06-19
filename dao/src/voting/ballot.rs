use crate::voting::types::VotingId;
use crate::voting::voting_engine::voting_state_machine::VotingType;
use odra::types::{Address, Balance};
use odra::OdraType;

/// Represents user's vote.
#[derive(OdraType)]
pub struct Ballot {
    /// The voter's address.
    pub voter: Address,
    /// A unique voting id.
    pub voting_id: VotingId,
    /// Voting type.
    pub voting_type: VotingType,
    /// Selected option.
    pub choice: Choice,
    /// Vote power.
    pub stake: Balance,
    /// Indicates if the vote counts in the total voting stake.
    pub unbound: bool,
    /// Indicates if it reverts the previous ballot casted by the voter.
    pub canceled: bool,
}

impl Ballot {
    pub fn new(
        voter: Address,
        voting_id: VotingId,
        voting_type: VotingType,
        choice: Choice,
        stake: Balance,
        unbound: bool,
        canceled: bool,
    ) -> Self {
        Self {
            voter,
            voting_id,
            voting_type,
            choice,
            stake,
            unbound,
            canceled,
        }
    }
}

/// Choice enum, can be converted to bool using `is_in_favor()`
#[derive(OdraType, Copy, PartialEq, Eq, Debug)]
pub enum Choice {
    /// `No` vote.
    Against,
    /// `Yes` vote.
    InFavor,
}

impl Choice {
    pub fn is_in_favor(&self) -> bool {
        match self {
            Choice::InFavor => true,
            Choice::Against => false,
        }
    }

    pub fn is_against(&self) -> bool {
        !self.is_in_favor()
    }
}

/// Short version of [`Ballot`] struct.
///
/// Derives from the [`Ballot`] struct.
/// Contains only the essential fields from the original [`Ballot`] required in cross-contract communication.
#[derive(OdraType, Debug)]
pub struct ShortenedBallot {
    /// The voter's address.
    pub voter: Address,
    /// Vote power.
    pub stake: Balance,
}

impl From<Ballot> for ShortenedBallot {
    fn from(value: Ballot) -> Self {
        Self {
            voter: value.voter,
            stake: value.stake,
        }
    }
}
