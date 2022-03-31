//! Governance Voting module.
pub mod consts;
pub mod events;
pub mod voting;

use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    casper_env::{call_contract, emit, get_block_time, revert, self_address},
    Address, Error, Mapping, Variable,
};
use casper_types::{runtime_args, RuntimeArgs, U256};

use crate::VariableRepositoryContractCaller;

use self::{
    events::{
        FormalVotingEnded, InformalVotingEnded, VoteCast, VotingContractCreated, VotingCreated,
    },
    voting::{Voting, VotingResult, VotingType},
};

use casper_dao_utils::consts as dao_consts;

use super::{Vote, vote::VotingId};

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

        emit(VotingContractCreated {
            variable_repo,
            reputation_token,
            repo_voter: self_address(),
        });
    }

    pub fn create_voting(&mut self, creator: Address, stake: U256, contract_to_call: Address, entry_point: String, runtime_args: RuntimeArgs, informal_voting_id: Option<VotingId>) {
        let repo_caller =
            VariableRepositoryContractCaller::at(self.get_variable_repo_address());
        let informal_voting_time = repo_caller.get_variable(dao_consts::INFORMAL_VOTING_TIME);
        let informal_voting_quorum = repo_caller.get_variable(dao_consts::INFORMAL_VOTING_QUORUM);
        let formal_voting_time = repo_caller.get_variable(dao_consts::FORMAL_VOTING_TIME);
        let formal_voting_quorum = repo_caller.get_variable(dao_consts::FORMAL_VOTING_QUORUM);
        let minimum_governance_reputation =
            repo_caller.get_variable(dao_consts::MINIMUM_GOVERNANCE_REPUTATION);
        let voting_id = self.votings_count.get();
        let formal_id;
        let informal_id;

        match informal_voting_id {
            Some(informal_voting_id) => {
                formal_id = Some(voting_id);
                informal_id = informal_voting_id;
            },
            None => {
                formal_id = None;
                informal_id = voting_id;
            },
        }
        
        let voting = Voting {
            voting_id: voting_id,
            completed: false,
            stake_in_favor: U256::zero(),
            stake_against: U256::zero(),
            finish_time: get_block_time() + informal_voting_time,
            informal_voting_id: informal_id,
            formal_voting_id: formal_id,
            formal_voting_quorum,
            formal_voting_time,
            informal_voting_quorum,
            informal_voting_time,
            contract_to_call: Some(contract_to_call),
            entry_point,
            runtime_args,
            minimum_governance_reputation,
        };

        // Add Voting
        self.votings.set(&voting.voting_id, voting.clone());
        self.votings_count.set(voting.voting_id + 1);

        emit(VotingCreated {
            creator,
            voting_id: voting.voting_id,
            stake,
        });

        // Cast first vote in favor
        self.vote(creator, voting.voting_id, true, stake);
    }

    pub fn finish_voting(&mut self, voting_id: VotingId) {
        let voting = self.votings.get(&voting_id);

        if voting.completed {
            revert(Error::FinishingCompletedVotingNotAllowed)
        }

        match voting.get_voting_type() {
            VotingType::Informal => self.finish_informal_voting(voting),
            VotingType::Formal => self.finish_formal_voting(voting),
            VotingType::Unknown => revert(Error::MalformedVoting),
        }
    }

    fn finish_informal_voting(&mut self, mut voting: Voting) {
        if !voting.is_in_time(get_block_time()) {
            revert(Error::InformalVotingTimeNotReached)
        }

        let voters = self.voters.get(&voting.voting_id);

        let result = match voting.get_result(voters.len()) {
            VotingResult::InFavor => {
                self.return_reputation(voting.voting_id);

                // Formal voting is created with an initial stake
                let formal_voting =
                    voting.convert_to_formal(self.votings_count.get(), get_block_time());
                let creator_address = voters.first().unwrap().unwrap();
                self.create_voting(
                    creator_address,
                    self.votes
                        .get(&(formal_voting.informal_voting_id, creator_address))
                        .stake,
                    formal_voting.contract_to_call.unwrap_or_revert(),
                    formal_voting.entry_point,
                    formal_voting.runtime_args,
                    Some(formal_voting.informal_voting_id),
                );

                // Informal voting is completed and saved with reference to a formal voting
                voting.complete();
                voting.formal_voting_id = formal_voting.formal_voting_id;
                self.votings.set(&voting.voting_id, voting.clone());

                consts::INFORMAL_VOTING_PASSED
            }
            VotingResult::Against => {
                self.burn_creators_and_return_others_reputation(voting.voting_id);
                voting.complete();
                self.votings.set(&voting.voting_id, voting.clone());

                consts::INFORMAL_VOTING_REJECTED
            }
            VotingResult::QuorumNotReached => {
                self.burn_creators_and_return_others_reputation(voting.voting_id);
                voting.complete();
                self.votings.set(&voting.voting_id, voting.clone());

                consts::INFORMAL_VOTING_QUORUM_NOT_REACHED
            }
            VotingResult::Unknown => revert(Error::MalformedVoting),
        };

        emit(InformalVotingEnded {
            result: result.into(),
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

        let result = match voting.get_result(voters.len()) {
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
                self.burn_creators_and_return_others_reputation(voting.voting_id);
                consts::FORMAL_VOTING_QUORUM_NOT_REACHED
            }
            VotingResult::Unknown => revert(Error::MalformedVoting),
        };

        voting.complete();
        self.votings.set(&voting.voting_id, voting.clone());

        emit(FormalVotingEnded {
            result: result.into(),
            votes_count: voters.len().into(),
            stake_in_favor: voting.stake_in_favor,
            stake_against: voting.stake_against,
            informal_voting_id: voting.informal_voting_id,
            formal_voting_id: voting.voting_id,
        });
    }

    pub fn vote(&mut self, voter: Address, voting_id: U256, choice: bool, stake: U256) {
        let mut voting = self.votings.get(&voting_id);

        // We cannot vote on a completed voting
        if voting.completed {
            revert(Error::VoteOnCompletedVotingNotAllowed)
        }

        // Stake the reputation
        self.transfer_reputation(voter, self_address(), stake);

        let mut vote = self.votes.get(&(voting_id, voter));
        match vote.voter {
            Some(_) => {
                // If already voted, update an existing vote
                vote.choice = choice;
                vote.stake += stake;
            }
            None => {
                // Otherwise, create a new vote
                vote = Vote {
                    voter: Some(voter),
                    choice,
                    voting_id,
                    stake,
                };
                let mut voters = self.voters.get(&voting_id);
                // Add a voter to the list
                voters.push(Some(voter));
                self.voters.set(&voting_id, voters);
            }
        }

        // Update the votes list
        self.votes.set(&(voting_id, voter), vote);

        // update voting
        match choice {
            true => voting.stake_in_favor += stake,
            false => voting.stake_against += stake,
        }
        self.votings.set(&voting_id, voting);

        emit(VoteCast {
            voter,
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

        for address in voters {
            let vote = self
                .votes
                .get(&(voting.voting_id, address.unwrap_or_revert()));
            if vote.choice == voting.is_in_favor() {
                let to_transfer = total_stake * vote.stake / voting.get_winning_stake();
                self.transfer_reputation(self_address(), address.unwrap_or_revert(), to_transfer);
                transferred += to_transfer;
            }
        }

        // mark leftovers
        let dust = total_stake - transferred;
        if dust > U256::zero() {
            self.dust_amount.set(self.get_dust_amount() + dust);
        }
    }
}
