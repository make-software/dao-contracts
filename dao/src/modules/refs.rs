//! Utility modules providing references to common contracts that are used by most of the voting contracts.
use odra::types::Address;
use odra::{UnwrapOrRevert, contract_env, Variable};

use crate::core_contracts::{
    KycNftContractRef, ReputationContractRef, VaNftContractRef, VariableRepositoryContractRef,
};
use crate::utils::Error;

const KEY_REPO: &[u8] = b"__odra_repository";
const KEY_REPUTATION: &[u8] = b"__odra_reputation";
const KEY_VA: &[u8] = b"__odra_va";
const KEY_KYC: &[u8] = b"__odra_kyc";

/// A module that stores addresses to common voting_contracts that are used by most of the voting voting_contracts.
#[odra::module]
pub struct ContractRefs;

impl ContractRefs {
    pub fn set_variable_repository(&mut self, variable_repository: Address) {
        contract_env::set_var(KEY_REPO, variable_repository);
    }

    pub fn set_reputation_token(&mut self, reputation_token: Address) {
        contract_env::set_var(KEY_REPUTATION, reputation_token);
    }

    pub fn set_va_token(&mut self, va_token: Address) {
        contract_env::set_var(KEY_VA, va_token);
    }

    pub fn set_kyc_token(&mut self, kyc_token: Address) {
        contract_env::set_var(KEY_KYC, kyc_token);
    }

    pub fn variable_repository_address(&self) -> Address {
        contract_env::get_var(KEY_REPO)
            .unwrap_or_revert_with(Error::VariableValueNotSet)
    }

    pub fn reputation_token_address(&self) -> Address {
        contract_env::get_var(KEY_REPUTATION)
            .unwrap_or_revert_with(Error::VariableValueNotSet)
    }

    pub fn va_token_address(&self) -> Address {
        contract_env::get_var(KEY_VA)
            .unwrap_or_revert_with(Error::VariableValueNotSet)
    }

    pub fn kyc_token_address(&self) -> Address {
        contract_env::get_var(KEY_KYC)
            .unwrap_or_revert_with(Error::VariableValueNotSet)
    }

    pub fn variable_repository(&self) -> VariableRepositoryContractRef {
        VariableRepositoryContractRef::at(&self.variable_repository_address())
    }

    pub fn reputation_token(&self) -> ReputationContractRef {
        ReputationContractRef::at(&self.reputation_token_address())
    }

    pub fn va_token(&self) -> VaNftContractRef {
        VaNftContractRef::at(&self.va_token_address())
    }

    pub fn kyc_token(&self) -> KycNftContractRef {
        KycNftContractRef::at(&self.kyc_token_address())
    }
}
