#![allow(dead_code, unused_imports)]

pub mod config;
pub mod dao;
pub mod helpers;
pub mod params;

use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    time::Duration,
};

use casper_dao_contracts::{
    escrow::types::{BidId, JobOfferId},
    repo_voter,
    AdminContractTest,
    BidEscrowContractTest,
    CSPRRateProviderContractTest,
    DaoIdsContractTest,
    KycNftContractTest,
    KycVoterContractTest,
    RepoVoterContractTest,
    ReputationContractTest,
    ReputationVoterContractTest,
    SimpleVoterContractTest,
    SlashingVoterContractTest,
    VaNftContractTest,
    VariableRepositoryContractTest,
};
use casper_dao_utils::{consts, Address, TestContract, TestEnv};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U512,
};

use self::params::Account;

// 1CSPR ~= 0.02924$
const DEFAULT_CSPR_USD_RATE: u64 = 34_000_000_000;

#[derive(cucumber::World)]
pub struct DaoWorld {
    pub env: TestEnv,
    pub bid_escrow: BidEscrowContractTest,
    pub reputation_token: ReputationContractTest,
    pub va_token: VaNftContractTest,
    pub kyc_token: KycNftContractTest,
    pub slashing_voter: SlashingVoterContractTest,
    pub kyc_voter: KycVoterContractTest,
    pub variable_repository: VariableRepositoryContractTest,
    pub repo_voter: RepoVoterContractTest,
    pub reputation_voter: ReputationVoterContractTest,
    pub simple_voter: SimpleVoterContractTest,
    pub admin: AdminContractTest,
    pub rate_provider: CSPRRateProviderContractTest,
    balances: HashMap<Address, U512>,
    starting_balances: HashMap<Address, U512>,
    bids: HashMap<(u32, Address), BidId>,
    offers: HashMap<Address, JobOfferId>,
}

impl DaoWorld {
    pub fn advance_time(&mut self, seconds: u64) {
        self.env.advance_block_time_by(Duration::from_secs(seconds));
    }

    // sets variable value
    pub fn set_variable(&mut self, name: String, value: Bytes) {
        self.variable_repository
            .update_at(name, value, None)
            .unwrap();
    }

    // gets variable value
    pub fn _get_variable(&self, name: String) -> Bytes {
        self.variable_repository.get(name).unwrap()
    }
}

impl Debug for DaoWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DaoWorld").finish()
    }
}

impl Default for DaoWorld {
    fn default() -> Self {
        let env = TestEnv::new();
        let variable_repository = VariableRepositoryContractTest::new(&env);
        let mut reputation_token = ReputationContractTest::new(&env);

        let mut va_token = VaNftContractTest::new(
            &env,
            "va_token".to_string(),
            "VAT".to_string(),
            "".to_string(),
        );

        let mut kyc_token = KycNftContractTest::new(
            &env,
            "kyc token".to_string(),
            "kyt".to_string(),
            "".to_string(),
        );

        let mut bid_escrow = BidEscrowContractTest::new(
            &env,
            variable_repository.address(),
            reputation_token.address(),
            kyc_token.address(),
            va_token.address(),
        );

        let mut slashing_voter = SlashingVoterContractTest::new(
            &env,
            variable_repository.address(),
            reputation_token.address(),
            va_token.address(),
        );

        let mut kyc_voter = KycVoterContractTest::new(
            &env,
            variable_repository.address(),
            reputation_token.address(),
            va_token.address(),
            kyc_token.address(),
        );

        let mut repo_voter = RepoVoterContractTest::new(
            &env,
            variable_repository.address(),
            reputation_token.address(),
            va_token.address(),
        );

        let reputation_voter = ReputationVoterContractTest::new(
            &env,
            variable_repository.address(),
            reputation_token.address(),
            va_token.address(),
        );

        let simple_voter = SimpleVoterContractTest::new(
            &env,
            variable_repository.address(),
            reputation_token.address(),
            va_token.address(),
        );

        let admin = AdminContractTest::new(
            &env,
            variable_repository.address(),
            reputation_token.address(),
            va_token.address(),
        );

        let rate_provider = CSPRRateProviderContractTest::new(&env, DEFAULT_CSPR_USD_RATE.into());
        let voting_ids = DaoIdsContractTest::new(&env);

        // Setup Reputation.
        // Setup VariableRepository.
        // Setup VaToken.
        // Setup KycToken.

        // Setup Admin.
        reputation_token.add_to_whitelist(admin.address()).unwrap();

        // Setup KycVoter.
        reputation_token
            .add_to_whitelist(kyc_voter.address())
            .unwrap();
        kyc_token.add_to_whitelist(kyc_voter.address()).unwrap();

        // Setup RepoVoter.
        reputation_token
            .add_to_whitelist(repo_voter.address())
            .unwrap();
        repo_voter.add_to_whitelist(repo_voter.address()).unwrap();

        // Setup ReputationVoter.
        reputation_token
            .add_to_whitelist(reputation_voter.address())
            .unwrap();

        // Setup SlashingVoter.
        reputation_token
            .add_to_whitelist(slashing_voter.address())
            .unwrap();
        va_token.add_to_whitelist(slashing_voter.address()).unwrap();
        repo_voter
            .add_to_whitelist(slashing_voter.address())
            .unwrap();
        kyc_voter
            .add_to_whitelist(slashing_voter.address())
            .unwrap();

        bid_escrow
            .add_to_whitelist(slashing_voter.address())
            .unwrap();

        // Setup SimpleVoter.
        repo_voter.add_to_whitelist(simple_voter.address()).unwrap();
        reputation_token
            .add_to_whitelist(simple_voter.address())
            .unwrap();

        // Setup BidEscrow.
        reputation_token
            .add_to_whitelist(bid_escrow.address())
            .unwrap();
        va_token.add_to_whitelist(bid_escrow.address()).unwrap();
        slashing_voter
            .update_bid_escrow_list(vec![bid_escrow.address()])
            .unwrap();

        // Build the DaoWorld!
        let mut dao = Self {
            env,
            bid_escrow,
            reputation_token,
            va_token,
            kyc_token,
            slashing_voter,
            kyc_voter,
            variable_repository,
            repo_voter,
            reputation_voter,
            simple_voter,
            admin,
            rate_provider,
            balances: Default::default(),
            starting_balances: Default::default(),
            bids: Default::default(),
            offers: Default::default(),
        };

        // Post install updates.
        // Set multisig account.
        let multisig_address = Bytes::from(
            dao.get_address(&Account::MultisigWallet)
                .to_bytes()
                .unwrap(),
        );
        let key = String::from(casper_dao_utils::consts::BID_ESCROW_WALLET_ADDRESS);
        dao.variable_repository
            .update_at(key, multisig_address, None)
            .unwrap();

        // Update rate provider.
        dao.variable_repository
            .update_at(
                consts::FIAT_CONVERSION_RATE_ADDRESS.to_string(),
                Bytes::from(dao.rate_provider.address().to_bytes().unwrap()),
                None,
            )
            .unwrap();

        // Update voting ids.
        dao.variable_repository
            .update_at(
                consts::VOTING_IDS_ADDRESS.to_string(),
                Bytes::from(voting_ids.address().to_bytes().unwrap()),
                None,
            )
            .unwrap();

        // Return dao.
        dao
    }
}
