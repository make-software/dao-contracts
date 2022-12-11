#![allow(dead_code, unused_imports)]

pub mod config;
pub mod dao;
pub mod helpers;
pub mod params;
pub mod setup;

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
use casper_dao_utils::{Address, TestContract, TestEnv};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U512,
};

use self::params::Account;

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

        // Setup SlashinVoter.
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
        let key = String::from(casper_dao_utils::consts::GOVERNANCE_WALLET_ADDRESS);
        dao.variable_repository
            .update_at(key, multisig_address, None)
            .unwrap();

        // Return dao.
        dao
    }
}
