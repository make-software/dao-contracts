use casper_dao_utils::{casper_dao_macros::Instance, Address, Mapping};

use crate::{refs::ContractRefsWithKycStorage, KycNftContractInterface};

/// A utility module that provides information about the current status of the KYC process.
#[derive(Instance)]
pub struct KycInfo {
    #[scoped = "contract"]
    refs: ContractRefsWithKycStorage,
    votings: Mapping<Address, bool>,
}

impl KycInfo {
    /// Returns true if the `address` has a non-zero balance of kyc token, false otherwise.
    pub fn is_kycd(&self, &address: &Address) -> bool {
        !self.refs.kyc_token().balance_of(address).is_zero()
    }

    /// Sets a flag indicating there is ongoing voting for the given `address`.
    pub fn set_voting(&self, address: &Address) {
        self.votings.set(address, true);
    }

    /// Clears the flag indicating there is ongoing voting for the given `address`.
    pub fn clear_voting(&self, address: &Address) {
        self.votings.set(address, false);
    }

    /// Indicates whether there is ongoing voting for the given `address`.
    pub fn exists_ongoing_voting(&self, address: &Address) -> bool {
        self.votings.get(address).unwrap_or(false)
    }
}
