#[cfg(test)]
mod tests {
    use utils::TestEnv;
    use variable_repository::VariableRepositoryContractTest;

    #[test]
    fn test_deploy() {
        let (env, contract) = setup();
        assert!(contract.is_whitelisted(env.get_account(0)));
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
