use std::collections::BTreeMap;

use casper_dao_utils::{
    casper_env::{call_contract, revert},
    consts,
    ContractCall,
    Error,
};
use casper_types::{
    bytesrepr::{Bytes, FromBytes},
    RuntimeArgs,
    U512,
};

use crate::{
    config::{dao_configuration::DaoConfiguration, voting_configuration::VotingConfiguration},
    refs::ContractRefs,
    Configuration,
    VaNftContractInterface,
    VariableRepositoryContractInterface,
};

pub struct ConfigurationBuilder {
    configuration: Configuration,
}

impl ConfigurationBuilder {
    pub fn new<T: ContractRefs>(refs: &T) -> Self {
        let total_onboarded = refs.va_token().total_supply();
        let variables = refs.variable_repository().all_variables();
        use consts::*;
        ConfigurationBuilder {
            configuration: Configuration::new(
                DaoConfiguration {
                    post_job_dos_fee: Self::get_variable(POST_JOB_DOS_FEE, &variables),
                    internal_auction_time: Self::get_variable(INTERNAL_AUCTION_TIME, &variables),
                    public_auction_time: Self::get_variable(PUBLIC_AUCTION_TIME, &variables),
                    default_policing_rate: Self::get_variable(DEFAULT_POLICING_RATE, &variables),
                    reputation_conversion_rate: Self::get_variable(
                        REPUTATION_CONVERSION_RATE,
                        &variables,
                    ),
                    fiat_conversion_rate_address: Self::get_variable(
                        FIAT_CONVERSION_RATE_ADDRESS,
                        &variables,
                    ),
                    forum_kyc_required: Self::get_variable(FORUM_KYC_REQUIRED, &variables),
                    bid_escrow_informal_quorum_ratio: Self::get_variable(
                        BID_ESCROW_INFORMAL_QUORUM_RATIO,
                        &variables,
                    ),
                    bid_escrow_formal_quorum_ratio: Self::get_variable(
                        BID_ESCROW_FORMAL_QUORUM_RATIO,
                        &variables,
                    ),
                    bid_escrow_informal_voting_time: Self::get_variable(
                        BID_ESCROW_INFORMAL_VOTING_TIME,
                        &variables,
                    ),
                    bid_escrow_formal_voting_time: Self::get_variable(
                        BID_ESCROW_FORMAL_VOTING_TIME,
                        &variables,
                    ),
                    informal_voting_time: Self::get_variable(INFORMAL_VOTING_TIME, &variables),
                    formal_voting_time: Self::get_variable(FORMAL_VOTING_TIME, &variables),
                    informal_stake_reputation: Self::get_variable(
                        INFORMAL_STAKE_REPUTATION,
                        &variables,
                    ),
                    time_between_informal_and_formal_voting: Self::get_variable(
                        TIME_BETWEEN_INFORMAL_AND_FORMAL_VOTING,
                        &variables,
                    ),
                    va_bid_acceptance_timeout: Self::get_variable(
                        VA_BID_ACCEPTANCE_TIMEOUT,
                        &variables,
                    ),
                    va_can_bid_on_public_auction: Self::get_variable(
                        VA_CAN_BID_ON_PUBLIC_AUCTION,
                        &variables,
                    ),
                    distribute_payment_to_non_voters: Self::get_variable(
                        DISTRIBUTE_PAYMENT_TO_NON_VOTERS,
                        &variables,
                    ),
                    bid_escrow_wallet_address: Self::get_variable(
                        BID_ESCROW_WALLET_ADDRESS,
                        &variables,
                    ),
                    default_reputation_slash: Self::get_variable(
                        DEFAULT_REPUTATION_SLASH,
                        &variables,
                    ),
                    voting_clearness_delta: Self::get_variable(VOTING_CLEARNESS_DELTA, &variables),
                    voting_start_after_job_worker_submission: Self::get_variable(
                        VOTING_START_AFTER_JOB_WORKER_SUBMISSION,
                        &variables,
                    ),
                    informal_quorum_ratio: Self::get_variable(INFORMAL_QUORUM_RATIO, &variables),
                    formal_quorum_ratio: Self::get_variable(FORMAL_QUORUM_RATIO, &variables),
                    bid_escrow_payment_ratio: Self::get_variable(
                        BID_ESCROW_PAYMENT_RATIO,
                        &variables,
                    ),
                    voting_ids_address: Self::get_variable(VOTING_IDS_ADDRESS, &variables),
                },
                VotingConfiguration {
                    is_bid_escrow: false,
                    contract_calls: Vec::new(),
                    only_va_can_create: true,
                    double_time_between_votings: false,
                },
                total_onboarded,
            ),
        }
    }

    pub fn get_variable<T: FromBytes>(key: &str, variables: &BTreeMap<String, Bytes>) -> T {
        let variable = variables.get(key);
        let bytes = match variable {
            None => revert(Error::ValueNotAvailable),
            Some(bytes) => bytes,
        };

        let (result, bytes) = <T>::from_bytes(bytes).unwrap_or_else(|_| {
            revert(Error::ValueNotAvailable);
        });
        if !bytes.is_empty() {
            revert(Error::ValueNotAvailable)
        }

        result
    }

    pub fn contract_call(self, contract_call: ContractCall) -> Self {
        self.contract_calls(vec![contract_call])
    }

    pub fn contract_calls(mut self, contract_calls: Vec<ContractCall>) -> Self {
        self.configuration.voting_configuration.contract_calls = contract_calls;
        self
    }

    pub fn only_va_can_create(mut self, only_va_can_create: bool) -> Self {
        self.configuration.voting_configuration.only_va_can_create = only_va_can_create;
        self
    }

    pub fn is_bid_escrow(mut self, is_bid_escrow: bool) -> ConfigurationBuilder {
        let rate: U512 = call_contract(
            self.configuration.fiat_conversion_rate_address(),
            "get_rate",
            RuntimeArgs::new(),
        );
        self.configuration.fiat_rate = Some(rate);
        self.configuration.voting_configuration.is_bid_escrow = is_bid_escrow;
        self
    }

    pub fn build(self) -> Configuration {
        self.configuration
    }
}
