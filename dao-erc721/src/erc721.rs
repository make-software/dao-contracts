use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    Address,
};
use casper_types::{bytesrepr::Bytes, U256};

use crate::{
    core::ERC721Token,
    extensions::{MetadataERC721, MintableERC721},
};

pub type TokenId = U256;
pub type TokenUri = String;

#[casper_contract_interface]
pub trait ERC721Interface {
    fn init(&mut self, name: String, symbol: String);
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn owner_of(&self, token_id: TokenId) -> Option<Address>;
    fn balance_of(&self, owner: Address) -> U256;
    fn total_supply(&self) -> U256;
    fn token_uri(&self, token_id: TokenId) -> TokenUri;
    fn base_uri(&self) -> TokenUri;
    fn approve(&mut self, to: Address, token_id: TokenId);
    fn get_approved(&self, token_id: TokenId) -> Option<Address>;
    fn set_approval_for_all(&mut self, operator: Address, approved: bool);
    fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool;
    fn transfer_from(&mut self, owner: Address, recipient: Option<Address>, token_id: TokenId);
    fn safe_transfer_from(&mut self, owner: Address, recipient: Option<Address>, token_id: TokenId);
    fn safe_transfer_from_with_data(
        &mut self,
        owner: Address,
        recipient: Option<Address>,
        token_id: TokenId,
        data: Bytes,
    );
    fn mint(&mut self, to: Address, token_id: TokenId);
}

#[derive(Instance)]
pub struct ERC721 {
    core: ERC721Token,
    metadata: MetadataERC721,
    mintable: MintableERC721,
}

impl ERC721Interface for ERC721 {
    fn init(&mut self, name: String, symbol: String) {
        self.metadata.set_name(name);
        self.metadata.set_symbol(symbol);
    }

    fn name(&self) -> String {
        self.metadata.name()
    }

    fn symbol(&self) -> String {
        self.metadata.symbol()
    }

    fn owner_of(&self, token_id: TokenId) -> Option<Address> {
        self.core.owner_of(token_id)
    }

    fn balance_of(&self, owner: Address) -> U256 {
        self.core.balance_of(owner)
    }

    fn total_supply(&self) -> U256 {
        self.core.total_supply()
    }

    fn token_uri(&self, token_id: TokenId) -> TokenUri {
        self.core.token_uri(token_id)
    }

    fn base_uri(&self) -> TokenUri {
        self.core.base_uri()
    }

    fn approve(&mut self, to: Address, token_id: TokenId) {
        self.core.approve(to, token_id)
    }

    fn get_approved(&self, token_id: TokenId) -> Option<Address> {
        self.core.get_approved(token_id)
    }

    fn set_approval_for_all(&mut self, operator: Address, approved: bool) {
        self.core.set_approval_for_all(operator, approved)
    }

    fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool {
        self.core.is_approved_for_all(owner, operator)
    }

    fn transfer_from(&mut self, owner: Address, recipient: Option<Address>, token_id: TokenId) {
        self.core.transfer_from(owner, recipient, token_id)
    }

    fn safe_transfer_from(
        &mut self,
        owner: Address,
        recipient: Option<Address>,
        token_id: TokenId,
    ) {
        self.core.safe_transfer_from(owner, recipient, token_id)
    }

    fn safe_transfer_from_with_data(
        &mut self,
        owner: Address,
        recipient: Option<Address>,
        token_id: TokenId,
        data: Bytes,
    ) {
        self.core
            .safe_transfer_from_with_data(owner, recipient, token_id, data)
    }

    fn mint(&mut self, to: Address, token_id: TokenId) {
        self.mintable.mint(&mut self.core, to, token_id);
    }
}
