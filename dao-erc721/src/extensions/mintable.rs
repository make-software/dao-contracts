use casper_dao_utils::{
    casper_env::{self, emit},
    Address,
    Error,
};

use crate::{core::ERC721Token, events::Transfer, TokenId};

/// Adds mint functionality to [`ERC721Token`].
pub struct MintableERC721;

impl MintableERC721 {
    pub fn mint(erc721: &mut ERC721Token, to: Address, token_id: TokenId) {
        if erc721.exists(&token_id) {
            casper_env::revert(Error::TokenAlreadyExists)
        }

        erc721.increment_balance(to);
        erc721.increment_total_supply();
        erc721.set_owner_of(token_id, Some(to));

        emit(Transfer {
            from: None,
            to: Some(to),
            token_id,
        });
    }
}
