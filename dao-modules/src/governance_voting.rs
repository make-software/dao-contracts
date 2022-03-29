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
use casper_types::{runtime_args, RuntimeArgs, U256};

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
    pub dust_amount: Variable<U256>,
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
            dust_amount: Variable::from(consts::NAME_DUST_AMOUNT),
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

    pub fn get_dust_amount(&self) -> U256 {
        self.dust_amount.get()
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

        if !voting.can_be_completed() {
            revert(Error::FinishingCompletedVotingNotAllowed)
        }

        if voting.is_informal() {
            self.finish_informal_voting(voting);
        } else if voting.is_formal() {
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
            // the stake of the other is returned
            // the stake of the creator is burned
            self.burn_creators_and_return_others_reputation(voting.voting_id);

            voting.complete();

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
            // the stake of all voters is returned to them (creator's reputation will be staked when creating a formal voting)
            self.return_reputation(voting.voting_id);

            // the voting is saved
            voting.formal_voting_id = Some(self.votings_count.get());
            voting.complete();

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

            voting.convert_to_formal(get_block_time());

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
            // the stake of the creator of the vote is burned
            // the stake of other voters is returned to them
            self.burn_creators_and_return_others_reputation(voting.voting_id);

            voting.complete();

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

        voting.complete();
        self.votings.set(&voting.voting_id, voting.clone());

        if U256::from(voters.len()) < voting.formal_voting_quorum {
            // quorum is not reached
            // the stake of the other is returned
            // the stake of the creator is burned
            self.burn_creators_and_return_others_reputation(voting.voting_id);

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
            // the stake of the losers is redistributed
            self.redistribute_reputation(&voting);

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
            // the stake of the losers is redistributed
            self.redistribute_reputation(&voting);

            // emit the event
            let event = FormalVotingEnded {
                result: "rejected".into(),
                votes_count: voters.len().into(),
                stake_in_favor: voting.stake_in_favor,
                stake_against: voting.stake_against,
                informal_voting_id: voting.informal_voting_id,
                formal_voting_id: voting.formal_voting_id,
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

    fn transfer_reputation(&mut self, owner: Address, recipient: Address, amount: U256) {
        let contract_package_hash = *self
            .reputation_token
            .get()
            .unwrap()
            .as_contract_package_hash()
            .unwrap_or_revert();

        let args: RuntimeArgs = runtime_args! {
            "owner" => owner,
            "recipient" => recipient,
            "amount" => amount,
        };

        runtime::call_versioned_contract::<()>(contract_package_hash, None, "transfer_from", args);
    }

    fn burn_reputation(&mut self, owner: Address, amount: U256) {
        let contract_package_hash = *self
            .reputation_token
            .get()
            .unwrap()
            .as_contract_package_hash()
            .unwrap_or_revert();

        let args: RuntimeArgs = runtime_args! {
            "owner" => owner,
            "amount" => amount,
        };

        runtime::call_versioned_contract::<()>(contract_package_hash, None, "burn", args);
    }

    fn burn_creators_and_return_others_reputation(&mut self, voting_id: VotingId) {
        let voters = self.voters.get(&voting_id);
        for (i, address) in voters.iter().enumerate() {
            let vote = self.votes.get(&(voting_id, address.unwrap_or_revert()));
            if i == 0 {
                // the creator
                self.burn_reputation(self_address(), vote.stake);
            } else {
                // the voters - transfer from contract to them
                self.transfer_reputation(self_address(), address.unwrap_or_revert(), vote.stake);
            }
        }
    }

    fn return_reputation(&mut self, voting_id: VotingId) {
        let voters = self.voters.get(&voting_id);
        for address in voters.iter() {
            let vote = self.votes.get(&(voting_id, address.unwrap_or_revert()));
            self.transfer_reputation(self_address(), address.unwrap_or_revert(), vote.stake);
        }
    }

    fn redistribute_reputation(&mut self, voting: &Voting) {
        let voters = self.voters.get(&voting.voting_id);
        let total_stake = voting.stake_in_favor + voting.stake_against;
        let mut transferred: U256 = 0.into();
        if voting.stake_in_favor >= voting.stake_against {
            for address in voters {
                let vote = self
                    .votes
                    .get(&(voting.voting_id, address.unwrap_or_revert()));
                if vote.choice {
                    let to_transfer = total_stake * vote.stake / voting.stake_in_favor;
                    self.transfer_reputation(
                        self_address(),
                        address.unwrap_or_revert(),
                        to_transfer,
                    );
                    transferred += to_transfer;
                }
            }
        } else {
            for address in voters {
                let vote = self
                    .votes
                    .get(&(voting.voting_id, address.unwrap_or_revert()));
                if !vote.choice {
                    let to_transfer = total_stake * vote.stake / voting.stake_against;
                    self.transfer_reputation(
                        self_address(),
                        address.unwrap_or_revert(),
                        to_transfer,
                    );
                    transferred += to_transfer;
                }
            }
        }

        if transferred < total_stake {
            self.dust_amount
                .set(self.get_dust_amount() + total_stake - transferred);
        }
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

        // Stake the reputation
        self.transfer_reputation(caller(), self_address(), stake);

        let mut vote = self.votes.get(&(voting_id, caller()));

        match vote.voter {
            Some(_) => {
                // If already voted, update an existing vote
                vote.choice = choice;
                vote.stake += stake;
            }
            None => {
                // Otherwise, create a new vote
                vote = Vote {
                    voter: Some(caller()),
                    choice,
                    voting_id,
                    stake,
                };
                let mut voters = self.voters.get(&voting_id);
                // Add a voter to the list
                voters.push(Some(caller()));
                self.voters.set(&voting_id, voters);
            }
        }

        // Add a vote to the list
        self.votes.set(&(voting_id, caller()), vote);

        // update voting
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
