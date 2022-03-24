use casper_dao_utils::{TestEnv, Error};
use casper_types::{U256};
use erc20::{ERC20Test};

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
        U256::from(INITIAL_SUPPLY)
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
    assert_eq!(token.balance_of(env.get_account(0)), U256::from(INITIAL_SUPPLY));
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

    // When transfer more then allowed.
    let result = token.transfer_from(owner, recipient, amount + 1);

    // Then it raises an error.
    assert_eq!(result.unwrap_err(), Error::InsufficientAllowance);

    // When spender transfers owner's tokens to recipient.
    token.as_account(spender).transfer_from(owner, recipient, amount).unwrap();
    
    // Then total supply does not change.
    assert_eq!(token.total_supply(), initial_supply);
    
    // Then balance of owner is decremented.
    assert_eq!(token.balance_of(owner), initial_supply - amount);

    // Then balance of recipient is incremented.
    assert_eq!(token.balance_of(recipient), amount);

    // Then allowance is decremented.
    assert_eq!(token.allowance(owner, spender), U256::zero());
}

fn main() {}