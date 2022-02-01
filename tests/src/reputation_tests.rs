#[cfg(test)]
mod tests {
    use casper_types::{ApiError, U256};
    use reputation_contract::{ReputationContractInterface, ReputationContractTest};
    use utils::{TestEnv, token::events::Transfer};

    #[test]
    fn test_deploy() {
        let (env, contract) = setup();
        assert_eq!(contract.total_supply(), U256::zero());
        assert_eq!(contract.balance_of(env.get_account(0)), U256::zero());
        assert_eq!(contract.balance_of(env.get_account(1)), U256::zero());
        assert!(contract.is_whitelisted(contract.get_owner().unwrap()));
    }

    #[test]
    fn test_init_cannot_be_called_twice() {
        let (_, mut contract) = setup();
        contract.init();
    }

    #[test]
    fn test_mint_as_owner() {
        let (env, mut contract) = setup();
        let recipient = env.get_account(1);
        let total_supply = 100.into();

        contract.mint(recipient, total_supply);
        assert_eq!(contract.balance_of(recipient), total_supply);
    }

    #[test]
    fn test_mint_as_non_owner() {
        let (env, mut contract) = setup();
        let non_owner = env.get_account(1);

        env.expect_error(utils::Error::NotWhitelisted);

        contract.as_account(non_owner).mint(non_owner, 10.into());
    }

    #[test]
    fn test_whitelisted_user_burn() {
        let total_supply = 100.into();
        let burn_amount = 10.into();
        let remaining_supply = total_supply - burn_amount;

        let (env, mut contract) = setup_with_initial_supply(total_supply);
        let owner = env.get_account(0);

        contract.burn(owner, burn_amount);

        assert_eq!(contract.total_supply(), remaining_supply);
        assert_eq!(contract.balance_of(owner), remaining_supply);
    }

    #[test]
    #[should_panic = "Unexpected execution error."]
    fn test_buring_amount_exceeding_balance() {
        let total_supply = 100.into();
        let burn_amount = 101.into();

        let (env, mut contract) = setup_with_initial_supply(total_supply);
        let owner = env.get_account(0);

        env.expect_error(ApiError::Unhandled);
        contract.burn(owner, burn_amount);
    }

    #[test]
    fn test_non_whitelisted_user_burn() {
        let (env, mut contract) = setup_with_initial_supply(100.into());
        let (user1, user2) = (env.get_account(0), env.get_account(1));

        env.expect_error(utils::Error::NotWhitelisted);
        contract.as_account(user2).burn(user1, 10.into());
    }

    #[test]
    #[should_panic = "Unexpected execution error."]
    fn test_total_supply_overflow() {
        let (env, mut contract) = setup();

        contract.mint(env.get_account(1), U256::MAX);

        env.expect_error(ApiError::Unhandled);
        contract.mint(env.get_account(2), U256::one());
    }

    #[test]
    fn test_whitelisting_as_owner() {
        let (env, mut contract) = setup();
        let (owner, user) = (env.get_account(0), env.get_account(1));

        assert!(contract.is_whitelisted(owner));
        assert_eq!(contract.is_whitelisted(user), false);

        contract.add_to_whitelist(user);
        assert!(contract.is_whitelisted(user));

        contract.remove_from_whitelist(user);
        assert_eq!(contract.is_whitelisted(user), false);
    }

    #[test]
    fn test_not_whitelisted_user_removal_has_no_effect() {
        let (env, mut contract) = setup();
        let user = env.get_account(1);

        assert_eq!(contract.is_whitelisted(user), false);

        contract.remove_from_whitelist(user);
        assert_eq!(contract.is_whitelisted(user), false);
    }

    #[test]
    fn test_duplicated_whitelisting() {
        let (env, mut contract) = setup();
        let user = env.get_account(1);

        contract.add_to_whitelist(user);
        contract.add_to_whitelist(user);
        assert!(contract.is_whitelisted(user));

        contract.remove_from_whitelist(user);
        assert_eq!(contract.is_whitelisted(user), false);
    }

    #[test]
    fn test_whitelisting_as_non_owner() {
        let (env, mut contract) = setup();
        let (user1, user2) = (env.get_account(1), env.get_account(2));

        contract.add_to_whitelist(user1);

        env.expect_error(utils::Error::NotAnOwner);
        contract.as_account(user1).add_to_whitelist(user2);

        env.expect_error(utils::Error::NotAnOwner);
        contract.as_account(user1).remove_from_whitelist(user2);
    }

    #[test]
    fn test_new_owner_whitelisting() {
        let (env, mut contract) = setup();
        let (owner, new_owner) = (env.get_account(0), env.get_account(1));

        assert!(contract.is_whitelisted(owner));

        contract.change_ownership(new_owner);
        assert!(contract.is_whitelisted(new_owner));
    }

    #[test]
    fn test_transfer_from() {
        let total_supply = 10.into();
        let transfer_amount = 4.into();

        let (env, mut contract) = setup_with_initial_supply(total_supply);
        let (owner, first_recipient) = (env.get_account(0), env.get_account(1));

        contract.transfer_from(owner, first_recipient, transfer_amount);

        assert_eq!(contract.balance_of(owner), total_supply - transfer_amount);
        assert_eq!(contract.balance_of(first_recipient), transfer_amount);

        let expected_event = Transfer { from: owner, to: first_recipient, value: transfer_amount};
        let transfer_event: Transfer = contract.event(0);
        assert_eq!(transfer_event, expected_event);
    }

    #[test]
    fn test_transfer_froxm_not_whitelisted_user() {
    }

    #[test]
    fn test_transfer_from_not_whitelisted_user() {
        let (env, mut contract) = setup();
        let (sender, recipient) = (env.get_account(1), env.get_account(2));

        contract.mint(sender, 10.into());

        env.expect_error(utils::Error::NotWhitelisted);

        contract
            .as_account(sender)
            .transfer_from(sender, recipient, 1.into());
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
    fn test_ownership() {
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
