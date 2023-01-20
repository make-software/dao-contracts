use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    casper_env::{self, emit},
    Address,
    Error,
    Mapping,
    Variable,
};
use casper_types::{bytesrepr::Bytes, U512};

use crate::{
    events::{Approval, ApprovalForAll, Transfer},
    receiver::{ERC721ReceiverCaller, IERC721Receiver},
    TokenId,
};

/// A module implementing ERC721 standard interface. 
#[derive(Instance)]
pub struct ERC721Token {
    total_supply: Variable<U512>,
    // Mapping owner address to token count
    balances: Mapping<Address, U512>,
    // Mapping from token ID to owner address
    owners: Mapping<U512, Option<Address>>,
    // Mapping from token ID to approved address
    token_approvals: Mapping<U512, Option<Address>>,
    // Mapping from owner to operator approvals
    operator_approvals: Mapping<(Address, Address), bool>,
}

impl ERC721Token {
    /// Finds the owner of a token with the given id.
    /// 
    /// # Errors
    /// Reverts with [`Error::TokenDoesNotExist`] if token with the given id does not exists.
    pub fn owner_of(&self, token_id: TokenId) -> Option<Address> {
        if !self.exists(&token_id) {
            casper_env::revert(Error::TokenDoesNotExist)
        }
        self.owners.get(&token_id).unwrap_or(None)
    }

    /// Counts all tokens assigned to an owner.
    pub fn balance_of(&self, owner: Address) -> U512 {
        self.balances.get(&owner).unwrap_or_default()
    }

    /// Return the total number of tokens.
    pub fn total_supply(&self) -> U512 {
        self.total_supply.get().unwrap_or_default()
    }

    /// Sets or revokes the approved address for a token.
    /// 
    /// # Errors
    /// Reverts with [`Error::TokenDoesNotExist`]  if token with the given id does not exists.
    /// Reverts with [`Error::ApprovalToCurrentOwner`] to the approved address is the owner.
    /// Reverts with [`Error::ApproveCallerIsNotOwnerNorApprovedForAll`] if the caller is neither the owner not approved for all.
    /// See [ERC721Token::set_approval_for_all()].
    pub fn approve(&mut self, approved: Option<Address>, token_id: TokenId) {
        let owner = self.owner_of_or_revert(token_id);
        if Some(owner) == approved {
            casper_env::revert(Error::ApprovalToCurrentOwner);
        }
        let caller = casper_env::caller();
        if caller != owner && !self.is_approved_for_all(owner, caller) {
            casper_env::revert(Error::ApproveCallerIsNotOwnerNorApprovedForAll);
        }

        self.approve_owner(Some(owner), approved, token_id);
    }

    /// Gets the approved address for a token.
    pub fn get_approved(&self, token_id: TokenId) -> Option<Address> {
        if !self.exists(&token_id) {
            casper_env::revert(Error::TokenDoesNotExist)
        }

        self.token_approvals.get(&token_id).unwrap_or(None)
    }

    /// Enables or disables approval for a third party ("operator") to manage all of caller's tokens
    /// 
    /// # Errors
    /// Reverts with [`Error::ApproveToCaller`] the caller tries to approve himself.
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

    /// Checks if an address is an authorized operator for another address
    pub fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool {
        self.operator_approvals
            .get(&(owner, operator))
            .unwrap_or(false)
    }

    /// Transfers the ownership of a token from one address to another address.
    /// 
    /// # Errors
    /// The caller must be an owner or be an approved address, otherwise the contract reverts with
    /// [`Error::CallerIsNotOwnerNorApproved`].
    pub fn transfer_from(&mut self, owner: Address, recipient: Address, token_id: TokenId) {
        if !self.is_approved_or_owner(casper_env::caller(), token_id) {
            casper_env::revert(Error::CallerIsNotOwnerNorApproved)
        }
        self.transfer(owner, recipient, token_id);
    }

    /// Transfers the ownership of a token from one address to another address.
    /// 
    /// Verifies whether the recipient is a smart contract that implements [`ERC721Receiver`](crate::receiver::IERC721Receiver).
    /// 
    /// # Errors
    /// The caller must be an owner or be an approved address, otherwise the contract reverts with
    /// [`Error::CallerIsNotOwnerNorApproved`].
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
    pub fn owner_of_or_revert(&self, token_id: TokenId) -> Address {
        if !self.exists(&token_id) {
            casper_env::revert(Error::TokenDoesNotExist)
        }
        match self
            .owners
            .get(&token_id)
            .unwrap_or_revert_with(Error::InvalidTokenOwner)
        {
            Some(owner) => owner,
            None => casper_env::revert(Error::InvalidTokenOwner),
        }
    }

    pub fn increment_balance(&mut self, owner: Address) {
        self.balances.set(&owner, self.balance_of(owner) + 1);
    }

    pub fn decrement_balance(&mut self, owner: Address) {
        self.balances.set(&owner, self.balance_of(owner) - 1);
    }

    pub fn increment_total_supply(&mut self) {
        self.total_supply.set(self.total_supply() + 1);
    }

    pub fn decrement_total_supply(&mut self) {
        self.total_supply.set(self.total_supply() - 1);
    }

    pub fn exists(&self, token_id: &TokenId) -> bool {
        self.owners.get(token_id).is_some()
    }

    pub fn set_owner_of(&mut self, token_id: TokenId, owner: Option<Address>) {
        self.owners.set(&token_id, owner);
    }

    pub fn approve_owner(
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
        let owner = self.owner_of_or_revert(token_id);
        if owner != from {
            casper_env::revert(Error::TransferFromIncorrectOwner)
        }

        // Clear approvals from the previous owner
        self.approve_owner(Some(owner), None, token_id);

        self.balances.set(&from, self.balance_of(from) - 1);
        self.balances.set(&to, self.balance_of(to) + 1);
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
        let owner = self.owner_of_or_revert(token_id);
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
