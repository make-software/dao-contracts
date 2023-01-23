//! Contains VA NFT Contract definition and related abstractions.
//!
//! # Definitions
//! * Voting Associate (or VA) - users of the system with Reputation and permissions to vote.
//! * External Worker - a Worker who completed the KYC and is not a Voting Associate.
//!
//! # Purpose
//! Ownership of a VA token indicates the address is a VA and is eligible to participate in voting.
//! If an `External Worker` finishes a job successfully, and wants to become a VA, receives a VA token
//! as a reward. See [`Bid Escrow`].
//!
//! Each [`Address`] can own only one VA token.
//! 
//! [`Bid Escrow`]: crate::bid_escrow::BidEscrowContractInterface.
use casper_dao_erc721::{
    BurnableERC721,
    ERC721Token,
    MetadataERC721,
    MintableERC721,
    TokenId,
    TokenUri,
};
use casper_dao_modules::{AccessControl, SequenceGenerator};
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller},
    Address,
    Error,
    Mapping,
};
use casper_types::U512;
use delegate::delegate;

#[casper_contract_interface]
pub trait VaNftContractInterface {
    /// Contract constructor.
    ///
    /// Initializes modules. Sets the deployer as the owner.
    ///
    /// See [MetadataERC721](MetadataERC721::init()), [AccessControl](AccessControl::init())
    fn init(&mut self, name: String, symbol: String, base_uri: TokenUri);
    /// Change ownership of the contract. Transfer the ownership to the `owner`. Only current owner
    /// is permitted to call this method.
    ///
    /// See [AccessControl](AccessControl::change_ownership())
    fn change_ownership(&mut self, owner: Address);
    /// Add new address to the whitelist.
    ///
    /// See [AccessControl](AccessControl::add_to_whitelist())
    fn add_to_whitelist(&mut self, address: Address);
    /// Remove address from the whitelist.
    ///
    /// See [AccessControl](AccessControl::remove_from_whitelist())
    fn remove_from_whitelist(&mut self, address: Address);
    /// Returns the address of the current owner.
    fn get_owner(&self) -> Option<Address>;
    /// Checks whether the given address is added to the whitelist.
    fn is_whitelisted(&self, address: Address) -> bool;
    /// Returns a descriptive name for a collection of tokens in this contract
    fn name(&self) -> String;
    /// Gets an abbreviated name for tokens in this contract
    fn symbol(&self) -> String;
    /// Returns the address of the owner of the token.
    ///
    /// If the given `token_id` does not exist the None value is returned.
    fn owner_of(&self, token_id: TokenId) -> Option<Address>;
    /// Returns a token id for the given the `address`.
    ///
    /// If the `owner` does not own any token the None value is returned.
    fn token_id(&self, address: Address) -> Option<TokenId>;
    /// Returns the number of tokens owned by `owner`.
    fn balance_of(&self, owner: Address) -> U512;
    /// Returns the total number of tokens.
    fn total_supply(&self) -> U512;
    /// Returns a distinct Uniform Resource Identifier (URI) for a given asset.
    fn token_uri(&self, token_id: TokenId) -> TokenUri;
    /// Returns a URI prefix that is used by all the assets.
    fn base_uri(&self) -> TokenUri;
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
    /// Throws [`UserAlreadyOwnsToken`](Error::UserAlreadyOwnsToken) if the `to` address
    /// already owns a token.
    ///
    /// # Events
    /// // TODO: Fix events documentation
    /// Emits [`Transfer`](casper_dao_erc721::events::Transfer) event when minted successfully.
    fn mint(&mut self, to: Address);
    /// Burns a token with the given id. Decrements the balance of the token owner
    /// and decrements the total supply.
    ///
    /// # Errors
    /// Throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// # Events
    /// Emits [`Burn`](casper_dao_modules::events::Burn) event.
    fn burn(&mut self, owner: Address);
}

/// Va Owned Nft contract acts like an erc-721 token and derives most of erc-721 standards from
/// [ERC721Token](ERC721Token) module.
///
/// Va Owned Nft token is mintable and burnable but the caller needs to have permissions to perform those actions.
///
/// For details see [VaNftContractInterface](VaNftContractInterface)
#[derive(Instance)]
pub struct VaNftContract {
    token: ERC721Token,
    metadata: MetadataERC721,
    access_control: AccessControl,
    tokens: Mapping<Address, Option<TokenId>>,
    id_gen: SequenceGenerator<TokenId>,
}

impl VaNftContractInterface for VaNftContract {
    delegate! {
        to self.access_control {
            fn is_whitelisted(&self, address: Address) -> bool;
            fn get_owner(&self) -> Option<Address>;
            fn change_ownership(&mut self, owner: Address);
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
        }

        to self.metadata {
            fn name(&self) -> String;
            fn symbol(&self) -> String;
            fn base_uri(&self) -> TokenUri;
        }

        to self.token {
            fn owner_of(&self, token_id: TokenId) -> Option<Address>;
            fn balance_of(&self, owner: Address) -> U512;
            fn total_supply(&self) -> U512;
        }
    }

    fn init(&mut self, name: String, symbol: String, base_uri: TokenUri) {
        let deployer = caller();
        self.metadata.init(name, symbol, base_uri);
        self.access_control.init(deployer);
    }

    fn token_id(&self, address: Address) -> Option<TokenId> {
        self.tokens.get(&address).unwrap_or(None)
    }

    fn token_uri(&self, token_id: TokenId) -> TokenUri {
        self.metadata.token_uri(&self.token, token_id)
    }

    fn mint(&mut self, to: Address) {
        self.access_control.ensure_whitelisted();
        self.assert_does_not_own_token(&to);

        let token_id = self.id_gen.next_value();
        MintableERC721::mint(&mut self.token, to, token_id);
        self.tokens.set(&to, Some(token_id));
    }

    fn burn(&mut self, owner: Address) {
        self.access_control.ensure_whitelisted();
        let token_id = self.token_id(owner);
        if let Some(token_id) = token_id {
            BurnableERC721::burn_unchecked(&mut self.token, token_id);
            self.tokens.set(&owner, None);
        }
    }
}

impl VaNftContract {
    fn assert_does_not_own_token(&self, address: &Address) {
        if self.tokens.get(address).is_some() {
            casper_env::revert(Error::UserAlreadyOwnsToken)
        }
    }
}
