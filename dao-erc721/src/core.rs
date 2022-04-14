use casper_dao_utils::{
    casper_dao_macros::Instance,
    casper_env::{self, emit},
    Address, Error, Mapping, Variable,
};
use casper_types::{bytesrepr::Bytes, U256};

use crate::{
    events::{Approval, ApprovalForAll, Transfer},
    receiver::{ERC721ReceiverCaller, IERC721Receiver},
    TokenId,
};

#[derive(Instance)]
pub struct ERC721Token {
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

impl ERC721Token {
    pub fn owner_of(&self, token_id: TokenId) -> Address {
        if !self.exists(&token_id) {
            casper_env::revert(Error::TokenDoesNotExist)
        }
        match self.owners.get(&token_id) {
            Some(owner) => owner,
            None => casper_env::revert(Error::InvalidTokenOwner),
        }
    }

    pub fn balance_of(&self, owner: Address) -> U256 {
        self.balances.get(&owner)
    }

    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    pub fn approve(&mut self, approved: Option<Address>, token_id: TokenId) {
        let owner = self.owner_of(token_id);
        if Some(owner) == approved {
            casper_env::revert(Error::ApprovalToCurrentOwner);
        }
        let caller = casper_env::caller();
        if caller != owner && !self.is_approved_for_all(owner, caller) {
            casper_env::revert(Error::ApproveCallerIsNotOwnerNorApprovedForAll);
        }

        self.approve_owner(Some(owner), approved, token_id);
    }

    pub fn get_approved(&self, token_id: TokenId) -> Option<Address> {
        if !self.exists(&token_id) {
            casper_env::revert(Error::TokenDoesNotExist)
        }

        self.token_approvals.get(&token_id)
    }

    pub fn set_approval_for_all(&mut self, operator: Address, approved: bool) {
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

    pub fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool {
        self.operator_approvals.get(&(owner, operator))
    }

    pub fn transfer_from(&mut self, owner: Address, recipient: Address, token_id: TokenId) {
        if !self.is_approved_or_owner(casper_env::caller(), token_id) {
            casper_env::revert(Error::CallerIsNotOwnerNorApproved)
        }
        self.transfer(owner, recipient, token_id);
    }

    pub fn safe_transfer_from(
        &mut self,
        owner: Address,
        recipient: Address,
        token_id: TokenId,
        data: Option<Bytes>,
    ) {
        if !self.is_approved_or_owner(casper_env::caller(), token_id) {
            casper_env::revert(Error::CallerIsNotOwnerNorApproved)
        }
        self.safe_transfer(owner, recipient, token_id, data);
    }
}

impl ERC721Token {
    pub fn increment_balance(&mut self, owner: Address) {
        self.balances.set(&owner, self.balance_of(owner) + 1);
    }

    pub fn decrement_balance(&mut self, owner: Address) {
        self.balances.set(&owner, self.balance_of(owner) - 1);
    }

    pub fn increment_total_supply(&mut self) {
        self.total_supply.set(self.total_supply() + 1);
    }

    pub fn exists(&self, token_id: &TokenId) -> bool {
        self.owners.get(token_id).is_some()
    }

    pub fn set_owner_of(&mut self, token_id: TokenId, owner: Option<Address>) {
        self.owners.set(&token_id, owner);
    }

    fn approve_owner(
        &mut self,
        owner: Option<Address>,
        approved: Option<Address>,
        token_id: TokenId,
    ) {
        self.token_approvals.set(&token_id, approved);
        emit(Approval {
            owner,
            approved,
            token_id,
        });
    }

    fn safe_transfer(
        &mut self,
        from: Address,
        to: Address,
        token_id: TokenId,
        data: Option<Bytes>,
    ) {
        self.transfer(from, to, token_id);
        if !self.check_on_erc721_received(from, to, token_id, data) {
            casper_env::revert(Error::TransferToNonERC721ReceiverImplementer)
        }
    }

    fn transfer(&mut self, from: Address, to: Address, token_id: TokenId) {
        let owner = self.owner_of(token_id);
        if owner != from {
            casper_env::revert(Error::TransferFromIncorrectOwner)
        }

        // Clear approvals from the previous owner
        self.approve_owner(Some(owner), None, token_id);

        self.balances.set(&from, self.balances.get(&from) - 1);
        self.balances.set(&to, self.balances.get(&to) + 1);
        self.owners.set(&token_id, Some(to));

        emit(Transfer {
            from: Some(from),
            to: Some(to),
            token_id,
        });
    }

    pub(crate) fn is_approved_or_owner(&mut self, approved: Address, token_id: TokenId) -> bool {
        if !self.exists(&token_id) {
            casper_env::revert(Error::TokenDoesNotExist)
        }
        let owner = self.owner_of(token_id);
        approved == owner
            || self.is_approved_for_all(owner, approved)
            || self.get_approved(token_id) == Some(approved)
    }

    fn check_on_erc721_received(
        &self,
        from: Address,
        to: Address,
        token_id: TokenId,
        data: Option<Bytes>,
    ) -> bool {
        match to.as_contract_package_hash() {
            Some(to_contract) => {
                let caller = ERC721ReceiverCaller::at(*to_contract);
                caller.on_erc_721_received(casper_env::caller(), from, token_id, data);
                true
            }
            None => true,
        }
    }
}
