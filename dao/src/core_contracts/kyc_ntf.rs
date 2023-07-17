//! Contains KYC NFT Contract definition and related abstractions.
//!
//! # Definitions
//! KYC - Know Your Customer, is a process that validates that the user can be the user of the system.
//!
//! # Purpose
//! Ownership of a KYC token indicates the address has been successfully verified and is eligible to participate in the system.
//! Minting token is usually done as a result of [`KYC Voting`].
//!
//! Each [`Address`] can own only one KYC token.
//!
//! [`KYC Voting`]: crate::voting_contracts::KycVoterContract
use crate::core_contracts::dao_nft::{DaoNft, TokenId, TokenUri};
use alloc::string::String;
use odra::types::{Address, Balance, U256};

/// NFT contract that tells the system if user is KYC'd.
/// Kyc Owned Nft contract acts like an erc-721 token and derives most of erc-721 standard.
///
/// Kyc Owned Nft token is mintable and burnable but the caller needs to have permissions to perform those actions.
#[odra::module]
pub struct KycNftContract {
    token: DaoNft,
}

#[odra::module]
impl KycNftContract {
    /// Contract constructor.
    ///
    /// Initializes [`DaoNft`] module.
    #[odra(init)]
    pub fn init(&mut self, name: String, symbol: String, base_uri: TokenUri) {
        self.token.init(name, symbol, base_uri);
    }

    delegate! {
        to self.token {
            pub fn change_ownership(&mut self, owner: Address);
            /// Adds a new address to the whitelist.
            pub fn add_to_whitelist(&mut self, address: Address);
            /// Remove address from the whitelist.
            pub fn remove_from_whitelist(&mut self, address: Address);
            /// Checks whether the given address is added to the whitelist.
            pub fn is_whitelisted(&self, address: Address) -> bool;
            /// Returns the address of the current owner.
            pub fn get_owner(&self) -> Option<Address>;
            /// Returns a descriptive name for a collection of tokens in this contract.
            pub fn name(&self) -> String;
            /// Gets an abbreviated name for tokens in this contract.
            pub fn symbol(&self) -> String;
            /// Returns the address of the owner of the token.
            ///
            /// If the given `token_id` does not exist the None value is returned.
            pub fn owner_of(&self, token_id: &TokenId) -> Address;
            /// Returns the token id for a given `address`.
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
            /// Only whitelisted addresses are permitted to call this function.
            ///
            /// Each user is entitled to own only one token.
            ///
            /// # Errors
            /// * [`UserAlreadyOwnsToken`](crate::utils::Error::UserAlreadyOwnsToken) if the `to` address
            /// already owns a token.
            ///
            /// # Events
            /// * [`Transfer`](odra_modules::erc721::events::Transfer) event when minted successfully.
            pub fn mint(&mut self, to: Address);
            /// Burns a token with the given id. Decrements the balance of the token owner
            /// and decrements the total supply.
            ///
            /// # Errors
            /// * [`NotWhitelisted`](crate::utils::Error::NotWhitelisted) if the caller
            /// is not whitelisted.
            ///
            /// # Events
            /// * [`Transfer`](odra_modules::erc721::events::Transfer) event when burnt successfully.
            pub fn burn(&mut self, owner: Address);
        }
    }
}
