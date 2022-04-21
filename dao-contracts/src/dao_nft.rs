use casper_dao_erc721::{
    core::ERC721Token, BurnableERC721, MetadataERC721, MintableERC721, TokenId, TokenUri,
};
use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    Address, Mapping,
};
use casper_types::U256;
use delegate::delegate;

#[casper_contract_interface]
pub trait DaoOwnedNftContractInterface {
    fn init(&mut self, name: String, symbol: String, base_uri: TokenUri);
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn get_owner(&self) -> Option<Address>;
    fn is_whitelisted(&self, address: Address) -> bool;
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn owner_of(&self, token_id: TokenId) -> Address;
    fn token_id(&self, address: Address) -> Option<TokenId>;
    fn balance_of(&self, owner: Address) -> U256;
    fn total_supply(&self) -> U256;
    fn token_uri(&self, token_id: TokenId) -> TokenUri;
    fn base_uri(&self) -> TokenUri;
    fn mint(&mut self, to: Address, token_id: TokenId);
    fn burn(&mut self, token_id: TokenId);
    fn approve(&mut self, approved: Option<Address>, token_id: TokenId);
    fn set_approval_for_all(&mut self, operator: Address, approved: bool);
}

#[derive(Instance)]
pub struct DaoOwnedNftContract {
    token: ERC721Token,
    metadata: MetadataERC721,
    access_control: AccessControl,
    tokens: Mapping<Address, Option<TokenId>>,
}

impl DaoOwnedNftContractInterface for DaoOwnedNftContract {
    fn init(&mut self, name: String, symbol: String, _base_uri: TokenUri) {
        self.metadata.init(name, symbol);
    }

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
            fn owner_of(&self, token_id: TokenId) -> Address;
            fn balance_of(&self, owner: Address) -> U256;
            fn total_supply(&self) -> U256;
            fn approve(&mut self, approved: Option<Address>, token_id: TokenId);
            fn set_approval_for_all(&mut self, operator: Address, approved: bool);
        }
    }

    fn token_uri(&self, token_id: TokenId) -> TokenUri {
        self.metadata.token_uri(&self.token, token_id)
    }

    fn mint(&mut self, to: Address, token_id: TokenId) {
        MintableERC721::mint(&mut self.token, to, token_id);
        self.tokens.set(&to, Some(token_id));
    }

    fn burn(&mut self, token_id: TokenId) {
        let owner = self.token.owner_of(token_id);
        BurnableERC721::burn(&mut self.token, token_id);
        self.tokens.set(&owner, None);
    }

    fn token_id(&self, address: Address) -> Option<TokenId> {
        self.tokens.get(&address)
    }
}
