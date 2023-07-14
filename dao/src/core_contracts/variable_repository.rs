//! Contains Variable Repository Contract definition and related abstractions.
//!
//!  Variable Repository Contract stores governance variables. Values can be altered
//!  as a result of [Repo Voting].
//!
//! # Available keys
//!
//! | Parameter name                     | Initial value | Stored value | Type    | Description                                                                                                                                                                                                                     |
//! |------------------------------------|---------------|--------------|---------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | PostJobDOSFee                      | 10            | 10000        | float   | A DOS fee that the JobPoster needs to attach to the Post Job query. The value is the minimum amount of Fiat currency to be attached as CSPR using FiatConversionRate                                                            |
//! | InternalAuctionTime                | 7 days        | 604800       | seconds | The time of the Internal Auction                                                                                                                                                                                                |
//! | PublicAuctionTime                  | 10 days       | 864000       | seconds | The time of the External Auction                                                                                                                                                                                                |
//! | DefaultPolicingRate                | 0.3           | 300          | float   | This rate defines how many Reputation tokens are given to the VA’s for their community audit/vote on a work product.. In case of value of 0.3, 30% of the payment is redistributed between VA’s and 70% is given to the Worker. |
//! | ReputationConversionRate           | 0.1           | 100          | float   | This parameter tells how much Reputation is minted for each unit of currency paid for Work. For value equal to 0.1, 1 Reputation is minted for each 10 CSPR.                                                                    |
//! | FiatConversionRateAddress          |               |              | address | An address of a contract that will return the conversion rate between Fiat and CSPR                                                                                                                                             |
//! | ForumKYCRequired                   | true          | true         | bool    | Defines if KYC is required to post on Forum                                                                                                                                                                                     |
//! | BidEscrowInformalQuorumRatio       | 0.5           | 500          | float   | How many holders of the Reputation tokens (VA’s) are needed for an informal voting quorum                                                                                                                                       |
//! | BidEscrowFormalQuorumRatio         | 0.5           | 500          | float   | How many holders of the Reputation tokens (VA’s) are needed for a bid escrow formal vote quorum. For example, if 100 accounts hold tokens, the quorum would be 51 votes.                                                        |
//! | InformalQuorumRatio                | 0.5           | 500          | float   | How many holders of the Reputation tokens (VA’s) are needed for a regular informal voting quorum.                                                                                                                               |
//! | FormalQuorumRatio                  | 0.5           | 500          | float   | How many holders of the Reputation tokens (VA’s) are needed for a regular formal voting quorum.                                                                                                                                 |
//! | BidEscrowInformalVotingTime        | 5 days        | 432000       | seconds | Time for the informal part of the Bid Escrow voting                                                                                                                                                                             |
//! | BidEscrowFormalVotingTime          | 5 days        | 432000       | seconds | Time for the formal part of the Bid Escrow voting                                                                                                                                                                               |
//! | InformalVotingTime                 | 5 days        | 432000       | seconds | Time for the informal part of other voting                                                                                                                                                                                      |
//! | FormalVotingTime                   | 5 days        | 432000       | seconds | Time for the formal part of other voting                                                                                                                                                                                        |
//! | InformalStakeReputation            | true          | true         | bool    | Tells if the Informal Voting should stake the reputation or only simulate it.                                                                                                                                                   |
//! | TimeBetweenInformalAndFormalVoting | 1 day         | 86400        | seconds | Time between Informal and Formal Voting                                                                                                                                                                                         |
//! | VABidAcceptanceTimeout             | 2 days        | 172800       | seconds | How much time the bid wait for the acceptance. After this time, the bid can be cancelled                                                                                                                                        |
//! | VACanBidOnPublicAuction            | false         | false        | bool    | Whether or not VA’s can take part in the Public Auction part of the Bidding process.                                                                                                                                            |
//! | DistributePaymentToNonVoters       | true          | false        | bool    | Determines if the Payment for the Job should be distributed between all VA’s or only to those who voted                                                                                                                         |
//! | DefaultReputationSlash             | 0.1           | 100          | float   | How much reputation of an Internal Worker is slashed after not completing a Job                                                                                                                                                 |
//! | VotingClearnessDelta               | 8             | 8000         | int     | If the difference between 50/50 and result of the Informal Voting is bigger than the value, the time between votings should be doubled.                                                                                         |
//! | VotingStartAfterJobWorkerSubmisson | 3 days        | 259200       | seconds | Time between the worker job submission and the internal voting start.                                                                                                                                                           |
//! | BidEscrowPaymentRatio              | 0.1           | 100          | float   | How much CSPR is sent to GovernanceWallet after the Job is finished                                                                                                                                                             |
//! | BidEscrowWalletAddress             |               |              | address | An address of a multisig wallet of the DAO.
//!
//!  [Repo Voting]: crate::voting_contracts::RepoVoterContract

use crate::modules::{AccessControl, Record, Repository};
use crate::utils::Error;
use odra::contract_env::caller;
use odra::types::{Address, Bytes};
use odra::UnwrapOrRevert;
use std::collections::BTreeMap;

/// Variable Repository Contract.
#[odra::module]
pub struct VariableRepositoryContract {
    pub access_control: AccessControl,
    pub repository: Repository,
}

#[odra::module]
impl VariableRepositoryContract {
    delegate! {
        to self.access_control {
            /// Checks whether the given address is added to the whitelist.
            /// [`Read more`](AccessControl::is_whitelisted()).
            pub fn is_whitelisted(&self, address: Address) -> bool;
            /// Returns the address of the current owner.
            /// [`Read more`](AccessControl::get_owner()).
            pub fn get_owner(&self) -> Option<Address>;
            /// Changes the ownership of the contract. Transfers ownership to the `owner`.
            /// Only the current owner is permitted to call this method.
            /// [`Read more`](AccessControl::change_ownership())
            pub fn change_ownership(&mut self, owner: Address);
            /// Adds a new address to the whitelist.
            /// [`Read more`](AccessControl::add_to_whitelist())
            pub fn add_to_whitelist(&mut self, address: Address);
            /// Remove address from the whitelist.
            /// [`Read more`](AccessControl::remove_from_whitelist())
            pub fn remove_from_whitelist(&mut self, address: Address);
        }
    }

    /// Constructor function.
    ///
    /// # Note
    /// Initializes contract elements:
    /// * Sets the default configuration of the [`Repository`](crate::modules::repository::Repository)
    /// * Sets [`caller`] as the owner of the contract.
    /// * Adds [`caller`] to the whitelist.
    ///
    /// # Events
    /// * [`OwnerChanged`](crate::modules::owner::events::OwnerChanged),
    /// * [`AddedToWhitelist`](crate::modules::whitelist::events::AddedToWhitelist),
    /// * multiple [`ValueUpdated`](crate::modules::repository::events::ValueUpdated) events,
    /// one per value of the default repository configuration.
    #[odra(init)]
    pub fn init(
        &mut self,
        fiat_conversion: Address,
        bid_escrow_wallet: Address,
        voting_ids: Address,
    ) {
        let deployer = caller();
        self.access_control.init(deployer);
        self.repository
            .init(fiat_conversion, bid_escrow_wallet, voting_ids);
    }

    /// Inserts or updates the value under the given key.
    ///
    /// # Note
    /// * The activation time is represented as a unix timestamp.
    /// * If the activation time is `None` the value is updated immediately.
    /// * If some future time in the future is passed as an argument, the [`Self::get`] function
    /// returns the previously set value.
    ///
    /// # Events
    /// * [`ValueUpdated`](crate::modules::repository::events::ValueUpdated).
    ///
    /// # Errors
    /// * [`NotWhitelisted`](crate::utils::Error::NotWhitelisted) if the caller
    /// is not a whitelisted user.
    /// * [`ActivationTimeInPast`](crate::utils::Error::ActivationTimeInPast) if
    /// the activation time has passed already.
    pub fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>) {
        self.access_control.ensure_whitelisted();
        self.repository.update_at(key, value, activation_time);
    }

    /// Returns the value stored under the given key.
    ///
    /// If the key does not exist, the `None` value is returned.
    pub fn get(&self, key: String) -> Option<Bytes> {
        self.repository.get(key)
    }

    /// Returns the full (current and future) value stored under the given key.
    /// See [`Record`](Record).
    ///
    /// If the key does not exist, the `None` value is returned.
    pub fn get_full_value(&self, key: String) -> Option<Record> {
        self.repository.get_full_value(key)
    }

    /// Returns the value stored under the given index.
    ///
    /// Every freshly added key has the previous key index increased by 1.
    /// The index range is 0 to #keys-1.
    ///
    /// If the given index exceeds #keys-1 the `None` value is returned.
    pub fn get_key_at(&self, index: u32) -> Option<String> {
        self.repository.keys2.get(index)
    }

    /// Returns the number of existing keys in the [`Repository`](crate::modules::repository::Repository).
    pub fn keys_count(&self) -> u32 {
        self.repository.keys2.len()
    }

    /// Reads all the stored variables and returns a map key to value.
    pub fn all_variables(&self) -> BTreeMap<String, Bytes> {
        let mut result: BTreeMap<String, Bytes> = BTreeMap::new();

        for key in 0..self.repository.keys2.len() {
            let repo_key = self
                .repository
                .keys2
                .get(key)
                .unwrap_or_revert_with(Error::RepositoryError);
            let value = self
                .repository
                .get(repo_key.clone())
                .unwrap_or_revert_with(Error::RepositoryError);
            result.insert(repo_key, value);
        }

        result
    }
}
