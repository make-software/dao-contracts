//! Governance Voting module.
pub mod consts;
pub mod events;
pub mod voting;

use casper_dao_utils::conversions::{u256_to_512, u512_to_u256};
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    casper_env::{emit, get_block_time, revert, self_address},
    Address, Error, Mapping, Variable,
};

use casper_types::{U256, U512};

use crate::proxy::reputation_proxy::ReputationContractProxy;

use self::voting::VotingSummary;
use self::{
    events::{BallotCast, VotingContractCreated, VotingCreated},
    voting::{Voting, VotingConfiguration, VotingResult, VotingType},
};

use casper_dao_utils::VecMapping;

use super::ballot::Choice;
use super::VotingEnded;
use super::{ballot::VotingId, Ballot};

pub trait GovernanceVotingTrait {
    fn init(&mut self, variable_repo: Address, reputation_token: Address);
}

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
pub struct GovernanceVoting {
    variable_repo: Variable<Address>,
    reputation_token: Variable<Address>,
    votings: Mapping<VotingId, Option<Voting>>,
    ballots: Mapping<(VotingId, Address), Ballot>,
    voters: VecMapping<VotingId, Address>,
    votings_count: Variable<U256>,
    dust_amount: Variable<U256>,
}

impl GovernanceVoting {
    /// Initializes the module with [Addresses](Address) of [Reputation Token](crate::ReputationContract) and [Variable Repo](crate::VariableRepositoryContract)
    ///
    /// # Events
    /// Emits [`VotingContractCreated`](VotingContractCreated)
    pub fn init(&mut self, variable_repo: Address, reputation_token: Address) {
        self.variable_repo.set(variable_repo);
        self.reputation_token.set(reputation_token);

        VotingContractCreated {
            variable_repo,
            reputation_token,
            voter_contract: self_address(),
        }
        .emit();
    }

    /// Creates new informal [Voting](Voting).
    ///
    /// `contract_to_call`, `entry_point` and `runtime_args` parameters define an action that will be performed  when formal voting passes.
    ///
    /// It collects configuration from [Variable Repo](crate::VariableRepositoryContract) and persists it, so they won't change during the voting process.
    ///
    /// It automatically casts first vote in favor in name of the creator.
    ///
    /// # Events
    /// Emits [`VotingCreated`](VotingCreated), [`BallotCast`](BallotCast)
    ///
    /// # Errors
    /// Throws [`Error::NotEnoughReputation`](casper_dao_utils::Error::NotEnoughReputation) when the creator does not have enough reputation to create a voting
    pub fn create_voting(
        &mut self,
        creator: Address,
        stake: U256,
        voting_configuration: VotingConfiguration,
    ) -> VotingId {
        if stake < voting_configuration.create_minimum_reputation {
            revert(Error::NotEnoughReputation)
        }

        let cast_first_vote = voting_configuration.cast_first_vote;
        let voting_id = self.next_voting_id();
        let voting = Voting::new(voting_id, get_block_time(), voting_configuration);

        self.set_voting(voting);

        VotingCreated {
            creator,
            voting_id,
            stake,
        }
        .emit();

        // Cast first vote in favor
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
    /// Throws [`FinishingCompletedVotingNotAllowed`](casper_dao_utils::Error::FinishingCompletedVotingNotAllowed) if trying to complete already finished voting
    ///
    /// Throws [`FormalVotingTimeNotReached`](casper_dao_utils::Error::FormalVotingTimeNotReached) if formal voting time did not pass
    ///
    /// Throws [`InformalVotingTimeNotReached`](casper_dao_utils::Error::InformalVotingTimeNotReached) if informal voting time did not pass
    ///
    /// Throws [`ArithmeticOverflow`](casper_dao_utils::Error::ArithmeticOverflow) in an unlikely event of a overflow when calculating reputation to redistribute
    pub fn finish_voting(&mut self, voting_id: VotingId) -> VotingSummary {
        let voting = self
            .get_voting(voting_id)
            .unwrap_or_revert_with(Error::VotingDoesNotExist);

        if voting.completed() {
            revert(Error::FinishingCompletedVotingNotAllowed)
        }

        match voting.get_voting_type() {
            VotingType::Informal => self.finish_informal_voting(voting),
            VotingType::Formal => self.finish_formal_voting(voting),
        }
    }

    fn finish_informal_voting(&mut self, mut voting: Voting) -> VotingSummary {
        if !voting.is_in_time(get_block_time()) {
            revert(Error::InformalVotingTimeNotReached)
        }
        let voters_len = self.voters.len(voting.voting_id());
        let voting_result = voting.get_result(voters_len);
        let result = match voting_result {
            VotingResult::InFavor => {
                self.return_reputation(voting.voting_id());

                let formal_voting_id = self.next_voting_id();
                let creator_address = self.voters.get(voting.voting_id(), 0).unwrap_or_revert();
                let creator_stake = self
                    .ballots
                    .get(&(voting.voting_id(), creator_address))
                    .unwrap_or_revert_with(Error::BallotDoesNotExist)
                    .stake;

                // Formal voting is created and first vote cast
                self.set_voting(voting.create_formal_voting(formal_voting_id, get_block_time()));

                emit(VotingCreated {
                    creator: creator_address,
                    voting_id: formal_voting_id,
                    stake: creator_stake,
                });
                if voting.voting_configuration().cast_first_vote {
                    self.vote(
                        creator_address,
                        formal_voting_id,
                        Choice::InFavor,
                        creator_stake,
                    );
                }

                // Informal voting is completed and referenced with formal voting
                voting.complete(Some(formal_voting_id));

                consts::INFORMAL_VOTING_PASSED
            }
            VotingResult::Against => {
                self.burn_creators_and_return_others_reputation(voting.voting_id());
                voting.complete(None);

                consts::INFORMAL_VOTING_REJECTED
            }
            VotingResult::QuorumNotReached => {
                self.burn_creators_and_return_others_reputation(voting.voting_id());
                voting.complete(None);

                consts::INFORMAL_VOTING_QUORUM_NOT_REACHED
            }
        };

        let informal_voting_id = voting.voting_id();
        let formal_voting_id = voting.formal_voting_id();
        VotingEnded {
            voting_id: informal_voting_id,
            result: result.into(),
            votes_count: voters_len.into(),
            stake_in_favor: voting.stake_in_favor(),
            stake_against: voting.stake_against(),
            informal_voting_id,
            formal_voting_id: voting.formal_voting_id(),
        }
        .emit();

        self.set_voting(voting);

        VotingSummary::new(
            voting_result,
            VotingType::Informal,
            informal_voting_id,
            formal_voting_id,
        )
    }

    fn finish_formal_voting(&mut self, mut voting: Voting) -> VotingSummary {
        if !voting.is_in_time(get_block_time()) {
            revert(Error::FormalVotingTimeNotReached)
        }

        let voters_len = self.voters.len(voting.voting_id());
        let voting_result = voting.get_result(voters_len);

        let result = match voting_result {
            VotingResult::InFavor => {
                self.redistribute_reputation(&voting);
                self.perform_action(&voting);
                consts::FORMAL_VOTING_PASSED
            }
            VotingResult::Against => {
                self.redistribute_reputation(&voting);
                consts::FORMAL_VOTING_REJECTED
            }
            VotingResult::QuorumNotReached => {
                self.burn_creators_and_return_others_reputation(voting.voting_id());
                consts::FORMAL_VOTING_QUORUM_NOT_REACHED
            }
        };

        let formal_voting_id = voting.voting_id();
        let informal_voting_id = voting.informal_voting_id();
        VotingEnded {
            voting_id: formal_voting_id,
            result: result.into(),
            votes_count: voters_len.into(),
            stake_in_favor: voting.stake_in_favor(),
            stake_against: voting.stake_against(),
            informal_voting_id,
            formal_voting_id: Some(formal_voting_id),
        }
        .emit();

        voting.complete(None);
        self.set_voting(voting);

        VotingSummary::new(
            voting_result,
            VotingType::Formal,
            informal_voting_id,
            Some(formal_voting_id),
        )
    }

    /// Casts a vote
    ///
    /// # Events
    /// Emits [`BallotCast`](BallotCast)
    ///
    /// # Errors
    /// Throws [`VoteOnCompletedVotingNotAllowed`](casper_dao_utils::Error::VoteOnCompletedVotingNotAllowed) if voting is completed
    ///
    /// Throws [`CannotVoteTwice`](casper_dao_utils::Error::CannotVoteTwice) if voter already voted
    pub fn vote(&mut self, voter: Address, voting_id: U256, choice: Choice, stake: U256) {
        let mut voting = self.get_voting(voting_id).unwrap_or_revert();

        // We cannot vote on a completed voting
        if voting.completed() {
            revert(Error::VoteOnCompletedVotingNotAllowed)
        }

        let vote = self.ballots.get(&(voting_id, voter));

        if vote.is_some() {
            revert(Error::CannotVoteTwice)
        }

        // Stake the reputation
        ReputationContractProxy::transfer(
            self.get_reputation_token_address(),
            voter,
            self_address(),
            stake,
        );

        // Create a new vote
        let vote = Ballot {
            voter,
            choice,
            voting_id,
            stake,
        };

        // Add a voter to the list
        self.voters.add(voting_id, voter);

        // Update the votes list
        self.ballots.set(&(voting_id, voter), vote);

        // update voting
        voting.stake(stake, choice);
        self.set_voting(voting);

        BallotCast {
            voter,
            voting_id,
            choice,
            stake,
        }
        .emit();
    }

    /// Returns the dust amount.
    ///
    /// Those are leftovers from redistribution of reputation tokens. For example, when 10 tokens needs to be redistributed between 3 voters,
    /// each will recieve 3 reputation, with 1 reputation left in the contract's balance.
    pub fn get_dust_amount(&self) -> U256 {
        self.dust_amount.get().unwrap_or_default()
    }

    /// Returns the address of [Variable Repo](crate::VariableRepositoryContract) connected to the contract
    pub fn get_variable_repo_address(&self) -> Address {
        self.variable_repo.get().unwrap_or_revert()
    }

    /// Returns the address of [Reputation Token](crate::ReputationContract) connected to the contract
    pub fn get_reputation_token_address(&self) -> Address {
        self.reputation_token.get().unwrap_or_revert()
    }

    /// Returns the [Ballot](Ballot) of voter with `address` and cast on `voting_id`
    pub fn get_ballot(&self, voting_id: U256, address: Address) -> Option<Ballot> {
        self.ballots.get_or_none(&(voting_id, address))
    }

    /// Returns the nth [Ballot](Ballot) of cast on `voting_id`
    pub fn get_ballot_at(&mut self, voting_id: U256, i: u32) -> Ballot {
        let address = self
            .get_voter(voting_id, i)
            .unwrap_or_revert_with(Error::VoterDoesNotExist);
        self.get_ballot(voting_id, address)
            .unwrap_or_revert_with(Error::BallotDoesNotExist)
    }

    /// Returns the address of nth voter who voted on Voting with `voting_id`
    pub fn get_voter(&self, voting_id: VotingId, at: u32) -> Option<Address> {
        self.voters.get_or_none(voting_id, at)
    }

    /// Returns the [Voting](Voting) for given id
    pub fn get_voting(&self, voting_id: VotingId) -> Option<Voting> {
        self.votings
            .get_or_none(&voting_id)
            .map(|x| x.unwrap_or_revert())
    }

    fn set_voting(&self, voting: Voting) {
        self.votings.set(&voting.voting_id(), Some(voting))
    }

    fn next_voting_id(&mut self) -> U256 {
        let voting_id = self.votings_count.get().unwrap_or_default();
        self.votings_count.set(voting_id + 1);
        voting_id
    }

    fn perform_action(&self, voting: &Voting) {
        match voting.contract_call() {
            Some(contract_call) => {
                contract_call.call();
            }
            None => {}
        }
    }

    fn burn_creators_and_return_others_reputation(&mut self, voting_id: VotingId) {
        for i in 0..self.voters.len(voting_id) {
            let ballot = self.get_ballot_at(voting_id, i);
            if i == 0 {
                // the creator
                ReputationContractProxy::burn(
                    self.get_reputation_token_address(),
                    self_address(),
                    ballot.stake,
                );
            } else {
                // the voters - transfer from contract to them
                ReputationContractProxy::transfer(
                    self.get_reputation_token_address(),
                    self_address(),
                    ballot.voter,
                    ballot.stake,
                );
            }
        }
    }

    fn return_reputation(&mut self, voting_id: VotingId) {
        for i in 0..self.voters.len(voting_id) {
            let ballot = self.get_ballot_at(voting_id, i);
            ReputationContractProxy::transfer(
                self.get_reputation_token_address(),
                self_address(),
                ballot.voter,
                ballot.stake,
            );
        }
    }

    fn redistribute_reputation(&mut self, voting: &Voting) {
        // TODO: update conversion after support for U256<>U512 conversion will be added to Casper
        let total_stake = u256_to_512(voting.total_stake()).unwrap_or_revert();
        let mut transferred = U512::zero();
        let result = voting.is_in_favor();
        let u256_max = u256_to_512(U256::MAX).unwrap_or_revert();

        for i in 0..self.voters.len(voting.voting_id()) {
            let ballot = self.get_ballot_at(voting.voting_id(), i);
            if ballot.choice.is_in_favor() == result {
                let to_transfer = total_stake * u256_to_512(ballot.stake).unwrap_or_revert()
                    / u256_to_512(voting.get_winning_stake()).unwrap_or_revert();

                if to_transfer > u256_max {
                    revert(Error::ArithmeticOverflow)
                }

                transferred += to_transfer;

                let to_transfer =
                    u512_to_u256(to_transfer).unwrap_or_revert_with(Error::ArithmeticOverflow);

                ReputationContractProxy::transfer(
                    self.get_reputation_token_address(),
                    self_address(),
                    ballot.voter,
                    to_transfer,
                );
            }
        }

        // mark leftovers
        let dust = total_stake - transferred;

        if dust > U512::zero() {
            if dust > u256_max {
                revert(Error::ArithmeticOverflow)
            }

            self.dust_amount.set(
                self.get_dust_amount()
                    + u512_to_u256(dust).unwrap_or_revert_with(Error::ArithmeticOverflow),
            );
        }
    }

    /// Get a reference to the governance voting's voters.
    pub fn voters(&self) -> &VecMapping<VotingId, Address> {
        &self.voters
    }
}
