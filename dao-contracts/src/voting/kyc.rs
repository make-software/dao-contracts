use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert, casper_dao_macros::Instance, Address, Error,
    Variable,
};
use casper_types::U256;

use crate::{DaoOwnedNftContractCaller, DaoOwnedNftContractInterface};

#[derive(Instance)]
pub struct KycInfo {
    kyc_token: Variable<Option<Address>>,
}

impl KycInfo {
    pub fn init(&mut self, kyc_token: Address) {
        self.kyc_token.set(Some(kyc_token));
    }

    pub fn get_kyc_token_address(&self) -> Address {
        self.kyc_token
            .get()
            .unwrap_or_revert_with(Error::VariableValueNotSet)
    }

    pub fn is_kycd(&self, &address: &Address) -> bool {
        let va_token_contract = DaoOwnedNftContractCaller::at(self.get_kyc_token_address());
        va_token_contract.balance_of(address) > U256::zero()
    }
}
