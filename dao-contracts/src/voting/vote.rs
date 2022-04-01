use casper_dao_utils::Address;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLType, CLTyped, U256,
};

pub type VotingId = U256;

#[derive(Debug)]
pub struct Vote {
    pub voter: Option<Address>,
    pub voting_id: VotingId,
    pub choice: bool,
    pub stake: U256,
}

impl Default for Vote {
    fn default() -> Self {
        Self {
            voter: None,
            voting_id: VotingId::from(0),
            choice: false,
            stake: U256::from(0),
        }
    }
}

impl ToBytes for Vote {
    fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
        let mut vec = Vec::with_capacity(self.serialized_length());
        vec.extend(self.voter.to_bytes()?);
        vec.extend(self.voting_id.to_bytes()?);
        vec.extend(self.choice.to_bytes()?);
        vec.extend(self.stake.to_bytes()?);
        Ok(vec)
    }

    fn serialized_length(&self) -> usize {
        let mut size = 0;
        size += self.voter.serialized_length();
        size += self.voting_id.serialized_length();
        size += self.choice.serialized_length();
        size += self.stake.serialized_length();
        size
    }
}

impl FromBytes for Vote {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
        let (voter, bytes) = FromBytes::from_bytes(bytes)?;
        let (voting_id, bytes) = FromBytes::from_bytes(bytes)?;
        let (choice, bytes) = FromBytes::from_bytes(bytes)?;
        let (stake, bytes) = FromBytes::from_bytes(bytes)?;
        let value = Vote {
            voter,
            voting_id,
            choice,
            stake,
        };
        Ok((value, bytes))
    }
}

impl CLTyped for Vote {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

#[test]
fn test_vote_serialization() {
    use casper_types::account::AccountHash;
    let address = Address::Account(AccountHash::default());

    let vote = Vote {
        voter: Some(address),
        voting_id: U256::from(123),
        choice: true,
        stake: U256::from(456),
    };

    let (vote2, _bytes) = Vote::from_bytes(&vote.to_bytes().unwrap()).unwrap();
    assert_eq!(vote.voter, vote2.voter);
    assert_eq!(vote.voting_id, vote2.voting_id);
    assert_eq!(vote.choice, vote2.choice);
    assert_eq!(vote.stake, vote2.stake);
}
