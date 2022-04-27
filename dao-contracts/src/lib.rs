pub mod action;
mod admin;
mod bid_escrow;
#[doc(hidden)]
pub mod mocks;
mod repo_voter;
mod reputation;
mod variable_repository;
/// Utilities to manage the voting process
pub mod voting;

pub use admin::{AdminContract, AdminContractCaller, AdminContractInterface};
pub use bid_escrow::{BidEscrowContract, BidEscrowContractCaller, BidEscrowContractInterface};
#[doc(hidden)]
pub use mocks::mock_voter::{
    MockVoterContract, MockVoterContractCaller, MockVoterContractInterface,
};
pub use repo_voter::{RepoVoterContract, RepoVoterContractCaller, RepoVoterContractInterface};
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

#[doc(hidden)]
#[cfg(feature = "test-support")]
pub use admin::AdminContractTest;

#[cfg(feature = "test-support")]
pub use bid_escrow::BidEscrowContractTest;

#[cfg(feature = "test-support")]
pub use mocks::mock_voter::MockVoterContractTest;
