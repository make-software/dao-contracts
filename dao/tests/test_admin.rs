use dao::{
    core_contracts::{
        ReputationContractDeployer, VaNftContractDeployer, VariableRepositoryContractDeployer,
    },
    utils_contracts::{CSPRRateProviderContractDeployer, DaoIdsContractDeployer},
    voting_contracts::{AdminAction, AdminContractDeployer},
};
use odra::{test_env, types::Balance};

#[test]
fn test_admin() {
    let default_account = test_env::get_account(0);
    let alice = test_env::get_account(1);
    test_env::set_caller(default_account);

    let multisig_wallet = test_env::get_account(8);
    let rate_provider = CSPRRateProviderContractDeployer::init(Balance::one());
    let mut ids = DaoIdsContractDeployer::init();
    let variable_repository = VariableRepositoryContractDeployer::init(
        *rate_provider.address(),
        multisig_wallet,
        *ids.address(),
    );
    let mut reputation_token = ReputationContractDeployer::init();
    let mut va_token =
        VaNftContractDeployer::init("va_token".to_string(), "VAT".to_string(), "".to_string());
    let mut admin = AdminContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *va_token.address(),
    );

    ids.add_to_whitelist(*admin.address());
    reputation_token.add_to_whitelist(*admin.address());
    reputation_token.mint(alice, Balance::one() * Balance::from(1000));

    // Onboard Alice
    va_token.mint(alice);

    test_env::set_caller(alice);
    admin.create_voting(
        *variable_repository.address(),
        AdminAction::AddToWhitelist,
        *reputation_token.address(),
        Balance::one() * Balance::from(1000),
    );
}
