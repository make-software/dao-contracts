use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_contract::contract_api::runtime::revert,
    casper_dao_macros::{casper_contract_interface, CLTyped, FromBytes, Instance, ToBytes},
    casper_env::caller,
    Address,
    ContractCall,
    Error,
    Mapping,
    Variable,
};
use casper_types::{runtime_args, RuntimeArgs, U512};
use delegate::delegate;

use crate::{
    voting::{
        types::VotingId,
        voting_state_machine::{VotingResult, VotingStateMachine, VotingType},
        Ballot,
        Choice,
        VotingEngine,
    },
    ConfigurationBuilder,
    ReputationContractInterface,
    VaNftContractInterface,
};

#[casper_contract_interface]
pub trait SlashingVoterContractInterface {
    /// see [VotingEngine](VotingEngine::init())
    fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);

    fn create_voting(&mut self, address_to_slash: Address, slash_ratio: u32, stake: U512);
    /// see [VotingEngine](VotingEngine::vote())
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
    /// see [VotingEngine](VotingEngine::finish_voting())
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
    /// see [VotingEngine](VotingEngine::get_variable_repo_address())
    fn variable_repo_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine::get_reputation_token_address())
    fn reputation_token_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine::get_voting())
    fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
    /// see [VotingEngine](VotingEngine::get_ballot())
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// see [VotingEngine](VotingEngine::get_voter())
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    fn update_bid_escrow_list(&mut self, bid_escrows: Vec<Address>);

    // Whitelisting set.
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn get_owner(&self) -> Option<Address>;
    fn is_whitelisted(&self, address: Address) -> bool;
    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
}

/// Slashing Voter contract uses [VotingEngine](VotingEngine) to vote on changes of ownership and managing whitelists of other contracts.
///
/// Slashing Voter contract needs to have permissions to perform those actions.
///
/// For details see [SlashingVoterContractInterface](SlashingVoterContractInterface)
#[derive(Instance)]
pub struct SlashingVoterContract {
    voting: VotingEngine,
    tasks: Mapping<VotingId, SlashTask>,
    bid_escrows: Variable<Vec<Address>>,
    access_control: AccessControl,
}

impl SlashingVoterContractInterface for SlashingVoterContract {
    delegate! {
        to self.voting {
            fn variable_repo_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
            fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
            fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
            fn get_voting(
                &self,
                voting_id: VotingId,
            ) -> Option<VotingStateMachine>;
            fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                address: Address,
            ) -> Option<Ballot>;
        }

        to self.access_control {
            fn change_ownership(&mut self, owner: Address);
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
            fn is_whitelisted(&self, address: Address) -> bool;
            fn get_owner(&self) -> Option<Address>;
        }
    }

    fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address) {
        self.voting.init(variable_repo, reputation_token, va_token);
        self.access_control.init(caller());
    }

    fn update_bid_escrow_list(&mut self, bid_escrows: Vec<Address>) {
        self.access_control.ensure_whitelisted();
        self.bid_escrows.set(bid_escrows);
    }

    fn create_voting(&mut self, address_to_slash: Address, slash_ratio: u32, stake: U512) {
        // TODO: contraints
        let current_reputation = self.voting.reputation_token().balance_of(address_to_slash);

        let voting_configuration = ConfigurationBuilder::new(
            self.voting.variable_repo_address(),
            self.voting.va_token_address(),
        )
        .build();

        let creator = caller();
        let voting_id = self
            .voting
            .create_voting(creator, stake, voting_configuration);

        let task = SlashTask {
            subject: address_to_slash,
            ratio: slash_ratio,
            reputation_at_voting_creation: current_reputation,
        };
        self.tasks.set(&voting_id, task);
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        // Check if the caller is not a subject for the voting.
        let task = self.tasks.get_or_revert(&voting_id);
        if caller() == task.subject {
            revert(Error::SubjectOfSlashing);
        }
        self.voting
            .vote(caller(), voting_id, voting_type, choice, stake);
    }

    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let summary = self.voting.finish_voting(voting_id, voting_type);
        if summary.is_formal() && summary.result() == VotingResult::InFavor {
            self.slash(voting_id);
        }
    }

    fn slash_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.voting.slash_voter(voter, voting_id);
    }
}

impl SlashingVoterContract {
    fn slash(&mut self, voting_id: VotingId) {
        let slash_task = self.tasks.get_or_revert(&voting_id);

        // Burn VA token.
        self.voting.va_token().burn(slash_task.subject);

        let mut reputation = self.voting.reputation_token();
        // If partial slash only burn reputation.
        if slash_task.ratio != 1000 {
            let slash_amount = slash_task.reputation_at_voting_creation * slash_task.ratio / 1000;
            reputation.burn(slash_task.subject, slash_amount);
            return;
        }

        // If full slash burn all reputation
        reputation.burn_all(slash_task.subject);

        // Load account stakes.
        let stakes = reputation.stakes_info(slash_task.subject);

        // Slash all open offers in bid escrows.
        let bid_escrows = self.bid_escrows.get().unwrap_or_default();
        for bid_escrow_address in bid_escrows {
            ContractCall {
                address: bid_escrow_address,
                entry_point: String::from("slash_all_active_job_offers"),
                runtime_args: runtime_args! {
                    "bidder" => slash_task.subject,
                },
            }
            .call();
        }

        // Slash all bids.
        for (bid_escrow_address, bid_id) in stakes.get_bids_stakes_origins() {
            ContractCall {
                address: bid_escrow_address,
                entry_point: String::from("slash_bid"),
                runtime_args: runtime_args! {
                    "bid_id" => bid_id,
                },
            }
            .call();
        }

        // Slash subject in all voter contracts.
        for (contract_address, voting_id) in stakes.get_voting_stakes_origins() {
            ContractCall {
                address: contract_address,
                entry_point: String::from("slash_voter"),
                runtime_args: runtime_args! {
                    "voter" => slash_task.subject,
                    "voting_id" => voting_id
                },
            }
            .call();
        }
    }
}

#[derive(Debug, Clone, CLTyped, ToBytes, FromBytes)]
struct SlashTask {
    pub subject: Address,
    pub ratio: u32,
    pub reputation_at_voting_creation: U512,
}
