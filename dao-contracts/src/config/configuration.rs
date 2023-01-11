use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    math,
    Address,
    BlockTime,
    ContractCall,
    Error,
};
use casper_types::U512;

use crate::config::{
    dao_configuration::DaoConfiguration,
    voting_configuration::VotingConfiguration,
};

#[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
pub struct Configuration {
    pub dao_configuration: DaoConfiguration,
    pub voting_configuration: VotingConfiguration,
    pub total_onboarded: U512,
    pub fiat_rate: Option<U512>,
}

impl Configuration {
    pub fn new(
        dao_configuration: DaoConfiguration,
        voting_configuration: VotingConfiguration,
        total_onboarded: U512,
    ) -> Configuration {
        Configuration {
            dao_configuration,
            voting_configuration,
            total_onboarded,
            fiat_rate: None,
        }
    }

    pub fn bound_ballot_for_successful_voting(&mut self, address: Address) {
        self.voting_configuration.bound_ballot_for_successful_voting = true;
        self.voting_configuration.bound_ballot_address = Some(address);
    }

    pub fn double_time_between_votings(&mut self) {
        self.voting_configuration.double_time_between_votings = true
    }

    pub fn should_double_time_between_votings(&self) -> bool {
        self.voting_configuration.double_time_between_votings
    }

    pub fn fiat_conversion_rate_address(&self) -> Address {
        self.dao_configuration.fiat_conversion_rate_address
    }

    pub fn formal_voting_quorum(&self) -> u32 {
        let ratio = match self.voting_configuration.is_bid_escrow {
            true => self.dao_configuration.bid_escrow_formal_quorum_ratio,
            false => self.dao_configuration.formal_quorum_ratio,
        };

        math::per_mil_of_as_u32(ratio, self.total_onboarded()).unwrap_or_revert()
    }

    pub fn informal_voting_quorum(&self) -> u32 {
        let ratio = match self.voting_configuration.is_bid_escrow {
            true => self.dao_configuration.bid_escrow_informal_quorum_ratio,
            false => self.dao_configuration.informal_quorum_ratio,
        };

        math::per_mil_of_as_u32(ratio, self.total_onboarded()).unwrap_or_revert()
    }

    pub fn informal_voting_time(&self) -> BlockTime {
        match self.voting_configuration.is_bid_escrow {
            true => self.dao_configuration.bid_escrow_informal_voting_time,
            false => self.dao_configuration.informal_voting_time,
        }
    }

    pub fn formal_voting_time(&self) -> BlockTime {
        match self.voting_configuration.is_bid_escrow {
            true => self.dao_configuration.bid_escrow_formal_voting_time,
            false => self.dao_configuration.formal_voting_time,
        }
    }

    pub fn informal_stake_reputation(&self) -> bool {
        self.dao_configuration.informal_stake_reputation
    }

    pub fn time_between_informal_and_formal_voting(&self) -> BlockTime {
        if self.voting_configuration.double_time_between_votings {
            self.dao_configuration
                .time_between_informal_and_formal_voting
                * 2
        } else {
            self.dao_configuration
                .time_between_informal_and_formal_voting
        }
    }

    pub fn bid_escrow_wallet_address(&self) -> Address {
        self.dao_configuration.bid_escrow_wallet_address
    }

    pub fn default_reputation_slash(&self) -> U512 {
        self.dao_configuration.default_reputation_slash
    }

    pub fn voting_clearness_delta(&self) -> U512 {
        self.dao_configuration.voting_clearness_delta
    }

    pub fn voting_delay(&self) -> BlockTime {
        if self.voting_configuration.is_bid_escrow {
            self.dao_configuration
                .voting_start_after_job_worker_submission
        } else {
            0
        }
    }

    pub fn is_post_job_dos_fee_too_low(&self, fiat_value: U512) -> bool {
        math::to_per_mils(self.dao_configuration.post_job_dos_fee) > fiat_value
    }

    pub fn internal_auction_time(&self) -> BlockTime {
        self.dao_configuration.internal_auction_time
    }

    pub fn public_auction_time(&self) -> BlockTime {
        self.dao_configuration.public_auction_time
    }

    pub fn va_bid_acceptance_timeout(&self) -> BlockTime {
        self.dao_configuration.va_bid_acceptance_timeout
    }

    pub fn va_can_bid_on_public_auction(&self) -> bool {
        self.dao_configuration.va_can_bid_on_public_auction
    }

    pub fn distribute_payment_to_non_voters(&self) -> bool {
        self.dao_configuration.distribute_payment_to_non_voters
    }

    pub fn total_onboarded(&self) -> U512 {
        self.total_onboarded
    }

    pub fn contract_calls(&self) -> &Vec<ContractCall> {
        &self.voting_configuration.contract_calls
    }

    pub fn only_va_can_create(&self) -> bool {
        self.voting_configuration.only_va_can_create
    }

    pub fn is_bid_escrow(&self) -> bool {
        self.voting_configuration.is_bid_escrow
    }

    pub fn voting_ids_address(&self) -> Address {
        self.dao_configuration.voting_ids_address
    }

    pub fn should_cast_first_vote(&self) -> bool {
        !self.is_bid_escrow()
    }

    pub fn apply_default_policing_rate_to(&self, amount: U512) -> U512 {
        math::per_mil_of(amount, self.dao_configuration.default_policing_rate).unwrap_or_revert()
    }

    pub fn apply_bid_escrow_payment_ratio_to(&self, amount: U512) -> U512 {
        math::per_mil_of(amount, self.dao_configuration.bid_escrow_payment_ratio).unwrap_or_revert()
    }

    pub fn apply_reputation_conversion_rate_to(&self, amount: U512) -> U512 {
        math::per_mil_of(amount, self.dao_configuration.reputation_conversion_rate)
            .unwrap_or_revert()
    }

    pub fn apply_default_reputation_slash_to(&self, amount: U512) -> U512 {
        math::per_mil_of(amount, self.dao_configuration.default_reputation_slash).unwrap_or_revert()
    }

    pub fn fiat_rate(&self) -> Option<U512> {
        self.fiat_rate
    }

    pub fn convert_to_fiat(&self, cspr_amount: U512) -> Result<U512, Error> {
        if let Some(fiat_rate) = self.fiat_rate {
            if let Some(fiat_amount) = cspr_amount.checked_div(fiat_rate) {
                Ok(fiat_amount)
            } else {
                Err(Error::ArithmeticOverflow)
            }
        } else {
            Err(Error::FiatRateNotSet)
        }
    }
}
