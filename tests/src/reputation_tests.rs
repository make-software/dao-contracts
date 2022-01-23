#[cfg(test)]
mod tests {
    use reputation_contract::{ReputationContractInterface, ReputationContractTest};
    use tests_utils::TestEnv;

    #[test]
    fn test_reputation() {
        let env = TestEnv::new();
        let mut rep = ReputationContractTest::new(&env, 10.into());
        rep.mint(10.into());
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
