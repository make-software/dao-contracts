use casper_dao_utils::Address;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLType, CLTyped, RuntimeArgs, U256,
};

pub type VotingId = U256;

#[derive(Debug, Default, Clone)]
pub struct Voting {
    pub voting_id: VotingId,
    pub completed: bool,
    pub stake_in_favor: U256,
    pub stake_against: U256,
    pub finish_time: U256,
    pub informal_voting_id: VotingId,
    pub formal_voting_id: Option<VotingId>,
    pub formal_voting_quorum: U256,
    pub formal_voting_time: U256,
    pub informal_voting_quorum: U256,
    pub informal_voting_time: U256,
    pub minimum_governance_reputation: U256,
    pub contract_to_call: Option<Address>,
    pub entry_point: String,
    pub runtime_args: RuntimeArgs,
}

impl ToBytes for Voting {
    fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
        let mut vec = Vec::with_capacity(self.serialized_length());
        vec.extend(self.voting_id.to_bytes()?);
        vec.extend(self.completed.to_bytes()?);
        vec.extend(self.stake_in_favor.to_bytes()?);
        vec.extend(self.stake_against.to_bytes()?);
        vec.extend(self.finish_time.to_bytes()?);
        vec.extend(self.formal_voting_id.to_bytes()?);
        vec.extend(self.informal_voting_id.to_bytes()?);
        vec.extend(self.formal_voting_quorum.to_bytes()?);
        vec.extend(self.formal_voting_time.to_bytes()?);
        vec.extend(self.informal_voting_quorum.to_bytes()?);
        vec.extend(self.informal_voting_time.to_bytes()?);
        vec.extend(self.minimum_governance_reputation.to_bytes()?);
        vec.extend(self.contract_to_call.to_bytes()?);
        vec.extend(self.entry_point.to_bytes()?);
        vec.extend(self.runtime_args.to_bytes()?);
        Ok(vec)
    }

    fn serialized_length(&self) -> usize {
        let mut size = 0;
        size += self.voting_id.serialized_length();
        size += self.completed.serialized_length();
        size += self.stake_in_favor.serialized_length();
        size += self.stake_against.serialized_length();
        size += self.finish_time.serialized_length();
        size += self.formal_voting_id.serialized_length();
        size += self.informal_voting_id.serialized_length();
        size += self.formal_voting_quorum.serialized_length();
        size += self.formal_voting_time.serialized_length();
        size += self.informal_voting_quorum.serialized_length();
        size += self.informal_voting_time.serialized_length();
        size += self.minimum_governance_reputation.serialized_length();
        size += self.contract_to_call.serialized_length();
        size += self.entry_point.serialized_length();
        size += self.runtime_args.serialized_length();
        size
    }
}

impl FromBytes for Voting {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
        let (voting_id, bytes) = FromBytes::from_bytes(bytes)?;
        let (completed, bytes) = FromBytes::from_bytes(bytes)?;
        let (stake_in_favor, bytes) = FromBytes::from_bytes(bytes)?;
        let (stake_against, bytes) = FromBytes::from_bytes(bytes)?;
        let (finish_time, bytes) = FromBytes::from_bytes(bytes)?;
        let (formal_voting_id, bytes) = FromBytes::from_bytes(bytes)?;
        let (informal_voting_id, bytes) = FromBytes::from_bytes(bytes)?;
        let (formal_voting_quorum, bytes) = FromBytes::from_bytes(bytes)?;
        let (formal_voting_time, bytes) = FromBytes::from_bytes(bytes)?;
        let (informal_voting_quorum, bytes) = FromBytes::from_bytes(bytes)?;
        let (informal_voting_time, bytes) = FromBytes::from_bytes(bytes)?;
        let (minimum_governance_reputation, bytes) = FromBytes::from_bytes(bytes)?;
        let (contract_to_call, bytes) = FromBytes::from_bytes(bytes)?;
        let (entry_point, bytes) = FromBytes::from_bytes(bytes)?;
        let (runtime_args, bytes) = FromBytes::from_bytes(bytes)?;
        let value = Voting {
            voting_id,
            formal_voting_id,
            informal_voting_id,
            formal_voting_quorum,
            formal_voting_time,
            informal_voting_quorum,
            informal_voting_time,
            minimum_governance_reputation,
            stake_in_favor,
            stake_against,
            contract_to_call,
            entry_point,
            runtime_args,
            completed,
            finish_time,
        };
        Ok((value, bytes))
    }
}

impl CLTyped for Voting {
    fn cl_type() -> CLType {
        CLType::Any
    }
}
