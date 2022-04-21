use casper_dao_erc721::TokenId;
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{CLTyped, FromBytes, Instance, ToBytes},
    Address, Mapping, Variable,
};
use casper_types::U256;

use crate::{DaoOwnedNftContractCaller, DaoOwnedNftContractInterface};

#[derive(Instance)]
pub struct OnboardingInfo {
    va_token: Variable<Option<Address>>,
    votings: Mapping<Address, bool>,
}

// TODO: replace unwrap_or_revert() with a custom Error
impl OnboardingInfo {
    pub fn init(&mut self, va_token: Address) {
        self.va_token.set(Some(va_token));
    }

    pub fn get_va_token_address(&self) -> Address {
        self.va_token.get().unwrap_or_revert()
    }

    pub fn set_voting(&mut self, address: &Address) {
        self.votings.set(address, true);
    }

    pub fn clear_voting(&mut self, address: &Address) {
        self.votings.set(address, false);
    }

    pub fn exists_ongoing_voting(&self, address: &Address) -> bool {
        self.votings.get(address)
    }

    pub fn is_onboarded(&self, &address: &Address) -> bool {
        let va_token_contract = DaoOwnedNftContractCaller::at(self.get_va_token_address());
        va_token_contract.balance_of(address) > U256::zero()
    }

    pub fn token_id_of(&self, address: &Address) -> Option<TokenId> {
        let va_token_contract = DaoOwnedNftContractCaller::at(self.get_va_token_address());
        va_token_contract.token_id(*address)
    }

    pub fn owner_of(&self, token_id: TokenId) -> Address {
        let va_token_contract = DaoOwnedNftContractCaller::at(self.get_va_token_address());
        va_token_contract.owner_of(token_id)
    }
}

#[derive(ToBytes, FromBytes, CLTyped)]
pub enum Action {
    Add,
    Remove,
}
