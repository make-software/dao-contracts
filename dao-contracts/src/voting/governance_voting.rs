//! Governance Voting module.
pub mod consts;
pub mod events;
pub mod voting;

use casper_dao_utils::bytes::BytesConversion;

use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    casper_env::{call_contract, emit, get_block_time, revert, self_address},
    Address, Error, Mapping, Variable,
};
use casper_types::{runtime_args, RuntimeArgs, U256, U512};

use crate::{
    ReputationContractCaller, ReputationContractInterface, VariableRepositoryContractCaller,
};

use self::{
    events::{
        FormalVotingEnded, InformalVotingEnded, VoteCast, VotingContractCreated, VotingCreated,
    },
    voting::{Voting, VotingConfiguration, VotingResult, VotingType},
};

use casper_dao_utils::consts as dao_consts;

use super::{vote::VotingId, Vote};

/// The Governance Voting module.

#[derive(Instance)]
pub struct GovernanceVoting {
    pub variable_repo: Variable<Option<Address>>,
    pub reputation_token: Variable<Option<Address>>,
    pub votings: Mapping<U256, Voting>,
    pub votes: Mapping<(U256, Address), Vote>,
    pub voters: Mapping<U256, Vec<Address>>,
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

    pub fn create_voting(
        &mut self,
        creator: Address,
        stake: U256,
        contract_to_call: Address,
        entry_point: String,
        runtime_args: RuntimeArgs,
    ) {
        let repo_caller =
            VariableRepositoryContractCaller::at_address(self.get_variable_repo_address());

        let reputation_caller =
            ReputationContractCaller::at_address(self.get_reputation_token_address());

        let informal_voting_time = repo_caller.get_variable(dao_consts::INFORMAL_VOTING_TIME);
        let formal_voting_time = repo_caller.get_variable(dao_consts::FORMAL_VOTING_TIME);
        let minimum_governance_reputation =
            repo_caller.get_variable(dao_consts::MINIMUM_GOVERNANCE_REPUTATION);
        let voting_id = self.votings_count.get();

        let informal_voting_quorum = VariableRepositoryContractCaller::mul_by_ratio(
            reputation_caller.total_onboarded(),
            repo_caller.get_variable(dao_consts::INFORMAL_VOTING_QUORUM),
        );
        let formal_voting_quorum = VariableRepositoryContractCaller::mul_by_ratio(
            reputation_caller.total_onboarded(),
            repo_caller.get_variable(dao_consts::FORMAL_VOTING_QUORUM),
        );

        let voting_configuration = VotingConfiguration {
            formal_voting_quorum,
            formal_voting_time,
            informal_voting_quorum,
            informal_voting_time,
            minimum_governance_reputation,
            contract_to_call: Some(contract_to_call),
            entry_point,
            runtime_args,
        };

        let voting = Voting::new(voting_id, get_block_time(), voting_configuration);

        // Add Voting
        self.votings_count.set(voting_id + 1);
        self.votings.set(&voting_id, voting);

        emit(VotingCreated {
            creator,
            voting_id,
            stake,
        });

        // Cast first vote in favor
        self.vote(creator, voting_id, true, stake);
    }

    pub fn finish_voting(&mut self, voting_id: VotingId) {
        let voting = self.votings.get(&voting_id);

        if voting.completed() {
            revert(Error::FinishingCompletedVotingNotAllowed)
        }

        match voting.get_voting_type() {
            VotingType::Informal => self.finish_informal_voting(voting),
            VotingType::Formal => self.finish_formal_voting(voting),
        }
    }

    fn finish_informal_voting(&mut self, mut voting: Voting) {
        if !voting.is_in_time(get_block_time()) {
            revert(Error::InformalVotingTimeNotReached)
        }

        let voters = self.voters.get(&voting.voting_id());

        let result = match voting.get_result(voters.len()) {
            VotingResult::InFavor => {
                self.return_reputation(voting.voting_id());

                let formal_voting_id = self.votings_count.get();
                let creator_address = *voters.first().unwrap_or_revert();
                let creator_stake = self.votes.get(&(voting.voting_id(), creator_address)).stake;

                // Formal voting is created and first vote cast
                self.votings.set(
                    &formal_voting_id,
                    voting.convert_to_formal(formal_voting_id, get_block_time()),
                );

                emit(VotingCreated {
                    creator: creator_address,
                    voting_id: formal_voting_id,
                    stake: creator_stake,
                });

                self.vote(creator_address, formal_voting_id, true, creator_stake);

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

        emit(InformalVotingEnded {
            result: result.into(),
            votes_count: voters.len().into(),
            stake_in_favor: voting.stake_in_favor(),
            stake_against: voting.stake_against(),
            informal_voting_id: voting.informal_voting_id(),
            formal_voting_id: voting.formal_voting_id(),
        });

        self.votings.set(&voting.voting_id(), voting);
    }

    fn finish_formal_voting(&mut self, mut voting: Voting) {
        if !voting.is_in_time(get_block_time()) {
            revert(Error::FormalVotingTimeNotReached)
        }

        let voters = self.voters.get(&voting.voting_id());

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
                self.burn_creators_and_return_others_reputation(voting.voting_id());
                consts::FORMAL_VOTING_QUORUM_NOT_REACHED
            }
        };

        emit(FormalVotingEnded {
            result: result.into(),
            votes_count: voters.len().into(),
            stake_in_favor: voting.stake_in_favor(),
            stake_against: voting.stake_against(),
            informal_voting_id: voting.informal_voting_id(),
            formal_voting_id: voting.voting_id(),
        });

        voting.complete(None);
        self.votings.set(&voting.voting_id(), voting);
    }

    pub fn vote(&mut self, voter: Address, voting_id: U256, choice: bool, stake: U256) {
        let mut voting = self.votings.get(&voting_id);

        // We cannot vote on a completed voting
        if voting.completed() {
            revert(Error::VoteOnCompletedVotingNotAllowed)
        }

        let mut vote = self.votes.get(&(voting_id, voter));
        match vote.voter {
            Some(_) => {
                // Cannot vote twice on the same voting
                revert(Error::CannotVoteTwice)
            }
            None => {
                // Stake the reputation
                self.transfer_reputation(voter, self_address(), stake);

                // Create a new vote
                vote = Vote {
                    voter: Some(voter),
                    choice,
                    voting_id,
                    stake,
                };
                let mut voters = self.voters.get(&voting_id);
                // Add a voter to the list
                voters.push(voter);
                self.voters.set(&voting_id, voters);
            }
        }

        // Update the votes list
        self.votes.set(&(voting_id, voter), vote);

        // update voting
        voting.stake(stake, choice);
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

    pub fn perform_action(&self, voting: &Voting) {
        call_contract(
            voting.contract_to_call().unwrap_or_revert(),
            voting.entry_point(),
            voting.runtime_args().clone(),
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
            let vote = self.votes.get(&(voting_id, *address));
            if i == 0 {
                // the creator
                self.burn_reputation(self_address(), vote.stake);
            } else {
                // the voters - transfer from contract to them
                self.transfer_reputation(self_address(), *address, vote.stake);
            }
        }
    }

    fn return_reputation(&mut self, voting_id: VotingId) {
        let voters = self.voters.get(&voting_id);
        for address in voters.iter() {
            let vote = self.votes.get(&(voting_id, *address));
            self.transfer_reputation(self_address(), *address, vote.stake);
        }
    }

    fn redistribute_reputation(&mut self, voting: &Voting) {
        // TODO: remove bytes conversion after support for U256<>U512 conversion will be added to Casper
        let voters = self.voters.get(&voting.voting_id());
        let total_stake = U512::convert_from_bytes(voting.total_stake().convert_to_bytes());
        let mut transferred = U512::zero();
        let result = voting.is_in_favor();

        for address in voters {
            let vote = self.votes.get(&(voting.voting_id(), address));
            if vote.choice == result {
                let to_transfer = total_stake
                    * U512::convert_from_bytes(vote.stake.convert_to_bytes())
                    / U512::convert_from_bytes(voting.get_winning_stake().convert_to_bytes());

                if to_transfer > U512::convert_from_bytes(U256::MAX.convert_to_bytes()) {
                    revert(Error::ArithmeticOverflow)
                }

                transferred += to_transfer;

                let to_transfer = U256::convert_from_bytes(to_transfer.convert_to_bytes());

                self.transfer_reputation(self_address(), address, to_transfer);
            }
        }

        // mark leftovers
        let dust = total_stake - transferred;

        if dust > U512::zero() {
            if dust > U512::convert_from_bytes(U256::MAX.convert_to_bytes()) {
                revert(Error::ArithmeticOverflow)
            }

            self.dust_amount
                .set(self.get_dust_amount() + U256::convert_from_bytes(dust.convert_to_bytes()));
        }
    }
}
