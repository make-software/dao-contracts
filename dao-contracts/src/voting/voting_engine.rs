//! Governance Voting module.
//! 
use std::collections::BTreeMap;

use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    casper_env::{get_block_time, revert},
    Address,
    Error,
    Mapping,
    VecMapping,
};
use casper_types::U512;

use self::{
    events::{BallotCast, VotingCreatedInfo},
    voting_state_machine::{VotingResult, VotingStateMachine, VotingSummary, VotingType},
};
use super::{
    ballot::Choice,
    ids,
    events::{BallotCanceled, Reason, VotingCanceled, VotingEnded},
    refs::{ContractRefs, ContractRefsStorage},
    types::VotingId,
    Ballot,
    ShortenedBallot,
};
use crate::{
    config::Configuration,
    reputation::ReputationContractInterface,
    rules::RulesBuilder,
    va_nft::VaNftContractInterface,
    rules::validation::voting::CanCreateVoting,
};

pub mod events;
pub mod voting_state_machine;

/// Governance voting is a struct that contracts can use to implement voting. 
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
/// 1. [Reputation Token](crate::reputation::ReputationContract) to handle reputation staking.
/// 2. [Variable Repo](crate::variable_repository::VariableRepositoryContract) for reading voting configuration.
///
/// For example implementation see [AdminContract](crate::admin::AdminContract).
#[derive(Instance)]
pub struct VotingEngine {
    #[scoped = "contract"]
    refs: ContractRefsStorage,
    voting_states: Mapping<VotingId, Option<VotingStateMachine>>,
    ballots: Mapping<(VotingId, VotingType, Address), Ballot>,
    voters: VecMapping<(VotingId, VotingType), Address>,
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
    /// [Variable Repo]: crate::variable_repository::VariableRepositoryContract
    /// [`Error::NotOnboarded`]: casper_dao_utils::Error::NotOnboarded
    /// [Dao Ids Contract]: crate::ids::DaoIdsContractInterface
    /// [`cast`]: Self::cast_ballot()
    pub fn create_voting(
        &mut self,
        creator: Address,
        stake: U512,
        configuration: Configuration,
    ) -> (VotingCreatedInfo, VotingStateMachine) {
        RulesBuilder::new()
            .add_validation(CanCreateVoting::create(
                self.is_va(creator),
                configuration.only_va_can_create(),
            ))
            .build()
            .validate_generic_validations();

        let should_cast_first_vote = configuration.should_cast_first_vote();

        let voting_ids_address = configuration.voting_ids_address();
        let voting_id = ids::get_next_voting_id(voting_ids_address);
        let mut voting =
            VotingStateMachine::new(voting_id, get_block_time(), creator, configuration);

        let mut used_stake = None;
        if should_cast_first_vote {
            self.cast_vote(
                creator,
                VotingType::Informal,
                Choice::InFavor,
                stake,
                &mut voting,
            );
            used_stake = Some(stake);
        }

        let info = VotingCreatedInfo::new(
            creator,
            voting_id,
            used_stake,
            voting.voting_configuration(),
        );
        self.set_voting(voting.clone());
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

        self.assert_voting_type(&voting, voting_type);

        if voting.completed() {
            revert(Error::FinishingCompletedVotingNotAllowed)
        }

        let mut rep_unstakes = BTreeMap::new();
        let mut rep_burns = BTreeMap::new();
        let mut rep_mints = BTreeMap::new();

        let summary = match voting.voting_type() {
            VotingType::Informal => {
                let informal_without_stake = voting.is_informal_without_stake();
                let voting_result = self.finish_informal_voting(&mut voting);
                if !informal_without_stake {
                    let yes_unstakes = self.return_yes_voters_rep(voting_id, VotingType::Informal);
                    let no_unstakes = self.return_no_voters_rep(voting_id, VotingType::Informal);
                    add_to_map(&mut rep_unstakes, Reason::InformalFinished, yes_unstakes);
                    add_to_map(&mut rep_unstakes, Reason::InformalFinished, no_unstakes);
                }

                match voting_result.result() {
                    VotingResult::InFavor | VotingResult::Against => {
                        // It emits BallotCast event, so no need to capture it in VotingEnded event.
                        self.recast_creators_ballot_from_informal_to_formal(&mut voting);
                    }
                    VotingResult::QuorumNotReached => {}
                    VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
                }
                voting_result
            }
            VotingType::Formal => {
                let voting_result = self.finish_formal_voting(&mut voting);
                match voting_result.result() {
                    VotingResult::InFavor => {
                        if voting
                            .voting_configuration()
                            .should_bind_ballot_for_successful_voting()
                        {
                            let worker = voting
                                .voting_configuration()
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
        let voting_ended_event = VotingEnded::new(
            &voting,
            summary.result(),
            stats,
            rep_unstakes,
            BTreeMap::new(),
            rep_burns,
            rep_mints,
        );
        voting_ended_event.emit();

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
    ) -> VotingSummary {
        let mut voting = self
            .get_voting(voting_id)
            .unwrap_or_revert_with(Error::VotingDoesNotExist);

        if voting.completed() {
            revert(Error::FinishingCompletedVotingNotAllowed)
        }

        let summary = match voting.voting_type() {
            VotingType::Informal => self.finish_informal_voting(&mut voting),
            VotingType::Formal => self.finish_formal_voting(&mut voting),
        };

        self.set_voting(voting);

        summary
    }

    fn finish_informal_voting(&mut self, voting: &mut VotingStateMachine) -> VotingSummary {
        if !voting.is_in_time(get_block_time()) {
            revert(Error::InformalVotingTimeNotReached)
        }

        let voting_id = voting.voting_id();
        let voters_len = self.voters.len((voting_id, voting.voting_type()));
        let voting_result = voting.get_result(voters_len);
        match voting_result {
            VotingResult::InFavor | VotingResult::Against => {
                voting.complete_informal_voting();
            }
            VotingResult::QuorumNotReached => {
                voting.finish();
            }
            VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
        };

        VotingSummary::new(voting_result, VotingType::Informal, voting_id)
    }

    fn finish_formal_voting(&mut self, voting: &mut VotingStateMachine) -> VotingSummary {
        voting.guard_finish_formal_voting(get_block_time());
        let voting_id = voting.voting_id();
        let voters_len = self.voters.len((voting_id, VotingType::Formal));
        let voting_result = voting.get_result(voters_len);

        if voting_result == VotingResult::InFavor {
            self.perform_action(voting.voting_id());
        }

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
        stake: U512,
    ) {
        let mut voting = self.get_voting_or_revert(voting_id);
        self.cast_vote(voter, voting_type, choice, stake, &mut voting);
        self.set_voting(voting);
    }

    fn cast_vote(
        &mut self,
        voter: Address,
        voting_type: VotingType,
        choice: Choice,
        stake: U512,
        voting: &mut VotingStateMachine,
    ) {
        let voting_id = voting.voting_id();
        self.assert_voting_type(voting, voting_type);
        voting.guard_vote(get_block_time());
        self.assert_vote_doesnt_exist(voting_id, voting.voting_type(), voter);
        self.cast_ballot(voter, voting_id, choice, stake, false, voting);
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
    /// [Reputation Token Contract]: crate::reputation::ReputationContractInterface
    pub fn cast_ballot(
        &mut self,
        voter: Address,
        voting_id: VotingId,
        choice: Choice,
        stake: U512,
        unbound: bool,
        voting: &mut VotingStateMachine,
    ) {
        let ballot = Ballot::new(
            voter,
            voting_id,
            voting.voting_type(),
            choice,
            stake,
            unbound,
            false,
        );

        if !unbound && !voting.is_informal_without_stake() {
            // Stake the reputation
            self.refs
                .reputation_token()
                .stake_voting(voting_id, ballot.clone().into());
        }

        BallotCast::new(&ballot).emit();

        // Add a voter to the list
        self.voters.add((voting_id, voting.voting_type()), voter);

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
        self.voters.get_all((voting_id, voting_type))
    }

    /// Returns the Voter's [`Ballot`].
    pub fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot> {
        self.ballots.get_or_none(&(voting_id, voting_type, address))
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
        self.voters.get_or_none((voting_id, voting_type), at)
    }

    /// Returns the [Voting](VotingStateMachine) for a given id.
    pub fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine> {
        self.voting_states
            .get_or_none(&voting_id)
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

    /// Updates voting storage.
    pub fn set_voting(&self, voting: VotingStateMachine) {
        self.voting_states.set(&voting.voting_id(), Some(voting))
    }

    fn perform_action(&self, voting_id: VotingId) {
        let voting = self.get_voting(voting_id).unwrap_or_revert();
        for contract_call in voting.contract_calls() {
            contract_call.call();
        }
    }

    /// Iterates over all the ballots and unstakes reputation. Returns a map of address to it's stake.
    /// 
    /// Calls [Reputation Token Contract](crate::reputation::ReputationContractInterface) to perform unstake operation.
    pub fn unstake_all_reputation(
        &mut self,
        voting_id: VotingId,
        voting_type: VotingType,
    ) -> BTreeMap<Address, U512> {
        let mut transfers = BTreeMap::new();
        let mut ballots = Vec::<ShortenedBallot>::new();
        for i in 0..self.voters.len((voting_id, voting_type)) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.unbound || ballot.canceled {
                continue;
            }
            transfers.insert(ballot.voter, ballot.stake);
            ballots.push(ballot.into());
        }
        self.refs
            .reputation_token()
            .bulk_unstake_voting(voting_id, ballots);
        transfers
    }

    fn recast_creators_ballot_from_informal_to_formal(
        &mut self,
        voting: &mut VotingStateMachine,
    ) {
        let voting_id = voting.voting_id();
        let creator = voting.creator();
        let creator_ballot = self
            .get_ballot(voting_id, VotingType::Informal, *creator)
            .unwrap_or_revert_with(Error::BallotDoesNotExist);

        self.cast_ballot(
            *creator,
            voting_id,
            Choice::InFavor,
            creator_ballot.stake,
            creator_ballot.unbound,
            voting,
        );
    }

    fn return_yes_voters_rep(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
    ) -> BTreeMap<Address, U512> {
        let mut summary = BTreeMap::new();
        let mut ballots = Vec::<ShortenedBallot>::new();
        for i in 0..self.voters.len((voting_id, voting_type)) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.choice.is_in_favor() && !ballot.unbound && !ballot.canceled {
                ballots.push(ballot.clone().into());
                summary.insert(ballot.voter, ballot.stake);
            }
        }
        self.refs
            .reputation_token()
            .bulk_unstake_voting(voting_id, ballots);
        summary
    }

    fn return_no_voters_rep(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
    ) -> BTreeMap<Address, U512> {
        let mut summary = BTreeMap::new();
        let mut ballots = Vec::<ShortenedBallot>::new();
        for i in 0..self.voters.len((voting_id, voting_type)) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.choice.is_against() && !ballot.unbound && !ballot.canceled {
                ballots.push(ballot.clone().into());
                summary.insert(ballot.voter, ballot.stake);
            }
        }
        self.refs
            .reputation_token()
            .bulk_unstake_voting(voting_id, ballots);
        summary
    }

    fn redistribute_reputation_of_no_voters(
        &self,
        voting: &VotingStateMachine,
        voting_type: VotingType,
    ) -> (BTreeMap<Address, U512>, BTreeMap<Address, U512>) {
        let total_stake_in_favor = voting.stake_in_favor();
        let voting_id = voting.voting_id();
        let total_stake_against = voting.stake_against();
        let mut burns: BTreeMap<Address, U512> = BTreeMap::new();
        let mut mints: BTreeMap<Address, U512> = BTreeMap::new();
        let mut ballots: Vec<ShortenedBallot> = Vec::new();

        for i in 0..self.voters.len((voting_id, voting_type)) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.unbound {
                continue;
            }
            if ballot.choice.is_against() {
                ballots.push(ballot.clone().into());
                burns.insert(ballot.voter, ballot.stake);
            } else {
                let amount_to_mint = total_stake_against * ballot.stake / total_stake_in_favor;
                mints.insert(ballot.voter, amount_to_mint);
            }
        }
        self.refs
            .reputation_token()
            .bulk_unstake_voting(voting_id, ballots);
        self.refs
            .reputation_token()
            .bulk_mint_burn(mints.clone(), burns.clone());
        (mints, burns)
    }

    fn redistribute_reputation_of_yes_voters(
        &self,
        voting: &VotingStateMachine,
        voting_type: VotingType,
    ) -> (BTreeMap<Address, U512>, BTreeMap<Address, U512>) {
        let voting_id = voting.voting_id();
        let total_stake_in_favor = voting.stake_in_favor();
        let total_stake_against = voting.stake_against();
        let mut burns: BTreeMap<Address, U512> = BTreeMap::new();
        let mut mints: BTreeMap<Address, U512> = BTreeMap::new();
        let mut ballots: Vec<ShortenedBallot> = Vec::new();
        for i in 0..self.voters.len((voting_id, voting_type)) {
            let ballot = self.get_ballot_at(voting_id, voting_type, i);
            if ballot.unbound {
                continue;
            }
            if ballot.choice.is_in_favor() {
                ballots.push(ballot.clone().into());
                burns.insert(ballot.voter, ballot.stake);
            } else {
                let amount_to_mint = total_stake_in_favor * ballot.stake / total_stake_against;
                mints.insert(ballot.voter, amount_to_mint);
            }
        }
        self.refs
            .reputation_token()
            .bulk_unstake_voting(voting_id, ballots);
        self.refs
            .reputation_token()
            .bulk_mint_burn(mints.clone(), burns.clone());
        (mints, burns)
    }

    fn is_va(&self, address: Address) -> bool {
        !self.refs.va_token().balance_of(address).is_zero()
    }

    /// Get a reference to the governance voting's voters.
    pub fn voters(&self) -> &VecMapping<(VotingId, VotingType), Address> {
        &self.voters
    }

    /// Gets the total number of users participated in voting.
    pub fn voters_count(&self, voting_id: VotingId, voting_type: VotingType) -> u32 {
        self.voters().len((voting_id, voting_type))
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
        self.refs
            .reputation_token()
            .stake_voting(voting.voting_id(), ballot.clone().into());

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

    /// Erases a voter from a given voting.
    /// 
    /// If the voter is also the creator, voting is canceled.
    /// Otherwise, only his vote is invalidated.
    pub fn slash_voter(&mut self, voter: Address, voting_id: VotingId) {
        let voting = self.get_voting_or_revert(voting_id);
        if &voter == voting.creator() {
            self.cancel_voting(voting);
        } else {
            self.cancel_ballot(voting, voter);
        }
    }

    fn cancel_voting(&mut self, mut voting: VotingStateMachine) {
        let voting_id = voting.voting_id();
        let voting_type = voting.voting_type();
        let unstakes = self.unstake_all_reputation(voting_id, voting_type);
        voting.cancel();
        self.set_voting(voting);

        // Emit event.
        VotingCanceled::new(voting_id, voting_type, unstakes).emit();
    }

    // Note: it doesn't remove a voter from self.votings to keep the quorum num right.
    fn cancel_ballot(&mut self, mut voting: VotingStateMachine, voter: Address) {
        let voting_id = voting.voting_id();
        let ballots_key = (voting_id, voting.voting_type(), voter);
        let mut ballot = self
            .ballots
            .get(&ballots_key)
            .unwrap_or_revert_with(Error::BallotDoesNotExist);

        // Unstake reputation.
        self.refs
            .reputation_token()
            .unstake_voting(voting_id, ballot.clone().into());

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
        BallotCanceled::new(&ballot).emit();

        // Update ballot.
        ballot.canceled = true;
        self.ballots.set(&ballots_key, ballot);
    }
}

fn add_to_map(
    target: &mut BTreeMap<(Address, Reason), U512>,
    reason: Reason,
    source: BTreeMap<Address, U512>,
) {
    for (addr, amount) in source {
        target.insert((addr, reason), amount);
    }
}
