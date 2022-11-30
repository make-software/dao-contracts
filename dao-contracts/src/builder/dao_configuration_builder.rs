use std::collections::BTreeMap;

use casper_dao_utils::{casper_env::revert, consts, Address, ContractCall, Error};
use casper_types::bytesrepr::{Bytes, FromBytes};

use crate::{
    DaoConfiguration,
    VaNftContractCaller,
    VaNftContractInterface,
    VariableRepositoryContractCaller,
    VariableRepositoryContractInterface,
};

pub struct DaoConfigurationBuilder {
    configuration: DaoConfiguration,
}

impl DaoConfigurationBuilder {
    pub fn new(
        variable_repo_address: Address,
        va_token_address: Address,
    ) -> DaoConfigurationBuilder {
        let total_onboarded = VaNftContractCaller::at(va_token_address).total_supply();
        let variables = VariableRepositoryContractCaller::at(variable_repo_address).all_variables();
        use consts::*;
        DaoConfigurationBuilder {
            configuration: DaoConfiguration {
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
                governance_informal_quorum_ratio: Self::get_variable(
                    GOVERNANCE_INFORMAL_QUORUM_RATIO,
                    &variables,
                ),
                governance_formal_quorum_ratio: Self::get_variable(
                    GOVERNANCE_FORMAL_QUORUM_RATIO,
                    &variables,
                ),
                governance_informal_voting_time: Self::get_variable(
                    GOVERNANCE_INFORMAL_VOTING_TIME,
                    &variables,
                ),
                governance_formal_voting_time: Self::get_variable(
                    GOVERNANCE_FORMAL_VOTING_TIME,
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
                governance_wallet_address: Self::get_variable(
                    GOVERNANCE_WALLET_ADDRESS,
                    &variables,
                ),
                default_reputation_slash: Self::get_variable(DEFAULT_REPUTATION_SLASH, &variables),
                voting_clearness_delta: Self::get_variable(VOTING_CLEARNESS_DELTA, &variables),
                voting_start_after_job_worker_submission: Self::get_variable(
                    VOTING_START_AFTER_JOB_WORKER_SUBMISSION,
                    &variables,
                ),
                informal_quorum_ratio: Self::get_variable(INFORMAL_QUORUM_RATIO, &variables),
                formal_quorum_ratio: Self::get_variable(FORMAL_QUORUM_RATIO, &variables),
                governance_payment_ratio: Self::get_variable(GOVERNANCE_PAYMENT_RATIO, &variables),
                total_onboarded,
                contract_call: None,
                only_va_can_create: true,
                unbounded_tokens_for_creator: false,
                onboard_creator: false,
            },
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

    pub fn contract_call(mut self, contract_call: ContractCall) -> DaoConfigurationBuilder {
        self.configuration.contract_call = Some(contract_call);
        self
    }

    pub fn only_va_can_create(mut self, only_va_can_create: bool) -> DaoConfigurationBuilder {
        self.configuration.only_va_can_create = only_va_can_create;
        self
    }

    pub fn unbounded_tokens_for_creator(
        mut self,
        unbounded_tokens_for_creator: bool,
    ) -> DaoConfigurationBuilder {
        self.configuration.unbounded_tokens_for_creator = unbounded_tokens_for_creator;
        self
    }

    pub fn onboard(mut self, onboard: bool) -> DaoConfigurationBuilder {
        self.configuration.onboard_creator = onboard;
        self
    }

    pub fn build(self) -> DaoConfiguration {
        self.configuration
    }
}
