use casper_dao_erc20::{
    events::{Approval, Transfer},
    ERC20Test,
};
use casper_dao_utils::{Error, TestContract, TestEnv};
use casper_types::{U256, U512};

static NAME: &str = "Plascoin";
static SYMBOL: &str = "PLS";
static DECIMALS: u8 = 2;
static INITIAL_SUPPLY: u32 = 1000;

fn setup() -> (TestEnv, ERC20Test) {
    let env = TestEnv::new();
    let token = ERC20Test::new(
        &env,
        String::from(NAME),
        String::from(SYMBOL),
        DECIMALS,
        U256::from(INITIAL_SUPPLY),
    );
    (env, token)
}

#[test]
fn test_erc20_initial_state() {
    let (env, token) = setup();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.decimals(), DECIMALS);
    assert_eq!(token.total_supply(), U256::from(INITIAL_SUPPLY));
    assert_eq!(
        token.balance_of(env.get_account(0)),
        U256::from(INITIAL_SUPPLY)
    );
    token.assert_event_at(
        0,
        Transfer {
            from: None,
            to: Some(env.get_account(0)),
            value: U256::from(INITIAL_SUPPLY),
        },
    );
}

#[test]
fn test_erc20_transfer() {
    // Given token with initial state.
    let (env, mut token) = setup();
    let owner = env.get_account(0);
    let recipient = env.get_account(1);
    let amount = U256::one();
    let initial_supply = U256::from(INITIAL_SUPPLY);

    // When transfer more then owned.
    let result = token.transfer(recipient, initial_supply + amount);

    // Then it raises an error.
    assert_eq!(result.unwrap_err(), Error::InsufficientBalance);

    // When transfer the amount to recipient as owner.
    token.transfer(recipient, amount).unwrap();

    // Then total supply does not change.
    assert_eq!(token.total_supply(), initial_supply);

    // Then balance of owner is decremented.
    assert_eq!(token.balance_of(owner), initial_supply - amount);

    // Then balance of recipient is incremented.
    assert_eq!(token.balance_of(recipient), amount);

    // Then Transfer event is emitted.
    token.assert_event_at(
        1,
        Transfer {
            from: Some(owner),
            to: Some(recipient),
            value: amount,
        },
    );
}

#[test]
fn test_erc20_transfer_from() {
    // Given token with initial state.
    let (env, mut token) = setup();
    let owner = env.get_account(0);
    let recipient = env.get_account(1);
    let spender = env.get_account(2);
    let amount = U256::one();
    let initial_supply = U256::from(INITIAL_SUPPLY);

    // When spender is approved by the owner.
    token.approve(spender, amount).unwrap();

    // Then allowance is incremented.
    assert_eq!(token.allowance(owner, spender), amount);

    // Then Approval event is emitted.
    token.assert_event_at(
        1,
        Approval {
            owner,
            spender,
            value: amount,
        },
    );

    // When transfer more then allowed.
    let result = token.transfer_from(owner, recipient, amount + 1);

    // Then it raises an error.
    assert_eq!(result.unwrap_err(), Error::InsufficientAllowance);

    // When spender transfers owner's tokens to recipient.
    token
        .as_account(spender)
        .transfer_from(owner, recipient, amount)
        .unwrap();

    // Then total supply does not change.
    assert_eq!(token.total_supply(), initial_supply);

    // Then balance of owner is decremented.
    assert_eq!(token.balance_of(owner), initial_supply - amount);

    // Then balance of recipient is incremented.
    assert_eq!(token.balance_of(recipient), amount);

    // Then allowance is decremented.
    assert_eq!(token.allowance(owner, spender), U256::zero());

    // Then Transfer event is emited.
    token.assert_event_at(
        2,
        Transfer {
            from: Some(owner),
            to: Some(recipient),
            value: amount,
        },
    );

    // And Approval event is emited.
    token.assert_event_at(
        3,
        Approval {
            owner,
            spender,
            value: U256::zero(),
        },
    );
}

#[test]
fn test_deposit() {
    let (env, mut token) = setup();
    let user1 = env.get_account(1);
    let user2 = env.get_account(2);
    let user3 = env.get_account(3);
    let initial_amount = env.get_account_cspr_balance(user1);
    let amount = U512::from(1_000_000_000);

    // Sending CSPR to contract works.
    token.as_account(user1).deposit_with_cspr(amount);
    assert_eq!(token.get_cspr_balance(), amount);
    assert_eq!(env.get_account_cspr_balance(user1), initial_amount - amount);

    // It still works when the same user calls it second time.
    token.as_account(user1).deposit_with_cspr(amount);
    assert_eq!(token.get_cspr_balance(), amount * 2);
    assert_eq!(
        env.get_account_cspr_balance(user1),
        initial_amount - amount * 2
    );

    // It even works when another user uses it.
    token.as_account(user2).deposit_with_cspr(amount);
    assert_eq!(token.get_cspr_balance(), amount * 3);
    assert_eq!(env.get_account_cspr_balance(user2), initial_amount - amount);

    // It's possible to withdraw
    token.as_account(user3).withdraw_all().unwrap();
    assert_eq!(token.get_cspr_balance(), U512::zero());
    assert_eq!(
        env.get_account_cspr_balance(user3),
        initial_amount + amount * 3
    );
}
