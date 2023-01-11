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
    Against,
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

/// Ballot struct
#[derive(Debug, FromBytes, ToBytes, CLTyped, Clone)]
pub struct Ballot {
    pub voter: Address,
    pub voting_id: VotingId,
    pub voting_type: VotingType,
    pub choice: Choice,
    pub stake: U512,
    pub unbounded: bool,
    pub canceled: bool,
}

impl Ballot {
    pub fn new(
        voter: Address,
        voting_id: VotingId,
        voting_type: VotingType,
        choice: Choice,
        stake: U512,
        unbounded: bool,
        canceled: bool,
    ) -> Self {
        Self {
            voter,
            voting_id,
            voting_type,
            choice,
            stake,
            unbounded,
            canceled,
        }
    }
}

/// ShortenedBallot struct
///
/// Derives from the [`Ballot`] struct.
/// Contains only the essential fields from the original [`Ballot`] required in cross-contract communication.
#[derive(Debug, FromBytes, ToBytes, CLTyped, Clone)]
pub struct ShortenedBallot {
    pub voter: Address,
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
        unbounded: false,
        canceled: false,
    };

    let (deserialized_vote, _) = Ballot::from_bytes(&vote.to_bytes().unwrap()).unwrap();
    assert_eq!(vote.voter, deserialized_vote.voter);
    assert_eq!(vote.voting_id, deserialized_vote.voting_id);
    assert_eq!(vote.choice, deserialized_vote.choice);
    assert_eq!(vote.stake, deserialized_vote.stake);
}
