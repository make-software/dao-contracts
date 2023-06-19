//! System configuration.
//!
//! A configuration is a mix of [`Governance Variables`] and voting configuration.
//! DAO supports a few types of voting. Each type may have a slightly different configuration.
//! Once voting is created, until the end, voting relies on the system's state at the moment of voting creation.
//! It mitigates unexpected behavior during voting if the internal DAO state changes.
//!
//! [`Governance Variables`]: crate::variable_repository
mod builder;
mod dao_configuration;
mod voting_configuration;

pub use builder::ConfigurationBuilder;
pub use dao_configuration::DaoConfiguration;
pub use voting_configuration::VotingConfiguration;

use crate::utils::{per_mil_of, per_mil_of_as_u32, to_per_mils, ContractCall, Error};
use odra::types::{Address, Balance, BlockTime};
use odra::{OdraType, UnwrapOrRevert};

/// Represents the current system configuration.
#[derive(OdraType)]
pub struct Configuration {
    dao_configuration: DaoConfiguration,
    voting_configuration: VotingConfiguration,
    total_onboarded: Balance,
    fiat_rate: Option<Balance>,
}

impl Configuration {
    pub fn set_bind_ballot_for_successful_voting(
        &mut self,
        bind_ballot_for_successful_voting: bool,
    ) {
        self.voting_configuration
            .set_bind_ballot_for_successful_voting(bind_ballot_for_successful_voting);
    }

    pub fn set_unbound_ballot_address(&mut self, unbound_ballot_address: Option<Address>) {
        self.voting_configuration
            .set_unbound_ballot_address(unbound_ballot_address);
    }

    pub fn set_is_bid_escrow(&mut self, is_bid_escrow: bool) {
        self.voting_configuration.set_is_bid_escrow(is_bid_escrow);
    }

    pub fn set_only_va_can_create(&mut self, only_va_can_create: bool) {
        self.voting_configuration
            .set_only_va_can_create(only_va_can_create);
    }

    pub fn set_contract_calls(&mut self, contract_calls: Vec<ContractCall>) {
        self.voting_configuration.set_contract_calls(contract_calls);
    }

    /// Indicates if the creator ballot should be bounded at the voting ends.
    pub fn should_bind_ballot_for_successful_voting(&self) -> bool {
        self.voting_configuration
            .should_bind_ballot_for_successful_voting()
    }

    /// Gets the address of the user who cast an unbound ballot.
    pub fn get_unbound_ballot_address(&self) -> Option<Address> {
        self.voting_configuration.get_unbound_ballot_address()
    }

    pub fn new(
        dao_configuration: DaoConfiguration,
        voting_configuration: VotingConfiguration,
        total_onboarded: Balance,
    ) -> Configuration {
        Configuration {
            dao_configuration,
            voting_configuration,
            total_onboarded,
            fiat_rate: None,
        }
    }

    pub fn set_fiat_rate(&mut self, fiat_rate: Option<Balance>) {
        self.fiat_rate = fiat_rate;
    }

    /// Sets the flag `bind_ballot_for_successful_voting` and the address of the voter.
    pub fn bind_ballot_for_successful_voting(&mut self, address: Address) {
        self.voting_configuration.bind_ballot_for_successful_voting = true;
        self.voting_configuration.unbound_ballot_address = Some(address);
    }

    /// Sets the flag `double_time_between_votings`. See [Self::should_double_time_between_votings()].
    pub fn double_time_between_votings(&mut self) {
        self.voting_configuration.double_time_between_votings = true
    }

    /// Indicates if the time between informal and formal voting should be doubled.
    ///
    /// See [Variable Repository](crate::variable_repository) TimeBetweenInformalAndFormalVoting
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn should_double_time_between_votings(&self) -> bool {
        self.voting_configuration.double_time_between_votings
    }

    /// Gets the address of the contract holding the current fiat conversion rate.
    ///
    /// See [Variable Repository](crate::variable_repository) FiatConversionRateAddress
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn fiat_conversion_rate_address(&self) -> Address {
        self.dao_configuration.fiat_conversion_rate_address
    }

    /// Gets formal voting quorum.
    ///
    /// See [Variable Repository](crate::variable_repository) BidEscrowFormalQuorumRatio/FormalQuorumRatio
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn formal_voting_quorum(&self) -> u32 {
        let ratio = match self.voting_configuration.is_bid_escrow {
            true => self.dao_configuration.bid_escrow_formal_quorum_ratio,
            false => self.dao_configuration.formal_quorum_ratio,
        };

        per_mil_of_as_u32(ratio, self.total_onboarded())
            .unwrap_or_revert_with(Error::ArithmeticOverflow)
    }

    /// Gets informal voting quorum.
    ///
    /// See [Variable Repository](crate::variable_repository) BidEscrowInformalQuorumRatio/InformalQuorumRatio
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn informal_voting_quorum(&self) -> u32 {
        let ratio = match self.voting_configuration.is_bid_escrow {
            true => self.dao_configuration.bid_escrow_informal_quorum_ratio,
            false => self.dao_configuration.informal_quorum_ratio,
        };

        per_mil_of_as_u32(ratio, self.total_onboarded())
            .unwrap_or_revert_with(Error::ArithmeticOverflow)
    }

    /// Gets informal voting time.
    ///
    /// See [Variable Repository](crate::variable_repository) BidEscrowInformalVotingTime/InformalVotingTime
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn informal_voting_time(&self) -> BlockTime {
        match self.voting_configuration.is_bid_escrow {
            true => self.dao_configuration.bid_escrow_informal_voting_time,
            false => self.dao_configuration.informal_voting_time,
        }
    }

    /// Gets formal voting time.
    ///
    /// See [Variable Repository](crate::variable_repository) BidEscrowInformalVotingTime/InformalVotingTime
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn formal_voting_time(&self) -> BlockTime {
        match self.voting_configuration.is_bid_escrow {
            true => self.dao_configuration.bid_escrow_formal_voting_time,
            false => self.dao_configuration.formal_voting_time,
        }
    }

    /// Gets formal voting time.
    ///
    /// See [Variable Repository](crate::variable_repository) InformalStakeReputation
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn informal_stake_reputation(&self) -> bool {
        self.dao_configuration.informal_stake_reputation
    }

    /// Gets the time between informal and formal voting.
    ///
    /// See [Variable Repository](crate::variable_repository) TimeBetweenInformalAndFormalVoting
    /// ([available keys](crate::variable_repository#available-keys)).
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

    /// Gets the address of a multisig wallet of the DAO.
    ///
    /// See [Variable Repository](crate::variable_repository) BidEscrowWalletAddress
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn bid_escrow_wallet_address(&self) -> Address {
        self.dao_configuration.bid_escrow_wallet_address
    }

    /// Gets the default reputation slash ratio.
    ///
    /// See [Variable Repository](crate::variable_repository) DefaultReputationSlash
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn default_reputation_slash(&self) -> Balance {
        self.dao_configuration.default_reputation_slash
    }

    /// Gets the voting clearness delta.
    ///
    /// See [Variable Repository](crate::variable_repository) VotingClearnessDelta
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn voting_clearness_delta(&self) -> Balance {
        self.dao_configuration.voting_clearness_delta
    }

    /// Gets the time between voting creation and the actual voting start.
    ///
    /// Non-BidEscrow voting always starts instantly.
    ///
    /// See [Variable Repository](crate::variable_repository) VotingClearnessDelta
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn voting_delay(&self) -> BlockTime {
        if self.voting_configuration.is_bid_escrow {
            self.dao_configuration
                .voting_start_after_job_worker_submission
        } else {
            0
        }
    }

    /// Indicates if the attached DOS Fee is too low.
    ///
    /// See [Variable Repository](crate::variable_repository) PostJobDOSFee
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn is_post_job_dos_fee_too_low(&self, fiat_value: Balance) -> bool {
        to_per_mils(self.dao_configuration.post_job_dos_fee) > fiat_value
    }

    /// Gets the time of an internal auction.
    pub fn internal_auction_time(&self) -> BlockTime {
        self.dao_configuration.internal_auction_time
    }

    /// Gets the time of a public auction.
    pub fn public_auction_time(&self) -> BlockTime {
        self.dao_configuration.public_auction_time
    }

    /// Gets the bid acceptance timeout.
    ///
    /// See [Variable Repository](crate::variable_repository) VABidAcceptanceTimeout
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn va_bid_acceptance_timeout(&self) -> BlockTime {
        self.dao_configuration.va_bid_acceptance_timeout
    }

    /// Indicates if a VA can bid on a public auction.
    ///
    /// See [Variable Repository](crate::variable_repository) VACanBidOnPublicAuction
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn va_can_bid_on_public_auction(&self) -> bool {
        self.dao_configuration.va_can_bid_on_public_auction
    }

    /// Indicates if the payment for the job should be distributed between all VA’s or only to those who voted
    ///
    /// See [Variable Repository](crate::variable_repository) DistributePaymentToNonVoters.
    /// ([available keys](crate::variable_repository#available-keys)).
    pub fn distribute_payment_to_non_voters(&self) -> bool {
        self.dao_configuration.distribute_payment_to_non_voters
    }

    /// Returns the number of onboarded users (VA's).
    pub fn total_onboarded(&self) -> Balance {
        self.total_onboarded
    }

    /// Returns a vec of calls to be performed once voting is finished.
    pub fn contract_calls(&self) -> &Vec<ContractCall> {
        &self.voting_configuration.contract_calls
    }

    /// Indicates only a VA can create voting.
    pub fn only_va_can_create(&self) -> bool {
        self.voting_configuration.only_va_can_create
    }

    /// Indicates if the voting is an instance of BidEscrow.
    pub fn is_bid_escrow(&self) -> bool {
        self.voting_configuration.is_bid_escrow
    }

    /// Returns the address of the voting id generator contract.
    pub fn voting_ids_address(&self) -> Address {
        self.dao_configuration.voting_ids_address
    }

    /// Indicates if the stake of the voting creator should be converted to a ballot.
    pub fn should_cast_first_vote(&self) -> bool {
        !self.is_bid_escrow()
    }

    /// Applies the value of `DefaultPolicingRate` variable to a given amount.
    pub fn apply_default_policing_rate_to(&self, amount: Balance) -> Balance {
        per_mil_of(amount, self.dao_configuration.default_policing_rate)
            .unwrap_or_revert_with(Error::ArithmeticOverflow)
    }

    /// Applies the value of `BidEscrowPaymentRatio` variable to a given amount.
    pub fn apply_bid_escrow_payment_ratio_to(&self, amount: Balance) -> Balance {
        per_mil_of(amount, self.dao_configuration.bid_escrow_payment_ratio)
            .unwrap_or_revert_with(Error::ArithmeticOverflow)
    }

    /// Applies the value of `ReputationConversionRate` variable to a given amount.
    pub fn apply_reputation_conversion_rate_to(&self, amount: Balance) -> Balance {
        per_mil_of(amount, self.dao_configuration.reputation_conversion_rate)
            .unwrap_or_revert_with(Error::ArithmeticOverflow)
    }

    /// Applies the value of `DefaultReputationSlash` variable to a given amount.
    pub fn apply_default_reputation_slash_to(&self, amount: Balance) -> Balance {
        per_mil_of(amount, self.dao_configuration.default_reputation_slash)
            .unwrap_or_revert_with(Error::ArithmeticOverflow)
    }

    /// Gets the current CSPR:Fiat rate.
    pub fn fiat_rate(&self) -> Option<Balance> {
        self.fiat_rate
    }

    /// Calculates the value CSPRs in Fiat currency.
    pub fn convert_to_fiat(&self, cspr_amount: Balance) -> Result<Balance, Error> {
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
