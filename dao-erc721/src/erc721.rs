use std::borrow::BorrowMut;

use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    Address,
};
use casper_types::{bytesrepr::Bytes, U256};

use crate::{
    core::ERC721Token,
    extensions::{BurnableERC721, MetadataERC721, MintableERC721},
};

use delegate::delegate;

pub type TokenId = U256;
pub type TokenUri = String;

#[casper_contract_interface]
pub trait ERC721Interface {
    fn init(&mut self, name: String, symbol: String);
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn owner_of(&self, token_id: TokenId) -> Address;
    fn balance_of(&self, owner: Address) -> U256;
    fn total_supply(&self) -> U256;
    fn token_uri(&self, token_id: TokenId) -> TokenUri;
    fn base_uri(&self) -> TokenUri;
    fn approve(&mut self, to: Option<Address>, token_id: TokenId);
    fn get_approved(&self, token_id: TokenId) -> Option<Address>;
    fn set_approval_for_all(&mut self, operator: Address, approved: bool);
    fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool;
    fn transfer_from(&mut self, owner: Address, recipient: Address, token_id: TokenId);
    fn safe_transfer_from(
        &mut self,
        owner: Address,
        recipient: Address,
        token_id: TokenId,
        data: Option<Bytes>,
    );
    fn mint(&mut self, to: Address, token_id: TokenId);
    fn burn(&mut self, token_id: TokenId);
}

#[derive(Instance)]
pub struct ERC721 {
    core: ERC721Token,
    metadata: MetadataERC721,
}

impl ERC721Interface for ERC721 {
    delegate! {
        to self.metadata {
            fn init(&mut self, name: String, symbol: String);
            fn name(&self) -> String;
            fn symbol(&self) -> String;
            fn base_uri(&self) -> TokenUri;
        }

        to self.core {
            fn owner_of(&self, token_id: TokenId) -> Address;
            fn balance_of(&self, owner: Address) -> U256;
            fn total_supply(&self) -> U256;
            fn approve(&mut self, to: Option<Address>, token_id: TokenId);
            fn get_approved(&self, token_id: TokenId) -> Option<Address>;
            fn set_approval_for_all(&mut self, operator: Address, approved: bool);
            fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool;
            fn transfer_from(&mut self, owner: Address, recipient: Address, token_id: TokenId);
            fn safe_transfer_from(&mut self, owner: Address, recipient: Address, token_id: TokenId, data: Option<Bytes>);
        }
    }

    fn token_uri(&self, token_id: TokenId) -> TokenUri {
        self.metadata.token_uri(&self.core, token_id)
    }

    fn mint(&mut self, to: Address, token_id: TokenId) {
        MintableERC721::mint(self.core.borrow_mut(), to, token_id);
    }

    fn burn(&mut self, token_id: TokenId) {
        BurnableERC721::burn(self.core.borrow_mut(), token_id);
    }
}
