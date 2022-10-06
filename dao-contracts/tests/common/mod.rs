pub mod setup;
use casper_dao_utils::{Address, TestContract, TestEnv};
use casper_types::{U256, U512};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(cucumber::World)]
pub struct DaoWorld {
    bid_escrow: casper_dao_contracts::BidEscrowContractTest,
    va_token: casper_dao_contracts::VaNftContractTest,
    reputation_token: casper_dao_contracts::ReputationContractTest,
    kyc_token: casper_dao_contracts::KycNftContractTest,
    addresses: HashMap<String, Address>,
    balances: HashMap<Address, U512>,
    accounts_count: usize,
    kyc_count: U256,
    va_count: U256,
}

impl DaoWorld {
    // sets relative amount of motes to the account
    pub fn set_cspr_balance(&mut self, account: Address, amount: U512) {
        self.balances.insert(
            account,
            self.test_env().get_account_cspr_balance(account) + amount,
        );
    }

    // gets relative amount of motes to the account
    pub fn get_cspr_balance(&self, account: Address) -> U512 {
        self.balances.get(&account).unwrap() - self.test_env().get_account_cspr_balance(account)
    }

    // sets amount of reputation on the account
    pub fn set_rep_balance(&mut self, account: Address, amount: U256) {
        assert_eq!(self.reputation_token.balance_of(account), U256::zero());
        self.reputation_token.mint(account, amount);
    }

    // performs kyc for an address
    pub fn kyc(&mut self, account: Address) {
        self.kyc_token.mint(account, self.kyc_count);
        self.kyc_count += U256::one();
    }

    // makes an address a va
    pub fn make_va(&mut self, account: Address) {
        self.va_token.mint(account, self.va_count);
        self.va_count += U256::one();
    }

    pub fn test_env(&self) -> &TestEnv {
        self.bid_escrow.get_env()
    }

    // returns address of the account with the given name
    pub fn named_address(&mut self, name: String) -> Address {
        match self.addresses.get(&*name) {
            None => {
                // add new address, but match the name
                match name.as_str() {
                    "Bid Escrow" => {
                        let address = self.bid_escrow.address();
                        self.addresses.insert(name, address);
                        return address.clone();
                    }
                    _ => {
                        let address = self.bid_escrow.get_env().get_account(self.accounts_count);
                        self.addresses.insert(name.clone(), address.clone());
                        self.accounts_count += 1;

                        if name.contains("Job Poster") {
                            self.kyc(address.clone());
                        }

                        if name.contains("Worker") {
                            self.kyc(address.clone());
                        }

                        if name.contains("VA") {
                            self.make_va(address.clone());
                        }

                        if name.contains("Internal") {
                            self.make_va(address.clone());
                        }

                        address.clone()
                    }
                }
            }
            Some(address) => address.clone(),
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
        let (mut bid_escrow, mut reputation_token, mut va_token, mut kyc_token) =
            setup::setup_bid_escrow();
        Self {
            bid_escrow,
            va_token,
            reputation_token,
            kyc_token,
            addresses: Default::default(),
            balances: Default::default(),
            accounts_count: 0,
            kyc_count: 0.into(),
            va_count: 0.into(),
        }
    }
}
