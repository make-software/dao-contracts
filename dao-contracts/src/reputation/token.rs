use std::collections::BTreeMap;

use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
};
use casper_types::U512;
use delegate::delegate;

use super::{
    agg::{AggregatedStake, AggregatedBalance, BalanceAggregates},
    balances::BalanceStorage,
    stakes::StakesStorage,
};
use crate::{
    bid_escrow::bid::ShortenedBid,
    voting::{ShortenedBallot, VotingId},
};

// Interface of the Reputation Contract.
//
// It should be implemented by [`ReputationContract`], [`ReputationContractCaller`]
// and [`ReputationContractTest`].
#[casper_contract_interface]
pub trait ReputationContractInterface {
    /// Constructor method.
    ///
    /// It initializes contract elements:
    /// * Events dictionary.
    /// * Named keys of [`AccessControl`].
    /// * Set [`caller`] as the owner of the contract.
    /// * Add [`caller`] to the whitelist.
    ///
    /// It emits [`OwnerChanged`](casper_dao_modules::events::OwnerChanged),
    /// [`AddedToWhitelist`](casper_dao_modules::events::AddedToWhitelist) events.
    fn init(&mut self);

    /// Mints new tokens. Adds `amount` of new tokens to the balance of the `recipient` and
    /// increments the total supply. Only whitelisted addresses are permitted to call this method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It emits [`Mint`](events::Mint) event.
    fn mint(&mut self, recipient: Address, amount: U512);

    /// Increases the balance of the passive reputation of the given address.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    fn mint_passive(&mut self, recipient: Address, amount: U512);

    /// Burns existing tokens. Removes `amount` of existing tokens from the balance of the `owner`
    /// and decrements the total supply. Only whitelisted addresses are permitted to call this
    /// method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It emits [`Burn`](events::Burn) event.
    fn burn(&mut self, owner: Address, amount: U512);

    /// Decreases the balance of the passive reputation of the given address.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It throws [`InsufficientBalance`](casper_dao_utils::Error::InsufficientBalance) if the passed
    /// amount exceeds the balance of the passive reputation of the given address.
    fn burn_passive(&mut self, owner: Address, amount: U512);

    /// Changes ownership of the contract. Transfer the ownership to the `owner`. Only the current owner
    /// is permitted to call this method.
    ///
    /// See [AccessControl](AccessControl::change_ownership())
    fn change_ownership(&mut self, owner: Address);

    /// Adds a  new address to the whitelist.
    ///
    /// See [AccessControl](AccessControl::add_to_whitelist())
    fn add_to_whitelist(&mut self, address: Address);

    /// Removes address from the whitelist.
    ///
    /// See [AccessControl](AccessControl::remove_from_whitelist())
    fn remove_from_whitelist(&mut self, address: Address);

    /// Returns the address of the current owner.
    fn get_owner(&self) -> Option<Address>;

    /// Returns the total token supply.
    fn total_supply(&self) -> U512;

    /// Returns the current token balance of the given address.
    fn balance_of(&self, address: Address) -> U512;

    /// Returns the current passive balance of the given address.
    fn passive_balance_of(&self, address: Address) -> U512;

    /// Checks whether the given address is added to the whitelist.
    fn is_whitelisted(&self, address: Address) -> bool;

    /// Stakes the reputation used as voting power.
    fn stake_voting(&mut self, voting_id: VotingId, ballot: ShortenedBallot);

    // Unstakes the reputation used as voting power.
    fn unstake_voting(&mut self, voting_id: VotingId, ballot: ShortenedBallot);

    /// Unstakes the reputation used as voting power.
    fn bulk_unstake_voting(&mut self, voting_id: VotingId, ballots: Vec<ShortenedBallot>);

    /// Stakes the reputation used as bid value.
    fn stake_bid(&mut self, bid: ShortenedBid);

    /// Unstakes the reputation used as bid value.
    fn unstake_bid(&mut self, bid: ShortenedBid);

    /// Unstakes the reputation used as bid value.
    fn bulk_unstake_bid(&mut self, bids: Vec<ShortenedBid>);

    /// Returns the total stake of the given account.
    fn get_stake(&self, address: Address) -> U512;

    /// Gets balances of all the token holders.
    fn all_balances(&self) -> AggregatedBalance;

    /// Gets balances of the given account addresses.
    fn partial_balances(&self, addresses: Vec<Address>) -> AggregatedBalance;

    /// Redistributes the reputation based on the voting summary
    fn bulk_mint_burn(&mut self, mints: BTreeMap<Address, U512>, burns: BTreeMap<Address, U512>);

    /// Burns all the tokens of the `owner`.
    fn burn_all(&mut self, owner: Address);

    /// Returns all the data about the given user stakes.
    fn stakes_info(&self, address: Address) -> AggregatedStake;
}

/// Implementation of the Reputation Contract. See [`ReputationContractInterface`].
#[derive(Instance)]
pub struct ReputationContract {
    pub reputation_storage: BalanceStorage,
    pub passive_reputation_storage: BalanceStorage,
    pub stakes_storage: StakesStorage,
    pub aggregates: BalanceAggregates,
    pub access_control: AccessControl,
}

impl ReputationContractInterface for ReputationContract {
    delegate! {
        to self.access_control {
            fn change_ownership(&mut self, owner: Address);
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
            fn is_whitelisted(&self, address: Address) -> bool;
            fn get_owner(&self) -> Option<Address>;
        }

        to self.passive_reputation_storage {
            #[call(mint)]
            fn mint_passive(&mut self, recipient: Address, amount: U512);
            #[call(burn)]
            fn burn_passive(&mut self, owner: Address, amount: U512);
            #[call(balance_of)]
            fn passive_balance_of(&self, address: Address) -> U512;
        }

        to self.reputation_storage {
            fn mint(&mut self, recipient: Address, amount: U512);
            fn burn(&mut self, owner: Address, amount: U512);
            fn total_supply(&self) -> U512;
            fn balance_of(&self, address: Address) -> U512;
            fn bulk_mint_burn(&mut self, mints: BTreeMap<Address, U512>, burns: BTreeMap<Address, U512>);
            fn burn_all(&mut self, owner: Address);
        }

        to self.stakes_storage {
            fn get_stake(&self, address: Address) -> U512;
            fn stake_voting(&mut self, voting_id: VotingId, ballot: ShortenedBallot);
            fn stake_bid(&mut self, bid: ShortenedBid);
            fn unstake_voting(&mut self, voting_id: VotingId, ballot: ShortenedBallot);
            fn unstake_bid(&mut self, bid: ShortenedBid);
            fn bulk_unstake_voting(&mut self,voting_id:VotingId,ballots:Vec<ShortenedBallot>);
            fn bulk_unstake_bid(&mut self, bids: Vec<ShortenedBid>);
        }

        to self.aggregates {
            fn all_balances(&self) -> AggregatedBalance;
            fn partial_balances(&self, addresses: Vec<Address>) -> AggregatedBalance;
            fn stakes_info(&self, address: Address) -> AggregatedStake;
        }
    }

    fn init(&mut self) {
        let deployer = caller();
        self.access_control.init(deployer);
    }
}

pub mod events {
    use casper_dao_utils::{casper_dao_macros::Event, Address};
    use casper_types::U512;

    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct Burn {
        pub address: Address,
        pub amount: U512,
    }

    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct Mint {
        pub address: Address,
        pub amount: U512,
    }
}
