use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    Address,
    Error,
    Mapping,
    Variable,
};

use crate::{KycNftContractCaller, KycNftContractInterface};

/// A utility module that provides information about the current status of the KYC process.
#[derive(Instance)]
pub struct KycInfo {
    kyc_token: Variable<Address>,
    votings: Mapping<Address, bool>,
}

impl KycInfo {
    /// Initializes `kyc_token` contract address.
    pub fn init(&mut self, kyc_token: Address) {
        self.kyc_token.set(kyc_token);
    }

    /// Returns the `address` of kyc token contract.
    ///
    /// If the variable is not initialized, reverts with [VariableValueNotSet](Error::VariableValueNotSet)
    pub fn get_kyc_token_address(&self) -> Address {
        self.kyc_token
            .get()
            .unwrap_or_revert_with(Error::VariableValueNotSet)
    }

    /// Returns true if the `address` has a non-zero balance of kyc token, false otherwise.
    pub fn is_kycd(&self, &address: &Address) -> bool {
        !KycNftContractCaller::at(self.get_kyc_token_address())
            .balance_of(address)
            .is_zero()
    }

    /// Sets a flag indicating there is ongoing voting for the given `address`.
    pub(crate) fn set_voting(&self, address: &Address) {
        self.votings.set(address, true);
    }

    /// Clears the flag indicating there is ongoing voting for the given `address`.
    pub(crate) fn clear_voting(&self, address: &Address) {
        self.votings.set(address, false);
    }

    /// Indicates whether there is ongoing voting for the given `address`.
    pub(crate) fn exists_ongoing_voting(&self, address: &Address) -> bool {
        self.votings.get(address).unwrap_or(false)
    }
}
