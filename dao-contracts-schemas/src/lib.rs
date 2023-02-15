use casper_dao_contracts::{
    admin::AdminContract,
    bid_escrow::BidEscrowContract,
    ids::DaoIdsContract,
    kyc_nft::KycNftContract,
    kyc_voter::KycVoterContract,
    onboarding_request::OnboardingRequestContract,
    rate_provider::CSPRRateProviderContract,
    repo_voter::RepoVoterContract,
    reputation::ReputationContract,
    reputation_voter::ReputationVoterContract,
    simple_voter::SimpleVoterContract,
    slashing_voter::SlashingVoterContract,
    va_nft::VaNftContract,
    variable_repository::VariableRepositoryContract,
};
use casper_dao_utils::{
    definitions::{ContractDef, ContractDefinition, ElemDef},
    Address,
    BlockTime,
    DocumentHash,
};
use casper_types::{CLTyped, U512};
use serde::Serialize;

pub fn all_contracts() -> Vec<ContractDef> {
    vec![
        // Core contracts.
        reputation(),
        variable_repository(),
        kyc_token(),
        va_token(),
        ids(),
        rate_provider(),
        // Voters.
        admin(),
        kyc_voter(),
        repo_voter(),
        reputation_voter(),
        slashing_voter(),
        simple_voter(),
        onboarding_request_voter(),
        // Bid Escrow.
        bid_escrow(),
    ]
}

// Core Contracts.

fn reputation() -> ContractDef {
    ReputationContract::contract_def()
        .with_events(casper_dao_contracts::reputation::event_schemas())
}

fn variable_repository() -> ContractDef {
    VariableRepositoryContract::contract_def()
        .with_events(casper_dao_contracts::variable_repository::event_schemas())
}

fn kyc_token() -> ContractDef {
    KycNftContract::contract_def().with_events(casper_dao_contracts::kyc_nft::event_schemas())
}

fn va_token() -> ContractDef {
    VaNftContract::contract_def().with_events(casper_dao_contracts::va_nft::event_schemas())
}

fn ids() -> ContractDef {
    DaoIdsContract::contract_def().with_events(casper_dao_contracts::ids::event_schemas())
}

fn rate_provider() -> ContractDef {
    CSPRRateProviderContract::contract_def()
        .with_events(casper_dao_contracts::rate_provider::event_schemas())
}

// Voters.

fn admin() -> ContractDef {
    AdminContract::contract_def().with_events(casper_dao_contracts::admin::event_schemas())
}

fn kyc_voter() -> ContractDef {
    KycVoterContract::contract_def().with_events(casper_dao_contracts::kyc_voter::event_schemas())
}

fn repo_voter() -> ContractDef {
    RepoVoterContract::contract_def().with_events(casper_dao_contracts::repo_voter::event_schemas())
}

fn reputation_voter() -> ContractDef {
    ReputationVoterContract::contract_def()
        .with_events(casper_dao_contracts::reputation_voter::event_schemas())
}

fn slashing_voter() -> ContractDef {
    SlashingVoterContract::contract_def()
        .with_events(casper_dao_contracts::slashing_voter::event_schemas())
}

fn simple_voter() -> ContractDef {
    SimpleVoterContract::contract_def()
        .with_events(casper_dao_contracts::simple_voter::event_schemas())
}

fn onboarding_request_voter() -> ContractDef {
    OnboardingRequestContract::contract_def()
        .with_events(casper_dao_contracts::onboarding_request::event_schemas())
}

// Bid Escrow.

fn bid_escrow() -> ContractDef {
    BidEscrowContract::contract_def().with_events(casper_dao_contracts::bid_escrow::event_schemas())
}

#[derive(Serialize)]
pub struct ProxyWasmDef {
    pub file: String,
    pub contract: String,
    pub method: String,
    pub args: Vec<ElemDef>,
}

impl ProxyWasmDef {
    pub fn new<T: ContractDefinition>(method: &str, file: &str) -> Self {
        ProxyWasmDef {
            file: String::from(file),
            contract: T::contract_def().name,
            method: String::from(method),
            args: Vec::new(),
        }
    }

    pub fn with_arg<T: CLTyped>(mut self, name: &str) -> Self {
        let arg = ElemDef::new::<T>(String::from(name));
        self.args.push(arg);
        self
    }
}

pub fn all_proxy_wasms() -> Vec<ProxyWasmDef> {
    vec![
        pick_bid(),
        post_job_offer(),
        submit_bid(),
        submit_job_proof_during_grace_period(),
        submit_onboarding_request(),
    ]
}

fn pick_bid() -> ProxyWasmDef {
    ProxyWasmDef::new::<BidEscrowContract>("pick_bid", "pick_bid.wasm")
        .with_arg::<Address>("bid_escrow_address")
        .with_arg::<u32>("job_offer_id")
        .with_arg::<u32>("bid_id")
        .with_arg::<U512>("cspr_amount")
}

fn post_job_offer() -> ProxyWasmDef {
    ProxyWasmDef::new::<BidEscrowContract>("post_job_offer", "post_job_offer.wasm")
        .with_arg::<Address>("bid_escrow_address")
        .with_arg::<U512>("cspr_amount")
        .with_arg::<BlockTime>("expected_timeframe")
        .with_arg::<U512>("budget")
}

fn submit_bid() -> ProxyWasmDef {
    ProxyWasmDef::new::<BidEscrowContract>("submit_bid", "submit_bid.wasm")
        .with_arg::<Address>("bid_escrow_address")
        .with_arg::<u32>("job_offer_id")
        .with_arg::<BlockTime>("time")
        .with_arg::<U512>("payment")
        .with_arg::<U512>("reputation_stake")
        .with_arg::<bool>("onboard")
        .with_arg::<U512>("cspr_amount")
}

fn submit_job_proof_during_grace_period() -> ProxyWasmDef {
    ProxyWasmDef::new::<BidEscrowContract>(
        "submit_job_proof_during_grace_period",
        "submit_job_proof_during_grace_period.wasm",
    )
    .with_arg::<Address>("bid_escrow_address")
    .with_arg::<u32>("job_id")
    .with_arg::<DocumentHash>("proof")
    .with_arg::<U512>("reputation_stake")
    .with_arg::<bool>("onboard")
    .with_arg::<U512>("cspr_amount")
}

fn submit_onboarding_request() -> ProxyWasmDef {
    ProxyWasmDef::new::<OnboardingRequestContract>(
        "create_voting",
        "submit_onboarding_request.wasm",
    )
    .with_arg::<Address>("onboarding_address")
    .with_arg::<U512>("cspr_amount")
    .with_arg::<DocumentHash>("reason")
}
