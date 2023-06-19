use cucumber::{given, then, when};

use crate::common::{params::Account, DaoWorld};

use super::suppress;

#[when(expr = "{account} sets {account} as a new owner of {account} contract")]
fn change_ownership(world: &mut DaoWorld, caller: Account, new_owner: Account, contract: Account) {
    suppress(|| world.change_ownership(&contract, &caller, &new_owner));
}

#[when(expr = "{account} adds {account} to whitelist in {account} contract")]
#[given(expr = "{account} added {account} to whitelist in {account} contract")]
fn whitelist(world: &mut DaoWorld, caller: Account, target: Account, contract: Account) {
    suppress(|| world.whitelist_account(&contract, &caller, &target));
}

#[when(expr = "{account} removes {account} from whitelist in {account} contract")]
fn remove_from_whitelist(world: &mut DaoWorld, caller: Account, user: Account, contract: Account) {
    suppress(|| world.remove_from_whitelist(&contract, &caller, &user));
}

#[then(expr = "{account} is not whitelisted in {account} contract")]
fn assert_not_whitelisted(world: &mut DaoWorld, account: Account, contract: Account) {
    assert!(!world.is_whitelisted(&contract, &account));
}

#[then(expr = "{account} is whitelisted in {account} contract")]
fn assert_whitelisted(world: &mut DaoWorld, account: Account, contract: Account) {
    assert!(
        world.is_whitelisted(&contract, &account),
        "{:?} is not whitelisted in {:?}",
        account,
        contract
    );
}

#[then(expr = "{account} is the owner of {account} contract")]
fn assert_ownership(world: &mut DaoWorld, user: Account, contract: Account) {
    let user_address = world.get_address(&user);

    let owner = world.get_owner(&contract);
    assert_eq!(owner, Some(user_address));
}

#[then(expr = "{account} is not the owner of {account} contract")]
fn assert_ne_ownership(world: &mut DaoWorld, user: Account, contract: Account) {
    let user_address = world.get_address(&user);
    let owner = world.get_owner(&contract);

    assert_ne!(owner, Some(user_address));
}
