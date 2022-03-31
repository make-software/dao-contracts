use casper_dao_erc721::{ERC721NonReceiverTest, ERC721ReceiverTest, ERC721Test};
use casper_dao_utils::{Address, Error, TestEnv};
use casper_types::U256;

static NAME: &str = "Plascoin";
static SYMBOL: &str = "PLS";

fn setup() -> (TestEnv, ERC721Test) {
    let env = TestEnv::new();
    let token = ERC721Test::new(&env, String::from(NAME), String::from(SYMBOL));
    (env, token)
}

fn full_setup() -> (
    TestEnv,
    ERC721Test,
    ERC721ReceiverTest,
    ERC721NonReceiverTest,
) {
    let env = TestEnv::new();
    let token = ERC721Test::new(&env, String::from(NAME), String::from(SYMBOL));
    let receiver = ERC721ReceiverTest::new(&env);
    let non_receiver = ERC721NonReceiverTest::new(&env);

    (env, token, receiver, non_receiver)
}

#[test]
fn test_erc721_initial_state() {
    let (env, token) = setup();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.total_supply(), U256::zero());
    assert_eq!(token.balance_of(env.get_account(0)), U256::zero());
}

#[test]
fn mint_works() {
    let (env, mut token) = setup();
    let token_id = 1.into();
    let token_owner = env.get_account(1);

    token.mint(token_owner, token_id).unwrap();

    assert_eq!(token.total_supply(), 1.into());
    assert_eq!(token.balance_of(token_owner), 1.into());
}

#[test]
fn test_safe_transfer() {
    let (env, mut erc721, receiver, _) = full_setup();

    let token_owner = env.get_account(1);
    let token_id = 1.into();

    erc721.mint(token_owner, token_id).unwrap();

    let receiver_address = Address::from(receiver.get_package_hash());

    match erc721
        .as_account(token_owner)
        .approve(env.get_account(0), token_id)
    {
        Ok(_) => println!("approve succedded"),
        Err(err) => println!("error {:?}", err),
    }
    match erc721.transfer_from(token_owner, Some(receiver_address), token_id) {
        Ok(_) => println!("transfer succedded"),
        Err(err) => println!("error {:?}", err),
    }

    assert_eq!(erc721.balance_of(token_owner), 0.into());
    assert_eq!(erc721.balance_of(receiver_address), 1.into());
}

#[test]
fn safe_transfer_works() {
    let (env, mut erc721, receiver, non_receiver) = full_setup();

    let token_owner = env.get_account(1);
    let token_id = 1.into();

    erc721.mint(token_owner, token_id).unwrap();

    let receiver_address = Address::from(receiver.get_package_hash());
    let non_receiver_address = Address::from(non_receiver.get_package_hash());

    erc721
        .as_account(token_owner)
        .approve(env.get_account(0), token_id)
        .unwrap();

    assert_eq!(
        erc721.safe_transfer_from(token_owner, Some(non_receiver_address), token_id),
        Err(Error::NoSuchMethod("on_erc_721_received".to_string()))
    );

    assert_eq!(erc721.balance_of(token_owner), 1.into());
    assert_eq!(erc721.balance_of(non_receiver_address), 0.into());

    erc721
        .safe_transfer_from(token_owner, Some(receiver_address), token_id)
        .unwrap();

    assert_eq!(erc721.balance_of(token_owner), 0.into());
    assert_eq!(erc721.balance_of(receiver_address), 1.into());
}
