use casper_dao_utils::{casper_dao_macros::Instance, Address, Variable};
use delegate::delegate;

use crate::{
    kyc_nft::KycNftContractCaller,
    reputation::ReputationContractCaller,
    va_nft::VaNftContractCaller,
    variable_repository::VariableRepositoryContractCaller,
};

/// Provides references to common contracts that are used by most of the voting contracts.
pub trait ContractRefs {
    /// Returns a reference to [Reputation Token](crate::ReputationContract) connected to the contract
    fn reputation_token(&self) -> ReputationContractCaller;
    /// Returns a reference to [Variable Repository](crate::VariableRepositoryContract) connected to the contract
    fn variable_repository(&self) -> VariableRepositoryContractCaller;
    /// Returns a reference to [VA Token](crate::VaNftContract) connected to the contract
    fn va_token(&self) -> VaNftContractCaller;
}

/// A module that stores addresses to common contracts that are used by most of the voting contracts.
#[derive(Instance)]
pub struct ContractRefsStorage {
    variable_repository: Variable<Address>,
    reputation_token: Variable<Address>,
    va_token: Variable<Address>,
}

impl ContractRefsStorage {
    pub fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        va_token: Address,
    ) {
        self.variable_repository.set(variable_repository);
        self.reputation_token.set(reputation_token);
        self.va_token.set(va_token);
    }

    /// Returns the address of [Reputation Token](crate::ReputationContract) contract.
    pub fn reputation_token_address(&self) -> Address {
        self.reputation_token.get_or_revert()
    }

    /// Returns the address of [Variable Repository](crate::VariableRepositoryContract) contract.
    pub fn variable_repository_address(&self) -> Address {
        self.variable_repository.get_or_revert()
    }
}

impl ContractRefs for ContractRefsStorage {
    fn reputation_token(&self) -> ReputationContractCaller {
        ReputationContractCaller::at(self.reputation_token.get_or_revert())
    }

    fn variable_repository(&self) -> VariableRepositoryContractCaller {
        VariableRepositoryContractCaller::at(self.variable_repository.get_or_revert())
    }

    fn va_token(&self) -> VaNftContractCaller {
        VaNftContractCaller::at(self.va_token.get_or_revert())
    }
}

/// A decorated [ContractRefsStorage] module that additionally stores addresses an address of [KYC Token](crate::KycNftContract).
#[derive(Instance)]
pub struct ContractRefsWithKycStorage {
    #[scoped = "contract"]
    refs: ContractRefsStorage,
    kyc_token: Variable<Address>,
}

impl ContractRefsWithKycStorage {
    delegate! {
        to self.refs {
            pub fn reputation_token_address(&self) -> Address;
            pub fn variable_repository_address(&self) -> Address;
        }
    }

    pub fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        va_token: Address,
        kyc_token: Address,
    ) {
        self.refs
            .init(variable_repository, reputation_token, va_token);
        self.kyc_token.set(kyc_token);
    }

    /// Returns a reference to [KYC Token](crate::KycNftContract) connected to the contract
    pub fn kyc_token(&self) -> KycNftContractCaller {
        KycNftContractCaller::at(self.kyc_token.get_or_revert())
    }

    /// Returns the address of [KYC Token](crate::KycNftContract) connected to the contract
    pub fn kyc_token_address(&self) -> Address {
        self.kyc_token.get_or_revert()
    }
}

impl ContractRefs for ContractRefsWithKycStorage {
    delegate! {
        to self.refs {
            fn reputation_token(&self) -> ReputationContractCaller;
            fn variable_repository(&self) -> VariableRepositoryContractCaller;
            fn va_token(&self) -> VaNftContractCaller;
        }
    }
}
