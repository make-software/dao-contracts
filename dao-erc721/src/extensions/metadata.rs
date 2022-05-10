use casper_dao_utils::{casper_dao_macros::Instance, casper_env, Error, Variable};

use crate::{core::ERC721Token, TokenId, TokenUri};

#[derive(Instance)]
pub struct MetadataERC721 {
    name: Variable<String>,
    symbol: Variable<String>,
    base_uri: Variable<String>,
}

impl MetadataERC721 {
    pub fn init(&mut self, name: String, symbol: String, base_uri: String) {
        self.name.set(name);
        self.symbol.set(symbol);
        self.base_uri.set(base_uri);
    }

    pub fn name(&self) -> String {
        self.name.get_or_revert()
    }

    pub fn symbol(&self) -> String {
        self.symbol.get_or_revert()
    }

    pub fn token_uri(&self, erc721: &ERC721Token, token_id: TokenId) -> TokenUri {
        if !erc721.exists(&token_id) {
            casper_env::revert(Error::TokenDoesNotExist)
        }
        format!("{}{}", self.base_uri(), token_id)
    }

    pub fn base_uri(&self) -> TokenUri {
        self.base_uri.get_or_revert()
    }
}
