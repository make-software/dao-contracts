//! Governance Voting module.
pub mod consts;
pub mod events;
pub mod voting_state_machine;

use std::collections::BTreeMap;

use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    casper_env::{caller, get_block_time, revert, self_address},
    Address,
    Error,
    Mapping,
    Variable,
    VecMapping,
};
use casper_types::U512;
use casper_dao_utils::casper_contract::contract_api::runtime::print;

use self::{
    events::{BallotCast, VotingContractCreated, VotingCreated},
    voting_state_machine::{VotingResult, VotingStateMachine, VotingSummary, VotingType},
};
use super::{ballot::Choice, types::VotingId, Ballot};
use crate::{
    Configuration,
    ReputationContractCaller,
    ReputationContractInterface,
    VaNftContractCaller,
    VaNftContractInterface,
};

/// Governance voting is a struct that contracts can use to implement voting. It consists of two phases:
/// 1. Informal voting
/// 2. Formal voting
///
/// Whether formal voting starts depends on informal voting results.
///
/// When formal voting passes, an action can be performed - a contract can be called with voted arguments.
///
/// Governance voting uses [Reputation Token](crate::ReputationContract) to handle reputation staking and
/// [Variable Repo](crate::VariableRepositoryContract) for reading voting configuration.
///
/// For example implementation see [AdminContract](crate::admin::AdminContract)
#[derive(Instance)]
pub struct VotingEngine {
    variable_repo: Variable<Address>,
    reputation_token: Variable<Address>,
    va_token: Variable<Address>,
    voting_states: Mapping<VotingId, Option<VotingStateMachine>>,
    ballots: Mapping<(VotingId, VotingType, Address), Ballot>,
    voters: VecMapping<(VotingId, VotingType), Address>,
    votings_count: Variable<VotingId>,
}

impl VotingEngine {
    /// Initializes the module with [Addresses](Address) of [Reputation Token](crate::ReputationContract) and [Variable Repo](crate::VariableRepositoryContract)
    ///
    /// # Events
    /// Emits [`VotingContractCreated`](VotingContractCreated)
    pub fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address) {
        self.variable_repo.set(variable_repo);
        self.reputation_token.set(reputation_token);
        self.va_token.set(va_token);

        VotingContractCreated {
            variable_repo,
            reputation_token,
            voter_contract: self_address(),
        }
        .emit();
    }

    /// Creates new informal [Voting](VotingStateMachine).
    ///
    /// `contract_to_call`, `entry_point` and `runtime_args` parameters define an action that will be performed  when formal voting passes.
    ///
    /// It collects configuration from [Variable Repo](crate::VariableRepositoryContract) and persists it, so they won't change during the voting process.
    ///
    ///
    /// # Events
    /// Emits [`VotingCreated`](VotingCreated), [`BallotCast`](BallotCast)
    ///
    /// # Errors
    /// Throws [`Error::NotEnoughReputation`](Error::NotEnoughReputation) when the creator does not have enough reputation to create a voting
    pub fn create_voting(
        &mut self,
        creator: Address,
        stake: U512,
        configuration: Configuration,
    ) -> VotingId {
        if configuration.only_va_can_create() && !self.is_va(creator) {
            revert(Error::VaNotOnboarded)
        }

        let cast_first_vote = !configuration.voting_configuration.is_bid_escrow;

        let voting_id = self.next_voting_id();

        VotingCreated::new(&creator, voting_id, voting_id, None, &configuration).emit();

        let voting =
            VotingStateMachine::new(voting_id, get_block_time(), creator, configuration);
        self.set_voting(voting);

        if cast_first_vote {
            self.vote(creator, voting_id, Choice::InFavor, stake);
        }

        voting_id
    }

    /// Finishes voting.
    ///
    /// Depending on type of voting, different actions are performed.
    ///
    /// For informal voting a new formal voting can be created. Reputation staked for this voting is returned to the voters, except for creator. When voting
    /// passes, it is used as a stake for a new voting, otherwise it is burned.
    ///
    /// For formal voting an action will be performed if the result is in favor. Reputation is redistributed to the winning voters. When no quorum is reached,
    /// the reputation is returned, except for the creator - its reputation is then burned.
    ///
    /// # Events
    /// Emits [`VotingEnded`](VotingEnded), [`VotingCreated`](VotingCreated), [`BallotCast`](BallotCast)
    ///
    /// # Errors
    /// Throws [`FinishingCompletedVotingNotAllowed`](Error::FinishingCompletedVotingNotAllowed) if trying to complete already finished voting
    ///
    /// Throws [`FormalVotingTimeNotReached`](Error::FormalVotingTimeNotReached) if formal voting time did not pass
    ///
    /// Throws [`InformalVotingTimeNotReached`](Error::InformalVotingTimeNotReached) if informal voting time did not pass
    ///
    /// Throws [`ArithmeticOverflow`](Error::ArithmeticOverflow) in an unlikely event of a overflow when calculating reputation to redistribute
    pub fn finish_voting(&mut self, voting_id: VotingId) -> VotingSummary {
        let voting = self
            .get_voting(voting_id)
            .unwrap_or_revert_with(Error::VotingDoesNotExist);

        if voting.completed() {
            revert(Error::FinishingCompletedVotingNotAllowed)
        }

        let informal_without_stake = voting.is_informal_without_stake();
        match voting.voting_type() {
            VotingType::Informal => {
                let voting_result = self.finish_informal_voting(voting);
                if !informal_without_stake {
                    self.return_reputation_of_yes_voters(voting_id, VotingType::Informal);
                    self.return_reputation_of_no_voters(voting_id, VotingType::Informal);
                }

                match voting_result.result() {
                    VotingResult::InFavor | VotingResult::Against => {
                        self.recast_creators_ballot_from_informal_to_formal(
                            voting_id,
                        );
                    }
                    VotingResult::QuorumNotReached => {
                        // TODO: Emit events
                    }
                    VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
                }
                voting_result
            }
            VotingType::Formal => {
                let voting_result = self.finish_formal_voting(voting);
                match voting_result.result() {
                    VotingResult::InFavor => {
                        self.return_reputation_of_yes_voters(voting_id, VotingType::Formal);
                        self.redistribute_reputation_of_no_voters(voting_id, VotingType::Formal);
                    }
                    VotingResult::Against => {
                        self.return_reputation_of_no_voters(voting_id, VotingType::Formal);
                        self.redistribute_reputation_of_yes_voters(voting_id, VotingType::Formal);
                    }
                    VotingResult::QuorumNotReached => {
                        self.return_reputation_of_yes_voters(voting_id, VotingType::Formal);
                        self.return_reputation_of_no_voters(voting_id, VotingType::Formal);
                    }
                    VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
                }
                voting_result
            }
        }
    }

    pub fn finish_voting_without_token_redistribution(
        &mut self,
        voting_id: VotingId,
    ) -> VotingSummary {
        let voting = self
            .get_voting(voting_id)
            .unwrap_or_revert_with(Error::VotingDoesNotExist);

        if voting.completed() {
            revert(Error::FinishingCompletedVotingNotAllowed)
        }

        match voting.voting_type() {
            VotingType::Informal => self.finish_informal_voting(voting),
            VotingType::Formal => self.finish_formal_voting(voting),
        }
    }

    pub fn summary(&self, voting_id: VotingId) -> VotingSummary {
        let voting = self.get_voting_or_revert(voting_id);
        let voters_len = self.voters.len((voting.voting_id(), voting.voting_type()));
        VotingSummary::new(
            voting.get_result(voters_len),
            voting.voting_type(),
            voting_id
        )
    }

    fn finish_informal_voting(&mut self, mut voting: VotingStateMachine) -> VotingSummary {
        if !voting.is_in_time(get_block_time()) {
            revert(Error::InformalVotingTimeNotReached)
        }

        let voting_id = voting.voting_id();
        let voters_len = self.voters.len((voting_id, voting.voting_type()));
        let voting_result = voting.get_result(voters_len);
        match voting_result {
            VotingResult::InFavor | VotingResult::Against => {
                // TODO: Emit event
                voting.complete_informal_voting();
            }
            VotingResult::QuorumNotReached => {
                voting.finish();
            }
            VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
        };

        // Move up in stack.
        // VotingEnded {
        //     voting_id: informal_voting_id,
        //     informal_voting_id,
        //     formal_voting_id: voting.formal_voting_id(),
        //     result: result.into(),
        //     votes_count: voters_len.into(),
        //     stake_in_favor: voting.stake_in_favor(),
        //     stake_against: voting.stake_against(),
        //     transfers,
        //     burns,
        //     mints: BTreeMap::new(),
        // }
        // .emit();

        self.set_voting(voting);

        VotingSummary::new(
            voting_result,
            VotingType::Informal,
            voting_id
        )

    }

    fn finish_formal_voting(&mut self, mut voting: VotingStateMachine) -> VotingSummary {
        voting.guard_finish_formal_voting(get_block_time());
        let voting_id = voting.voting_id();
        let voters_len = self.voters.len((voting_id, VotingType::Formal));
        let voting_result = voting.get_result(voters_len);

        if voting_result == VotingResult::InFavor {
            self.perform_action(voting.voting_id());
        }

        // TODO: Emit events

        voting.finish();
        self.set_voting(voting);

        VotingSummary::new(
            voting_result,
            VotingType::Formal,
            voting_id,
        )
    }

    /// Casts a vote
    ///
    /// # Events
    /// Emits [`BallotCast`](BallotCast)
    ///
    /// # Errors
    /// Throws [`VoteOnCompletedVotingNotAllowed`](Error::VoteOnCompletedVotingNotAllowed) if voting is completed
    ///
    /// Throws [`CannotVoteTwice`](Error::CannotVoteTwice) if voter already voted
    pub fn vote(&mut self, voter: Address, voting_id: VotingId, choice: Choice, stake: U512) {
        let voting = self.get_voting(voting_id).unwrap_or_revert();
        voting.guard_vote(get_block_time());
        self.assert_vote_doesnt_exist(voting_id, voting.voting_type(), voter);
        self.cast_ballot(voter, voting_id, choice, stake, false, voting);
    }

    fn assert_vote_doesnt_exist(&mut self, voting_id: VotingId, voting_type: VotingType, voter: Address) {
        let vote = self.ballots.get(&(voting_id, voting_type, voter));

        if vote.is_some() {
            let voting = self.get_voting(voting_id).unwrap_or_revert();
            print(format!("Voting type: {:?}", voting.voting_type()).as_str());
            revert(Error::CannotVoteTwice)
        }
    }

    // TODO: REFACTOR EVERYTHING
    pub fn cast_ballot(
        &mut self,
        voter: Address,
        voting_id: VotingId,
        choice: Choice,
        stake: U512,
        unbounded: bool,
        mut voting: VotingStateMachine,
    ) {
        if !unbounded && !voting.is_informal_without_stake() {
            // Stake the reputation
            ReputationContractCaller::at(self.reputation_token_address())
                .stake_voting(voter, voting_id, choice, stake);
        }

        let vote = Ballot {
            voter,
            choice,
            voting_id,
            stake,
            unbounded,
            canceled: false,
        };

        BallotCast::new(&vote).emit();

        // Add a voter to the list
        self.voters.add((voting_id, voting.voting_type()), voter);

        // Update the votes list
        self.ballots.set(&(voting_id, voting.voting_type(), voter), vote);

        // update voting
        if unbounded {
            voting.add_unbounded_stake(stake, choice)
        } else {
            voting.add_stake(stake, choice);
        }
        self.set_voting(voting);
    }

    /// Returns the address of [Variable Repo](crate::VariableRepositoryContract) connected to the contract
    pub fn variable_repo_address(&self) -> Address {
        self.variable_repo.get().unwrap_or_revert()
    }

    /// Returns the address of [Reputation Token](crate::ReputationContract) connected to the contract
    pub fn reputation_token_address(&self) -> Address {
        self.reputation_token.get().unwrap_or_revert()
    }

    pub fn va_token_address(&self) -> Address {
        self.va_token.get().unwrap_or_revert()
    }

    pub fn all_voters(&self, voting_id: VotingId, voting_type: VotingType) -> Vec<Address> {
        self.voters.get_all((voting_id, voting_type))
    }

    /// Returns the [Ballot](Ballot) of voter with `address` and cast on `voting_id`
    pub fn get_ballot(&self, voting_id: VotingId, voting_type: VotingType, address: Address) -> Option<Ballot> {
        self.ballots.get_or_none(&(voting_id, voting_type, address))
    }

    /// Returns the nth [Ballot](Ballot) of cast on `voting_id`
    pub fn get_ballot_at(&self, voting_id: VotingId, voting_type: VotingType, i: u32) -> Ballot {
        let address = self
            .get_voter(voting_id, voting_type, i)
            .unwrap_or_revert_with(Error::VoterDoesNotExist);
        self.get_ballot(voting_id, voting_type, address)
            .unwrap_or_revert_with(Error::BallotDoesNotExist)
    }

    /// Returns the address of nth voter who voted on Voting with `voting_id`
    pub fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address> {
        self.voters.get_or_none((voting_id, voting_type), at)
    }

    /// Returns the [Voting](VotingStateMachine) for given id
    pub fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine> {
        self.voting_states
            .get_or_none(&voting_id)
            .map(|x| x.unwrap_or_revert())
    }

    pub fn get_voting_or_revert(&self, voting_id: VotingId) -> VotingStateMachine {
        self.get_voting(voting_id)
            .unwrap_or_revert_with(Error::VotingDoesNotExist)
    }

    fn set_voting(&self, voting: VotingStateMachine) {
        self.voting_states.set(&voting.voting_id(), Some(voting))
    }

    fn next_voting_id(&mut self) -> VotingId {
        let voting_id = self.votings_count.get().unwrap_or_default();
        self.votings_count.set(voting_id + 1);
        voting_id
    }

    fn perform_action(&self, voting_id: VotingId) {
        let voting = self.get_voting(voting_id).unwrap_or_revert();
        for contract_call in voting.contract_calls() {
            contract_call.call();
        }
    }

    pub fn unstake_all_reputation(&mut self, voting_id: VotingId, voting_type: VotingType) -> BTreeMap<Address, U512> {
        let mut transfers = BTreeMap::new();
        for i in 0..self.voters.len((voting_id, voting_type)) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.unbounded || ballot.canceled {
                continue;
            }
            transfers.insert(ballot.voter, ballot.stake);
            ReputationContractCaller::at(self.reputation_token_address())
                .unstake_voting(ballot.voter, ballot.voting_id);
        }

        transfers
    }

    pub fn recast_creators_ballot_from_informal_to_formal(&mut self, voting_id: VotingId) {
        let voting = self.get_voting_or_revert(voting_id);
        let creator = voting.creator();
        let creator_ballot = self
            .get_ballot(voting_id, VotingType::Informal, *creator)
            .unwrap_or_revert_with(Error::BallotDoesNotExist);

        self.cast_ballot(
            *creator,
            voting_id,
            Choice::InFavor,
            creator_ballot.stake,
            creator_ballot.unbounded,
            voting,
        );
    }

    pub fn return_reputation_of_yes_voters(&self, voting_id: VotingId, voting_type: VotingType) {
        for i in 0..self.voters.len((voting_id, voting_type)) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.choice.is_in_favor() && !ballot.unbounded {
                self.reputation_token()
                    .unstake_voting(ballot.voter, voting_id);
            }
        }
    }

    pub fn return_reputation_of_no_voters(&self, voting_id: VotingId, voting_type: VotingType) {
        for i in 0..self.voters.len((voting_id, voting_type)) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.choice.is_against() && !ballot.unbounded {
                self.reputation_token()
                    .unstake_voting(ballot.voter, voting_id);
            }
        }
    }

    pub fn redistribute_reputation_of_no_voters(&self, voting_id: VotingId, voting_type: VotingType) {
        let voting = self.get_voting_or_revert(voting_id);
        let total_stake_in_favor = voting.stake_in_favor();
        let total_stake_against = voting.stake_against();
        let mut burns: BTreeMap<Address, U512> = BTreeMap::new();
        let mut mints: BTreeMap<Address, U512> = BTreeMap::new();
        for i in 0..self.voters.len((voting_id, voting_type)) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.unbounded {
                continue;
            }
            if ballot.choice.is_against() {
                self.reputation_token()
                    .unstake_voting(ballot.voter, voting_id);
                burns.insert(ballot.voter, ballot.stake);
            } else {
                let amount_to_mint = total_stake_against * ballot.stake / total_stake_in_favor;
                mints.insert(ballot.voter, amount_to_mint);
            }
        }
        self.reputation_token().bulk_mint_burn(mints, burns);
    }

    pub fn redistribute_reputation_of_yes_voters(&self, voting_id: VotingId, voting_type: VotingType) {
        let voting = self.get_voting_or_revert(voting_id);
        let total_stake_in_favor = voting.stake_in_favor();
        let total_stake_against = voting.stake_against();
        let mut burns: BTreeMap<Address, U512> = BTreeMap::new();
        let mut mints: BTreeMap<Address, U512> = BTreeMap::new();
        for i in 0..self.voters.len((voting_id, voting_type)) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.unbounded {
                continue;
            }
            if ballot.choice.is_in_favor() {
                self.reputation_token()
                    .unstake_voting(ballot.voter, voting_id);
                burns.insert(ballot.voter, ballot.stake);
            } else {
                let amount_to_mint = total_stake_in_favor * ballot.stake / total_stake_against;
                mints.insert(ballot.voter, amount_to_mint);
            }
        }
        self.reputation_token().bulk_mint_burn(mints, burns);
    }

    fn is_va(&self, address: Address) -> bool {
        !self.va_token().balance_of(address).is_zero()
    }

    fn va_token(&self) -> VaNftContractCaller {
        VaNftContractCaller::at(self.va_token_address())
    }

    pub fn reputation_token(&self) -> ReputationContractCaller {
        ReputationContractCaller::at(self.reputation_token_address())
    }

    /// Get a reference to the governance voting's voters.
    pub fn voters(&self) -> &VecMapping<(VotingId, VotingType), Address> {
        &self.voters
    }

    pub fn bound_ballot(&mut self, voting_id: u32, worker: Address, voting_type: VotingType) {
        let mut ballot = self
            .get_ballot(voting_id, voting_type,worker)
            .unwrap_or_revert_with(Error::BallotDoesNotExist);

        let mut voting = self.get_voting_or_revert(voting_id);
        voting.bound_stake(ballot.stake, ballot.choice);
        self.set_voting(voting);

        self.reputation_token().mint(worker, ballot.stake);
        self.reputation_token()
            .stake_voting(worker, voting_id, ballot.choice, ballot.stake);

        ballot.unbounded = false;
        self.ballots.set(&(voting_id, voting_type, worker), ballot);
    }

    pub fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool {
        let voting = self.get_voting(voting_id);
        match voting {
            None => false,
            Some(voting) => {
                voting.voting_type() == voting_type
            },
        }
    }

    pub fn cancel_voter(&mut self, voter: Address, voting_id: VotingId) {
        if caller() != self.reputation_token_address() {
            revert(Error::OnlyReputationTokenContractCanCancel);
        }
        let voting = self.get_voting_or_revert(voting_id);
        if &voter == voting.creator() {
            self.cancel_voting(voting);
        } else {
            self.cancel_ballot(voting, voter);
        }
    }

    fn cancel_voting(&mut self, mut voting: VotingStateMachine) {
        voting.cancel();
        self.unstake_all_reputation(voting.voting_id(), voting.voting_type());
        self.set_voting(voting);

        // Emit event.
    }

    // Note: it doesn't remove a voter from self.votings to keep the quorum num right.
    fn cancel_ballot(&mut self, mut voting: VotingStateMachine, voter: Address) {
        let voting_id = voting.voting_id();
        let ballots_key = (voting_id, voting.voting_type(), voter);
        let mut ballot = self.ballots.get(&ballots_key).unwrap_or_revert();

        // Unstake reputation.
        ReputationContractCaller::at(self.reputation_token_address())
            .unstake_voting(voter, voting_id);

        // Update voting.
        let stake = ballot.stake;
        let choice = ballot.choice;
        if ballot.unbounded {
            voting.remove_unbounded_stake(stake, choice)
        } else {
            voting.remove_stake(stake, choice);
        }
        self.set_voting(voting);

        // Update ballot.
        ballot.canceled = true;
        self.ballots.set(&ballots_key, ballot);

        // Emit event.
    }
}
