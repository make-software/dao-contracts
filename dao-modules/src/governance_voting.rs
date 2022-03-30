//! Governance Voting module.

pub mod events;
pub mod vote;
pub mod voting;

use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    casper_env::{call_contract, caller, emit, get_block_time, revert, self_address},
    Address, Error, Mapping, Variable,
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

#[derive(Instance)]
pub struct GovernanceVoting {
    pub variable_repo: Variable<Option<Address>>,
    pub reputation_token: Variable<Option<Address>>,
    pub votings: Mapping<U256, Voting>,
    pub votes: Mapping<(U256, Address), Vote>,
    pub voters: Mapping<U256, Vec<Option<Address>>>,
    pub votings_count: Variable<U256>,
    pub dust_amount: Variable<U256>,
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

        if voting.completed {
            revert(Error::FinishingCompletedVotingNotAllowed)
        }

        match voting.get_voting_type() {
            voting::VotingType::Informal => self.finish_informal_voting(voting),
            voting::VotingType::Formal => self.finish_formal_voting(voting),
            voting::VotingType::Unknown => revert(Error::MalformedVoting),
        }
    }

    fn finish_informal_voting(&mut self, mut voting: Voting) {
        if !voting.is_in_time(get_block_time()) {
            revert(Error::InformalVotingTimeNotReached)
        }

        let voters = self.voters.get(&voting.voting_id);

        let result: Option<&str> = match voting.get_result(voters.len()) {
            voting::VotingResult::InFavor => {
                self.return_reputation(voting.voting_id);

                // Formal voting is created with an initial stake
                let formal_voting =
                    voting.convert_to_formal(self.votings_count.get(), get_block_time());
                let creator_address = voters.first().unwrap().unwrap();
                self.create_voting(
                    &formal_voting,
                    self.votes
                        .get(&(formal_voting.informal_voting_id, creator_address))
                        .stake,
                );

                // Informal voting is completed and saved with reference to a formal voting
                voting.complete();
                voting.formal_voting_id = formal_voting.formal_voting_id;
                self.votings.set(&voting.voting_id, voting.clone());

                Some("converted_to_formal")
            }
            voting::VotingResult::Against => {
                self.burn_creators_and_return_others_reputation(voting.voting_id);
                voting.complete();
                self.votings.set(&voting.voting_id, voting.clone());

                Some("rejected")
            }
            voting::VotingResult::QuorumNotReached => {
                self.burn_creators_and_return_others_reputation(voting.voting_id);
                voting.complete();
                self.votings.set(&voting.voting_id, voting.clone());

                Some("quorum_not_reached")
            }
            voting::VotingResult::Unknown => revert(Error::MalformedVoting),
        };

        emit(InformalVotingEnded {
            result: result.unwrap_or_revert().into(),
            votes_count: voters.len().into(),
            stake_in_favor: voting.stake_in_favor,
            stake_against: voting.stake_against,
            informal_voting_id: voting.informal_voting_id,
            formal_voting_id: voting.formal_voting_id,
        });
    }

    fn finish_formal_voting(&mut self, mut voting: Voting) {
        if !voting.is_in_time(get_block_time()) {
            revert(Error::FormalVotingTimeNotReached)
        }

        let voters = self.voters.get(&voting.voting_id);

        let result: Option<&str> = match voting.get_result(voters.len()) {
            voting::VotingResult::InFavor => {
                self.redistribute_reputation(&voting);
                self.perform_action(&voting);
                Some("passed")
            }
            voting::VotingResult::Against => {
                self.redistribute_reputation(&voting);
                Some("rejected")
            }
            voting::VotingResult::QuorumNotReached => {
                self.burn_creators_and_return_others_reputation(voting.voting_id);
                Some("quorum_not_reached")
            }
            voting::VotingResult::Unknown => revert(Error::MalformedVoting),
        };

        voting.complete();
        self.votings.set(&voting.voting_id, voting.clone());

        emit(FormalVotingEnded {
            result: result.unwrap_or_revert().into(),
            votes_count: voters.len().into(),
            stake_in_favor: voting.stake_in_favor,
            stake_against: voting.stake_against,
            informal_voting_id: voting.informal_voting_id,
            formal_voting_id: voting.voting_id,
        });
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

        // Update the votes list
        self.votes.set(&(voting_id, caller()), vote);

        // update voting
        match choice {
            true => voting.stake_in_favor += stake,
            false => voting.stake_against += stake,
        }
        self.votings.set(&voting_id, voting);

        emit(VoteCast {
            voter: caller(),
            voting_id,
            choice,
            stake,
        });
    }

    pub fn get_dust_amount(&self) -> U256 {
        self.dust_amount.get()
    }

    pub fn get_variable_repo_address(&self) -> Address {
        self.variable_repo.get().unwrap_or_revert()
    }

    pub fn get_reputation_token_address(&self) -> Address {
        self.reputation_token.get().unwrap_or_revert()
    }

    fn perform_action(&mut self, voting: &Voting) {
        call_contract(
            voting.contract_to_call.unwrap_or_revert(),
            &voting.entry_point,
            voting.runtime_args.clone(),
        )
    }

    fn transfer_reputation(&mut self, owner: Address, recipient: Address, amount: U256) {
        let args: RuntimeArgs = runtime_args! {
            "owner" => owner,
            "recipient" => recipient,
            "amount" => amount,
        };

        call_contract(self.get_reputation_token_address(), "transfer_from", args)
    }

    fn burn_reputation(&mut self, owner: Address, amount: U256) {
        let args: RuntimeArgs = runtime_args! {
            "owner" => owner,
            "amount" => amount,
        };

        call_contract(self.get_reputation_token_address(), "burn", args)
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
        let mut transferred: U256 = U256::zero();
        if voting.is_in_favor() {
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

        // mark leftovers
        if transferred < total_stake {
            self.dust_amount
                .set(self.get_dust_amount() + total_stake - transferred);
        }
    }
}
