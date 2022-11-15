use casper_dao_erc721::TokenId;
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{CLTyped, FromBytes, Instance, ToBytes},
    Address,
    Error,
    Mapping,
    Variable,
};

use crate::{VaNftContractCaller, VaNftContractInterface};

/// A utility module that provides information about the current status of the onboarding process.
#[derive(Instance)]
pub struct OnboardingInfo {
    va_token: Variable<Address>,
    votings: Mapping<Address, bool>,
}

impl OnboardingInfo {
    /// Initializes `va_token` contract address.
    pub fn init(&mut self, va_token: Address) {
        self.va_token.set(va_token);
    }

    /// Returns the `address` of kyc token contract.
    ///
    /// If the variable is not initialized, reverts with [VariableValueNotSet](Error::VariableValueNotSet)
    pub fn get_va_token_address(&self) -> Address {
        self.va_token
            .get()
            .unwrap_or_revert_with(Error::VariableValueNotSet)
    }

    /// Sets a flag indicating there is ongoing voting for the given `address`.
    pub fn set_voting(&mut self, address: &Address) {
        self.votings.set(address, true);
    }

    /// Clears the flag indicating there is ongoing voting for the given `address`.
    pub fn clear_voting(&mut self, address: &Address) {
        self.votings.set(address, false);
    }

    /// Indicates whether there is ongoing voting for the given `address`.
    pub fn exists_ongoing_voting(&self, address: &Address) -> bool {
        self.votings.get(address).unwrap_or(false)
    }

    /// Returns true if the `address` has a non-zero balance of va token, false otherwise.
    pub fn is_onboarded(&self, &address: &Address) -> bool {
        !self.va_nft_contract().balance_of(address).is_zero()
    }

    /// Returns the `token id` of the `address`.
    ///
    /// If the `address` does not own any token, reverts with [`InvalidTokenOwner`](Error:InvalidTokenOwner) error.
    pub fn token_id_of(&self, address: &Address) -> TokenId {
        self.va_nft_contract()
            .token_id(*address)
            .unwrap_or_revert_with(Error::InvalidTokenOwner)
    }

    /// Returns the owner of a token with the given id.
    ///
    /// If the `token_id` does not have an owner, None value is return.
    pub fn owner_of(&self, token_id: TokenId) -> Option<Address> {
        self.va_nft_contract().owner_of(token_id)
    }

    fn va_nft_contract(&self) -> VaNftContractCaller {
        VaNftContractCaller::at(self.get_va_token_address())
    }
}

#[derive(ToBytes, FromBytes, CLTyped)]
pub enum OnboardingAction {
    Add,
    Remove,
}
