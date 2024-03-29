use crate::configuration::dao_configuration::DaoConfiguration;
use crate::configuration::voting_configuration::VotingConfiguration;
use crate::configuration::{get_variable, Configuration};
use crate::utils::consts;
use crate::utils::ContractCall;
use odra::call_contract;
use odra::prelude::{collections::BTreeMap, string::String, vec, vec::Vec};
use odra::types::{Address, Balance, Bytes, CallArgs};

/// Utility to crate a [Configuration] instance.
pub struct ConfigurationBuilder {
    configuration: Configuration,
}

impl ConfigurationBuilder {
    /// Creates a new instance of ConfigurationBuilder.
    pub fn new(total_onboarded: Balance, variables: &BTreeMap<String, Bytes>) -> Self {
        use consts::*;
        ConfigurationBuilder {
            configuration: Configuration::new(
                DaoConfiguration {
                    post_job_dos_fee: get_variable(POST_JOB_DOS_FEE, variables),
                    internal_auction_time: get_variable(INTERNAL_AUCTION_TIME, variables),
                    public_auction_time: get_variable(PUBLIC_AUCTION_TIME, variables),
                    default_policing_rate: get_variable(DEFAULT_POLICING_RATE, variables),
                    reputation_conversion_rate: get_variable(REPUTATION_CONVERSION_RATE, variables),
                    fiat_conversion_rate_address: get_variable(
                        FIAT_CONVERSION_RATE_ADDRESS,
                        variables,
                    ),
                    forum_kyc_required: get_variable(FORUM_KYC_REQUIRED, variables),
                    bid_escrow_informal_quorum_ratio: get_variable(
                        BID_ESCROW_INFORMAL_QUORUM_RATIO,
                        variables,
                    ),
                    bid_escrow_formal_quorum_ratio: get_variable(
                        BID_ESCROW_FORMAL_QUORUM_RATIO,
                        variables,
                    ),
                    bid_escrow_informal_voting_time: get_variable(
                        BID_ESCROW_INFORMAL_VOTING_TIME,
                        variables,
                    ),
                    bid_escrow_formal_voting_time: get_variable(
                        BID_ESCROW_FORMAL_VOTING_TIME,
                        variables,
                    ),
                    informal_voting_time: get_variable(INFORMAL_VOTING_TIME, variables),
                    formal_voting_time: get_variable(FORMAL_VOTING_TIME, variables),
                    informal_stake_reputation: get_variable(INFORMAL_STAKE_REPUTATION, variables),
                    time_between_informal_and_formal_voting: get_variable(
                        TIME_BETWEEN_INFORMAL_AND_FORMAL_VOTING,
                        variables,
                    ),
                    va_bid_acceptance_timeout: get_variable(VA_BID_ACCEPTANCE_TIMEOUT, variables),
                    va_can_bid_on_public_auction: get_variable(
                        VA_CAN_BID_ON_PUBLIC_AUCTION,
                        variables,
                    ),
                    distribute_payment_to_non_voters: get_variable(
                        DISTRIBUTE_PAYMENT_TO_NON_VOTERS,
                        variables,
                    ),
                    bid_escrow_wallet_address: get_variable(BID_ESCROW_WALLET_ADDRESS, variables),
                    default_reputation_slash: get_variable(DEFAULT_REPUTATION_SLASH, variables),
                    voting_clearness_delta: get_variable(VOTING_CLEARNESS_DELTA, variables),
                    voting_start_after_job_worker_submission: get_variable(
                        VOTING_START_AFTER_JOB_WORKER_SUBMISSION,
                        variables,
                    ),
                    informal_quorum_ratio: get_variable(INFORMAL_QUORUM_RATIO, variables),
                    formal_quorum_ratio: get_variable(FORMAL_QUORUM_RATIO, variables),
                    bid_escrow_payment_ratio: get_variable(BID_ESCROW_PAYMENT_RATIO, variables),
                    voting_ids_address: get_variable(VOTING_IDS_ADDRESS, variables),
                    cancel_finished_voting_timeout: get_variable(
                        CANCEL_FINISHED_VOTING_TIMEOUT,
                        variables,
                    ),
                },
                VotingConfiguration {
                    is_bid_escrow: false,
                    bind_ballot_for_successful_voting: false,
                    unbound_ballot_address: None,
                    contract_calls: Vec::new(),
                    only_va_can_create: true,
                    double_time_between_votings: false,
                },
                total_onboarded,
            ),
        }
    }

    /// Sets the `contract_calls` field with a vec with a single call.
    pub fn contract_call(self, contract_call: ContractCall) -> Self {
        self.contract_calls(vec![contract_call])
    }

    /// Sets the `contract_calls` field.
    pub fn contract_calls(mut self, contract_calls: Vec<ContractCall>) -> Self {
        self.configuration.set_contract_calls(contract_calls);
        self
    }

    /// Sets the `only_va_can_create` field.
    pub fn only_va_can_create(mut self, only_va_can_create: bool) -> Self {
        self.configuration
            .set_only_va_can_create(only_va_can_create);
        self
    }

    /// Sets the `is_bid_escrow` field and inits the fiat rate.
    pub fn set_is_bid_escrow(mut self, is_bid_escrow: bool) -> ConfigurationBuilder {
        let rate: Balance = call_contract(
            self.configuration.fiat_conversion_rate_address(),
            "get_rate",
            &CallArgs::new(),
            None,
        );
        self.configuration.set_fiat_rate(Some(rate));
        self.configuration.set_is_bid_escrow(is_bid_escrow);
        self
    }

    /// Sets the `unbound_ballot_address` field.
    pub fn bind_ballot_for_successful_voting(mut self, address: Address) -> ConfigurationBuilder {
        self.configuration
            .set_bind_ballot_for_successful_voting(true);
        self.configuration.set_unbound_ballot_address(Some(address));
        self
    }

    /// Builds the final [Configuration].
    pub fn build(self) -> Configuration {
        self.configuration
    }
}
