use cucumber::{given, then, when};

use crate::common::{
    params::{Account, TokenId, U256},
    DaoWorld,
};

#[given(expr = "{account} that owns a KYC Token")]
fn setup_user_with_token(world: &mut DaoWorld, user: Account) {
    world.mint_kyc_token(&Account::Owner, &user);

    assert_eq!(world.balance_of(&user), U256::one());
}

#[when(expr = "{account} mints a KYC Token to {account}")]
fn mint(world: &mut DaoWorld, minter: Account, recipient: Account) {
    let _ = world.checked_mint_kyc_token(&minter, &recipient);
}

#[when(expr = "{account} burns {account}'s KYC token")]
fn burn(world: &mut DaoWorld, burner: Account, holder: Account) {
    let _ = world.checked_burn_kyc_token(&burner, &holder);
}

#[then(expr = "the {account}'s balance of KYC Token is {u256}")]
fn assert_balance(world: &mut DaoWorld, user: Account, expected_balance: U256) {
    assert_eq!(world.balance_of(&user), expected_balance);
}

#[then(expr = "KYC Token with id {token_id} belongs to {account}")]
fn assert_token_ownership(world: &mut DaoWorld, token_id: TokenId, user: Account) {
    let token_owner = world.kyc_token.owner_of(*token_id);
    let user_address = world.get_address(&user);

    assert_eq!(token_owner, Some(user_address));
    assert_eq!(world.get_kyc_token_id(&user), U256::one());
}

#[then(expr = "total supply of KYC Token is {u256} token(s)")]
fn assert_total_supply(world: &mut DaoWorld, expected_total_supply: U256) {
    let total_supply = world.kyc_token.total_supply();
    assert_eq!(total_supply, expected_total_supply.0);
}

#[then(expr = "{account} is kyced")]
fn assert_kyced(world: &mut DaoWorld, account: Account) {
    assert!(world.is_account_kyced(&account));
}

impl DaoWorld {
    fn balance_of(&self, account: &Account) -> U256 {
        U256(self.kyc_token.balance_of(self.get_address(account)))
    }
}
