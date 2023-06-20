use dao::bid_escrow::contract::{BidEscrowContractDeployer, BidEscrowContractRef};
use dao::bid_escrow::types::{BidId, JobOfferId};
use dao::voting_contracts::{OnboardingRequestContractDeployer, OnboardingRequestContractRef};
use dao::{
    core_contracts::{
        KycNftContractDeployer, KycNftContractRef, ReputationContractDeployer,
        ReputationContractRef, VaNftContractDeployer, VaNftContractRef,
        VariableRepositoryContractDeployer, VariableRepositoryContractRef,
    },
    utils_contracts::{
        CSPRRateProviderContractDeployer, CSPRRateProviderContractRef, DaoIdsContractDeployer,
    },
    voting_contracts::{
        AdminContractDeployer, AdminContractRef, KycVoterContractDeployer, KycVoterContractRef,
        RepoVoterContractDeployer, RepoVoterContractRef, ReputationVoterContractDeployer,
        ReputationVoterContractRef, SimpleVoterContractDeployer, SimpleVoterContractRef,
        SlashingVoterContractDeployer, SlashingVoterContractRef,
    },
};
use odra::test_env;
use odra::types::Address;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::OpenOptions;

use super::{contracts::cspr::VirtualBalances, params::Account};

// 1CSPR ~= 0.02924$
const DEFAULT_CSPR_USD_RATE: u64 = 34_000_000_000;

macro_rules! whitelist {
    ( $( $source:ident => [$( $target:ident ),+] ),+ ) => {
        $(
            $(
                $source.add_to_whitelist(*$target.address());
            )+
        )+
    }
}

#[derive(cucumber::World, Clone)]
pub struct DaoWorld {
    pub virtual_balances: VirtualBalances,
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
    pub bids: HashMap<(u32, Address), BidId>,
    pub offers: HashMap<Address, JobOfferId>,
}

impl DaoWorld {
    pub fn advance_time(&mut self, milliseconds: u64) {
        test_env::advance_block_time_by(milliseconds);
    }

    pub fn set_caller(&mut self, caller: &Account) {
        test_env::set_caller(self.get_address(caller));
    }
}

impl Default for DaoWorld {
    fn default() -> Self {
        let default_account = test_env::get_account(0);
        test_env::set_caller(default_account);

        // TODO: extract it using DAOWorld get_account.
        let multisig_wallet = test_env::get_account(8);
        let rate_provider = CSPRRateProviderContractDeployer::init(DEFAULT_CSPR_USD_RATE.into());
        let mut ids = DaoIdsContractDeployer::init();
        let mut variable_repository = VariableRepositoryContractDeployer::init(
            *rate_provider.address(),
            multisig_wallet,
            *ids.address(),
        );
        let mut reputation_token = ReputationContractDeployer::init();
        let mut kyc_token = KycNftContractDeployer::init(
            "kyc_token".to_string(),
            "KYC".to_string(),
            "".to_string(),
        );
        let mut va_token =
            VaNftContractDeployer::init("va_token".to_string(), "VAT".to_string(), "".to_string());
        let mut admin = AdminContractDeployer::init(
            *variable_repository.address(),
            *reputation_token.address(),
            *va_token.address(),
        );

        // Voters
        let mut reputation_voter = ReputationVoterContractDeployer::init(
            *variable_repository.address(),
            *reputation_token.address(),
            *va_token.address(),
        );
        let mut kyc_voter = KycVoterContractDeployer::init(
            *variable_repository.address(),
            *reputation_token.address(),
            *va_token.address(),
            *kyc_token.address(),
        );
        let mut repo_voter = RepoVoterContractDeployer::init(
            *variable_repository.address(),
            *reputation_token.address(),
            *va_token.address(),
        );
        let mut simple_voter = SimpleVoterContractDeployer::init(
            *variable_repository.address(),
            *reputation_token.address(),
            *va_token.address(),
        );
        let mut slashing_voter = SlashingVoterContractDeployer::init(
            *variable_repository.address(),
            *reputation_token.address(),
            *va_token.address(),
        );
        let mut bid_escrow = BidEscrowContractDeployer::init(
            *variable_repository.address(),
            *reputation_token.address(),
            *kyc_token.address(),
            *va_token.address(),
        );
        let mut onboarding = OnboardingRequestContractDeployer::init(
            *variable_repository.address(),
            *reputation_token.address(),
            *kyc_token.address(),
            *va_token.address(),
        );

        whitelist!(
            ids => [admin, kyc_voter, slashing_voter, repo_voter, reputation_voter, simple_voter, bid_escrow, onboarding],
            variable_repository => [repo_voter],
            reputation_token => [admin, repo_voter, reputation_voter, kyc_voter, slashing_voter, simple_voter, bid_escrow, onboarding],
            va_token => [slashing_voter, bid_escrow, onboarding],
            kyc_token => [kyc_voter],
            admin => [slashing_voter],
            kyc_voter => [slashing_voter],
            onboarding => [slashing_voter],
            repo_voter => [slashing_voter, simple_voter],
            reputation_voter => [slashing_voter],
            simple_voter => [slashing_voter],
            slashing_voter => [slashing_voter],
            bid_escrow => [slashing_voter]
        );

        let slashable_contracts: Vec<Address> = vec![
            admin.address(),
            kyc_voter.address(),
            onboarding.address(),
            repo_voter.address(),
            reputation_voter.address(),
            simple_voter.address(),
            slashing_voter.address(),
            bid_escrow.address(),
        ]
        .into_iter()
        .cloned()
        .collect();

        // WON'T DO: Maybe in variable repo?
        slashing_voter.update_slashable_contracts(slashable_contracts);

        Self {
            virtual_balances: Default::default(),
            admin,
            variable_repository,
            kyc_token,
            va_token,
            reputation_token,
            rate_provider,
            reputation_voter,
            kyc_voter,
            repo_voter,
            simple_voter,
            slashing_voter,
            bid_escrow,
            onboarding,
            bids: Default::default(),
            offers: Default::default(),
        }
    }
}

impl Drop for DaoWorld {
    fn drop(&mut self) {
        use std::io::Write;
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("../gas_report.txt")
            .unwrap();

        let gas_report = test_env::gas_report();
        for (reason, gas_cost) in gas_report {
            let gas: f64 = gas_cost.as_u128() as f64;
            writeln!(file, "{}: ${}", reason, (gas / 1_000_000_000.0) / 21.0).unwrap();
        }
        writeln!(file, "\n").unwrap();
    }
}

impl Debug for DaoWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DaoWorld").finish()
    }
}
