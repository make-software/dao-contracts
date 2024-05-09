use crate::modules::AccessControl;
use crate::utils::Error;
use odra::{
    contract_env,
    prelude::{format, string::String},
    types::{event::OdraEvent, Address, Balance, U256},
    Mapping, Sequence, Variable,
};
use odra_modules::erc721::events::Transfer;
use odra_modules::erc721::{
    erc721_base::Erc721Base,
    extensions::erc721_metadata::{Erc721Metadata, Erc721MetadataExtension},
    Erc721,
};

/// A unique token id.
pub type TokenId = U256;
/// A distinct Uniform Resource Identifier (URI) for a token.
pub type TokenUri = String;

/// NFT module used by DAO.
#[odra::module(events = [Transfer])]
pub struct DaoNft {
    core: Erc721Base,
    metadata: Erc721MetadataExtension,
    access_control: AccessControl,
    tokens: Mapping<Address, Option<TokenId>>,
    id_gen: Sequence<TokenId>,
    total_supply: Variable<Balance>,
}

#[odra::module]
impl DaoNft {
    delegate! {
        to self.access_control {
            /// Changes the ownership of the contract. Transfers ownership to the `owner`.
            /// Only the current owner is permitted to call this method.
            /// [`Read more`](AccessControl::propose_new_owner())
            pub fn propose_new_owner(&mut self, owner: Address);
            /// Accepts the new owner proposition. This can be called only by the proposed owner.
            /// [`Read more`](AccessControl::accept_new_owner())
            pub fn accept_new_owner(&mut self);
            /// Adds a new address to the whitelist.
            /// [`Read more`](AccessControl::add_to_whitelist())
            pub fn add_to_whitelist(&mut self, address: Address);
            /// Remove address from the whitelist.
            /// [`Read more`](AccessControl::remove_from_whitelist())
            pub fn remove_from_whitelist(&mut self, address: Address);
            /// Checks whether the given address is added to the whitelist.
            /// [`Read more`](AccessControl::is_whitelisted()).
            pub fn is_whitelisted(&self, address: Address) -> bool;
            /// Returns the address of the current owner.
            /// [`Read more`](AccessControl::get_owner()).
            pub fn get_owner(&self) -> Option<Address>;
        }

        to self.metadata {
            /// Returns a descriptive name for a collection of tokens in this contract.
            pub fn name(&self) -> String;
            /// Gets an abbreviated name for tokens in this contract.
            pub fn symbol(&self) -> String;
            /// Returns a URI prefix that is used by all the assets.
            pub fn base_uri(&self) -> TokenUri;
        }

        to self.core {
            /// Returns the address of the owner of the token.
            ///
            /// If the given `token_id` does not exist the None value is returned.
            pub fn owner_of(&self, token_id: &TokenId) -> Address;
            /// Returns the number of tokens owned by `owner`.
            pub fn balance_of(&self, owner: &Address) -> U256;
        }
    }

    /// Module constructor.
    ///
    /// Initializes modules. Sets the deployer as the owner.
    ///
    /// See [Erc721MetadataExtension](Erc721MetadataExtension::init()), [AccessControl](AccessControl::init())
    pub fn init(&mut self, name: String, symbol: String, base_uri: TokenUri) {
        let deployer = contract_env::caller();
        self.metadata.init(name, symbol, base_uri);
        self.access_control.init(deployer);
    }

    /// Returns the total number of tokens.
    pub fn total_supply(&self) -> Balance {
        self.total_supply.get_or_default()
    }

    /// Returns the token id for a given `address`.
    ///
    /// If the `owner` does not own any token the None value is returned.
    pub fn token_id(&self, address: Address) -> Option<TokenId> {
        self.tokens.get(&address).unwrap_or(None)
    }

    /// Returns a distinct Uniform Resource Identifier (URI) for a given asset.
    pub fn token_uri(&self, token_id: TokenId) -> TokenUri {
        if !self.core.exists(&token_id) {
            contract_env::revert(Error::TokenDoesNotExist)
        }
        format!("{}{}", self.metadata.base_uri(), token_id)
    }

    /// Creates a new token with the next id and transfers it to a new owner.
    /// Increments the total supply and the balance of the `to` address.
    ///
    /// # Note
    /// Only whitelisted addresses are permitted to call this function.
    ///
    /// Each user is entitled to own only one token.
    ///
    /// # Errors
    /// * [`UserAlreadyOwnsToken`](Error::UserAlreadyOwnsToken) if the `to` address
    /// already owns a token.
    ///
    /// # Events
    /// * [`Transfer`] event when minted successfully.
    pub fn mint(&mut self, to: Address) {
        // Check if the caller is whitelisted
        self.access_control.ensure_whitelisted();
        self.assert_does_not_own_token(&to);

        let token_id = self.id_gen.next_value();

        // Mint token
        if self.core.exists(&token_id) {
            contract_env::revert(Error::TokenAlreadyExists)
        }
        self.core.balances.add(&to, U256::one());
        self.total_supply.add(Balance::one());
        self.core.owners.set(&token_id, Some(to));

        self.tokens.set(&to, Some(token_id));

        Transfer {
            from: None,
            to: Some(to),
            token_id,
        }
        .emit();
    }

    /// Burns a token with the given id. Decrements the balance of the token owner
    /// and decrements the total supply.
    ///
    /// # Errors
    /// * [`NotWhitelisted`](crate::utils::Error::NotWhitelisted) if the caller
    /// is not whitelisted.
    ///
    /// # Events
    /// * [`Transfer`] event when burnt successfully.
    pub fn burn(&mut self, owner: Address) {
        self.access_control.ensure_whitelisted();
        let token_id = self.token_id(owner);

        if let Some(token_id) = token_id {
            self.core.balances.subtract(&owner, U256::from(1));
            self.core.owners.set(&token_id, None);
            self.core.clear_approval(&token_id);
            self.total_supply.subtract(Balance::from(1));
            self.tokens.set(&owner, None);

            Transfer {
                from: Some(owner),
                to: None,
                token_id,
            }
            .emit();
        }
    }
}

impl DaoNft {
    fn assert_does_not_own_token(&self, address: &Address) {
        if self.tokens.get(address).is_some() {
            contract_env::revert(Error::UserAlreadyOwnsToken)
        }
    }
}
