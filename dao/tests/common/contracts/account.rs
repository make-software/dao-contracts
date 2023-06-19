use odra::{test_env, types::Address};

use crate::common::{
    params::{Account, Contract},
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn get_address(&self, account: &Account) -> Address {
        match account {
            Account::Owner => test_env::get_account(0),
            Account::Deployer => test_env::get_account(0),
            Account::Alice => test_env::get_account(1),
            Account::Bob => test_env::get_account(2),
            Account::Holder => test_env::get_account(3),
            Account::Any => test_env::get_account(4),
            Account::JobPoster => test_env::get_account(5),
            Account::ExternalWorker => test_env::get_account(6),
            Account::InternalWorker => test_env::get_account(7),
            Account::MultisigWallet => test_env::get_account(8),
            Account::VA(n) => test_env::get_account(8 + n),
            Account::Contract(contract) => self.get_contract_address(contract),
        }
    }

    pub fn get_contract_address(&self, contract: &Contract) -> Address {
        *match contract {
            Contract::Admin => self.admin.address(),
            Contract::KycToken => self.kyc_token.address(),
            Contract::VaToken => self.va_token.address(),
            Contract::ReputationToken => self.reputation_token.address(),
            Contract::VariableRepository => self.variable_repository.address(),
            Contract::KycVoter => self.kyc_voter.address(),
            Contract::RepoVoter => self.repo_voter.address(),
            Contract::SlashingVoter => self.slashing_voter.address(),
            Contract::SimpleVoter => self.simple_voter.address(),
            Contract::ReputationVoter => self.reputation_voter.address(),
            Contract::BidEscrow => self.bid_escrow.address(),
            Contract::Onboarding => self.onboarding.address(),
            Contract::CSPRRateProvider => self.rate_provider.address(),
        }
    }
}
