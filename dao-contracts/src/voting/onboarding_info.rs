use casper_dao_erc721::TokenId;
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    Address,
    Error,
};

use crate::{
    refs::{ContractRefs, ContractRefsWithKycStorage},
    va_nft::VaNftContractInterface,
};

/// A utility module that provides information about the current status of the onboarding process.
#[derive(Instance)]
pub struct OnboardingInfo {
    #[scoped = "contract"]
    refs: ContractRefsWithKycStorage,
}

impl OnboardingInfo {
    /// Returns true if the `address` has a non-zero balance of va token, false otherwise.
    pub fn is_onboarded(&self, &address: &Address) -> bool {
        !self.refs.va_token().balance_of(address).is_zero()
    }

    /// Returns the `token id` of the `address`.
    ///
    /// If the `address` does not own any token, reverts with [`InvalidTokenOwner`](Error::InvalidTokenOwner) error.
    pub fn token_id_of(&self, address: &Address) -> TokenId {
        self.refs
            .va_token()
            .token_id(*address)
            .unwrap_or_revert_with(Error::InvalidTokenOwner)
    }

    /// Returns the owner of a token with the given id.
    ///
    /// If the `token_id` does not have an owner, None value is return.
    pub fn owner_of(&self, token_id: TokenId) -> Option<Address> {
        self.refs.va_token().owner_of(token_id)
    }
}
