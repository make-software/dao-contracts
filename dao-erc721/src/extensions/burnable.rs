use casper_dao_utils::{
    casper_env::{self, emit},
    Error,
};

use crate::{core::ERC721Token, events::Transfer, TokenId};

pub struct BurnableERC721 {}

impl BurnableERC721 {
    pub fn burn(erc721: &mut ERC721Token, token_id: TokenId) {
        if !erc721.is_approved_or_owner(casper_env::caller(), token_id) {
            casper_env::revert(Error::CallerIsNotOwnerNorApproved);
        }

        let owner = erc721.owner_of(token_id);

        erc721.approve(None, token_id);
        erc721.decrement_balance(owner);
        erc721.set_owner_of(token_id, None);

        emit(Transfer {
            from: Some(owner),
            to: None,
            token_id,
        });
    }
}
