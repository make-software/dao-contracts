//! A selection of contracts implemented for usage in DAO

#[doc(hidden)]
pub mod action;
mod admin;
mod bid_escrow;
mod builder;
pub mod escrow;
mod ids;
mod kyc_nft;
mod kyc_voter;
mod onboarding_request;
mod rate_provider;
pub mod rules;
mod slashing_voter;
mod va_nft;

pub mod config;
#[doc(hidden)]
pub mod mocks;
pub mod repo_voter;
pub mod reputation;
pub mod reputation_voter;
pub mod simple_voter;
pub mod variable_repository;
pub mod voting;

#[doc(hidden)]
#[cfg(feature = "test-support")]
pub use admin::AdminContractTest;
pub use admin::{AdminContract, AdminContractCaller, AdminContractInterface};
#[cfg(feature = "test-support")]
pub use bid_escrow::BidEscrowContractTest;
pub use bid_escrow::{BidEscrowContract, BidEscrowContractCaller, BidEscrowContractInterface};
pub use builder::configuration_builder::ConfigurationBuilder;
pub use config::configuration::*;
#[cfg(feature = "test-support")]
pub use ids::DaoIdsContractTest;
pub use ids::{DaoIdsContract, DaoIdsContractCaller, DaoIdsContractInterface};
#[cfg(feature = "test-support")]
pub use kyc_nft::KycNftContractTest;
pub use kyc_nft::{KycNftContract, KycNftContractCaller, KycNftContractInterface};
#[cfg(feature = "test-support")]
pub use kyc_voter::KycVoterContractTest;
pub use kyc_voter::{KycVoterContract, KycVoterContractCaller, KycVoterContractInterface};
#[cfg(feature = "test-support")]
pub use mocks::mock_voter::MockVoterContractTest;
#[doc(hidden)]
pub use mocks::mock_voter::{
    MockVoterContract,
    MockVoterContractCaller,
    MockVoterContractInterface,
};
#[cfg(feature = "test-support")]
pub use onboarding_request::OnboardingRequestContractTest;
pub use onboarding_request::{
    OnboardingRequestContract,
    OnboardingRequestContractCaller,
    OnboardingRequestContractInterface,
};
#[cfg(feature = "test-support")]
pub use rate_provider::CSPRRateProviderContractTest;
pub use rate_provider::{
    CSPRRateProviderContract,
    CSPRRateProviderContractCaller,
    CSPRRateProviderContractInterface,
};
#[cfg(feature = "test-support")]
pub use repo_voter::RepoVoterContractTest;
pub use repo_voter::{RepoVoterContract, RepoVoterContractCaller, RepoVoterContractInterface};
#[cfg(feature = "test-support")]
pub use reputation::token::ReputationContractTest;
pub use reputation::token::{
    ReputationContract,
    ReputationContractCaller,
    ReputationContractInterface,
};
#[cfg(feature = "test-support")]
pub use reputation_voter::ReputationVoterContractTest;
pub use reputation_voter::{
    ReputationVoterContract,
    ReputationVoterContractCaller,
    ReputationVoterContractInterface,
};
#[cfg(feature = "test-support")]
pub use simple_voter::SimpleVoterContractTest;
pub use simple_voter::{
    SimpleVoterContract,
    SimpleVoterContractCaller,
    SimpleVoterContractInterface,
};
#[cfg(feature = "test-support")]
pub use slashing_voter::SlashingVoterContractTest;
pub use slashing_voter::{
    SlashingVoterContract,
    SlashingVoterContractCaller,
    SlashingVoterContractInterface,
};
#[cfg(feature = "test-support")]
pub use va_nft::VaNftContractTest;
pub use va_nft::{VaNftContract, VaNftContractCaller, VaNftContractInterface};
#[cfg(feature = "test-support")]
pub use variable_repository::VariableRepositoryContractTest;
pub use variable_repository::{
    VariableRepositoryContract,
    VariableRepositoryContractCaller,
    VariableRepositoryContractInterface,
};
