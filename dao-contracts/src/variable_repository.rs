//! Contains Variable Repository Contract definition and related abstractions.
//!
//!  Variable Repository Contract stores governance variables. Values can be altered
//!  as a result of voting in [Repo Voting]
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
//! | GovernanceWalletAddress            |               |              | address | An address of a multisig wallet of the DAO.         
//!
//!  [Repo Voting]: crate::repo_voter::RepoVoterContractInterface   
use std::collections::BTreeMap;

use casper_dao_modules::{AccessControl, Record, Repository};
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
};
use casper_types::bytesrepr::Bytes;
use delegate::delegate;

// Interface of the Variable Repository Contract.
//
// It should be implemented by [`VariableRepositoryContract`], [`VariableRepositoryContractCaller`]
// and [`VariableRepositoryContractTest`].
#[casper_contract_interface]
pub trait VariableRepositoryContractInterface {
    /// Constructor function.
    ///
    /// # Note
    /// Initializes contract elements:
    /// * Sets the default configuration of the [`Repository`](casper_dao_modules::Repository)
    /// * Sets [`caller`] as the owner of the contract.
    /// * Adds [`caller`] to the whitelist.
    ///
    /// # Events
    /// Emits:
    /// * [`OwnerChanged`](casper_dao_modules::events::OwnerChanged),
    /// * [`AddedToWhitelist`](casper_dao_modules::events::AddedToWhitelist),
    /// * multiple [`ValueUpdated`](casper_dao_modules::events::ValueUpdated) events,
    /// one per value of the default repository configuration.
    fn init(&mut self);
    /// Changes the ownership of the contract. Transfers the ownership to the `owner`.
    /// Only the current owner is permitted to call this method.
    ///
    /// [`Read more`](AccessControl::change_ownership())
    fn change_ownership(&mut self, owner: Address);
    /// Adds a new address to the whitelist.
    ///
    /// [`Read more`](AccessControl::add_to_whitelist())
    fn add_to_whitelist(&mut self, address: Address);
    /// Remove address from the whitelist.
    ///
    /// [`Read more`](AccessControl::remove_from_whitelist())
    fn remove_from_whitelist(&mut self, address: Address);
    /// Checks whether the given address is added to the whitelist.
    /// 
    /// [`Read more`](AccessControl::is_whitelisted()).
    fn is_whitelisted(&self, address: Address) -> bool;
    /// Returns the address of the current owner.
    /// 
    /// [`Read more`](AccessControl::get_owner()).
    fn get_owner(&self) -> Option<Address>;
    /// Inserts or updates the value under the given key.
    ///
    /// # Note
    /// * The activation time is represented as a unix timestamp.
    /// * If the activitation time is `None` the value is updated immediately.
    /// * If some future time in the future is passed as an argument, the [`Self::get`] function
    /// returns the previously set value.
    ///
    /// # Events
    /// * Emits [`ValueUpdated`](casper_dao_modules::events::ValueUpdated) event.
    ///
    /// # Errors
    /// * Throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if the caller
    /// is not a whitelisted user.
    /// * Throws [`ActivationTimeInPast`](casper_dao_utils::Error::ActivationTimeInPast) if
    /// the activation time has passed already.
    /// * Throws [`ValueNotAvailable`](casper_dao_utils::Error::ValueNotAvailable) on
    /// the future value update if the current value has not been set.
    fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>);
    /// Returns the value stored under the given key.
    ///
    /// If the key does not exist, the `None` value is returned.
    fn get(&self, key: String) -> Option<Bytes>;
    /// Returns the full (current and future) value stored under the given key.
    /// See [`Record`](casper_dao_modules::Record).
    ///
    /// If the key does not exist, the `None` value is returned.
    fn get_full_value(&self, key: String) -> Option<Record>;
    /// Returns the value stored under the given index.
    ///
    /// Every freshly added key has the previous key index increased by 1.
    /// The index range is 0 to #keys-1.
    ///
    /// If the given index exceeds #keys-1 the `None` value is returned.
    fn get_key_at(&self, index: u32) -> Option<String>;
    /// Returns the number of existing keys in the [`Repository`](casper_dao_modules::Repository).
    fn keys_count(&self) -> u32;
    /// Reads all the stored variables and returns a map key to value.
    fn all_variables(&self) -> BTreeMap<String, Bytes>;
}

/// Variable Repository Contract implementation. See [`VariableRepositoryContractInterface`].
#[derive(Instance)]
pub struct VariableRepositoryContract {
    pub access_control: AccessControl,
    pub repository: Repository,
}

impl VariableRepositoryContractInterface for VariableRepositoryContract {
    delegate! {
        to self.access_control {
            fn is_whitelisted(&self, address: Address) -> bool;
            fn get_owner(&self) -> Option<Address>;
            fn change_ownership(&mut self, owner: Address);
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
        }
    }

    fn init(&mut self) {
        let deployer = caller();
        self.access_control.init(deployer);
        self.repository.init();
    }

    fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>) {
        self.access_control.ensure_whitelisted();
        self.repository.update_at(key, value, activation_time);
    }

    fn get(&self, key: String) -> Option<Bytes> {
        self.repository.get(key)
    }

    fn get_full_value(&self, key: String) -> Option<Record> {
        self.repository.get_full_value(key)
    }

    fn get_key_at(&self, index: u32) -> Option<String> {
        self.repository.keys.get(index)
    }

    fn keys_count(&self) -> u32 {
        self.repository.keys.size()
    }

    fn all_variables(&self) -> BTreeMap<String, Bytes> {
        let mut result: BTreeMap<String, Bytes> = BTreeMap::new();

        for key in 0..self.repository.keys.length.get().unwrap() {
            let repo_key = self.repository.keys.get(key).unwrap();
            let value = self.repository.get(repo_key.clone()).unwrap();
            result.insert(repo_key, value);
        }

        result
    }
}
