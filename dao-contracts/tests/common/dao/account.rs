use casper_dao_utils::{Address, TestContract};

use crate::common::{
    params::{Account, Contract},
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn get_address(&self, account: &Account) -> Address {
        let idx = match account {
            Account::Owner => 0,
            Account::Deployer => 0,
            Account::Alice => 1,
            Account::Bob => 2,
            Account::Holder => 3,
            Account::Any => 4,
            Account::VA(n) => 4 + n,
        };
        self.env.get_account(idx)
    }

    pub fn get_contract_address(&self, contract: &Contract) -> Address {
        match contract {
            Contract::KycToken => self.kyc_token.address(),
            Contract::KycVoter => self.kyc_voter.address(),
            Contract::VaToken => self.va_token.address(),
            Contract::ReputationToken => self.reputation_token.address(),
            Contract::BidEscrow => self.bid_escrow.address(),
            Contract::VariableRepository => self.variable_repo.address(),
            Contract::SlashingVoter => self.slashing_voter.address(),
        }
    }
}
