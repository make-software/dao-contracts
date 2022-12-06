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

use casper_dao_contracts::bid::types::{BidId, JobOfferId};
use casper_dao_utils::{Address, TestEnv};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U256,
    U512,
};

use self::params::Account;

#[derive(cucumber::World)]
pub struct DaoWorld {
    pub env: TestEnv,
    pub bid_escrow: casper_dao_contracts::BidEscrowContractTest,
    pub va_token: casper_dao_contracts::VaNftContractTest,
    pub reputation_token: casper_dao_contracts::ReputationContractTest,
    pub kyc_token: casper_dao_contracts::KycNftContractTest,
    pub slashing_voter: casper_dao_contracts::SlashingVoterContractTest,
    pub kyc_voter: casper_dao_contracts::KycVoterContractTest,
    pub variable_repo: casper_dao_contracts::VariableRepositoryContractTest,
    balances: HashMap<Address, U512>,
    starting_balances: HashMap<Address, U512>,
    bids: HashMap<(u32, Address), BidId>,
    offers: HashMap<Address, JobOfferId>,
}

impl DaoWorld {
    pub fn advance_time(&mut self, seconds: u32) {
        self.env
            .advance_block_time_by(Duration::from_secs(seconds as u64));
    }

    // sets variable value
    pub fn set_variable(&mut self, name: String, value: Bytes) {
        self.variable_repo.update_at(name, value, None).unwrap();
    }

    // gets variable value
    pub fn _get_variable(&self, name: String) -> Bytes {
        self.variable_repo.get(name).unwrap()
    }

    // TODO: to remove
    // makes an address a va
    pub fn make_va(&mut self, account: Address) {
        self.va_token.mint(account).unwrap();
    }

    // TODO: to remove
    pub fn is_va(&self, account: Address) -> bool {
        self.va_token.balance_of(account) > U256::zero()
    }
}

impl Debug for DaoWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DaoWorld").finish()
    }
}

impl Default for DaoWorld {
    fn default() -> Self {
        let (
            env,
            bid_escrow,
            reputation_token,
            va_token,
            kyc_token,
            variable_repo,
            slashing_voter,
            kyc_voter,
        ) = dao::setup_dao();
        let mut dao = Self {
            env,
            bid_escrow,
            va_token,
            reputation_token,
            kyc_token,
            variable_repo,
            slashing_voter,
            kyc_voter,
            balances: Default::default(),
            starting_balances: Default::default(),
            bids: Default::default(),
            offers: Default::default(),
        };

        // Set multisig account.
        let multisig_address = Bytes::from(
            dao.get_address(&Account::MultisigWallet)
                .to_bytes()
                .unwrap(),
        );
        let key = String::from(casper_dao_utils::consts::GOVERNANCE_WALLET_ADDRESS);
        dao.variable_repo
            .update_at(key, multisig_address, None)
            .unwrap();

        // Return DaoWorld!
        dao
    }
}
