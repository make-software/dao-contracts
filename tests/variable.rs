#[cfg(test)]
mod tests {
    use std::time::Duration;

    use casper_dao_contracts::{
        VariableRepositoryContractInterface, VariableRepositoryContractTest,
    };
    use casper_dao_utils::{
        consts,
        repository::{events::ValueUpdated, RepositoryDefaults},
        BytesConversion, Error, TestEnv,
    };
    use casper_types::U256;

    static KEY: &str = "key";
    static KEY_2: &str = "key_2";
    static KEY_3: &str = "key_3";
    static VALUE: u32 = 1;
    static VALUE_2: u32 = 2;
    static VALUE_3: u32 = 3;

    // Moments in time for interaction with activision_time param.
    static AT_DAY_ONE: u64 = 60 * 60 * 24;
    static AT_DAY_TWO: u64 = 2 * AT_DAY_ONE;
    static AT_DAY_THREE: u64 = 3 * AT_DAY_ONE;

    // Durations for moving time.
    static TWO_DAYS: Duration = Duration::from_secs(AT_DAY_TWO);

    struct ContractWrapper {
        contract: VariableRepositoryContractTest,
    }

    impl std::ops::Deref for ContractWrapper {
        type Target = VariableRepositoryContractTest;

        fn deref(&self) -> &Self::Target {
            &self.contract
        }
    }

    impl std::ops::DerefMut for ContractWrapper {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.contract
        }
    }

    impl ContractWrapper {
        fn new(contract: VariableRepositoryContractTest) -> Self {
            Self { contract }
        }

        pub fn update_at<K: ToString, V: BytesConversion>(
            &mut self,
            key: K,
            value: V,
            activation_time: Option<u64>,
        ) {
            self.contract
                .update_at(key.to_string(), value.convert_to_bytes(), activation_time).unwrap()
        }
    }

    #[test]
    fn test_deploy() {
        use consts::*;
        let (env, c) = setup();
        assert!(c.is_whitelisted(env.get_account(0)));

        assert_eq!(U256::from(300), c.get_value(DEFAULT_POLICING_RATE));
        assert_eq!(U256::from(10), c.get_value(REPUTATION_CONVERSION_RATE));
        assert_eq!(true, c.get_value::<_, bool>(FORUM_KYC_REQUIRED));
        assert_eq!(U256::from(500), c.get_value(FORMAL_VOTING_QUORUM));
        assert_eq!(U256::from(50), c.get_value(INFORMAL_VOTING_QUORUM));
        assert_eq!(U256::from(200), c.get_value(VOTING_QUORUM));
        assert_eq!(U256::from(432000000), c.get_value(FORMAL_VOTING_TIME));
        assert_eq!(U256::from(86400000), c.get_value(INFORMAL_VOTING_TIME));
        assert_eq!(U256::from(172800000), c.get_value(VOTING_TIME));
        assert_eq!(U256::from(100), c.get_value(MINIMUM_GOVERNANCE_REPUTATION));
        assert_eq!(U256::from(10), c.get_value(MINIMUM_VOTING_REPUTATION));
    }
    #[test]
    fn test_get_uninitialized_value() {
        let (env, mut contract) = setup();

        // env.expect_error(Error::ValueNotAvailable);
        let result = contract.get(String::from(KEY));
        assert_eq!(result, Err(String::from("asd")));
    }

    // To test `update_at` entry point all possible cases should be checked.
    // To find out what could happen we consider following possibilities:
    // - current_value: not_set | set
    // - next_activation_time: in_past | in_future | None
    // - arg_activation_time: in_past | in_future | None
    //
    // That gives 18 different scenarios.
    //
    // Possibles:
    // 1. current_value(not_set) + current_activation_time(None) + arg_activation_time(None)
    // 2. current_value(not_set) + current_activation_time(None) + arg_activation_time(in_past)
    // 3. current_value(not_set) + current_activation_time(None) + arg_activation_time(in_future)
    // 4. current_value(set) + current_activation_time(None) + arg_activation_time(in_past)
    // 5. current_value(set) + current_activation_time(None) + arg_activation_time(in_future)
    // 6. current_value(set) + current_activation_time(None) + arg_activation_time(None)
    // 7. current_value(set) + current_activation_time(in_past) + arg_activation_time(in_past)
    // 8. current_value(set) + current_activation_time(in_past) + arg_activation_time(in_future)
    // 9. current_value(set) + current_activation_time(in_past) + arg_activation_time(None)
    // 10. current_value(set) + current_activation_time(in_future) + arg_activation_time(in_past)
    // 11. current_value(set) + current_activation_time(in_future) + arg_activation_time(in_future)
    // 12. current_value(set) + current_activation_time(in_future) + arg_activation_time(None)

    // Impossible to to have a the next value set without the current value.
    // 13. current_value(not_set) + current_activation_time(in_past) + arg_activation_time(in_past)
    // 14. current_value(not_set) + current_activation_time(in_past) + arg_activation_time(in_future)
    // 15. current_value(not_set) + current_activation_time(in_past) + arg_activation_time(None)
    // 16. current_value(not_set) + current_activation_time(in_future) + arg_activation_time(in_past)
    // 17. current_value(not_set) + current_activation_time(in_future) + arg_activation_time(in_future)
    // 18. current_value(not_set) + current_activation_time(in_future) + arg_activation_time(None)

    #[test]
    fn test_update_at_1() {
        // Given no record.
        let (_, mut contract) = setup();

        // When update_at with new value without activation time
        contract.update_at(KEY, VALUE, None);

        // Then it sets a value.
        assert_eq!(contract.get_full_value(KEY), (VALUE, None));

        // And it throws an event.
        contract.assert_event_at(
            RepositoryDefaults::len() + 2,
            ValueUpdated {
                key: String::from(KEY),
                value: VALUE.convert_to_bytes(),
                activation_time: None,
            },
        );
    }

    #[test]
    fn test_update_at_2() {
        // Given no record.
        let (env, mut contract) = setup();
        env.advance_block_time_by(TWO_DAYS);

        // When update_at with new value and activation_time in past.
        // Then it throws an error.
        env.expect_error(Error::ActivationTimeInPast);
        contract.update_at(KEY, VALUE, Some(AT_DAY_ONE));
    }

    #[test]
    fn test_update_at_3() {
        // Given no record.
        let (env, mut contract) = setup();

        // When update_at with new value and activation_time in future.
        // Then it throws an error.
        env.expect_error(Error::ValueNotAvailable);
        contract.update_at(KEY, VALUE, Some(AT_DAY_ONE));
    }

    #[test]
    fn test_update_at_4() {
        // Given value and no next value.
        let (env, mut contract) = setup_with(KEY, VALUE);
        env.advance_block_time_by(TWO_DAYS);

        // When update_at with activation_time in past.
        // Then it throws an error.
        env.expect_error(Error::ActivationTimeInPast);
        contract.update_at(KEY, VALUE_2, Some(AT_DAY_ONE));
    }

    #[test]
    fn test_update_at_5() {
        // Given value and no next value.
        let (_, mut contract) = setup_with(KEY, VALUE);

        // When update_at with activation_time in future.
        contract.update_at(KEY, VALUE_2, Some(AT_DAY_ONE));

        // Then it updates the next value.
        assert_eq!(
            contract.get_full_value::<_, u32>(KEY),
            (VALUE, Some((VALUE_2, AT_DAY_ONE)))
        );
    }

    #[test]
    fn test_update_at_6() {
        // Given value and no next value.
        let (_, mut contract) = setup_with(KEY, VALUE);

        // When update_at with new value without activation time
        contract.update_at(KEY, VALUE_2, None);

        // Then it sets a value.
        assert_eq!(contract.get_full_value(KEY), (VALUE_2, None));
    }

    #[test]
    fn test_update_at_7() {
        // Given value and next value with activation_time in past.
        let (env, mut contract) = setup_with(KEY, VALUE);
        contract.update_at(KEY, VALUE_2, Some(AT_DAY_ONE));
        env.advance_block_time_by(TWO_DAYS);

        // When update_at with activation_time in past.
        // Then it throws an error.
        env.expect_error(Error::ActivationTimeInPast);
        contract.update_at(KEY, VALUE, Some(AT_DAY_ONE));
    }

    #[test]
    fn test_update_at_8() {
        // Given value and next value with activation_time in past.
        let (env, mut contract) = setup_with(KEY, VALUE);
        contract.update_at(KEY, VALUE_2, Some(AT_DAY_ONE));
        env.advance_block_time_by(TWO_DAYS);

        // When update_at with activation_time in future.
        contract.update_at(KEY, VALUE_3, Some(AT_DAY_THREE));

        // Then it updates the current value with the current next value and
        // sets a next value wit given activation time.
        assert_eq!(
            contract.get_full_value::<_, u32>(KEY),
            (VALUE_2, Some((VALUE_3, AT_DAY_THREE)))
        );
    }

    #[test]
    fn test_update_at_9() {
        // Given value and next value with activation_time in past.
        let (env, mut contract) = setup_with(KEY, VALUE);
        contract.update_at(KEY, VALUE_2, Some(AT_DAY_ONE));
        env.advance_block_time_by(TWO_DAYS);

        // When update_at without activation time.
        contract.update_at(KEY, VALUE_3, None);

        // Then it updates the value an clear next value.
        assert_eq!(contract.get_full_value::<_, u32>(KEY), (VALUE_3, None));
    }

    #[test]
    fn test_update_at_10() {
        // Given value and next value with activation_time in future.
        let (env, mut contract) = setup_with(KEY, VALUE);
        contract.update_at(KEY, VALUE_2, Some(AT_DAY_THREE));
        env.advance_block_time_by(TWO_DAYS);

        // When update_at with activation_time in past.
        // Then it throws an error.
        env.expect_error(Error::ActivationTimeInPast);
        contract.update_at(KEY, VALUE_3, Some(AT_DAY_ONE));
    }

    #[test]
    fn test_update_at_11() {
        // Given value and next value with activation_time in future.
        let (_, mut contract) = setup_with(KEY, VALUE);
        contract.update_at(KEY, VALUE_2, Some(AT_DAY_THREE));

        // When update_at with activation_time in future.
        contract.update_at(KEY, VALUE_3, Some(AT_DAY_TWO));

        // Then it updates the current value with the current next value and
        // sets a next value wit given activation time.
        assert_eq!(
            contract.get_full_value::<_, u32>(KEY),
            (VALUE, Some((VALUE_3, AT_DAY_TWO)))
        );
    }

    #[test]
    fn test_update_at_12() {
        // Given value and next value with activation_time in future.
        let (_, mut contract) = setup_with(KEY, VALUE);
        contract.update_at(KEY, VALUE_2, Some(AT_DAY_THREE));

        // When update_at with activation_time in future.
        contract.update_at(KEY, VALUE_3, None);

        // Then it updates the value and clears next value.
        assert_eq!(contract.get_full_value::<_, u32>(KEY), (VALUE_3, None));
    }

    #[test]
    fn test_keys_indexing() {
        let (_, mut contract) = setup();
        contract.update_at(KEY, VALUE, None);
        contract.update_at(KEY_2, VALUE_2, None);
        contract.update_at(KEY_3, VALUE_3, None);

        assert_eq!(contract.get_non_default_keys_length(), 3);
        assert_eq!(&contract.get_non_default_key_at(0).unwrap(), KEY);
        assert_eq!(&contract.get_non_default_key_at(1).unwrap(), KEY_2);
        assert_eq!(&contract.get_non_default_key_at(2).unwrap(), KEY_3);
    }

    #[test]
    fn test_change_ownership() {
        let (env, mut contract) = setup();
        let (owner, new_owner) = (env.get_account(0), env.get_account(1));
        assert_eq!(contract.get_owner().unwrap(), owner);

        contract.change_ownership(new_owner);
        assert_eq!(contract.get_owner().unwrap(), new_owner);

        env.expect_error(Error::NotAnOwner);
        contract.change_ownership(new_owner);
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
    fn test_whitelisting() {
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

        env.expect_error(Error::NotAnOwner);
        contract.as_account(user1).add_to_whitelist(user2);

        env.expect_error(Error::NotAnOwner);
        contract.as_account(user1).remove_from_whitelist(user2);
    }

    #[test]
    fn test_whitelisted_only_has_write_access() {
        let (env, mut contract) = setup();
        let user = env.get_account(1);

        env.expect_error(Error::NotWhitelisted);
        contract
            .as_account(user)
            .update_at("key".to_string(), "value".as_bytes().into(), None);
    }

    #[test]
    fn test_anyone_can_read_data() {
        let (env, mut contract) = setup();
        let user = env.get_account(1);

        contract.update_at(KEY, VALUE_2, None);
        let value = contract.as_account(user).get(KEY.to_string()).unwrap();
        assert_eq!(VALUE_2.convert_to_bytes(), value);
    }

    fn setup() -> (TestEnv, ContractWrapper) {
        let env = TestEnv::new();
        let contract = VariableRepositoryContractTest::new(&env);
        let contract = ContractWrapper::new(contract);
        (env, contract)
    }

    fn setup_with<K: ToString, T: BytesConversion>(key: K, value: T) -> (TestEnv, ContractWrapper) {
        let (env, mut contract) = setup();
        contract.update_at(key, value, None);
        (env, contract)
    }
}
