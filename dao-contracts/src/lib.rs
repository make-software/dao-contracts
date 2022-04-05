pub mod mocks;
mod repo_voter;
mod reputation;
mod variable_repository;
mod admin;
pub mod action;
pub mod voting;

pub use mocks::mock_voter::{
    MockVoterContract, MockVoterContractCaller, MockVoterContractInterface,
};
pub use repo_voter::{RepoVoterContract, RepoVoterContractCaller, RepoVoterContractInterface};
pub use admin::{AdminContract, AdminContractCaller, AdminContractInterface};
pub use reputation::{ReputationContract, ReputationContractCaller, ReputationContractInterface};
pub use variable_repository::{
    VariableRepositoryContract, VariableRepositoryContractCaller,
    VariableRepositoryContractInterface,
};

#[cfg(feature = "test-support")]
pub use reputation::ReputationContractTest;

#[cfg(feature = "test-support")]
pub use variable_repository::VariableRepositoryContractTest;

#[cfg(feature = "test-support")]
pub use repo_voter::RepoVoterContractTest;

#[cfg(feature = "test-support")]
pub use admin::AdminContractTest;

#[cfg(feature = "test-support")]
pub use mocks::mock_voter::MockVoterContractTest;
