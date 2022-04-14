use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert, casper_dao_macros::Instance, Address,
    Variable,
};

#[derive(Instance)]
pub struct OnboardingContractStorage {
    kyc_token: Variable<Option<Address>>,
    va_token: Variable<Option<Address>>,
}

// TODO: replace unwrap_or_revert() with a custom Error
impl OnboardingContractStorage {
    pub fn init(&mut self, kyc_token: Address, va_token: Address) {
        self.kyc_token.set(Some(kyc_token));
        self.va_token.set(Some(va_token));
    }

    pub fn get_kyc_token_address(&self) -> Address {
        self.kyc_token.get().unwrap_or_revert()
    }

    pub fn get_va_token_address(&self) -> Address {
        self.va_token.get().unwrap_or_revert()
    }
}
