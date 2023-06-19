//! Utility modules providing references to common contracts that are used by most of the voting contracts.
use odra::types::Address;
use odra::{UnwrapOrRevert, Variable};

use crate::core_contracts::{
    KycNftContractRef, ReputationContractRef, VaNftContractRef, VariableRepositoryContractRef,
};
use crate::utils::Error;

// /// A module that stores addresses to common voting_contracts that are used by most of the voting voting_contracts.
// #[odra::module]
// pub struct ContractRefsStorage {
//     variable_repository: Variable<Address>,
//     reputation_token: Variable<Address>,
//     va_token: Variable<Address>,
// }

// #[odra::module]
// impl ContractRefsStorage {
//     pub fn init(
//         &mut self,
//         variable_repository: Address,
//         reputation_token: Address,
//         va_token: Address,
//     ) {
//         self.variable_repository.set(variable_repository);
//         self.reputation_token.set(reputation_token);
//         self.va_token.set(va_token);
//     }

//     /// Returns the address of [Reputation Token](crate::core_contracts::ReputationContract) contract.
//     pub fn reputation_token_address(&self) -> Address {
//         self.reputation_token
//             .get()
//             .unwrap_or_revert_with(Error::VariableValueNotSet)
//     }

//     /// Returns the address of [Variable Repository](crate::core_contracts::VariableRepositoryContract) contract.
//     pub fn variable_repository_address(&self) -> Address {
//         self.variable_repository
//             .get()
//             .unwrap_or_revert_with(Error::VariableValueNotSet)
//     }

//     /// Returns the address of [VA Token](crate::core_contracts::VaNftContract) contract.
//     pub fn va_token_address(&self) -> Address {
//         self.va_token
//             .get()
//             .unwrap_or_revert_with(Error::VariableValueNotSet)
//     }
// }

// impl ContractRefsStorage {
//     /// Returns the Ref of [Reputation Token](crate::core_contracts::ReputationContract) contract.
//     pub fn reputation_token(&self) -> ReputationContractRef {
//         ReputationContractRef::at(&self.reputation_token_address())
//     }

//     /// Returns the Ref of [Variable Repository](crate::core_contracts::VariableRepositoryContract) contract.
//     pub fn variable_repository(&self) -> VariableRepositoryContractRef {
//         VariableRepositoryContractRef::at(
//             &self
//                 .variable_repository
//                 .get()
//                 .unwrap_or_revert_with(Error::VariableValueNotSet),
//         )
//     }

//     /// Returns the Ref of [VA Token](crate::core_contracts::VaNftContract) contract.
//     pub fn va_token(&self) -> VaNftContractRef {
//         VaNftContractRef::at(&self.va_token_address())
//     }
// }

// #[odra::module]
// pub struct ContractRefsWithKycStorage {
//     refs: ContractRefsStorage,
//     kyc_token: Variable<Address>,
// }

// #[odra::module]
// impl ContractRefsWithKycStorage {
//     delegate! {
//         to self.refs {
//             pub fn reputation_token_address(&self) -> Address;
//             pub fn variable_repository_address(&self) -> Address;
//             pub fn va_token_address(&self) -> Address;
//         }
//     }

//     pub fn init(
//         &mut self,
//         variable_repository: Address,
//         reputation_token: Address,
//         va_token: Address,
//         kyc_token: Address,
//     ) {
//         self.refs
//             .init(variable_repository, reputation_token, va_token);
//         self.kyc_token.set(kyc_token);
//     }

//     pub fn kyc_token_address(&self) -> Address {
//         self.kyc_token
//             .get()
//             .unwrap_or_revert_with(Error::VariableValueNotSet)
//     }
// }

// impl ContractRefsWithKycStorage {
//     pub fn kyc_token(&self) -> KycNftContractRef {
//         KycNftContractRef::at(&self.kyc_token_address())
//     }

//     /// Returns the Ref of [Reputation Token](crate::core_contracts::ReputationContract) contract.
//     pub fn reputation_token(&self) -> ReputationContractRef {
//         self.refs.reputation_token()
//     }

//     /// Returns the Ref of [Variable Repository](crate::core_contracts::VariableRepository) contract.
//     pub fn variable_repository(&self) -> VariableRepositoryContractRef {
//         self.refs.variable_repository()
//     }

//     /// Returns the Ref of [VA Token](crate::core_contracts::VaNftContract) contract.
//     pub fn va_token(&self) -> VaNftContractRef {
//         self.refs.va_token()
//     }
// }

/// A module that stores addresses to common voting_contracts that are used by most of the voting voting_contracts.
#[odra::module]
pub struct ContractRefs {
    variable_repository: Variable<Address>,
    reputation_token: Variable<Address>,
    va_token: Variable<Address>,
    kyc_token: Variable<Address>,
}

impl ContractRefs {
    pub fn set_variable_repository(&mut self, variable_repository: Address) {
        self.variable_repository.set(variable_repository);
    }

    pub fn set_reputation_token(&mut self, reputation_token: Address) {
        self.reputation_token.set(reputation_token);
    }

    pub fn set_va_token(&mut self, va_token: Address) {
        self.va_token.set(va_token);
    }

    pub fn set_kyc_token(&mut self, kyc_token: Address) {
        self.kyc_token.set(kyc_token);
    }

    pub fn variable_repository_address(&self) -> Address {
        self.variable_repository
            .get()
            .unwrap_or_revert_with(Error::VariableValueNotSet)
    }

    pub fn reputation_token_address(&self) -> Address {
        self.reputation_token
            .get()
            .unwrap_or_revert_with(Error::VariableValueNotSet)
    }

    pub fn va_token_address(&self) -> Address {
        self.va_token
            .get()
            .unwrap_or_revert_with(Error::VariableValueNotSet)
    }

    pub fn kyc_token_address(&self) -> Address {
        self.kyc_token
            .get()
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
