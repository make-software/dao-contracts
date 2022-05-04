//! Voting struct with logic for governance voting
use crate::voting::ballot::{Choice, VotingId};
use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
};
use casper_types::{RuntimeArgs, U256};

/// Result of a Voting
#[derive(PartialEq)]
pub enum VotingResult {
    InFavor,
    Against,
    QuorumNotReached,
}

/// Type of Voting (Formal or Informal)
pub enum VotingType {
    Informal,
    Formal,
}

/// Finished Voting summary
#[allow(dead_code)]
pub struct VotingSummary {
    result: VotingResult,
    ty: VotingType,
    informal_voting_id: VotingId,
    formal_voting_id: Option<VotingId>,
}

impl VotingSummary {
    pub fn new(
        result: VotingResult,
        ty: VotingType,
        informal_voting_id: VotingId,
        formal_voting_id: Option<VotingId>,
    ) -> Self {
        Self {
            result,
            ty,
            informal_voting_id,
            formal_voting_id,
        }
    }

    pub fn is_voting_process_finished(&self) -> bool {
        match self.ty {
            VotingType::Informal => self.is_rejected(),
            VotingType::Formal => true,
        }
    }

    pub fn formal_voting_id(&self) -> Option<VotingId> {
        self.formal_voting_id
    }

    fn is_rejected(&self) -> bool {
        vec![VotingResult::Against, VotingResult::QuorumNotReached].contains(&self.result)
    }
}

/// Voting configuration, created and persisted since voting start
#[derive(Debug, Default, Clone, CLTyped, ToBytes, FromBytes, PartialEq)]
pub struct VotingConfiguration {
    pub formal_voting_quorum: U256,
    pub formal_voting_time: u64,
    pub informal_voting_quorum: U256,
    pub informal_voting_time: u64,
    pub minimum_governance_reputation: U256,
    pub contract_to_call: Option<Address>,
    pub entry_point: String,
    pub runtime_args: RuntimeArgs,
}

/// Voting struct
#[derive(Debug, Default, Clone, CLTyped, ToBytes, FromBytes, PartialEq)]
pub struct Voting {
    voting_id: VotingId,
    completed: bool,
    stake_in_favor: U256,
    stake_against: U256,
    start_time: u64,
    informal_voting_id: VotingId,
    formal_voting_id: Option<VotingId>,
    voting_configuration: VotingConfiguration,
}

impl Voting {
    /// Creates new Voting with immutable VotingConfiguration
    pub fn new(
        voting_id: U256,
        start_time: u64,
        voting_configuration: VotingConfiguration,
    ) -> Self {
        Voting {
            voting_id,
            completed: false,
            stake_in_favor: U256::zero(),
            stake_against: U256::zero(),
            start_time,
            informal_voting_id: voting_id,
            formal_voting_id: None,
            voting_configuration,
        }
    }

    /// Returns the type of voting
    pub fn get_voting_type(&self) -> VotingType {
        if self.voting_id == self.informal_voting_id {
            VotingType::Informal
        } else {
            VotingType::Formal
        }
    }

    /// Creates new formal voting from self, cloning existing VotingConfiguration
    pub fn create_formal_voting(&self, new_voting_id: U256, start_time: u64) -> Self {
        let mut voting = self.clone();
        voting.formal_voting_id = Some(new_voting_id);
        voting.voting_id = new_voting_id;
        voting.start_time = start_time;
        voting.stake_against = U256::zero();
        voting.stake_in_favor = U256::zero();
        voting.completed = false;
        voting
    }

    pub fn can_be_completed(&self, block_time: u64) -> bool {
        !self.completed && !self.is_in_time(block_time)
    }

    /// Sets voting as completed, optionally saves id of newly created formal voting
    pub fn complete(&mut self, formal_voting_id: Option<U256>) {
        if formal_voting_id.is_some() {
            self.formal_voting_id = formal_voting_id
        }
        self.completed = true;
    }

    /// Returns if voting is still in voting phase
    pub fn is_in_time(&self, block_time: u64) -> bool {
        match self.get_voting_type() {
            VotingType::Informal => {
                self.start_time + self.voting_configuration.informal_voting_time < block_time
            }
            VotingType::Formal => {
                self.start_time + self.voting_configuration.formal_voting_time < block_time
            }
        }
    }

    pub fn is_in_favor(&self) -> bool {
        self.stake_in_favor >= self.stake_against
    }

    /// Depending on the result of the voting, returns the amount of reputation staked on the winning side
    pub fn get_winning_stake(&self) -> U256 {
        match self.is_in_favor() {
            true => self.stake_in_favor,
            false => self.stake_against,
        }
    }

    pub fn get_quorum(&self) -> U256 {
        match self.get_voting_type() {
            VotingType::Informal => self.voting_configuration.informal_voting_quorum,
            VotingType::Formal => self.voting_configuration.formal_voting_quorum,
        }
    }

    pub fn get_result(&self, voters_number: u32) -> VotingResult {
        if self.get_quorum().as_u32() > voters_number {
            VotingResult::QuorumNotReached
        } else if self.is_in_favor() {
            VotingResult::InFavor
        } else {
            VotingResult::Against
        }
    }

    pub fn stake(&mut self, stake: U256, choice: Choice) {
        // overflow is not possible due to reputation token having U256 as max
        match choice {
            Choice::InFavor => self.stake_in_favor += stake,
            Choice::Against => self.stake_against += stake,
        }
    }

    pub fn total_stake(&self) -> U256 {
        // overflow is not possible due to reputation token having U256 as max
        self.stake_in_favor + self.stake_against
    }

    /// Get the voting's voting id.
    pub fn voting_id(&self) -> U256 {
        self.voting_id
    }

    /// Get the voting's completed.
    pub fn completed(&self) -> bool {
        self.completed
    }

    /// Get the voting's stake in favor.
    pub fn stake_in_favor(&self) -> U256 {
        self.stake_in_favor
    }

    /// Get the voting's stake against.
    pub fn stake_against(&self) -> U256 {
        self.stake_against
    }

    /// Get the voting's informal voting id.
    pub fn informal_voting_id(&self) -> U256 {
        self.informal_voting_id
    }

    /// Get the voting's formal voting id.
    pub fn formal_voting_id(&self) -> Option<U256> {
        self.formal_voting_id
    }

    /// Get the voting's formal voting quorum.
    pub fn formal_voting_quorum(&self) -> U256 {
        self.voting_configuration.formal_voting_quorum
    }

    /// Get the voting's informal voting quorum.
    pub fn informal_voting_quorum(&self) -> U256 {
        self.voting_configuration.informal_voting_quorum
    }

    /// Get the voting's formal voting time.
    pub fn formal_voting_time(&self) -> u64 {
        self.voting_configuration.formal_voting_time
    }

    /// Get the voting's informal voting time.
    pub fn informal_voting_time(&self) -> u64 {
        self.voting_configuration.informal_voting_time
    }

    /// Get the voting's contract to call.
    pub fn contract_to_call(&self) -> Option<Address> {
        self.voting_configuration.contract_to_call
    }

    /// Get a reference to the voting's entry point.
    pub fn entry_point(&self) -> &str {
        &self.voting_configuration.entry_point
    }

    /// Get a reference to the voting's runtime args.
    pub fn runtime_args(&self) -> &RuntimeArgs {
        &self.voting_configuration.runtime_args
    }

    /// Get the voting's minimum governance reputation.
    pub fn minimum_governance_reputation(&self) -> U256 {
        self.voting_configuration.minimum_governance_reputation
    }
}

#[test]
fn test_voting_serialization() {
    use casper_types::bytesrepr::FromBytes;
    use casper_types::bytesrepr::ToBytes;

    let voting = Voting {
        voting_id: VotingId::from(1),
        completed: false,
        stake_in_favor: U256::zero(),
        stake_against: U256::zero(),
        start_time: 123,
        informal_voting_id: VotingId::from(1),
        formal_voting_id: None,
        voting_configuration: VotingConfiguration {
            formal_voting_quorum: U256::from(2),
            formal_voting_time: 2,
            informal_voting_quorum: U256::from(2),
            informal_voting_time: 2,
            minimum_governance_reputation: U256::from(2),
            contract_to_call: None,
            entry_point: "update_variable".into(),
            runtime_args: RuntimeArgs::new(),
        },
    };

    let (voting2, _bytes) = Voting::from_bytes(&voting.to_bytes().unwrap()).unwrap();

    assert_eq!(voting.voting_id(), voting2.voting_id());
    assert_eq!(voting.informal_voting_id, voting2.informal_voting_id);
    assert_eq!(voting.formal_voting_id, voting2.formal_voting_id);
    assert_eq!(
        voting.voting_configuration.informal_voting_quorum,
        voting2.voting_configuration.informal_voting_quorum
    );
    assert_eq!(
        voting.voting_configuration.formal_voting_quorum,
        voting2.voting_configuration.formal_voting_quorum
    );
    assert_eq!(voting.stake_against, voting2.stake_against);
    assert_eq!(voting.stake_in_favor, voting2.stake_in_favor);
    assert_eq!(voting.completed, voting2.completed);
    assert_eq!(
        voting.voting_configuration.contract_to_call,
        voting2.voting_configuration.contract_to_call
    );
    assert_eq!(
        voting.voting_configuration.entry_point,
        voting2.voting_configuration.entry_point
    );
    assert_eq!(
        voting.voting_configuration.runtime_args,
        voting2.voting_configuration.runtime_args
    );
    assert_eq!(
        voting.voting_configuration.formal_voting_time,
        voting2.voting_configuration.formal_voting_time
    );
    assert_eq!(
        voting.voting_configuration.informal_voting_time,
        voting2.voting_configuration.informal_voting_time
    );
    assert_eq!(
        voting.voting_configuration.minimum_governance_reputation,
        voting2.voting_configuration.minimum_governance_reputation
    );
    assert_eq!(voting.start_time, voting2.start_time);
}
