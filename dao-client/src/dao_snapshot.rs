use dao::{
    bid_escrow::contract::{BidEscrowContractDeployer, BidEscrowContractRef},
    core_contracts::{
        KycNftContractDeployer, KycNftContractRef, ReputationContractDeployer,
        ReputationContractRef, VaNftContractDeployer, VaNftContractRef,
        VariableRepositoryContractDeployer, VariableRepositoryContractRef,
    },
    utils_contracts::{
        CSPRRateProviderContractDeployer, CSPRRateProviderContractRef, DaoIdsContractDeployer,
        DaoIdsContractRef,
    },
    voting_contracts::{
        AdminContractDeployer, AdminContractRef, KycVoterContractDeployer, KycVoterContractRef,
        OnboardingRequestContractDeployer, OnboardingRequestContractRef, RepoVoterContractDeployer,
        RepoVoterContractRef, ReputationVoterContractDeployer, ReputationVoterContractRef,
        SimpleVoterContractDeployer, SimpleVoterContractRef, SlashingVoterContractDeployer,
        SlashingVoterContractRef,
    },
};

use crate::DeployedContractsToml;

/// DAO addresses.
pub struct DaoSnapshot {
    pub ids: DaoIdsContractRef,
    pub admin: AdminContractRef,
    pub variable_repository: VariableRepositoryContractRef,
    pub kyc_token: KycNftContractRef,
    pub va_token: VaNftContractRef,
    pub reputation_token: ReputationContractRef,
    pub rate_provider: CSPRRateProviderContractRef,
    pub reputation_voter: ReputationVoterContractRef,
    pub kyc_voter: KycVoterContractRef,
    pub repo_voter: RepoVoterContractRef,
    pub simple_voter: SimpleVoterContractRef,
    pub slashing_voter: SlashingVoterContractRef,
    pub bid_escrow: BidEscrowContractRef,
    pub onboarding: OnboardingRequestContractRef,
}

impl DaoSnapshot {
    /// Load DAO addresses from file.    
    pub fn load() -> DaoSnapshot {
        let contracts =
            DeployedContractsToml::load().expect("Failed to load deployed contracts from file.");
        Self::from(contracts)
    }
}

impl From<DeployedContractsToml> for DaoSnapshot {
    fn from(contracts: DeployedContractsToml) -> Self {
        Self {
            ids: DaoIdsContractDeployer::register(contracts.address("DaoIdsContract")),
            admin: AdminContractDeployer::register(contracts.address("AdminContract")),
            variable_repository: VariableRepositoryContractDeployer::register(
                contracts.address("VariableRepositoryContract"),
            ),
            kyc_token: KycNftContractDeployer::register(contracts.address("KycNftContract")),
            va_token: VaNftContractDeployer::register(contracts.address("VaNftContract")),
            reputation_token: ReputationContractDeployer::register(
                contracts.address("ReputationContract"),
            ),
            rate_provider: CSPRRateProviderContractDeployer::register(
                contracts.address("CSPRRateProviderContract"),
            ),
            reputation_voter: ReputationVoterContractDeployer::register(
                contracts.address("ReputationVoterContract"),
            ),
            kyc_voter: KycVoterContractDeployer::register(contracts.address("KycVoterContract")),
            repo_voter: RepoVoterContractDeployer::register(contracts.address("RepoVoterContract")),
            simple_voter: SimpleVoterContractDeployer::register(
                contracts.address("SimpleVoterContract"),
            ),
            slashing_voter: SlashingVoterContractDeployer::register(
                contracts.address("SlashingVoterContract"),
            ),
            bid_escrow: BidEscrowContractDeployer::register(contracts.address("BidEscrowContract")),
            onboarding: OnboardingRequestContractDeployer::register(
                contracts.address("OnboardingRequestContract"),
            ),
        }
    }
}
