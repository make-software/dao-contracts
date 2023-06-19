use crate::core_contracts::TokenId;
use crate::utils::Error;
use odra::types::Address;
use odra::UnwrapOrRevert;

use super::refs::ContractRefs;

/// A utility module that provides information about the current status of the onboarding process.
#[odra::module]
pub struct OnboardingInfo {
    refs: ContractRefs,
}

impl OnboardingInfo {
    /// Returns true if the `address` has a non-zero balance of va token, false otherwise.
    pub fn is_onboarded(&self, address: &Address) -> bool {
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
    pub fn owner_of(&self, token_id: &TokenId) -> Address {
        self.refs.va_token().owner_of(token_id)
    }
}
