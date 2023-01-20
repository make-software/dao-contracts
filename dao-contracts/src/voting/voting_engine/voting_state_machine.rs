//! Voting struct with logic for governance voting
use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
    BlockTime,
    ContractCall,
};
use casper_types::U512;

use crate::{
    config::Configuration,
    voting::{ballot::Choice, types::VotingId},
};

/// Result of a Voting
#[derive(PartialEq, Eq, Clone, CLTyped, FromBytes, ToBytes, Debug)]
pub enum VotingResult {
    InFavor,
    Against,
    QuorumNotReached,
    Canceled,
}

/// Type of Voting (Formal or Informal)
#[derive(CLTyped, FromBytes, ToBytes, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum VotingType {
    Informal,
    Formal,
}

/// State of Voting
#[derive(CLTyped, FromBytes, ToBytes, Debug, Clone, PartialEq, Eq)]
pub enum VotingState {
    Created,
    Informal,
    BetweenVotings,
    Formal,
    Finished,
    Canceled,
}

#[derive(CLTyped, FromBytes, ToBytes, Debug, Clone, PartialEq, Eq)]
pub enum VotingStateInTime {
    BeforeInformal,
    Informal,
    BetweenVotings,
    Formal,
    AfterFormal,
}

/// Finished Voting summary
#[allow(dead_code)]
#[derive(CLTyped, FromBytes, ToBytes, Clone, Debug)]
pub struct VotingSummary {
    result: VotingResult,
    ty: VotingType,
    voting_id: VotingId,
}

impl VotingSummary {
    pub fn new(result: VotingResult, ty: VotingType, voting_id: VotingId) -> Self {
        Self {
            result,
            ty,
            voting_id,
        }
    }

    pub fn is_voting_process_finished(&self) -> bool {
        match self.ty {
            VotingType::Informal => self.is_rejected(),
            VotingType::Formal => true,
        }
    }

    pub fn is_formal(&self) -> bool {
        self.voting_type() == VotingType::Formal
    }

    fn is_rejected(&self) -> bool {
        vec![VotingResult::Against, VotingResult::QuorumNotReached].contains(&self.result)
    }

    pub fn result(&self) -> VotingResult {
        self.result.clone()
    }

    /// Get a reference to the voting summary's ty.
    pub fn voting_type(&self) -> VotingType {
        self.ty
    }
}

#[derive(Debug, Clone, Default, CLTyped, ToBytes, FromBytes)]
pub struct Stats {
    pub stake_in_favor: U512,
    pub stake_against: U512,
    pub unbound_stake_in_favor: U512,
    pub unbound_stake_against: U512,
    pub votes_in_favor: u32,
    pub votes_against: u32,
}

/// Voting struct
#[derive(Debug, Clone, CLTyped, ToBytes, FromBytes)]
pub struct VotingStateMachine {
    voting_id: VotingId,
    state: VotingState,
    voting_type: VotingType,
    informal_stats: Stats,
    formal_stats: Stats,
    created_at: u64,
    creator: Address,
    configuration: Configuration,
}

impl VotingStateMachine {
    /// Creates new Voting with immutable VotingConfiguration
    pub fn new(
        voting_id: VotingId,
        created_at: u64,
        creator: Address,
        voting_configuration: Configuration,
    ) -> Self {
        VotingStateMachine {
            voting_id,
            state: VotingState::Created,
            voting_type: VotingType::Informal,
            informal_stats: Default::default(),
            formal_stats: Default::default(),
            created_at,
            creator,
            configuration: voting_configuration,
        }
    }

    pub fn complete_informal_voting(&mut self) {
        self.state = VotingState::Formal;
        if self.is_result_close() {
            self.configuration.double_time_between_votings();
        }
        self.voting_type = VotingType::Formal;
    }

    pub fn finish(&mut self) {
        self.state = VotingState::Finished;
    }

    pub fn cancel(&mut self) {
        self.state = VotingState::Canceled;
    }

    /// Returns the type of voting
    pub fn voting_type(&self) -> VotingType {
        self.voting_type
    }

    pub fn is_informal_without_stake(&self) -> bool {
        !self.voting_configuration().informal_stake_reputation()
            && self.voting_type() == VotingType::Informal
    }

    /// Returns if voting is still in voting phase
    pub fn is_in_time(&self, block_time: u64) -> bool {
        match self.voting_type() {
            VotingType::Informal => {
                let start_time = self.informal_voting_start_time();
                let voting_time = self.configuration.informal_voting_time();
                start_time + voting_time <= block_time
            }
            VotingType::Formal => {
                self.informal_voting_start_time() + self.configuration.formal_voting_time()
                    <= block_time
            }
        }
    }

    pub fn informal_voting_end_time(&self) -> BlockTime {
        self.informal_voting_start_time() + self.configuration.informal_voting_time()
    }

    pub fn time_between_votings_end_time(&self) -> BlockTime {
        self.informal_voting_end_time()
            + self.configuration.time_between_informal_and_formal_voting()
    }

    pub fn formal_voting_end_time(&self) -> BlockTime {
        self.time_between_votings_end_time() + self.configuration.formal_voting_time()
    }

    pub fn is_in_favor(&self) -> bool {
        match self.voting_type() {
            VotingType::Informal => {
                self.informal_stats.stake_in_favor >= self.informal_stats.stake_against
            }
            VotingType::Formal => {
                self.formal_stats.stake_in_favor >= self.formal_stats.stake_against
            }
        }
    }

    /// Depending on the result of the voting, returns the amount of reputation staked on the winning side
    pub fn get_winning_stake(&self) -> U512 {
        match (self.voting_type(), self.is_in_favor()) {
            (VotingType::Informal, true) => self.informal_stats.stake_in_favor,
            (VotingType::Informal, false) => self.informal_stats.stake_against,
            (VotingType::Formal, true) => self.formal_stats.stake_in_favor,
            (VotingType::Formal, false) => self.formal_stats.stake_against,
        }
    }

    pub fn is_result_close(&self) -> bool {
        let stake_in_favor = self.stake_in_favor() + self.unbound_stake_in_favor();
        let stake_against = self.stake_against() + self.unbound_stake_against();
        let stake_diff = stake_in_favor.abs_diff(stake_against);
        let stake_diff_percent = stake_diff.saturating_mul(U512::from(100)) / self.total_stake();
        stake_diff_percent <= self.configuration.voting_clearness_delta()
    }

    pub fn get_quorum(&self) -> u32 {
        match self.voting_type() {
            VotingType::Informal => self.configuration.informal_voting_quorum(),
            VotingType::Formal => self.configuration.formal_voting_quorum(),
        }
    }

    pub fn get_result(&self, voters_number: u32) -> VotingResult {
        if self.get_quorum() > voters_number {
            VotingResult::QuorumNotReached
        } else if self.is_in_favor() {
            VotingResult::InFavor
        } else {
            VotingResult::Against
        }
    }

    pub fn add_stake(&mut self, stake: U512, choice: Choice) {
        // overflow is not possible due to reputation token having U512 as max
        match (self.voting_type(), choice) {
            (VotingType::Informal, Choice::InFavor) => self.informal_stats.stake_in_favor += stake,
            (VotingType::Informal, Choice::Against) => self.informal_stats.stake_against += stake,
            (VotingType::Formal, Choice::InFavor) => self.formal_stats.stake_in_favor += stake,
            (VotingType::Formal, Choice::Against) => self.formal_stats.stake_against += stake,
        }
    }

    pub fn add_unbound_stake(&mut self, stake: U512, choice: Choice) {
        // overflow is not possible due to reputation token having U512 as max
        match (self.voting_type(), choice) {
            (VotingType::Informal, Choice::InFavor) => {
                self.informal_stats.unbound_stake_in_favor += stake
            }
            (VotingType::Informal, Choice::Against) => {
                self.informal_stats.unbound_stake_against += stake
            }
            (VotingType::Formal, Choice::InFavor) => {
                self.formal_stats.unbound_stake_in_favor += stake
            }
            (VotingType::Formal, Choice::Against) => {
                self.formal_stats.unbound_stake_against += stake
            }
        }
    }

    pub fn remove_stake(&mut self, stake: U512, choice: Choice) {
        // overflow is not possible due to reputation token having U512 as max
        match (self.voting_type(), choice) {
            (VotingType::Informal, Choice::InFavor) => self.informal_stats.stake_in_favor -= stake,
            (VotingType::Informal, Choice::Against) => self.informal_stats.stake_against -= stake,
            (VotingType::Formal, Choice::InFavor) => self.formal_stats.stake_in_favor -= stake,
            (VotingType::Formal, Choice::Against) => self.formal_stats.stake_against -= stake,
        }
    }

    pub fn remove_unbound_stake(&mut self, stake: U512, choice: Choice) {
        // overflow is not possible due to reputation token having U512 as max
        match (self.voting_type(), choice) {
            (VotingType::Informal, Choice::InFavor) => {
                self.informal_stats.unbound_stake_in_favor -= stake
            }
            (VotingType::Informal, Choice::Against) => {
                self.informal_stats.unbound_stake_against -= stake
            }
            (VotingType::Formal, Choice::InFavor) => {
                self.formal_stats.unbound_stake_in_favor -= stake
            }
            (VotingType::Formal, Choice::Against) => {
                self.formal_stats.unbound_stake_against -= stake
            }
        }
    }

    pub fn bind_stake(&mut self, stake: U512, choice: Choice) {
        self.remove_unbound_stake(stake, choice);
        self.add_stake(stake, choice);
    }

    pub fn total_stake(&self) -> U512 {
        // overflow is not possible due to reputation token having U512 as max
        self.total_bound_stake() + self.total_unbound_stake()
    }

    pub fn total_bound_stake(&self) -> U512 {
        // overflow is not possible due to reputation token having U512 as max
        match self.voting_type() {
            VotingType::Informal => {
                self.informal_stats.stake_in_favor + self.informal_stats.stake_against
            }
            VotingType::Formal => {
                self.formal_stats.stake_in_favor + self.formal_stats.stake_against
            }
        }
    }

    pub fn total_unbound_stake(&self) -> U512 {
        // overflow is not possible due to reputation token having U512 as max
        match self.voting_type() {
            VotingType::Informal => {
                self.informal_stats.unbound_stake_in_favor
                    + self.informal_stats.unbound_stake_against
            }
            VotingType::Formal => {
                self.formal_stats.unbound_stake_in_favor + self.formal_stats.unbound_stake_against
            }
        }
    }

    /// Get the voting's voting id.
    pub fn voting_id(&self) -> VotingId {
        self.voting_id
    }

    /// Get the voting's stake in favor.
    pub fn stake_in_favor(&self) -> U512 {
        match self.voting_type() {
            VotingType::Informal => self.informal_stats.stake_in_favor,
            VotingType::Formal => self.formal_stats.stake_in_favor,
        }
    }

    /// Get the voting's stake against.
    pub fn stake_against(&self) -> U512 {
        match self.voting_type() {
            VotingType::Informal => self.informal_stats.stake_against,
            VotingType::Formal => self.formal_stats.stake_against,
        }
    }

    pub fn unbound_stake_in_favor(&self) -> U512 {
        match self.voting_type() {
            VotingType::Informal => self.informal_stats.unbound_stake_in_favor,
            VotingType::Formal => self.formal_stats.unbound_stake_in_favor,
        }
    }

    pub fn unbound_stake_against(&self) -> U512 {
        match self.voting_type() {
            VotingType::Informal => self.informal_stats.unbound_stake_against,
            VotingType::Formal => self.formal_stats.unbound_stake_against,
        }
    }

    /// Get the voting's formal voting quorum.
    pub fn formal_voting_quorum(&self) -> u32 {
        self.configuration.formal_voting_quorum()
    }

    /// Get the voting's informal voting quorum.
    pub fn informal_voting_quorum(&self) -> u32 {
        self.configuration.informal_voting_quorum()
    }

    pub fn informal_voting_start_time(&self) -> u64 {
        self.created_at() + self.configuration.voting_delay()
    }

    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    /// Get the voting's formal voting time.
    pub fn formal_voting_time(&self) -> u64 {
        self.configuration.formal_voting_time()
    }

    /// Get the voting's informal voting time.
    pub fn informal_voting_time(&self) -> u64 {
        self.configuration.informal_voting_time()
    }

    /// Get the voting's contract call reference.
    pub fn contract_calls(&self) -> &Vec<ContractCall> {
        self.configuration.contract_calls()
    }

    /// Get a reference to the voting's voting configuration.
    pub fn voting_configuration(&self) -> &Configuration {
        &self.configuration
    }

    pub fn creator(&self) -> &Address {
        &self.creator
    }

    pub fn state(&self) -> &VotingState {
        &self.state
    }

    pub fn completed(&self) -> bool {
        self.state() == &VotingState::Finished || self.state() == &VotingState::Canceled
    }

    pub fn state_in_time(&self, block_time: BlockTime) -> VotingState {
        let informal_voting_start = self.informal_voting_start_time();
        let informal_voting_end = self.informal_voting_end_time();
        let between_voting_end = self.time_between_votings_end_time();
        let voting_end = self.formal_voting_end_time();

        if block_time < informal_voting_start {
            VotingState::Created
        } else if block_time >= informal_voting_start && block_time <= informal_voting_end {
            VotingState::Informal
        } else if block_time > informal_voting_end && block_time <= between_voting_end {
            VotingState::BetweenVotings
        } else if block_time > between_voting_end && block_time <= voting_end {
            VotingState::Formal
        } else {
            VotingState::Finished
        }
    }

    pub fn informal_stats(&self) -> &Stats {
        &self.informal_stats
    }

    pub fn formal_stats(&self) -> &Stats {
        &self.formal_stats
    }
}
