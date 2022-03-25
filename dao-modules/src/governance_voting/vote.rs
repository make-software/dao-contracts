use casper_dao_utils::Address;
use casper_types::{U256, bytesrepr::{ToBytes, FromBytes}, CLType, CLTyped, account::{AccountHash}};

use super::voting::VotingId;

#[derive(Debug)]
pub struct Vote {
    pub voter: Address,
    pub voting_id: VotingId,
    pub choice: bool,
    pub stake: U256
}

impl Default for Vote {
    fn default() -> Self {
        Self {
            voter: Address::Account(AccountHash::default()),
            voting_id: U256::from(0),
            choice: false,
            stake: U256::from(0)
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
        return size;
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