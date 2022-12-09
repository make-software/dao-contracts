use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
    BlockTime,
    ContractCall,
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
        }
    }

    pub fn double_time_between_votings(&mut self) {
        self.voting_configuration.double_time_between_votings = true
    }

    pub fn reputation_conversion_rate(&self) -> U512 {
        self.dao_configuration.reputation_conversion_rate
    }

    pub fn fiat_conversion_rate_address(&self) -> Address {
        self.dao_configuration.fiat_conversion_rate_address
    }

    pub fn governance_informal_quorum_ratio(&self) -> U512 {
        self.dao_configuration.governance_informal_quorum_ratio
    }

    pub fn governance_formal_quorum_ratio(&self) -> U512 {
        self.dao_configuration.governance_formal_quorum_ratio
    }

    pub fn governance_informal_voting_time(&self) -> BlockTime {
        self.dao_configuration.governance_informal_voting_time
    }

    pub fn governance_formal_voting_time(&self) -> BlockTime {
        self.dao_configuration.governance_formal_voting_time
    }

    pub fn informal_quorum_ratio(&self) -> U512 {
        self.dao_configuration.informal_quorum_ratio
    }

    pub fn formal_quorum_ratio(&self) -> U512 {
        self.dao_configuration.formal_quorum_ratio
    }

    pub fn governance_formal_voting_quorum(&self) -> u32 {
        // TODO: make the math not fail and reusable
        self.governance_formal_quorum_ratio()
            .checked_mul(self.total_onboarded())
            .unwrap()
            .checked_div(U512::from(1000))
            .unwrap()
            .as_u32()
    }

    pub fn governance_informal_voting_quorum(&self) -> u32 {
        // TODO: make the math not fail and reusable
        self.governance_informal_quorum_ratio()
            .checked_mul(self.total_onboarded())
            .unwrap()
            .checked_div(U512::from(1000))
            .unwrap()
            .as_u32()
    }

    pub fn formal_voting_quorum(&self) -> u32 {
        // TODO: make the math not fail and reusable
        self.formal_quorum_ratio()
            .checked_mul(self.total_onboarded())
            .unwrap()
            .checked_div(U512::from(1000))
            .unwrap()
            .as_u32()
    }

    pub fn informal_voting_quorum(&self) -> u32 {
        // TODO: make the math not fail and reusable
        self.informal_quorum_ratio()
            .checked_mul(self.total_onboarded())
            .unwrap()
            .checked_div(U512::from(1000))
            .unwrap()
            .as_u32()
    }

    pub fn informal_voting_time(&self) -> BlockTime {
        self.dao_configuration.informal_voting_time
    }

    pub fn formal_voting_time(&self) -> BlockTime {
        self.dao_configuration.formal_voting_time
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

    pub fn governance_wallet_address(&self) -> Address {
        self.dao_configuration.governance_wallet_address
    }

    pub fn default_reputation_slash(&self) -> U512 {
        self.dao_configuration.default_reputation_slash
    }

    pub fn voting_clearness_delta(&self) -> U512 {
        self.dao_configuration.voting_clearness_delta
    }

    pub fn voting_start_after_job_submition(&self) -> BlockTime {
        self.dao_configuration
            .voting_start_after_job_worker_submission
    }

    pub fn governance_payment_ratio(&self) -> U512 {
        self.dao_configuration.governance_payment_ratio
    }

    pub fn post_job_dos_fee(&self) -> U512 {
        self.dao_configuration.post_job_dos_fee
    }

    pub fn internal_auction_time(&self) -> BlockTime {
        self.dao_configuration.internal_auction_time
    }

    pub fn public_auction_time(&self) -> BlockTime {
        self.dao_configuration.public_auction_time
    }

    pub fn default_policing_rate(&self) -> U512 {
        self.dao_configuration.default_policing_rate
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
}
