use casper_dao_utils::Address;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLType, CLTyped, RuntimeArgs, U256,
};

use crate::voting::vote::VotingId;

pub enum VotingResult {
    InFavor,
    Against,
    QuorumNotReached,
    Unknown,
}

pub enum VotingType {
    Informal,
    Formal,
    Unknown,
}

#[derive(Debug, Default, Clone)]
pub struct Voting {
    pub voting_id: VotingId,
    pub completed: bool,
    pub stake_in_favor: U256,
    pub stake_against: U256,
    pub finish_time: u64,
    pub informal_voting_id: VotingId,
    pub formal_voting_id: Option<VotingId>,
    pub formal_voting_quorum: U256,
    pub formal_voting_time: u64,
    pub informal_voting_quorum: U256,
    pub informal_voting_time: u64,
    pub minimum_governance_reputation: U256,
    pub contract_to_call: Option<Address>,
    pub entry_point: String,
    pub runtime_args: RuntimeArgs,
}

impl Voting {
    pub fn get_voting_type(&self) -> VotingType {
        if Some(self.voting_id) == self.formal_voting_id {
            VotingType::Formal
        } else if self.voting_id == self.informal_voting_id {
            VotingType::Informal
        } else {
            VotingType::Unknown
        }
    }

    pub fn convert_to_formal(&self, new_voting_id: U256, block_time: u64) -> Self {
        let mut voting = self.clone();
        voting.formal_voting_id = Some(new_voting_id);
        voting.voting_id = new_voting_id;
        voting.finish_time = block_time + self.formal_voting_time;
        voting.stake_against = U256::zero();
        voting.stake_in_favor = U256::zero();
        voting.completed = false;
        voting
    }

    pub fn can_be_completed(&self, block_time: u64) -> bool {
        !self.completed && !self.is_in_time(block_time)
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn is_in_time(&self, block_time: u64) -> bool {
        self.finish_time < block_time
    }

    pub fn is_in_favor(&self) -> bool {
        self.stake_in_favor >= self.stake_against
    }

    pub fn get_winning_stake(&self) -> U256 {
        match self.is_in_favor() {
            true => self.stake_in_favor,
            false => self.stake_against,
        }
    }

    pub fn get_result(&self, voters_number: usize) -> VotingResult {
        match self.get_voting_type() {
            VotingType::Informal => {
                if voters_number < self.informal_voting_quorum.as_usize() {
                    VotingResult::QuorumNotReached
                } else if self.is_in_favor() {
                    VotingResult::InFavor
                } else {
                    VotingResult::Against
                }
            }
            VotingType::Formal => {
                if voters_number < self.formal_voting_quorum.as_usize() {
                    VotingResult::QuorumNotReached
                } else if self.is_in_favor() {
                    VotingResult::InFavor
                } else {
                    VotingResult::Against
                }
            }
            VotingType::Unknown => VotingResult::Unknown,
        }
    }
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
