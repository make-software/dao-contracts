#[cfg(test)]
mod tests {
    use casper_types::{ApiError, U256};
    use reputation_contract::{ReputationContractInterface, ReputationContractTest};
    use utils::TestEnv;

    #[test]
    fn test_deploy() {
        let (env, contract) = setup();
        assert_eq!(contract.total_supply(), U256::zero());
        assert_eq!(contract.balance_of(env.get_account(0)), U256::zero());
        assert_eq!(contract.balance_of(env.get_account(1)), U256::zero());
    }

    #[test]
    fn test_whitelisting_owner_by_default() {
        let (_, contract) = setup();

        assert_eq!(contract.is_whitelisted(contract.get_owner().unwrap()), true);
    }

    #[test]
    fn test_mint_as_owner() {
        let (env, mut contract) = setup();
        let recipient = env.get_account(1);

        contract.mint(recipient, 100.into());

        assert_eq!(contract.balance_of(recipient), 100.into());
    }

    #[test]
    fn test_mint_as_non_owner() {
        let (env, mut contract) = setup();
        let non_owner = env.get_account(1);

        env.as_account(non_owner);
        env.expect_error(utils::Error::NotWhitelisted);

        contract.mint(non_owner, 10.into());
    }

    #[test]
    #[should_panic = "Unexpected execution error."]
    fn test_total_supply_overflow() {
        let (env, mut contract) = setup();

        contract.mint(env.get_account(1), U256::MAX);

        env.expect_error(ApiError::Unhandled);
        contract.mint(env.get_account(2), 1.into());
    }

    #[test]
    fn test_new_owner_whitelisting() {
        let (env, mut contract) = setup();

        assert_eq!(contract.is_whitelisted(contract.get_owner().unwrap()), true);

        let new_owner = env.get_account(1);
        contract.change_ownership(new_owner);

        assert_eq!(contract.is_whitelisted(contract.get_owner().unwrap()), true);
    }

    #[test]
    fn test_transfer() {
        let total_supply = 10.into();
        let transfer_amount = 4.into();

        let (env, mut contract) = setup_with_initial_supply(total_supply);
        let (owner, first_recipient) = (env.get_account(0), env.get_account(1));

        contract.transfer_from(owner, first_recipient, transfer_amount);

        assert_eq!(contract.balance_of(owner), total_supply - transfer_amount);
        assert_eq!(contract.balance_of(first_recipient), transfer_amount);
    }

    #[test]
    fn test_transfer_from_not_whitelisted_user() {
        let (env, mut contract) = setup();
        let (sender, recipient) = (env.get_account(1), env.get_account(2));

        contract.mint(sender, 10.into());

        env.as_account(sender);
        env.expect_error(utils::Error::NotWhitelisted);

        contract.transfer_from(sender, recipient, 1.into());
    }

    #[test]
    #[should_panic = "Unexpected execution error."]
    fn test_transfer_amount_higher_than_balance() {
        let total_supply = 10.into();
        let transfer_amount = 11.into();

        let (env, mut contract) = setup_with_initial_supply(total_supply);
        let (owner, first_recipient) = (env.get_account(0), env.get_account(1));

        env.expect_error(ApiError::Unhandled);

        contract.transfer_from(owner, first_recipient, transfer_amount);
    }

    #[test]
    fn test_onwership() {
        let (env, mut contract) = setup();
        assert_eq!(contract.get_owner().unwrap(), env.active_account());
        let new_owner = env.get_account(1);
        contract.change_ownership(new_owner);
        assert_eq!(contract.get_owner().unwrap(), new_owner);

        env.expect_error(utils::Error::NotAnOwner);
        contract.change_ownership(new_owner);
    }

    fn setup() -> (TestEnv, ReputationContractTest) {
        let env = TestEnv::new();
        let contract = ReputationContractTest::new(&env);

        (env, contract)
    }

    fn setup_with_initial_supply(amount: U256) -> (TestEnv, ReputationContractTest) {
        let (env, mut contract) = setup();

        contract.mint(env.get_account(0), amount);

        (env, contract)
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
