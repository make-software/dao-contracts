//! Voting Engine.
use crate::configuration::Configuration;
use crate::modules::refs::ContractRefs;
use crate::rules::validation::voting::CanCreateVoting;
use crate::rules::RulesBuilder;
use crate::utils::Error;
use crate::voting::ballot::{Ballot, Choice};
use crate::voting::ids::get_next_voting_id;
use crate::voting::types::VotingId;
use crate::voting::voting_engine::events::{
    BallotCanceled, BallotCast, Reason, VotingCanceled, VotingCreatedInfo, VotingEnded,
};
use crate::voting::voting_engine::voting_state_machine::{
    VotingResult, VotingStateMachine, VotingSummary, VotingType,
};
use odra::contract_env::{emit_event, get_block_time, revert};
use odra::prelude::{collections::BTreeMap, vec, vec::Vec};
use odra::types::{Address, Balance};
use odra::{List, Mapping, UnwrapOrRevert, Variable};

pub mod events;
pub mod voting_state_machine;

/// Governance voting is a struct that voting_contracts can use to implement voting.
///
/// It consists of two phases:
/// 1. Informal voting
/// 2. Formal voting
///
/// Whether formal voting starts depends on informal voting results.
///
/// When formal voting passes, an action can be performed - a contract can be called with voted arguments.
///
/// Governance voting uses:
/// 1. [Reputation Token](crate::core_contracts::ReputationContract) to handle reputation staking.
/// 2. [Variable Repo](crate::core_contracts::VariableRepositoryContract) for reading voting configuration.
///
/// For example implementation see [AdminContract](crate::voting_contracts::AdminContract).
#[odra::module(events = [VotingCreatedInfo, BallotCast, VotingEnded, VotingCanceled, BallotCanceled])]
pub struct VotingEngine {
    refs: ContractRefs,
    voting_states: Mapping<VotingId, Option<VotingStateMachine>>,
    ballots: Mapping<(VotingId, VotingType, Address), Ballot>,
    voters: Mapping<(VotingId, VotingType), List<Address>>,
    configurations: Mapping<VotingId, Configuration>,
    active_votings: Variable<Vec<VotingId>>,
}

impl VotingEngine {
    /// Creates new informal [Voting].
    ///
    /// `contract_to_call`, `entry_point` and `runtime_args` parameters define an action that will be performed when formal voting passes.
    ///
    /// It collects configuration from [Variable Repo] and persists it, so they won't change during the voting process.
    ///
    /// Interacts with [Dao Ids Contract] to generate voting id.
    ///
    /// Depending on the configuration may [`cast`] the first vote.
    ///
    /// # Errors
    /// * [`Error::NotEnoughReputation`] when the creator does not have enough reputation to create a voting.
    /// * [`Error::NotOnboarded`] if the configuration requires the creator to be a VA but is not.
    ///
    /// [Voting]: VotingStateMachine
    /// [Variable Repo]: crate::core_contracts::VariableRepositoryContract
    /// [`Error::NotOnboarded`]: Error::NotOnboarded
    /// [Dao Ids Contract]: crate::utils_contracts::DaoIdsContract
    /// [`cast`]: Self::cast_ballot()
    pub fn create_voting(
        &mut self,
        creator: Address,
        stake: Balance,
        configuration: Configuration,
    ) -> (VotingCreatedInfo, VotingStateMachine) {
        RulesBuilder::new()
            .add_validation(CanCreateVoting::create(
                self.is_va(&creator),
                configuration.only_va_can_create(),
            ))
            .build()
            .validate_generic_validations();

        let should_cast_first_vote = configuration.should_cast_first_vote();

        let voting_ids_address = configuration.voting_ids_address();
        let voting_id = get_next_voting_id(voting_ids_address);
        let mut voting = VotingStateMachine::new(voting_id, get_block_time(), creator);

        self.configurations.set(&voting_id, configuration.clone());
        let mut used_stake = None;
        if should_cast_first_vote {
            self.cast_vote(
                creator,
                VotingType::Informal,
                Choice::InFavor,
                stake,
                &mut voting,
                &configuration,
            );
            used_stake = Some(stake);
        }

        let info = VotingCreatedInfo::new(creator, voting_id, used_stake, &configuration);
        self.set_voting(voting.clone());

        // Register voting in active votings list.
        self.add_to_active_list(voting_id);

        (info, voting)
    }

    /// Finishes voting.
    ///
    /// Depending on type of voting, different actions are performed.
    ///
    /// For informal voting a new formal voting can be created. Reputation staked for this voting is returned to the voters,
    /// except for the creator. When voting passes, it is used as a stake for a new voting, otherwise it is burned.
    ///
    /// For formal voting an action will be performed if the result is `in favor`. Reputation is redistributed to the winning voters.
    /// When no quorum is reached, the reputation is returned, except for the creator - its reputation is then burned.
    ///
    /// # Events
    /// * [`VotingEnded`](VotingEnded)
    /// * [`BallotCast`](BallotCast)
    ///
    /// # Errors
    /// * [`FinishingCompletedVotingNotAllowed`](Error::FinishingCompletedVotingNotAllowed) if trying to complete already finished voting.
    /// * [`FormalVotingTimeNotReached`](Error::FormalVotingTimeNotReached) if formal voting time did not pass.
    /// * [`InformalVotingTimeNotReached`](Error::InformalVotingTimeNotReached) if informal voting time did not pass.
    /// * [`ArithmeticOverflow`](Error::ArithmeticOverflow) in an unlikely event of a overflow when calculating reputation to redistribute.
    pub fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) -> VotingSummary {
        let mut voting = self.get_voting_or_revert(voting_id);
        let mut configuration = self.get_configuration_or_revert(voting_id);
        self.assert_voting_type(&voting, voting_type);

        if voting.completed() {
            revert(Error::FinishingCompletedVotingNotAllowed)
        }

        let mut rep_unstakes = BTreeMap::new();
        let mut rep_burns = BTreeMap::new();
        let mut rep_mints = BTreeMap::new();

        let summary = match voting.voting_type() {
            VotingType::Informal => {
                let informal_without_stake = voting.is_informal_without_stake(&configuration);
                let voting_result = self.finish_informal_voting(&mut voting, &mut configuration);
                if !informal_without_stake {
                    let yes_unstakes = self.return_yes_voters_rep(voting_id, VotingType::Informal);
                    let no_unstakes = self.return_no_voters_rep(voting_id, VotingType::Informal);
                    add_to_map(&mut rep_unstakes, Reason::InformalFinished, yes_unstakes);
                    add_to_map(&mut rep_unstakes, Reason::InformalFinished, no_unstakes);
                }

                match voting_result.result() {
                    VotingResult::InFavor | VotingResult::Against => {
                        // It emits BallotCast event, so no need to capture it in VotingEnded event.
                        self.recast_creators_ballot_from_informal_to_formal(
                            &mut voting,
                            &configuration,
                        );
                    }
                    VotingResult::QuorumNotReached => {}
                    VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
                }
                voting_result
            }
            VotingType::Formal => {
                let voting_result = self.finish_formal_voting(&mut voting, &configuration);
                match voting_result.result() {
                    VotingResult::InFavor => {
                        if configuration.should_bind_ballot_for_successful_voting() {
                            let worker = configuration
                                .get_unbound_ballot_address()
                                .unwrap_or_revert_with(Error::InvalidAddress);
                            self.bound_ballot(&mut voting, worker, VotingType::Formal);
                        }
                        let yes_unstakes =
                            self.return_yes_voters_rep(voting_id, VotingType::Formal);
                        let (mints, burns) =
                            self.redistribute_reputation_of_no_voters(&voting, VotingType::Formal);
                        add_to_map(&mut rep_unstakes, Reason::FormalFinished, yes_unstakes);
                        add_to_map(&mut rep_mints, Reason::FormalWon, mints);
                        add_to_map(&mut rep_burns, Reason::FormalLost, burns);
                    }
                    VotingResult::Against => {
                        let no_unstakes = self.return_no_voters_rep(voting_id, VotingType::Formal);
                        let (mints, burns) =
                            self.redistribute_reputation_of_yes_voters(&voting, VotingType::Formal);
                        add_to_map(&mut rep_unstakes, Reason::FormalFinished, no_unstakes);
                        add_to_map(&mut rep_mints, Reason::FormalWon, mints);
                        add_to_map(&mut rep_burns, Reason::FormalLost, burns);
                    }
                    VotingResult::QuorumNotReached => {
                        let yes_unstakes =
                            self.return_yes_voters_rep(voting_id, VotingType::Formal);
                        let no_unstakes = self.return_no_voters_rep(voting_id, VotingType::Formal);
                        add_to_map(&mut rep_unstakes, Reason::FormalFinished, yes_unstakes);
                        add_to_map(&mut rep_unstakes, Reason::FormalFinished, no_unstakes);
                    }
                    VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
                }
                voting_result
            }
        };

        let stats = match summary.voting_type() {
            VotingType::Informal => voting.informal_stats(),
            VotingType::Formal => voting.formal_stats(),
        };

        // Emit VotingEnded event.
        emit_event(VotingEnded::new(
            &voting,
            summary.result(),
            stats,
            rep_unstakes,
            BTreeMap::new(),
            rep_burns,
            rep_mints,
        ));

        self.set_voting(voting);
        summary
    }

    /// Marks voting finished but do nothing with the staked reputation.
    ///
    /// # Errors
    /// * [`Error::VotingDoesNotExist`] - voting with the given id does not exists.
    /// * [`Error::FinishingCompletedVotingNotAllowed`] - voting is finished already.
    pub fn finish_voting_without_token_redistribution(
        &mut self,
        voting_id: VotingId,
        configuration: &mut Configuration,
    ) -> VotingSummary {
        let mut voting = self
            .get_voting(voting_id)
            .unwrap_or_revert_with(Error::VotingDoesNotExist);

        if voting.completed() {
            revert(Error::FinishingCompletedVotingNotAllowed)
        }

        let summary = match voting.voting_type() {
            VotingType::Informal => self.finish_informal_voting(&mut voting, configuration),
            VotingType::Formal => self.finish_formal_voting(&mut voting, configuration),
        };

        self.set_voting(voting);

        summary
    }

    fn finish_informal_voting(
        &mut self,
        voting: &mut VotingStateMachine,
        configuration: &mut Configuration,
    ) -> VotingSummary {
        if !voting.is_in_time(get_block_time(), configuration) {
            revert(Error::InformalVotingTimeNotReached)
        }

        let voting_id = voting.voting_id();
        let voters_count = self.voters_count(voting_id, voting.voting_type());
        let voting_result = voting.get_result(voters_count, configuration);
        let double_time_between_votings = match voting_result {
            VotingResult::InFavor | VotingResult::Against => {
                voting.complete_informal_voting(configuration)
            }
            VotingResult::QuorumNotReached => {
                self.remove_from_active_list(voting_id);
                voting.finish();
                false
            }
            VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
        };

        if double_time_between_votings {
            configuration.double_time_between_votings();
            self.configurations.set(&voting_id, configuration.clone());
        }

        VotingSummary::new(voting_result, VotingType::Informal, voting_id)
    }

    fn finish_formal_voting(
        &mut self,
        voting: &mut VotingStateMachine,
        configuration: &Configuration,
    ) -> VotingSummary {
        voting.guard_finish_formal_voting(get_block_time(), configuration);
        let voting_id = voting.voting_id();
        let voters_count = self.voters_count(voting_id, VotingType::Formal);
        let voting_result = voting.get_result(voters_count, configuration);

        if voting_result == VotingResult::InFavor {
            self.perform_action(configuration);
        }

        self.remove_from_active_list(voting_id);
        voting.finish();

        VotingSummary::new(voting_result, VotingType::Formal, voting_id)
    }

    /// Writes a vote in the storage.
    ///
    /// # Events
    /// * [`BallotCast`](BallotCast)
    ///
    /// # Errors
    /// * [`VoteOnCompletedVotingNotAllowed`](Error::VoteOnCompletedVotingNotAllowed) if voting is completed.
    /// * [`CannotVoteTwice`](Error::CannotVoteTwice) if the voter already voted.
    pub fn vote(
        &mut self,
        voter: Address,
        voting_id: VotingId,
        voting_type: VotingType,
        choice: Choice,
        stake: Balance,
    ) {
        let mut voting = self.get_voting_or_revert(voting_id);
        let configuration = self.get_configuration_or_revert(voting_id);
        self.cast_vote(
            voter,
            voting_type,
            choice,
            stake,
            &mut voting,
            &configuration,
        );
        self.set_voting(voting);
    }

    fn cast_vote(
        &mut self,
        voter: Address,
        voting_type: VotingType,
        choice: Choice,
        stake: Balance,
        voting: &mut VotingStateMachine,
        configuration: &Configuration,
    ) {
        let voting_id = voting.voting_id();
        self.assert_voting_type(voting, voting_type);
        voting.guard_vote(get_block_time(), configuration);
        self.assert_vote_doesnt_exist(voting_id, voting.voting_type(), voter);
        self.cast_ballot(voter, choice, stake, false, voting, configuration);
    }

    fn assert_vote_doesnt_exist(
        &mut self,
        voting_id: VotingId,
        voting_type: VotingType,
        voter: Address,
    ) {
        let vote = self.ballots.get(&(voting_id, voting_type, voter));

        if vote.is_some() {
            revert(Error::CannotVoteTwice)
        }
    }

    fn assert_voting_type(&self, voting: &VotingStateMachine, voting_type: VotingType) {
        if voting.voting_type() != voting_type {
            revert(Error::VotingWithGivenTypeNotInProgress)
        }
    }

    /// Records voter's vote.
    ///
    /// Writes into the storage the vote details and stakes reputation (for a bound ballot).
    ///
    /// Calls [Reputation Token Contract] to stake reputation.
    ///
    /// # Events
    /// * [`BallotCast`] event.
    ///
    /// [Reputation Token Contract]: crate::core_contracts::ReputationContract
    pub fn cast_ballot(
        &mut self,
        voter: Address,
        choice: Choice,
        stake: Balance,
        unbound: bool,
        voting: &mut VotingStateMachine,
        configuration: &Configuration,
    ) {
        let voting_id = voting.voting_id();
        let ballot = Ballot::new(
            voter,
            voting_id,
            voting.voting_type(),
            choice,
            stake,
            unbound,
            false,
        );

        if !unbound && !voting.is_informal_without_stake(configuration) {
            // Stake the reputation
            self.refs.reputation_token().stake(voter, stake);
        }

        emit_event(BallotCast::new(&ballot));

        // Add a voter to the list
        let mut voters = self.voters(voting_id, voting.voting_type());
        voters.push(voter);

        // Update the votes list
        self.ballots
            .set(&(voting_id, voting.voting_type(), voter), ballot);

        // update voting
        if unbound {
            voting.add_unbound_stake(stake, choice)
        } else {
            voting.add_stake(stake, choice);
        }
    }

    /// Gets a vector of all voters' addresses.
    pub fn all_voters(&self, voting_id: VotingId, voting_type: VotingType) -> Vec<Address> {
        self.voters(voting_id, voting_type).iter().collect()
    }

    /// Returns the Voter's [`Ballot`].
    pub fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot> {
        self.ballots.get(&(voting_id, voting_type, address))
    }

    /// Returns the nth [Ballot](Ballot) of voting with a given id.
    pub fn get_ballot_at(&self, voting_id: VotingId, voting_type: VotingType, i: u32) -> Ballot {
        let address = self
            .get_voter(voting_id, voting_type, i)
            .unwrap_or_revert_with(Error::VoterDoesNotExist);
        self.get_ballot(voting_id, voting_type, address)
            .unwrap_or_revert_with(Error::BallotDoesNotExist)
    }

    /// Returns the address of the nth voter who voted on Voting with a given id.
    pub fn get_voter(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        at: u32,
    ) -> Option<Address> {
        self.voters(voting_id, voting_type).get(at)
    }

    /// Returns the [Voting](VotingStateMachine) for a given id.
    pub fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine> {
        self.voting_states
            .get(&voting_id)
            .map(|x| x.unwrap_or_revert())
    }

    /// Gets voting with a given id or stops contract execution.
    ///
    /// # Errors
    /// * [Error::VotingDoesNotExist] if the given id does not exist.
    pub fn get_voting_or_revert(&self, voting_id: VotingId) -> VotingStateMachine {
        self.get_voting(voting_id)
            .unwrap_or_revert_with(Error::VotingDoesNotExist)
    }

    /// Gets configuration with a given voting_id or stops contract execution.
    ///
    /// # Error
    /// * [Error::ConfigurationNotFound] if the given id does not exist.
    pub fn get_configuration_or_revert(&self, voting_id: VotingId) -> Configuration {
        self.configurations
            .get(&voting_id)
            .unwrap_or_revert_with(Error::ConfigurationNotFound)
    }

    /// Updates voting storage.
    pub fn set_voting(&mut self, voting: VotingStateMachine) {
        self.voting_states.set(&voting.voting_id(), Some(voting))
    }

    fn perform_action(&self, configuration: &Configuration) {
        for contract_call in configuration.contract_calls() {
            contract_call.call();
        }
    }

    /// Iterates over all the ballots and unstakes reputation. Returns a map of address to it's stake.
    ///
    /// Calls [Reputation Token Contract](crate::core_contracts::ReputationContract) to perform unstake operation.
    pub fn unstake_all_reputation(
        &mut self,
        voting_id: VotingId,
        voting_type: VotingType,
    ) -> BTreeMap<Address, Balance> {
        let mut transfers = BTreeMap::new();
        let mut stakes: Vec<(Address, Balance)> = Vec::new();
        for i in 0..self.voters_count(voting_id, voting_type) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.unbound || ballot.canceled {
                continue;
            }
            transfers.insert(ballot.voter, ballot.stake);
            stakes.push((ballot.voter, ballot.stake));
        }
        self.refs.reputation_token().bulk_unstake(stakes);
        transfers
    }

    fn recast_creators_ballot_from_informal_to_formal(
        &mut self,
        voting: &mut VotingStateMachine,
        configuration: &Configuration,
    ) {
        let creator = voting.creator();
        let creator_ballot = self
            .get_ballot(voting.voting_id(), VotingType::Informal, *creator)
            .unwrap_or_revert_with(Error::BallotDoesNotExist);

        self.cast_ballot(
            *creator,
            Choice::InFavor,
            creator_ballot.stake,
            creator_ballot.unbound,
            voting,
            configuration,
        );
    }

    fn return_yes_voters_rep(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
    ) -> BTreeMap<Address, Balance> {
        let mut summary = BTreeMap::new();
        let mut stakes: Vec<(Address, Balance)> = Vec::new();
        for i in 0..self.voters_count(voting_id, voting_type) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.choice.is_in_favor() && !ballot.unbound && !ballot.canceled {
                stakes.push((ballot.voter, ballot.stake));
                summary.insert(ballot.voter, ballot.stake);
            }
        }
        self.refs.reputation_token().bulk_unstake(stakes);
        summary
    }

    fn return_no_voters_rep(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
    ) -> BTreeMap<Address, Balance> {
        let mut summary = BTreeMap::new();
        let mut stakes: Vec<(Address, Balance)> = Vec::new();
        for i in 0..self.voters_count(voting_id, voting_type) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.choice.is_against() && !ballot.unbound && !ballot.canceled {
                stakes.push((ballot.voter, ballot.stake));
                summary.insert(ballot.voter, ballot.stake);
            }
        }
        self.refs.reputation_token().bulk_unstake(stakes);
        summary
    }

    fn redistribute_reputation_of_no_voters(
        &self,
        voting: &VotingStateMachine,
        voting_type: VotingType,
    ) -> (BTreeMap<Address, Balance>, BTreeMap<Address, Balance>) {
        let total_stake_in_favor = voting.stake_in_favor();
        let voting_id = voting.voting_id();
        let total_stake_against = voting.stake_against();
        let mut burns: BTreeMap<Address, Balance> = BTreeMap::new();
        let mut mints: BTreeMap<Address, Balance> = BTreeMap::new();
        let mut stakes: Vec<(Address, Balance)> = Vec::new();

        for i in 0..self.voters_count(voting_id, voting_type) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.unbound || ballot.canceled {
                continue;
            }
            if ballot.choice.is_against() {
                stakes.push((ballot.voter, ballot.stake));
                burns.insert(ballot.voter, ballot.stake);
            } else {
                let amount_to_mint = total_stake_against * ballot.stake / total_stake_in_favor;
                mints.insert(ballot.voter, amount_to_mint);
            }
        }
        self.refs.reputation_token().bulk_unstake(stakes);
        self.refs
            .reputation_token()
            .bulk_mint_burn(mints.clone(), burns.clone());
        (mints, burns)
    }

    fn redistribute_reputation_of_yes_voters(
        &self,
        voting: &VotingStateMachine,
        voting_type: VotingType,
    ) -> (BTreeMap<Address, Balance>, BTreeMap<Address, Balance>) {
        let voting_id = voting.voting_id();
        let total_stake_in_favor = voting.stake_in_favor();
        let total_stake_against = voting.stake_against();
        let mut burns: BTreeMap<Address, Balance> = BTreeMap::new();
        let mut mints: BTreeMap<Address, Balance> = BTreeMap::new();
        let mut stakes: Vec<(Address, Balance)> = Vec::new();
        for i in 0..self.voters_count(voting_id, voting_type) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.unbound || ballot.canceled {
                continue;
            }
            if ballot.choice.is_in_favor() {
                stakes.push((ballot.voter, ballot.stake));
                burns.insert(ballot.voter, ballot.stake);
            } else {
                let amount_to_mint = total_stake_in_favor * ballot.stake / total_stake_against;
                mints.insert(ballot.voter, amount_to_mint);
            }
        }
        self.refs.reputation_token().bulk_unstake(stakes);
        self.refs
            .reputation_token()
            .bulk_mint_burn(mints.clone(), burns.clone());
        (mints, burns)
    }

    fn is_va(&self, address: &Address) -> bool {
        !self.refs.va_token().balance_of(address).is_zero()
    }

    /// Get the governance voting's voters list.
    pub fn voters(&self, voting_id: VotingId, voting_type: VotingType) -> List<Address> {
        self.voters.get_instance(&(voting_id, voting_type))
    }

    /// Gets the total number of users participated in voting.
    pub fn voters_count(&self, voting_id: VotingId, voting_type: VotingType) -> u32 {
        self.voters(voting_id, voting_type).len()
    }

    fn bound_ballot(
        &mut self,
        voting: &mut VotingStateMachine,
        address: Address,
        voting_type: VotingType,
    ) {
        let mut ballot = self
            .get_ballot(voting.voting_id(), voting_type, address)
            .unwrap_or_revert_with(Error::BallotDoesNotExist);

        voting.bind_stake(ballot.stake, ballot.choice);

        self.refs.reputation_token().mint(address, ballot.stake);
        self.refs.reputation_token().stake(address, ballot.stake);

        ballot.unbound = false;
        self.ballots
            .set(&(voting.voting_id(), voting_type, address), ballot);
    }

    /// Checks if voting with the given id and type exists.
    pub fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool {
        let voting = self.get_voting(voting_id);
        match voting {
            None => false,
            Some(voting) => voting.voting_type() == voting_type,
        }
    }

    /// Erases a voter from all active votings.
    ///
    /// If the voter is also the creator, voting is canceled.
    /// Otherwise, only his vote is invalidated.
    ///
    /// Returns a tuple of vectors listing canceled and affected votings.j
    pub fn slash_voter(&mut self, voter: Address) -> (Vec<VotingId>, Vec<VotingId>) {
        let active_voting_ids = self.active_votings.get_or_default();
        let mut affected_votings = vec![];
        let mut canceled_votings = vec![];
        for voting_id in active_voting_ids.into_iter() {
            let voting = self.get_voting_or_revert(voting_id);
            if voting.creator() == &voter {
                self.cancel_voting(voting);
                canceled_votings.push(voting_id);
            } else if self.cancel_ballot(voting, voter) {
                affected_votings.push(voting_id);
            }
        }
        (canceled_votings, affected_votings)
    }

    fn cancel_voting(&mut self, mut voting: VotingStateMachine) {
        let voting_id = voting.voting_id();
        let voting_type = voting.voting_type();
        let unstakes = self.unstake_all_reputation(voting_id, voting_type);
        voting.cancel();
        self.set_voting(voting);
        self.remove_from_active_list(voting_id);

        // Emit event.
        emit_event(VotingCanceled::new(voting_id, voting_type, unstakes));
    }

    // Note: it doesn't remove a voter from self.votings to keep the quorum num right.
    /// Cancels voter's ballot in the given voting.
    /// Returns true if the voter has voted in the voting and the ballot was canceled.
    fn cancel_ballot(&mut self, mut voting: VotingStateMachine, voter: Address) -> bool {
        let voting_id = voting.voting_id();
        let ballots_key = (voting_id, voting.voting_type(), voter);
        let mut ballot = match self.ballots.get(&ballots_key) {
            Some(ballot) => ballot,
            None => return false, // End method if voter never voted in this voting.
        };

        // Unstake reputation.
        self.refs.reputation_token().unstake(voter, ballot.stake);

        // Update voting.
        let stake = ballot.stake;
        let choice = ballot.choice;
        if ballot.unbound {
            voting.remove_unbound_stake(stake, choice)
        } else {
            voting.remove_stake(stake, choice);
        }
        self.set_voting(voting);

        // Emit event.
        emit_event(BallotCanceled::new(&ballot));

        // Update ballot.
        ballot.canceled = true;
        self.ballots.set(&ballots_key, ballot);

        true
    }

    fn add_to_active_list(&mut self, voting_id: VotingId) {
        let mut active_list = self.active_votings.get_or_default();
        active_list.push(voting_id);
        self.active_votings.set(active_list);
    }

    fn remove_from_active_list(&mut self, voting_id: VotingId) {
        let mut active_list = self.active_votings.get_or_default();
        active_list.retain(|&id| id != voting_id);
        self.active_votings.set(active_list);
    }
}

fn add_to_map(
    target: &mut BTreeMap<(Address, Reason), Balance>,
    reason: Reason,
    source: BTreeMap<Address, Balance>,
) {
    for (addr, amount) in source {
        target.insert((addr, reason), amount);
    }
}
