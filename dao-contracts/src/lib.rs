mod reputation;
mod variable_repository;

pub use reputation::{ReputationContract, ReputationContractCaller, ReputationContractInterface};
pub use variable_repository::{
    VariableRepositoryContract, VariableRepositoryContractCaller,
    VariableRepositoryContractInterface,
};

#[cfg(feature = "test-support")]
pub use reputation::ReputationContractTest;

#[cfg(feature = "test-support")]
pub use variable_repository::VariableRepositoryContractTest;
