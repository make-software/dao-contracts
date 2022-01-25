#[cfg(test)]
mod tests {
    use std::thread;

    use casper_types::U256;
    use reputation_contract::{ReputationContractInterface, ReputationContractTest};
    use test_utils::TestEnv;

    #[test]
    fn test_deploy() {
        let env = TestEnv::new();
        let mut rep = ReputationContractTest::new(&env, 10.into());
        assert_eq!(rep.total_supply(), 10.into());
        assert_eq!(rep.balance_of(env.get_account(0)), 10.into());
        assert_eq!(rep.balance_of(env.get_account(1)), 0.into());
    }

    #[test]
    fn test_transfer() {
        let env = TestEnv::new();
        let total_supply = 10.into();
        let amount = 3.into();
        let recipient = env.get_account(1);

        let mut rep = ReputationContractTest::new(&env, total_supply);

        rep.transfer(recipient, amount);
        assert_eq!(rep.balance_of(env.get_account(0)), total_supply - amount);
        assert_eq!(rep.balance_of(env.get_account(1)), amount);

        env.as_account(recipient);
        
        let second_recipient = env.get_account(2);
        rep.transfer(second_recipient, amount);
        assert_eq!(rep.balance_of(env.get_account(0)), total_supply - amount);
        assert_eq!(rep.balance_of(env.get_account(1)), U256::zero());
        assert_eq!(rep.balance_of(env.get_account(2)), amount);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
