use std::str::FromStr;

use casper_dao_utils::{Address, TestContract};
use casper_types::U256;
use cucumber::{given, then, when, Parameter};


use crate::common::DaoWorld;

#[then(expr = "total supply is {int} tokens")]
fn total_reputation(w: &mut DaoWorld, expected_total_supply: u32) {
    let total_supply = w.kyc_token.total_supply();
    assert_eq!(
        total_supply,
        U256::from(expected_total_supply)
    );
}

#[when(expr = "the {account} adds {account} to the whitelist")]
fn whitelist(world: &mut DaoWorld, active_account: Account, candidate: Account) {
    let active_account = active_account.get_address(world);
    let candidate = candidate.get_address(world);

    world.kyc_token
        .as_account(active_account)
        .add_to_whitelist(candidate)
        .expect("{active_account} Should be added to the whitelist");
}

#[given(expr = "a {account}")]
async fn setup(world: &mut DaoWorld, user: Account) {
    let minter = user.get_address(world);
    world.env.as_account(minter);
}

#[when(expr = "a {account} mints a token to any user.")]
async fn mint(world: &mut DaoWorld, minter: Account) {
    let minter = minter.get_address(world);
    let any_user = Account::Any.get_address(world);

    let _ = world.kyc_token
        .as_account(minter)
        .mint(any_user);
}


#[then(expr = "the balance is {int}.")]
fn balance(world: &mut DaoWorld, expected_balance: u32) {
    let any_user = Account::Any.get_address(world);

    let balance = world.kyc_token.balance_of(any_user);
    assert_eq!(balance, expected_balance.into());
}

#[derive(Debug, Default, Parameter)]
#[param(name = "account", regex = "Bob|Alice|Owner|user|any||")]
enum Account {
    Alice,
    Bob,
    Owner,
    #[default]
    Any,
}

impl Account {
    fn get_address(&self, world: &DaoWorld) -> Address {
        let idx = match self {
            Account::Owner => 0,
            Account::Alice => 1,
            Account::Bob => 2,
            Account::Any => 3,
        };
        world.env.get_account(idx)
    }
}

impl FromStr for Account {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Bob" => Self::Bob,
            "Alice" => Self::Alice,
            "Owner" => Self::Owner,
            _ => Self::Any,
        })
    }
}
