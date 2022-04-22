use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert, casper_dao_macros::Instance, Address, Error,
    Mapping, Variable,
};
use casper_types::U256;

use crate::{DaoOwnedNftContractCaller, DaoOwnedNftContractInterface};

use super::VotingId;

#[derive(Instance)]
pub struct KycInfo {
    kyc_token: Variable<Option<Address>>,
    votings: Mapping<Address, bool>,
    documents: Mapping<VotingId, String>,
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
        DaoOwnedNftContractCaller::at(self.get_kyc_token_address()).balance_of(address)
            > U256::zero()
    }

    pub(crate) fn set_voting(&self, address: &Address) {
        self.votings.set(address, true);
    }

    pub(crate) fn clear_voting(&self, address: &Address) {
        self.votings.set(address, false);
    }

    pub(crate) fn exists_ongoing_voting(&self, address: &Address) -> bool {
        self.votings.get(address)
    }

    pub(crate) fn set_document_hash(&mut self, voting_id: VotingId, document_hash: String) {
        self.documents.set(&voting_id, document_hash);
    }

    pub(crate) fn get_document_hash(&self, voting_id: VotingId) -> Option<String> {
        self.documents.get_or_none(&voting_id)
    }
}
