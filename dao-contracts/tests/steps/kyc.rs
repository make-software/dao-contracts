use std::str::FromStr;

use casper_dao_utils::{Address, TestContract};
use cucumber::{given, then, when, Parameter, gherkin::Step};

use crate::common::DaoWorld;

#[given(expr = "users")]
fn whitelisting(world: &mut DaoWorld, step: &Step) {
    let users_iter = step.table.as_ref().unwrap().rows.iter().skip(1);

    for row in users_iter {
        let account: Account = row[0].parse().unwrap();
        let should_whitelist: bool = row[1].parse().unwrap();

        if should_whitelist {
            account.whitelist(world);
        }
    }
}

#[given(expr = "user {account} that owns a KYC Token.")]
fn setup_user_with_token(world: &mut DaoWorld, user: Account) {
    user.whitelist(world);
    user.mint(world, Account::Owner);

    assert_eq!(user.balance(world), U256::one());
}

#[when(expr = "{account} mints a KYC Token to {account}.")]
async fn mint(world: &mut DaoWorld, minter: Account, recipient: Account) {
    let minter = minter.get_address(world);
    let recipient = recipient.get_address(world);

    let _ = world.kyc_token
        .as_account(minter)
        .mint(recipient);
}


#[then(expr = "the {account}'s balance is {u256}.")]
fn assert_balance(world: &mut DaoWorld, user: Account, expected_balance: U256) {
    assert_eq!(user.balance(world), expected_balance);
}

#[then(expr = "Token with id {token} belongs to {account}.")]
fn assert_token_ownership(world: &mut DaoWorld, token_id: TokenId, user: Account) {
    let contract = &world.kyc_token;
    let user = user.get_address(world);
    
    assert_eq!(
        contract.owner_of(*token_id),
        Some(user)
    );

    assert_eq!(
        contract.token_id(user),
        Some(0.into())
    );
}

#[then(expr = "total supply is {u256} tokens")]
fn assert_total_supply(world: &mut DaoWorld, expected_total_supply: U256) {
    let total_supply = world.kyc_token.total_supply();
    assert_eq!(total_supply, expected_total_supply.0);
}

#[derive(Debug, Default, derive_more::FromStr, derive_more::Deref, Parameter)]
#[param(name = "token", regex = r"\d+")]
struct TokenId(pub casper_dao_erc721::TokenId);

#[derive(Debug, Default, derive_more::FromStr, derive_more::Deref, Parameter, PartialEq, Eq)]
#[param(name = "u256", regex = r"\d+")]
struct U256(pub casper_types::U256);

impl U256 {
    fn one() -> Self {
        U256(casper_types::U256::one())
    }
}

#[derive(Debug, Default, Parameter)]
#[param(name = "account", regex = "Bob|Alice|Owner|user|any|any user|")]
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

    fn whitelist(&self, world: &mut DaoWorld) {
        let owner = Account::Owner.get_address(world);
        let user = self.get_address(world);

        world.kyc_token
            .as_account(owner)
            .add_to_whitelist(user)
            .expect("User should be added to the whitelist");
    }

    fn mint(&self, world: &mut DaoWorld, minter: Account) {
        let minter = minter.get_address(world);
        let user = self.get_address(world);

        world.kyc_token
            .as_account(minter)
            .mint(user)
            .expect("A token should be minted");
    }

    fn balance(&self, world: &DaoWorld) -> U256 {
        U256(world.kyc_token.balance_of(self.get_address(world)))
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
