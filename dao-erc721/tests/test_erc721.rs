use casper_dao_erc721::ERC721Test;
use casper_dao_utils::TestEnv;
use casper_types::U256;

static NAME: &str = "Plascoin";
static SYMBOL: &str = "PLS";

fn setup() -> (TestEnv, ERC721Test) {
    let env = TestEnv::new();
    let token = ERC721Test::new(&env, String::from(NAME), String::from(SYMBOL));
    (env, token)
}

#[test]
fn test_erc721_initial_state() {
    let (env, token) = setup();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    // assert_eq!(token.total_supply(), U256::zero());
    // assert_eq!(token.balance_of(env.get_account(0)), U256::zero());
}
