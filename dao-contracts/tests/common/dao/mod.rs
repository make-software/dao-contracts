use casper_dao_contracts::{
    BidEscrowContractTest,
    KycNftContractTest,
    ReputationContractTest,
    SlashingVoterContractTest,
    VaNftContractTest,
    VariableRepositoryContractTest,
};
use casper_dao_utils::{TestContract, TestEnv, Address};
use crate::common::{DaoWorld, params::{nft::Account, common::Contract}};

impl DaoWorld {
    pub fn whitelist(&mut self, contract: &Contract, caller: &Account, user: &Account) -> Result<(), casper_dao_utils::Error> {
        let user = user.get_address(self);
        let caller = caller.get_address(self);

        match contract {
            Contract::KycToken => self.kyc_token.as_account(caller).add_to_whitelist(user),
            Contract::VaToken => self.va_token.as_account(caller).add_to_whitelist(user),
            Contract::ReputationToken => self.reputation_token.as_account(caller).add_to_whitelist(user),
            Contract::VariableRepository => self.variable_repo.as_account(caller).add_to_whitelist(user),
            _ => Err(casper_dao_utils::Error::Unknown),
        }
    }

    pub fn remove_from_whitelist(&mut self, contract: &Contract, caller: &Account, user: &Account) -> Result<(), casper_dao_utils::Error> {
        let user = user.get_address(self);
        let caller = caller.get_address(self);

        match contract {
            Contract::KycToken => self.kyc_token.as_account(caller).remove_from_whitelist(user),
            Contract::VaToken => self.va_token.as_account(caller).remove_from_whitelist(user),
            Contract::ReputationToken => self.reputation_token.as_account(caller).remove_from_whitelist(user),
            Contract::VariableRepository => self.variable_repo.as_account(caller).remove_from_whitelist(user),
            _ => Err(casper_dao_utils::Error::Unknown),
        }
    }

    pub fn get_owner(&mut self, contract: &Contract) -> Option<Address> {
        match contract {
            Contract::KycToken => self.kyc_token.get_owner(),
            Contract::VaToken => self.va_token.get_owner(),
            Contract::ReputationToken => self.reputation_token.get_owner(),
            Contract::VariableRepository => self.variable_repo.get_owner(),
            _ => None,
        }
    }

    pub fn change_ownership(&mut self, contract: &Contract, caller: &Account, new_owner: &Account) -> Result<(), casper_dao_utils::Error> {
        let new_owner = new_owner.get_address(self);
        let caller = caller.get_address(self);

        match contract {
            Contract::KycToken => self.kyc_token.as_account(caller).change_ownership(new_owner),
            Contract::VaToken => self.va_token.as_account(caller).change_ownership(new_owner),
            Contract::ReputationToken => self.reputation_token.as_account(caller).change_ownership(new_owner),
            Contract::VariableRepository => self.variable_repo.as_account(caller).change_ownership(new_owner),
            _ => Err(casper_dao_utils::Error::Unknown),
        }
    }

    pub fn is_whitelisted(&mut self, contract: &Contract, account: &Account) -> bool {
        let account = account.get_address(self);

        match contract {
            Contract::KycToken => self.kyc_token.is_whitelisted(account),
            Contract::VaToken => self.va_token.is_whitelisted(account),
            Contract::ReputationToken => self.reputation_token.is_whitelisted(account),
            Contract::VariableRepository => self.variable_repo.is_whitelisted(account),
            _ => false,
        }
    }
}

#[allow(dead_code)]
pub fn setup_dao() -> (
    TestEnv,
    BidEscrowContractTest,
    ReputationContractTest,
    VaNftContractTest,
    KycNftContractTest,
    VariableRepositoryContractTest,
    SlashingVoterContractTest,
) {
    let env = TestEnv::new();
    let variable_repo = VariableRepositoryContractTest::new(&env);
    let mut reputation_token = ReputationContractTest::new(&env);

    let mut va_token = VaNftContractTest::new(
        &env,
        "va_token".to_string(),
        "VAT".to_string(),
        "".to_string(),
    );

    let kyc_token = KycNftContractTest::new(
        variable_repo.get_env(),
        "kyc token".to_string(),
        "kyt".to_string(),
        "".to_string(),
    );

    let bid_escrow = BidEscrowContractTest::new(
        variable_repo.get_env(),
        variable_repo.address(),
        reputation_token.address(),
        kyc_token.address(),
        va_token.address(),
    );

    let slashing_voter = SlashingVoterContractTest::new(
        variable_repo.get_env(),
        variable_repo.address(),
        reputation_token.address(),
        va_token.address(),
    );

    reputation_token
        .add_to_whitelist(bid_escrow.address())
        .unwrap();

    reputation_token
        .add_to_whitelist(slashing_voter.address())
        .unwrap();

    va_token.add_to_whitelist(bid_escrow.address()).unwrap();
    (
        env,
        bid_escrow,
        reputation_token,
        va_token,
        kyc_token,
        variable_repo,
        slashing_voter,
    )
}
