use casper_dao_erc721::{
    events::{Approval, ApprovalForAll, Transfer},
    ERC721Test, MockERC721NonReceiverTest, MockERC721ReceiverTest,
};
use casper_dao_utils::{Address, Error, TestEnv};
use casper_types::U256;

static NAME: &str = "Plascoin";
static SYMBOL: &str = "PLS";

static TOKEN_ID_1: u32 = 1;
static TOKEN_ID_2: u32 = 2;

fn setup() -> (TestEnv, ERC721Test) {
    let env = TestEnv::new();
    let token = ERC721Test::new(&env, String::from(NAME), String::from(SYMBOL));
    (env, token)
}

fn full_setup() -> (
    TestEnv,
    ERC721Test,
    MockERC721ReceiverTest,
    MockERC721NonReceiverTest,
) {
    let env = TestEnv::new();
    let token = ERC721Test::new(&env, String::from(NAME), String::from(SYMBOL));
    let receiver = MockERC721ReceiverTest::new(&env);
    let non_receiver = MockERC721NonReceiverTest::new(&env);

    (env, token, receiver, non_receiver)
}

fn full_setup_with_minted_tokens() -> (
    TestEnv,
    ERC721Test,
    MockERC721ReceiverTest,
    MockERC721NonReceiverTest,
) {
    let mut config = full_setup();
    let env = &config.0;
    let erc721 = &mut config.1;
    erc721.mint(env.get_account(1), TOKEN_ID_1.into()).unwrap();
    erc721.mint(env.get_account(1), TOKEN_ID_2.into()).unwrap();

    assert_eq!(erc721.total_supply(), 2.into());
    assert_eq!(erc721.balance_of(env.get_account(1)), 2.into());
    config
}

#[test]
fn test_erc721_initial_state() {
    let (env, token) = setup();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.total_supply(), U256::zero());
    assert_eq!(token.balance_of(env.get_account(0)), U256::zero());
}

// Scenario:
// 1. Mint a token to some user
// 2. Ensure total supply, balance and owner of token are set properly
// 3. Ensure an event has been emmited
// 4. Mint a token with the same id
// 5. Expect an error
#[test]
fn mint_works() {
    // Given token with initial state.
    let (env, mut token) = setup();
    let token_id = 1.into();
    let token_owner = env.get_account(1);

    // When mint a new token
    token.mint(token_owner, token_id).unwrap();

    // Then total supply and the minter balance increases, token ownership is set
    assert_eq!(token.total_supply(), 1.into());
    assert_eq!(token.balance_of(token_owner), 1.into());
    assert_eq!(token.owner_of(token_id).unwrap(), token_owner);

    // Then emits Transfer event
    token.assert_event_at(
        0,
        Transfer {
            from: None,
            to: Some(token_owner),
            token_id,
        },
    );

    // When mint a token with exisiting id
    let result = token.mint(token_owner, token_id);

    // Then it raises an error
    assert_eq!(result, Err(Error::TokenAlreadyExists));
}

// Scenario:
// 1. Mint two tokens
// 2. The current owner tries to approve himself
// 3. Expect an error
// 4. A user (not owner, not approved for all) tries to approve
// 5. Expect an error
// 6. The current owner approves some operator
// 7. The operator is approved
// 8. Approval event is emitted.
#[test]
fn approve_1() {
    // Given initial state: account(1) has two tokens
    let (env, mut token, _, _) = full_setup_with_minted_tokens();
    let token_id = TOKEN_ID_1.into();
    let tokens_owner = env.get_account(1);
    let operator = env.get_account(0);

    // When approves the token's owner
    let result = token.approve(Some(tokens_owner), token_id);

    // Then raises an error
    assert_eq!(result, Err(Error::ApprovalToCurrentOwner));

    // When the caller is not the owner and approved for all
    let result = token.approve(Some(operator), token_id);

    // Then raises an error
    assert_eq!(result, Err(Error::ApproveCallerIsNotOwnerNorApprovedForAll));

    // When the owner approves a different address
    token
        .as_account(tokens_owner)
        .approve(Some(env.get_account(0)), token_id)
        .unwrap();

    // Then the given address should be approved
    assert_eq!(token.get_approved(token_id).unwrap(), env.get_account(0));

    // Then an Approval event is emitted
    token.assert_event_at(
        2,
        Approval {
            owner: Some(tokens_owner),
            operator: Some(env.get_account(0)),
            token_id,
        },
    );
}

// Scenario:
// 1. Mint two tokens
// 2. Any user tries to approve himself
// 3. Expect an error
// 4. A user approves another user for all tokens
// 5. Approval for all is set
// 6. ApprovalForAll event is emitted
// 7. The operator is approved
// 8. The operator gives himself approval for a particular owner's token
// 9. Approval is granted
// 10. Approval event is emitted.
// 11. Approval for all is unset
// 12. ApprovalForAll event is emitted.
#[test]
fn approve_2() {
    // Given initial state
    let (env, mut erc721, _, _) = full_setup_with_minted_tokens();
    let owner = env.get_account(1);
    let operator = env.get_account(0);
    let token_id = TOKEN_ID_1.into();

    // When the owner approves himself
    let result = erc721.as_account(owner).set_approval_for_all(owner, true);

    // Then raises an error
    assert_eq!(result, Err(Error::ApproveToCaller));

    // When the owner approves some operator
    erc721
        .as_account(owner)
        .set_approval_for_all(operator, true)
        .unwrap();

    // Then approval is granted
    assert_eq!(erc721.is_approved_for_all(owner, operator), true);

    // Then an event is emmited
    erc721.assert_event_at(
        2,
        ApprovalForAll {
            owner,
            operator,
            approved: true,
        },
    );

    // When the operator make self-approval having approval for all
    erc721.approve(Some(operator), token_id).unwrap();

    // Then has a single token approval
    assert_eq!(erc721.get_approved(token_id).unwrap(), operator);

    // The Approval event is emmited
    erc721.assert_event_at(
        3,
        Approval {
            owner: Some(owner),
            operator: Some(operator),
            token_id,
        },
    );

    // When the owner revokes approval
    erc721
        .as_account(owner)
        .set_approval_for_all(operator, false)
        .unwrap();

    // Then the operator does not have approval for all tokens
    assert_eq!(erc721.is_approved_for_all(owner, operator), false);

    // Then an event is emmited
    erc721.assert_event_at(
        4,
        ApprovalForAll {
            owner,
            operator,
            approved: false,
        },
    );
}

// Scenario:
// 1. Mint two tokens to account(1)
// 2. Transfer token with non existent id
// 3. Expect an error
// 4. Transfer token to account(2) (not approved yet)
// 5. Expect an error
// 6. Approve account(2)
// 7. account(2) transfers token to himself
// 8. balance of both accounts is equal 1
// 9. Transfer event is emitted
#[test]
fn unsafe_transfer_1() {
    // Given initial state: account(1) has two tokens
    let (env, mut erc721, _, _) = full_setup_with_minted_tokens();
    let token_owner = env.get_account(1);
    let token_id = TOKEN_ID_1.into();

    // When transfer non existent token
    let result = erc721.transfer_from(token_owner, Some(token_owner), 999.into());

    // Then transfer ends with an error
    assert_eq!(result, Err(Error::TokenDoesNotExist));

    // When transfer a token to a not approved receiver
    let receiver_address = env.get_account(2);
    let result = erc721.transfer_from(token_owner, Some(receiver_address), token_id);

    // Then transfer ends with an error
    assert_eq!(result, Err(Error::CallerIsNotOwnerNorApproved));

    // When the owner approves the receiver and transfers a token
    erc721
        .as_account(token_owner)
        .approve(Some(receiver_address), token_id)
        .unwrap();

    erc721
        .as_account(receiver_address)
        .transfer_from(token_owner, Some(receiver_address), token_id)
        .unwrap();

    // Then both users have one token
    assert_eq!(erc721.balance_of(token_owner), 1.into());
    assert_eq!(erc721.balance_of(receiver_address), 1.into());

    // Then emits Transfer event
    erc721.assert_event_at(
        4,
        Transfer {
            from: Some(token_owner),
            to: Some(receiver_address),
            token_id,
        },
    );

    let token_id = TOKEN_ID_2.into();
    erc721
        .as_account(token_owner)
        .approve(None, token_id)
        .unwrap();
    erc721
        .as_account(token_owner)
        .set_approval_for_all(receiver_address, true)
        .unwrap();
}

// Scenario:
// 1. Mint two tokens to account(1)
// 2. account(1) transfers a token to account(2)
// 3. balance of both accounts is equal 1
#[test]
fn unsafe_transfer_2() {
    // Given initial state: account(1) has two tokens
    let (env, mut erc721, _, _) = full_setup_with_minted_tokens();
    let token_owner = env.get_account(1);
    let token_id = TOKEN_ID_1.into();
    let receiver_address = env.get_account(2);

    // When the owner transfers a token
    erc721
        .as_account(token_owner)
        .transfer_from(token_owner, Some(receiver_address), token_id)
        .unwrap();

    // Then both users have one token
    assert_eq!(erc721.balance_of(token_owner), 1.into());
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

    println!("{:?}", env.get_account(0).as_contract_package_hash());
    println!("{:?}", env.get_account(0).as_account_hash());

    erc721
        .as_account(token_owner)
        .approve(Some(env.get_account(0)), token_id)
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
