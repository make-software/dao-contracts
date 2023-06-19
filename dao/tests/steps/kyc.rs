use cucumber::{given, then, when};

use crate::common::{
    params::{Account, Contract, TokenId},
    DaoWorld,
};

use super::suppress;

#[given(expr = "{account} that owns a KYC Token")]
fn setup_user_with_token(world: &mut DaoWorld, user: Account) {
    world.mint_nft_token(Contract::KycToken, &Account::Owner, &user);

    assert_eq!(world.nft_balance_of(Contract::KycToken, &user), 1);
}

#[when(expr = "{account} mints a KYC Token to {account}")]
fn mint(world: &mut DaoWorld, minter: Account, recipient: Account) {
    suppress(|| world.mint_nft_token(Contract::KycToken, &minter, &recipient));
}

#[when(expr = "{account} burns {account}'s KYC token")]
fn burn(world: &mut DaoWorld, burner: Account, holder: Account) {
    suppress(|| world.burn_nft_token(Contract::KycToken, &burner, &holder));
}

#[then(expr = "the {account}'s balance of KYC Token is {int}")]
fn assert_balance(world: &mut DaoWorld, user: Account, expected_balance: u32) {
    assert_eq!(
        world.nft_balance_of(Contract::KycToken, &user),
        expected_balance
    );
}

#[then(expr = "KYC Token with id {token_id} belongs to {account}")]
fn assert_token_ownership(world: &mut DaoWorld, token_id: TokenId, user: Account) {
    let token_owner = world.nft_owner_of(Contract::KycToken, token_id);
    let user_address = world.get_address(&user);

    assert_eq!(token_owner, user_address);
    assert_eq!(world.get_nft_token_id(Contract::KycToken, &user), token_id);
}

#[then(expr = "total supply of KYC Token is {int} token(s)")]
fn assert_total_supply(world: &mut DaoWorld, expected_total_supply: u32) {
    let total_supply = world.total_supply(Contract::KycToken);
    assert_eq!(total_supply.as_u32(), expected_total_supply);
}

#[then(expr = "{account} is kyced")]
fn assert_kyced(world: &mut DaoWorld, account: Account) {
    assert!(world.has_nft_token(Contract::KycToken, &account));
}

#[then(expr = "{account} is not kyced")]
fn assert_not_kyced(world: &mut DaoWorld, account: Account) {
    assert!(!world.has_nft_token(Contract::KycToken, &account));
}
