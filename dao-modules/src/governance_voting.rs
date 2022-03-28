//! Governance Voting module.

pub mod addresses_collection;
pub mod events;
pub mod vote;
pub mod voting;

use casper_dao_utils::{
    casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert},
    casper_env::{caller, emit, get_block_time, revert, self_address},
    consts, Address, Error, Mapping, Variable,
};
use casper_types::U256;

use self::{
    events::{
        FormalVotingEnded, InformalVotingEnded, VoteCast, VotingContractCreated, VotingCreated,
    },
    vote::Vote,
    voting::{Voting, VotingId},
};

/// The Governance Voting module.

pub struct GovernanceVoting {
    pub variable_repo: Variable<Option<Address>>,
    pub reputation_token: Variable<Option<Address>>,
    pub votings: Mapping<U256, Voting>,
    pub votes: Mapping<(U256, Address), Vote>,
    pub voters: Mapping<U256, Vec<Option<Address>>>,
    pub votings_count: Variable<U256>,
}

impl Default for GovernanceVoting {
    fn default() -> Self {
        Self {
            variable_repo: Variable::from(consts::NAME_VARIABLE_REPO),
            reputation_token: Variable::from(consts::NAME_REPUTATION_TOKEN),
            votings: Mapping::from(consts::NAME_VOTINGS),
            votes: Mapping::from(consts::NAME_VOTES),
            voters: Mapping::from(consts::NAME_VOTERS),
            votings_count: Variable::from(consts::NAME_VOTINGS_COUNT),
        }
    }
}

impl GovernanceVoting {
    /// Initialize the module.
    pub fn init(&mut self, variable_repo: Address, reputation_token: Address) {
        self.variable_repo.set(Some(variable_repo));
        self.reputation_token.set(Some(reputation_token));
        // Emit the event
        let event = VotingContractCreated {
            variable_repo,
            reputation_token,
            repo_voter: self_address(),
        };

        emit(event);
    }

    pub fn get_variable_repo_address(&self) -> Address {
        self.variable_repo.get().unwrap_or_revert()
    }

    pub fn create_voting(&mut self, voting: &Voting, stake: U256) {
        // Add Voting
        self.votings.set(&voting.voting_id, voting.clone());
        self.votings_count.set(voting.voting_id + 1);

        // Emit the event
        let event = VotingCreated {
            creator: caller(),
            voting_id: voting.voting_id,
            stake,
        };

        emit(event);

        // Cast first vote in favor
        self.vote(voting.voting_id, true, stake);
    }

    pub fn finish_voting(&mut self, voting_id: VotingId) {
        let voting = self.votings.get(&voting_id);

        // we cannot finish already completed voting
        if voting.completed {
            revert(Error::FinishingCompletedVotingNotAllowed)
        }

        // is it formal or informal?
        if voting.voting_id == voting.informal_voting_id {
            self.finish_informal_voting(voting);
        } else if voting.voting_id == voting.formal_voting_id.unwrap() {
            self.finish_formal_voting(voting);
        } else {
            revert(Error::MalformedVoting)
        }
    }

    fn finish_informal_voting(&mut self, mut voting: Voting) {
        if !self.is_voting_in_time(&voting) {
            revert(Error::InformalVotingTimeNotReached)
        }

        let voters = self.voters.get(&voting.voting_id);
        if U256::from(voters.len()) < voting.informal_voting_quorum {
            // quorum is not reached
            // TODO the stake of the other is returned
            // TODO the stake of the creator is burned

            // the voting is marked as completed
            voting.completed = true;
            self.votings.set(&voting.voting_id, voting.clone());

            // emit the event
            let event = InformalVotingEnded {
                result: "quorum_not_reached".into(),
                votes_count: voters.len().into(),
                stake_in_favor: voting.stake_in_favor,
                stake_against: voting.stake_against,
                informal_voting_id: voting.informal_voting_id,
                formal_voting_id: voting.formal_voting_id,
            };
            emit(event);
        } else if self.calculate_result(&voting) {
            // the voting passed
            // TODO: the stake of the other voters is returned to them

            // the voting is marked as completed and saved
            voting.formal_voting_id = Some(self.votings_count.get());
            voting.completed = true;
            self.votings.set(&voting.voting_id, voting.clone());

            // emit the event
            let event = InformalVotingEnded {
                result: "converted_to_formal".into(),
                votes_count: voters.len().into(),
                stake_in_favor: voting.stake_in_favor,
                stake_against: voting.stake_against,
                informal_voting_id: voting.informal_voting_id,
                formal_voting_id: voting.formal_voting_id,
            };
            emit(event);

            // the voting is converted to a formal one
            voting.voting_id = voting.formal_voting_id.unwrap();
            voting.finish_time = U256::from(get_block_time() + voting.formal_voting_time.as_u64());
            voting.stake_against = 0.into();
            voting.stake_in_favor = 0.into();
            voting.completed = false;

            // and created with an initial stake
            let creator_address = voters.first().unwrap().unwrap();
            self.create_voting(
                &voting,
                self.votes
                    .get(&(voting.informal_voting_id, creator_address))
                    .stake,
            );
        } else {
            // the voting did not pass
            // TODO: the stake of the creator of the vote is burned
            // TODO: the stake of other voters is returned to them

            // the voting is marked as completed
            voting.completed = true;
            self.votings.set(&voting.voting_id, voting.clone());
            // finally, emit the event
            let event = InformalVotingEnded {
                result: "rejected".into(),
                votes_count: voters.len().into(),
                stake_in_favor: voting.stake_in_favor,
                stake_against: voting.stake_against,
                informal_voting_id: voting.informal_voting_id,
                formal_voting_id: None,
            };
            emit(event);
        }
    }

    fn finish_formal_voting(&mut self, mut voting: Voting) {
        if !self.is_voting_in_time(&voting) {
            revert(Error::FormalVotingTimeNotReached)
        }

        let voters = self.voters.get(&voting.voting_id);

        if U256::from(voters.len()) < voting.formal_voting_quorum {
            // quorum is not reached
            // TODO the stake of the other is returned
            // TODO the stake of the creator is burned

            // the voting is marked as completed
            voting.completed = true;
            self.votings.set(&voting.voting_id, voting.clone());

            // emit the event
            let event = FormalVotingEnded {
                result: "quorum_not_reached".into(),
                votes_count: voters.len().into(),
                stake_in_favor: voting.stake_in_favor,
                stake_against: voting.stake_against,
                informal_voting_id: voting.informal_voting_id,
                formal_voting_id: voting.formal_voting_id,
            };
            emit(event);
        } else if self.calculate_result(&voting) {
            // the voting passed
            // TODO: the stake of the losers is redistributed

            // the voting is marked as completed and saved
            voting.formal_voting_id = Some(self.votings_count.get());
            voting.completed = true;
            self.votings.set(&voting.voting_id, voting.clone());

            // emit the event
            let event = FormalVotingEnded {
                result: "passed".into(),
                votes_count: voters.len().into(),
                stake_in_favor: voting.stake_in_favor,
                stake_against: voting.stake_against,
                informal_voting_id: voting.informal_voting_id,
                formal_voting_id: voting.formal_voting_id,
            };
            emit(event);

            // perform the action
            self.perform_action(&voting);
        } else {
            // the voting did not pass
            // TODO: the stake of the losers is redistributed

            // the voting is marked as completed
            voting.completed = true;
            self.votings.set(&voting.voting_id, voting.clone());
            // finally, emit the event
            let event = FormalVotingEnded {
                result: "rejected".into(),
                votes_count: voters.len().into(),
                stake_in_favor: voting.stake_in_favor,
                stake_against: voting.stake_against,
                informal_voting_id: voting.informal_voting_id,
                formal_voting_id: None,
            };
            emit(event);
        }
    }

    fn perform_action(&mut self, voting: &Voting) {
        let contract_package_hash = *voting
            .contract_to_call
            .unwrap_or_revert()
            .as_contract_package_hash()
            .unwrap_or_revert();

        runtime::call_versioned_contract::<()>(
            contract_package_hash,
            None,
            &voting.entry_point,
            voting.runtime_args.clone(),
        );
    }

    fn calculate_result(&mut self, voting: &Voting) -> bool {
        if voting.stake_in_favor >= voting.stake_against {
            return true;
        }

        false
    }

    fn is_voting_in_time(&mut self, voting: &Voting) -> bool {
        if voting.finish_time.as_u64() < get_block_time() {
            return true;
        }

        false
    }

    pub fn vote(&mut self, voting_id: U256, choice: bool, stake: U256) {
        let mut voting = self.votings.get(&voting_id);

        // We cannot vote on a completed voting
        if voting.completed {
            revert(Error::VoteOnCompletedVotingNotAllowed)
        }

        // Add a vote to the list
        let vote = Vote {
            voter: caller(),
            choice: true,
            voting_id,
            stake,
        };

        self.votes.set(&(voting_id, caller()), vote);

        // Add a voter to the list
        let mut voters = self.voters.get(&voting_id);
        voters.push(Some(caller()));

        self.voters.set(&voting_id, voters);

        match choice {
            true => voting.stake_in_favor += stake,
            false => voting.stake_against += stake,
        }

        self.votings.set(&voting_id, voting);

        // Emit event
        let event = VoteCast {
            voter: caller(),
            voting_id,
            choice,
            stake,
        };

        emit(event);
    }
}
