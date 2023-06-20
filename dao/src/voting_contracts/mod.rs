//! Module containg voting contracts.
mod admin;
mod kyc_voter;
mod onboarding_request;
mod repo_voter;
mod reputation_voter;
mod simple_voter;
mod slashing_voter;

pub use admin::{Action as AdminAction, AdminContract, AdminContractDeployer, AdminContractRef};
pub use kyc_voter::{KycVoterContract, KycVoterContractDeployer, KycVoterContractRef};
pub use onboarding_request::{
    OnboardingRequestContract, OnboardingRequestContractDeployer, OnboardingRequestContractRef,
    OnboardingVotingCreated,
};
pub use repo_voter::{RepoVoterContract, RepoVoterContractDeployer, RepoVoterContractRef};
pub use reputation_voter::{
    Action as ReputationAction, ReputationVoterContract, ReputationVoterContractDeployer,
    ReputationVoterContractRef,
};
pub use simple_voter::{SimpleVoterContract, SimpleVoterContractDeployer, SimpleVoterContractRef};
pub use slashing_voter::{
    SlashingVoterContract, SlashingVoterContractDeployer, SlashingVoterContractRef,
};
