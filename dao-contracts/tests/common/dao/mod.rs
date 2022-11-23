use casper_dao_contracts::{
    BidEscrowContractTest,
    KycNftContractTest,
    ReputationContractTest,
    SlashingVoterContractTest,
    VaNftContractTest,
    VariableRepositoryContractTest,
};
use casper_dao_utils::{TestContract, TestEnv};
use crate::common::{DaoWorld, params::{nft::Account, common::Contract}};

impl DaoWorld {
    pub fn whitelist(&mut self, contract: &Contract, caller: &Account, user: &Account) {
        let user = user.get_address(self);
        let caller = caller.get_address(self);

        match contract {
            Contract::KycToken => self.kyc_token.as_account(caller).add_to_whitelist(user).unwrap(),
            Contract::VaToken => todo!(),
            Contract::ReputationToken => todo!(),
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
