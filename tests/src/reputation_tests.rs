#[cfg(test)]
mod tests {
    use casper_types::U256;
    use reputation_contract::{ReputationContractInterface, ReputationContractTest};
    use utils::TestEnv;

    #[test]
    fn test_deploy() {
        let env = TestEnv::new();
        let rep = ReputationContractTest::new(&env);
        assert_eq!(rep.total_supply(), U256::zero());
        assert_eq!(rep.balance_of(env.get_account(0)), U256::zero());
        assert_eq!(rep.balance_of(env.get_account(1)), U256::zero());
    }

    #[test]
    fn test_transfer() {
        let _env = TestEnv::new();
        // let total_supply = 10.into();
        // let amount = 3.into();
        // let recipient = env.get_account(1);

        // let mut rep = ReputationContractTest::new(&env, total_supply);

        // rep.transfer_from(recipient, amount);
        // assert_eq!(rep.balance_of(env.get_account(0)), total_supply - amount);
        // assert_eq!(rep.balance_of(env.get_account(1)), amount);

        // env.as_account(recipient);

        // let second_recipient = env.get_account(2);
        // rep.transfer(second_recipient, amount);
        // assert_eq!(rep.balance_of(env.get_account(0)), total_supply - amount);
        // assert_eq!(rep.balance_of(env.get_account(1)), U256::zero());
        // assert_eq!(rep.balance_of(env.get_account(2)), amount);
    }

    #[test]
    fn test_onwership() {
        let env = TestEnv::new();
        let mut rep = ReputationContractTest::new(&env);
        assert_eq!(rep.get_owner().unwrap(), env.active_account());
        let new_owner = env.get_account(1);
        rep.change_ownership(new_owner);
        assert_eq!(rep.get_owner().unwrap(), new_owner);

        env.expect_error(utils::Error::NotAnOwner);
        rep.change_ownership(new_owner);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
