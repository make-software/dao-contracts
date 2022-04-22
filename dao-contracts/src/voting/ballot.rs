use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
};
use casper_types::U256;

pub type VotingId = U256;
pub type AccountId = usize;
pub type ReputationAmount = usize;

#[derive(Debug, FromBytes, ToBytes, CLTyped, PartialEq, Clone, Copy)]
pub enum Choice {
    Against,
    InFavor,
}

impl Default for Choice {
    fn default() -> Self {
        Self::InFavor
    }
}

impl Choice {
    pub fn is_in_favor(&self) -> bool {
        match self {
            Choice::InFavor => true,
            Choice::Against => false,
        }
    }
}

#[derive(Debug, Default, FromBytes, ToBytes, CLTyped)]
pub struct Ballot {
    pub voter: Option<Address>,
    pub voting_id: VotingId,
    pub choice: Choice,
    pub stake: U256,
}

#[test]
fn test_vote_serialization() {
    use casper_types::account::AccountHash;
    use casper_types::bytesrepr::FromBytes;
    use casper_types::bytesrepr::ToBytes;
    let address = Address::Account(AccountHash::default());

    let vote = Ballot {
        voter: Some(address),
        voting_id: U256::from(123),
        choice: Choice::InFavor,
        stake: U256::from(456),
    };

    let (deserialized_vote, _) = Ballot::from_bytes(&vote.to_bytes().unwrap()).unwrap();
    assert_eq!(vote.voter, deserialized_vote.voter);
    assert_eq!(vote.voting_id, deserialized_vote.voting_id);
    assert_eq!(vote.choice, deserialized_vote.choice);
    assert_eq!(vote.stake, deserialized_vote.stake);
}