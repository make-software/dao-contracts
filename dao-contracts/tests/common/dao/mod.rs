use casper_dao_contracts::{
    BidEscrowContractTest, KycNftContractTest, ReputationContractTest, VaNftContractTest,
    VariableRepositoryContractTest,
};

use casper_dao_utils::{TestContract, TestEnv};

#[allow(dead_code)]
pub fn setup_dao() -> (
    BidEscrowContractTest,
    ReputationContractTest,
    VaNftContractTest,
    KycNftContractTest,
    VariableRepositoryContractTest,
) {
    let env = TestEnv::new();
    let variable_repo = VariableRepositoryContractTest::new(&env);
    let mut reputation_token = ReputationContractTest::new(&env);

    let va_token = VaNftContractTest::new(
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

    reputation_token
        .add_to_whitelist(bid_escrow.address())
        .unwrap();

    (
        bid_escrow,
        reputation_token,
        va_token,
        kyc_token,
        variable_repo,
    )
}
