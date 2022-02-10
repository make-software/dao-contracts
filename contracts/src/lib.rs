mod reputation;
mod variable;

pub use reputation::{ReputationContract, ReputationContractCaller, ReputationContractInterface};
pub use variable::{
    VariableRepositoryContract, VariableRepositoryContractCaller,
    VariableRepositoryContractInterface,
};

#[cfg(feature = "test-support")]
pub use reputation::ReputationContractTest;

#[cfg(feature = "test-support")]
pub use variable::VariableRepositoryContractTest;
