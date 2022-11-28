pub mod config;
pub mod dao;
pub mod helpers;
pub mod params;
pub mod setup;

use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};

use casper_dao_contracts::bid::{
    bid::Bid,
    types::{BidId, JobOfferId},
};
use casper_dao_utils::{Address, BlockTime, TestContract, TestEnv};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U256,
    U512,
};

#[derive(cucumber::World)]
pub struct DaoWorld {
    pub env: TestEnv,
    pub bid_escrow: casper_dao_contracts::BidEscrowContractTest,
    pub va_token: casper_dao_contracts::VaNftContractTest,
    pub reputation_token: casper_dao_contracts::ReputationContractTest,
    pub kyc_token: casper_dao_contracts::KycNftContractTest,
    pub slashing_voter: casper_dao_contracts::SlashingVoterContractTest,
    pub kyc_voter: casper_dao_contracts::KycVoterContractTest,
    variable_repo: casper_dao_contracts::VariableRepositoryContractTest,
    addresses: HashMap<String, Address>,
    balances: HashMap<Address, U512>,
    starting_balances: HashMap<Address, U512>,
    bids: HashMap<(u32, Address), BidId>,
    offers: HashMap<Address, JobOfferId>,
    accounts_count: usize,
    kyc_count: U256,
    va_count: U256,
}

impl DaoWorld {
    // sets relative amount of motes to the account
    pub fn set_cspr_balance(&mut self, account: Address, amount: U512) {
        assert!(
            !self.balances.contains_key(&account),
            "Cannot set cspr balance twice"
        );

        self.balances.insert(account, amount);

        self.starting_balances
            .insert(account, self.test_env().get_address_cspr_balance(account));
    }

    pub fn get_bid(&self, offer_id: JobOfferId, poster: Address) -> Option<Bid> {
        let bid_id = self.bids.get(&(offer_id, poster)).unwrap();
        self.bid_escrow.get_bid(*bid_id)
    }

    pub fn post_bid(
        &mut self,
        offer_id: JobOfferId,
        bidder: Address,
        timeframe: BlockTime,
        budget: u64,
        stake: u64,
        onboarding: bool,
        cspr_stake: Option<u64>,
    ) -> BidId {
        match cspr_stake {
            None => {
                self.bid_escrow
                    .as_account(bidder)
                    .submit_bid(
                        0,
                        timeframe,
                        U512::from(budget * 1_000_000_000),
                        U256::from(stake * 1_000_000_000),
                        onboarding,
                        None,
                    )
                    .unwrap();
            }
            Some(cspr_stake) => self
                .bid_escrow
                .as_account(bidder)
                .submit_bid_with_cspr_amount(
                    0,
                    timeframe,
                    U512::from(budget * 1_000_000_000),
                    U256::from(stake * 1_000_000_000),
                    onboarding,
                    U512::from(cspr_stake * 1_000_000_000),
                ).unwrap(),
        }

        let bid_id = self.bid_escrow.bids_count();
        self.bids.insert((offer_id, bidder), bid_id);
        bid_id
    }

    pub fn post_offer(
        &mut self,
        poster: Address,
        timeframe: BlockTime,
        maximum_budget: u64,
        dos_fee: u64,
    ) -> JobOfferId {
        self.bid_escrow
            .as_account(poster)
            .post_job_offer_with_cspr_amount(
                timeframe,
                U512::from(maximum_budget * 1_000_000_000),
                U512::from(dos_fee * 1_000_000_000),
            ).unwrap();

        let offer_id = self.bid_escrow.job_offers_count();
        self.offers.insert(poster, offer_id);
        offer_id
    }

    // gets relative amount of motes of the account
    pub fn get_cspr_balance(&self, account: Address) -> U512 {
        let balance = self.balances.get(&account).unwrap()
            + self.test_env().get_address_cspr_balance(account);
        let result = balance
            .checked_sub(*self.starting_balances.get(&account).unwrap())
            .unwrap();
        result
    }

    // sets amount of reputation on the account
    pub fn set_rep_balance(&mut self, account: Address, amount: U256) {
        self.reputation_token.mint(account, amount).unwrap();
    }

    // gets amount of reputation on the account
    pub fn get_rep_balance(&self, account: Address) -> U256 {
        self.reputation_token.balance_of(account)
    }

    // sets variable value
    pub fn set_variable(&mut self, name: String, value: Bytes) {
        self.variable_repo.update_at(name, value, None).unwrap();
    }

    // gets variable value
    pub fn get_variable(&self, name: String) -> Bytes {
        self.variable_repo.get(name).unwrap()
    }

    // performs kyc for an address
    pub fn kyc(&mut self, account: Address) {
        self.kyc_token.mint(account).unwrap();
        self.kyc_count += U256::one();
    }

    // makes an address a va
    pub fn make_va(&mut self, account: Address) {
        self.va_token.mint(account).unwrap();
        self.va_count += U256::one();
    }

    pub fn is_va(&self, account: Address) -> bool {
        self.va_token.balance_of(account) > U256::zero()
    }

    pub fn test_env(&self) -> &TestEnv {
        self.bid_escrow.get_env()
    }

    // returns address of the account with the given name
    pub fn named_address<T: AsRef<str>>(&mut self, name: T) -> Address {
        let name = String::from(name.as_ref());
        match self.addresses.get(&*name) {
            None => {
                // add new address, but match the name
                match name.as_str() {
                    "BidEscrow" => {
                        let address = self.bid_escrow.address();
                        self.addresses.insert(name, address);
                        address
                    }
                    _ => {
                        let address = self.env.get_account(self.accounts_count);
                        self.addresses.insert(name.clone(), address);
                        self.accounts_count += 1;

                        if name.contains("JobPoster") {
                            self.kyc(address);
                        }

                        if name.contains("Worker") {
                            self.kyc(address);
                        }

                        if name.contains("VA") {
                            self.make_va(address);
                        }

                        if name.contains("Internal") {
                            self.make_va(address);
                        }

                        address
                    }
                }
            }
            Some(address) => *address,
        }
    }

    pub fn _named_address2(&self, name: String) -> Address {
        match name.as_ref() {
            "Owner" => self.env.get_account(0),
            _ => panic!("Unknown address {:?}", name),
        }
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
            addresses: Default::default(),
            balances: Default::default(),
            starting_balances: Default::default(),
            bids: Default::default(),
            offers: Default::default(),
            accounts_count: 0,
            kyc_count: 0.into(),
            va_count: 0.into(),
        };

        // Set multisig account.
        let multisig_address = Bytes::from(dao.named_address("MultisigWallet").to_bytes().unwrap());
        let key = String::from(casper_dao_utils::consts::GOVERNANCE_WALLET_ADDRESS);
        dao.variable_repo
            .update_at(key, multisig_address, None)
            .unwrap();

        // Return DaoWorld!
        dao
    }
}
