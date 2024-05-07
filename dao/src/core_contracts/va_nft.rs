//! Contains VA NFT Contract definition and related abstractions.
//!
//! # Definitions
//! * Voting Associate (or VA) - users of the system with Reputation and permissions to vote.
//! * External Worker - a Worker who completed the KYC and is not a Voting Associate.
//!
//! # Purpose
//! Ownership of a VA token indicates the address is a VA and is eligible to participate in Voting.
//! If an `External Worker` finishes a job successfully, and wants to become a VA, receives a VA token
//! as a reward. See [`Bid Escrow`].
//!
//! Each [`Address`] can own only one VA token.
//!
//! [`Bid Escrow`]: crate::bid_escrow::BidEscrowContractInterface.
use odra::{
    prelude::string::String,
    types::{Address, Balance, U256},
};

use crate::core_contracts::dao_nft::{DaoNft, TokenId, TokenUri};

/// NFT contract holding information about members of the DAO.
#[odra::module]
pub struct VaNftContract {
    token: DaoNft,
}

#[odra::module]
impl VaNftContract {
    /// Contract constructor.
    ///
    /// Initializes [`DaoNft`] module.
    #[odra(init)]
    pub fn init(&mut self, name: String, symbol: String, base_uri: TokenUri) {
        self.token.init(name, symbol, base_uri);
    }

    delegate! {
        to self.token {
            /// Changes the ownership of the contract. Transfers ownership to the `owner`.
            /// Only the current owner is permitted to call this method.
            /// [`Read more`](crate::modules::access_control::AccessControl::propose_new_owner())
            pub fn propose_new_owner(&mut self, owner: Address);
            /// Accepts the new owner proposition. This can be called only by the proposed owner.
            /// [`Read more`](crate::modules::access_control::AccessControl::accept_new_owner())
            pub fn accept_new_owner(&mut self);
            /// Changes the ownership of the contract to the new address.
            /// [`Read more`](AccessControl::change_ownership())
            pub fn change_ownership(&mut self, owner: Address);
            /// Adds a new address to the whitelist.
            /// [`Read more`](crate::modules::access_control::AccessControl::add_to_whitelist())
            pub fn add_to_whitelist(&mut self, address: Address);
            /// Remove address from the whitelist.
            /// [`Read more`](crate::modules::access_control::AccessControl::remove_from_whitelist())
            pub fn remove_from_whitelist(&mut self, address: Address);
            /// Checks whether the given address is added to the whitelist.
            /// [`Read more`](crate::modules::access_control::AccessControl::is_whitelisted()).
            pub fn is_whitelisted(&self, address: Address) -> bool;
            /// Returns the address of the current owner.
            /// [`Read more`](crate::modules::access_control::AccessControl::get_owner()).
            pub fn get_owner(&self) -> Option<Address>;
            /// Returns a descriptive name for a collection of tokens in this contract.
            pub fn name(&self) -> String;
            /// Gets an abbreviated name for tokens in this contract.
            pub fn symbol(&self) -> String;
            /// Returns the address of the owner of the token.
            ///
            /// If the given `token_id` does not exist the None value is returned.
            pub fn owner_of(&self, token_id: &TokenId) -> Address;
            /// Returns a token id for the given the `address`.
            ///
            /// If the `owner` does not own any token the None value is returned.
            pub fn token_id(&self, address: Address) -> Option<TokenId>;
            /// Returns the number of tokens owned by `owner`.
            pub fn balance_of(&self, owner: &Address) -> U256;
            /// Returns the total number of tokens.
            pub fn total_supply(&self) -> Balance;
            /// Returns a distinct Uniform Resource Identifier (URI) for a given asset.
            pub fn token_uri(&self, token_id: TokenId) -> TokenUri;
            /// Returns a URI prefix that is used by all the assets.
            pub fn base_uri(&self) -> TokenUri;
            /// Creates a new token with the next id and transfers it to a new owner.
            /// Increments the total supply and the balance of the `to` address.
            ///
            /// # Note
            /// Only whitelisted addresses are permitted to call this
            /// method.
            ///
            /// Each user is entitled to own only one token.
            ///
            /// # Errors
            /// * [`UserAlreadyOwnsToken`](crate::utils::Error::UserAlreadyOwnsToken) if the `to` address
            /// already owns a token.
            ///
            /// # Events
            /// * [`Transfer`](odra_modules::erc721::events::Transfer) when minted successfully.
            pub fn mint(&mut self, to: Address);
            /// Burns a token with a given id. Decrements the balance of the token owner
            /// and decrements the total supply.
            ///
            /// # Errors
            /// * [`NotWhitelisted`](crate::utils::Error::NotWhitelisted) if caller
            /// is not whitelisted.
            ///
            /// # Events
            /// * [`Transfer`](odra_modules::erc721::events::Transfer) when burnt successfully.
            pub fn burn(&mut self, owner: Address);
        }
    }
}
