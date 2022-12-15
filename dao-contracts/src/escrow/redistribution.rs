use casper_dao_utils::{Error, casper_dao_macros::Instance, Address, Variable, casper_contract::unwrap_or_revert::UnwrapOrRevert};
use casper_types::U512;

use crate::{Configuration, ReputationContractInterface, voting::{voting_state_machine::VotingType, VotingId, VotingEngine}};

#[derive(Instance)]
pub struct Redistribution {
    #[scoped = "contract"]
    voting: VotingEngine,
}

pub struct RedistributionConfig {
    amount: U512,
    governance_share: bool,
    cspr: bool,
}

impl Redistribution {

    pub fn redistribute_to_governance<C>(&mut self, configuration: &C, cspr_amount: U512) -> U512
    where 
        C: WithConfiguration, 
    {
        let configuration = configuration.get_configuration(&self.voting);
        let governance_wallet: Address = configuration.bid_escrow_wallet_address();
        let governance_wallet_payment = configuration.apply_bid_escrow_payment_ratio_to(cspr_amount);
        casper_dao_utils::transfer::withdraw_cspr(governance_wallet, governance_wallet_payment);

        cspr_amount - governance_wallet_payment
    }

    pub fn redistribute_cspr_to_all_vas(&mut self, amount: U512) {
        let (total_supply, balances) = self.voting.reputation_token().all_balances();
        for (address, balance) in balances.balances {
            let amount = amount * balance / total_supply;
            casper_dao_utils::transfer::withdraw_cspr(address, amount);
        }
    }

    pub fn redistribute_cspr_to_voters<V: WithVotingId>(&mut self, v: &V, cspr_amount: U512) {
        let voting_id = v.get_voting_id();
        let all_voters = self.voting.all_voters(voting_id, VotingType::Formal);
        let reputation = self.voting.reputation_token();
        
        let (partial_supply, balances) = reputation.partial_balances(all_voters);
        for (address, balance) in balances.balances {
            let amount = cspr_amount * balance / partial_supply;
            casper_dao_utils::transfer::withdraw_cspr(address, amount);
        }
    }

    pub fn burn_reputation<C>(&mut self, owner: Address, configuration: &C, cspr_amount: U512) 
    where 
        C: WithConfiguration, 
    {
        let config = configuration.get_configuration(&self.voting);

        let stake = config.apply_reputation_conversion_rate_to(cspr_amount);
        self.voting.reputation_token().burn(owner, stake);
    }

    fn mint_reputation_for_recipient<T>(&mut self, recipient: Address, t: &T, cspr_amount: U512) -> U512 
    where 
        T: WithConfiguration, 
    {
        let configuration = t.get_configuration(&self.voting);

        let reputation_to_mint = configuration.apply_reputation_conversion_rate_to(cspr_amount);
        let reputation_to_redistribute =
            configuration.apply_default_policing_rate_to(reputation_to_mint);

        // Worker
        self.voting.reputation_token().mint(
            recipient,
            reputation_to_mint - reputation_to_redistribute,
        );

        reputation_to_redistribute
    }

    pub fn mint_passive_reputation_for_recipient<T>(&mut self, recipient: Address, t: &T, cspr_amount: U512) -> U512 
    where 
        T: WithConfiguration, 
    {
        let configuration = t.get_configuration(&self.voting);
        let reputation_to_mint = configuration.apply_reputation_conversion_rate_to(cspr_amount);
        let reputation_to_redistribute =
            configuration.apply_default_policing_rate_to(reputation_to_mint);

        // Worker
        self.voting.reputation_token().mint_passive(
            recipient,
            reputation_to_mint - reputation_to_redistribute,
        );

        reputation_to_redistribute
    }

    pub fn mint_reputation_for_voters<V: WithVotingId>(&mut self, v: &V, rep_amount: U512) {
        let voting = self
            .voting
            .get_voting(v.get_voting_id())
            .unwrap_or_revert();

        for i in 0..self
            .voting
            .voters()
            .len((voting.voting_id(), VotingType::Formal))
        {
            let ballot = self
                .voting
                .get_ballot_at(voting.voting_id(), VotingType::Formal, i);
            if ballot.unbounded {
                continue;
            }
            let to_transfer = ballot.stake * rep_amount / voting.total_bounded_stake();
            self.voting.reputation_token().mint(ballot.voter, to_transfer);
        }
    }
}


pub trait WithVotingId {
    fn get_voting_id(&self) -> VotingId;
}

pub trait WithStake {
    fn get_stake(&self) -> U512;
}

pub trait WithCsprStake {
    fn get_cspr_stake(&self) -> U512;
}

pub trait WithConfiguration {
    fn get_configuration(&self, engine: &VotingEngine) -> Configuration;
}

pub trait WithRecipient {
    fn get_recipient(&self) -> Address;
}