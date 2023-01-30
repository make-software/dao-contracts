use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
};
use casper_types::U512;

use super::voting_state_machine::VotingType;
use crate::voting::VotingId;

/// Choice enum, can be converted to bool using `is_in_favor()`
#[derive(Debug, FromBytes, ToBytes, CLTyped, PartialEq, Eq, Clone, Copy)]
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

/// Represents user's vote.
#[derive(Debug, FromBytes, ToBytes, CLTyped, Clone)]
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
    pub stake: U512,
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
        stake: U512,
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

/// Short version of [`Ballot`] struct.
///
/// Derives from the [`Ballot`] struct.
/// Contains only the essential fields from the original [`Ballot`] required in cross-contract communication.
#[derive(Debug, FromBytes, ToBytes, CLTyped, Clone)]
pub struct ShortenedBallot {
    /// The voter's address.
    pub voter: Address,
    /// Vote power.
    pub stake: U512,
}

impl From<Ballot> for ShortenedBallot {
    fn from(value: Ballot) -> Self {
        Self {
            voter: value.voter,
            stake: value.stake,
        }
    }
}

#[cfg(test)]
#[test]
fn test_vote_serialization() {
    use casper_types::{
        account::AccountHash,
        bytesrepr::{FromBytes, ToBytes},
    };
    let address = Address::Account(AccountHash::default());

    let vote = Ballot {
        voter: address,
        voting_id: 123,
        voting_type: VotingType::Formal,
        choice: Choice::InFavor,
        stake: U512::from(456),
        unbound: false,
        canceled: false,
    };

    let (deserialized_vote, _) = Ballot::from_bytes(&vote.to_bytes().unwrap()).unwrap();
    assert_eq!(vote.voter, deserialized_vote.voter);
    assert_eq!(vote.voting_id, deserialized_vote.voting_id);
    assert_eq!(vote.choice, deserialized_vote.choice);
    assert_eq!(vote.stake, deserialized_vote.stake);
}
