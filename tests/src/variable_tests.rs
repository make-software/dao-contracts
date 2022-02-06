#[cfg(test)]
mod tests {
    use utils::TestEnv;
    use variable_repository::{
        VariableRepositoryContractInterface, VariableRepositoryContractTest,
    };

    #[test]
    fn test_deploy() {
        let (env, contract) = setup();
        assert!(contract.is_whitelisted(env.get_account(0)));
    }

    #[test]
    fn test_get_uninitialized_value() {
        let (env, mut contract) = setup();

        env.expect_error(utils::Error::ValueNotAvailable);
        contract.get("key".to_string());
    }

    #[test]
    fn test_set_value() {
        let (_, mut contract) = setup();

        let key = "key".to_string();
        let value = "some value".as_bytes();

        contract.set_or_update(key.clone(), value.into());

        assert_eq!(contract.get(key.clone()), value.into());
    }

    #[test]
    fn test_update_value() {
        let (_, mut contract) = setup();

        let key = "key".to_string();
        let initial_value = "some value".as_bytes();
        let new_value = "new value".as_bytes();

        contract.set_or_update(key.clone(), initial_value.into());
        contract.set_or_update(key.clone(), new_value.into());

        let result = contract.get(key.clone());
        assert_eq!(result, new_value.into());
    }

    #[test]
    fn test_keys_indexing() {
        let (_, mut contract) = setup();
        let into_bytes = |val: &str| val.as_bytes().into();

        let key_first = "first";
        let key_second = "second";
        let key_third = "third";
        let key_fourth = "fourth";

        contract.set_or_update(key_first.to_string(), into_bytes("aa"));
        contract.set_or_update(key_second.to_string(), into_bytes("bb"));
        contract.set_or_update(key_third.to_string(), into_bytes("cc"));
        //state: [("first", "aa"), ("second", "bb"), ("thrid", "cc")]

        assert_eq!(contract.get_keys_length(), 3);
        assert_eq!(&contract.get_key_at(0).unwrap(), key_first);
        assert_eq!(&contract.get_key_at(1).unwrap(), key_second);
        assert_eq!(&contract.get_key_at(2).unwrap(), key_third);

        contract.delete(key_first.to_string());
        //state: [("thrid", "cc"), ("second", "bb")]

        assert_eq!(contract.get_keys_length(), 2);
        assert_eq!(&contract.get_key_at(0).unwrap(), key_third);
        assert_eq!(contract.get_key_at(2), None);

        //continuous indexing after deletion
        contract.set_or_update(key_fourth.to_string(), into_bytes("dd"));
        //state: [("third", "cc"), ("second", "bb"), ("fourth", "dd")]

        assert_eq!(contract.get_keys_length(), 3);
        assert_eq!(&contract.get_key_at(2).unwrap(), key_fourth);

        //the key remains the same - does not increase the length
        contract.set_or_update(key_third.to_string(), into_bytes("new value"));
        //state: [("thrid", "new value"), ("second", "bb"),  ("fourth", "dd")]

        assert_eq!(contract.get_keys_length(), 3);
        assert_eq!(&contract.get_key_at(0).unwrap(), key_third);
    }

    fn setup() -> (TestEnv, VariableRepositoryContractTest) {
        let env = TestEnv::new();
        let contract = VariableRepositoryContractTest::new(&env);

        (env, contract)
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
