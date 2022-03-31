use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, emit},
    Address, Error, Mapping, Variable,
};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U256,
};

use crate::receiver::{ERC721ReceiverCaller, IERC721Receiver};

use self::events::{Approval, ApprovalForAll, Transfer};

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
    name: Variable<String>,
    symbol: Variable<String>,
    total_supply: Variable<U256>,
    // Mapping owner address to token count
    balances: Mapping<Address, U256>,
    // Mapping from token ID to owner address
    owners: Mapping<U256, Option<Address>>,
    // Mapping from token ID to approved address
    token_approvals: Mapping<U256, Option<Address>>,
    // Mapping from owner to operator approvals
    operator_approvals: Mapping<(Address, Address), bool>,
}

impl ERC721Interface for ERC721 {
    fn init(&mut self, name: String, symbol: String) {
        self.name.set(name);
        self.symbol.set(symbol);
    }

    fn name(&self) -> String {
        self.name.get()
    }

    fn symbol(&self) -> String {
        self.symbol.get()
    }

    fn owner_of(&self, token_id: TokenId) -> Option<Address> {
        self.owners.get(&token_id)
    }

    fn balance_of(&self, owner: Address) -> U256 {
        self.balances.get(&owner)
    }

    fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    fn token_uri(&self, token_id: TokenId) -> TokenUri {
        if !self.exists(&token_id) {
            casper_env::revert(Error::TokenDoesNotExist)
        }
        format!("{}{}", self.base_uri(), token_id)
    }

    fn base_uri(&self) -> TokenUri {
        "ipfs://".to_string()
    }

    fn approve(&mut self, to: Address, token_id: TokenId) {
        let owner = self.owner_of(token_id.clone()).unwrap_or_revert();
        if owner == to {
            casper_env::revert(Error::ApprovalToCurrentOwner);
        }

        let caller = casper_env::caller();
        if caller != owner && !self.is_approved_for_all(owner, caller) {
            casper_env::revert(Error::ApproveCallerIsNotOwnerNorApprovedForAll);
        }

        self.approve(Some(owner), Some(to), token_id);
    }

    fn get_approved(&self, token_id: TokenId) -> Option<Address> {
        if !self.exists(&token_id) {
            casper_env::revert(Error::TokenDoesNotExist)
        }

        self.token_approvals.get(&token_id)
    }

    fn set_approval_for_all(&mut self, operator: Address, approved: bool) {
        let caller = casper_env::caller();
        if caller == operator {
            casper_env::revert(Error::ApproveToCaller)
        }

        self.operator_approvals.set(&(caller, operator), approved);
        emit(ApprovalForAll {
            owner: caller,
            operator,
            approved,
        });
    }

    fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool {
        self.operator_approvals.get(&(owner, operator))
    }

    fn transfer_from(&mut self, owner: Address, recipient: Option<Address>, token_id: TokenId) {
        if !self.is_approved_or_owner(casper_env::caller(), token_id) {
            casper_env::revert(Error::TransferCallerIsNotOwnerNorApproved)
        }
        self.transfer(owner, recipient, token_id);
    }

    fn safe_transfer_from(
        &mut self,
        owner: Address,
        recipient: Option<Address>,
        token_id: TokenId,
    ) {
        self.safe_transfer_from_with_data(
            owner,
            recipient,
            token_id,
            Bytes::from("".to_bytes().unwrap()),
        );
    }

    fn safe_transfer_from_with_data(
        &mut self,
        owner: Address,
        recipient: Option<Address>,
        token_id: TokenId,
        data: Bytes,
    ) {
        if !self.is_approved_or_owner(casper_env::caller(), token_id) {
            casper_env::revert(Error::TransferCallerIsNotOwnerNorApproved)
        }
        self.safe_transfer(owner, recipient, token_id, data);
    }

    fn mint(&mut self, to: Address, token_id: TokenId) {
        if self.exists(&token_id) {
            casper_env::revert(Error::TokenAlreadyExists)
        }

        self.balances.set(&to, self.balances.get(&to) + 1);
        self.total_supply.set(self.total_supply.get() + 1);
        self.owners.set(&token_id, Some(to));

        emit(Transfer {
            from: None,
            to: Some(to),
            token_id,
        });
    }
}

impl ERC721 {
    fn exists(&self, token_id: &TokenId) -> bool {
        self.owners.get(token_id).is_some()
    }

    fn approve(&mut self, owner: Option<Address>, operator: Option<Address>, token_id: TokenId) {
        self.token_approvals.set(&token_id, operator);
        emit(Approval {
            owner,
            operator,
            token_id,
        });
    }

    fn safe_transfer(
        &mut self,
        from: Address,
        to: Option<Address>,
        token_id: TokenId,
        _data: Bytes,
    ) {
        self.transfer(from, to, token_id);
        if !self.check_on_erc721_received(from, to, token_id, _data) {
            casper_env::revert(Error::TransferToNonERC721ReceiverImplementer)
        }
    }

    fn transfer(&mut self, from: Address, to: Option<Address>, token_id: TokenId) {
        let owner = self.owner_of(token_id.clone());
        if let Some(owner_address) = owner {
            if owner_address != from {
                casper_env::revert(Error::TransferFromIncorrectOwner)
            }
        }
        if to.is_none() {
            casper_env::revert(Error::TransferToNone)
        }

        // Clear approvals from the previous owner
        self.approve(owner, None, token_id);

        let to = to.unwrap();

        self.balances.set(&from, self.balances.get(&from) - 1);
        self.balances.set(&to, self.balances.get(&to) + 1);
        self.owners.set(&token_id, Some(to.clone()));

        emit(Transfer {
            from: Some(from.clone()),
            to: Some(to),
            token_id,
        });
    }

    fn is_approved_or_owner(&mut self, spender: Address, token_id: TokenId) -> bool {
        if !self.exists(&token_id) {
            casper_env::revert(Error::TokenDoesNotExist)
        }
        let owner = self.owner_of(token_id);
        spender == owner.unwrap()
            || self.is_approved_for_all(owner.unwrap(), spender)
            || self.get_approved(token_id) == Some(spender)
    }

    fn check_on_erc721_received(
        &self,
        from: Address,
        to: Option<Address>,
        token_id: TokenId,
        data: Bytes,
    ) -> bool {
        match to {
            Some(to_address) => match to_address.as_contract_package_hash() {
                Some(to_contract) => {
                    let mut caller = ERC721ReceiverCaller::at(to_contract.clone());
                    caller.on_erc_721_received(casper_env::caller(), from, token_id, data);
                    true
                }
                None => true,
            },
            None => false,
        }
    }
}

pub mod events {
    use casper_dao_utils::{casper_dao_macros::Event, Address};

    use crate::TokenId;

    #[derive(Debug, PartialEq, Event)]
    pub struct Transfer {
        pub from: Option<Address>,
        pub to: Option<Address>,
        pub token_id: TokenId,
    }

    #[derive(Debug, PartialEq, Event)]
    pub struct Approval {
        pub owner: Option<Address>,
        pub operator: Option<Address>,
        pub token_id: TokenId,
    }

    #[derive(Debug, PartialEq, Event)]
    pub struct ApprovalForAll {
        pub owner: Address,
        pub operator: Address,
        pub approved: bool,
    }
}
