use casper_dao_utils::{Address, TestContract};

use crate::{
    common::{
        params::{Account, Contract},
        DaoWorld,
    },
    on_contract,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn get_address(&self, account: &Account) -> Address {
        match account {
            Account::Owner => self.env.get_account(0),
            Account::Deployer => self.env.get_account(0),
            Account::Alice => self.env.get_account(1),
            Account::Bob => self.env.get_account(2),
            Account::Holder => self.env.get_account(3),
            Account::Any => self.env.get_account(4),
            Account::JobPoster => self.env.get_account(5),
            Account::ExternalWorker => self.env.get_account(6),
            Account::InternalWorker => self.env.get_account(7),
            Account::MultisigWallet => self.env.get_account(8),
            Account::VA(n) => self.env.get_account(8 + n),
            Account::Contract(contract) => self.get_contract_address(contract),
        }
    }

    pub fn get_contract_address(&self, contract: &Contract) -> Address {
        on_contract!(self, contract, address())
    }
}
