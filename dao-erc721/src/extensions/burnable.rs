use casper_dao_utils::casper_dao_macros::Instance;

use crate::{ERC721Interface, TokenId, ERC721};

#[derive(Instance)]
struct BurnableERC721 {}

impl BurnableERC721 {
    fn burn(&mut self, erc721: ERC721, token_id: TokenId) {
        let owner = erc721.owner_of(token_id);

        // // Clear approvals
        // _approve(address(0), tokenId);
        // erc721.approve(owner, operator, token_id)
        // _balances[owner] -= 1;
        // delete _owners[tokenId];

        // emit Transfer(owner, address(0), tokenId);

        // _afterTokenTransfer(owner, address(0), tokenId);
    }
}
