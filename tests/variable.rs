#[cfg(test)]
mod tests {
    use casper_dao_contracts::{
        VariableRepositoryContractInterface, VariableRepositoryContractTest,
    };
    use casper_dao_utils::{repository::events::ValueSet, Error, TestEnv};

    #[test]
    fn test_deploy() {
        let (env, contract) = setup();
        assert!(contract.is_whitelisted(env.get_account(0)));
    }

    #[test]
    fn test_get_uninitialized_value() {
        let (env, mut contract) = setup();

        env.expect_error(Error::ValueNotAvailable);
        contract.get("key".to_string());
    }

    #[test]
    fn test_set_value() {
        let (_, mut contract) = setup();

        let key = "key".to_string();
        let value = "some value".as_bytes();

        contract.set_or_update(key.clone(), value.into());

        assert_eq!(contract.get_value(key.clone()), value.into());

        contract.assert_event_at(
            2,
            ValueSet {
                key,
                value: value.into(),
            },
        );
    }

    #[test]
    fn test_update_value() {
        let (_, mut contract) = setup();

        let key = "key".to_string();
        let initial_value = "some value".as_bytes();
        let new_value = "new value".as_bytes();

        contract.set_or_update(key.clone(), initial_value.into());
        contract.set_or_update(key.clone(), new_value.into());

        let result = contract.get_value(key);
        assert_eq!(result, new_value.into());
    }

    #[test]
    fn test_keys_indexing() {
        let (_, mut contract) = setup();
        let into_bytes = |val: &str| val.as_bytes().into();

        let key_first = "first";
        let key_second = "second";
        let key_third = "third";

        contract.set_or_update(key_first.to_string(), into_bytes("aa"));
        contract.set_or_update(key_second.to_string(), into_bytes("bb"));
        contract.set_or_update(key_third.to_string(), into_bytes("cc"));
        //state: [("first", "aa"), ("second", "bb"), ("thrid", "cc")]

        assert_eq!(contract.get_keys_length(), 3);
        assert_eq!(&contract.get_key_at(0).unwrap(), key_first);
        assert_eq!(&contract.get_key_at(1).unwrap(), key_second);
        assert_eq!(&contract.get_key_at(2).unwrap(), key_third);
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
            .set_or_update("key".to_string(), "value".as_bytes().into());

        env.expect_error(Error::NotWhitelisted);
        contract
            .as_account(user)
            .set_or_update("key".to_string(), "value".as_bytes().into());
    }

    #[test]
    fn test_anyone_can_read_data() {
        let (env, mut contract) = setup();
        let user = env.get_account(1);

        contract.set_or_update("key".to_string(), "value".as_bytes().into());
        contract.as_account(user).get("key".to_string());
    }

    fn setup() -> (TestEnv, VariableRepositoryContractTest) {
        let env = TestEnv::new();
        let contract = VariableRepositoryContractTest::new(&env);

        (env, contract)
    }
}