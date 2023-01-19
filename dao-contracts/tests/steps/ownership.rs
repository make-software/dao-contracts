use cucumber::{given, then, when};

use crate::common::{
    params::{Account, Contract},
    DaoWorld,
};

#[when(expr = "{account} sets {account} as a new owner of {contract} contract")]
fn change_ownership(world: &mut DaoWorld, caller: Account, new_owner: Account, contract: Contract) {
    let _ = world.change_ownership(&contract, &caller, &new_owner);
}

#[when(expr = "{account} adds {account} to whitelist in {contract} contract")]
#[given(expr = "{account} added {account} to whitelist in {contract} contract")]
fn whitelist(world: &mut DaoWorld, caller: Account, target: Account, contract: Contract) {
    let _ = world.whitelist_account(&contract, &caller, &target);
}

#[when(expr = "{account} removes {account} from whitelist in {contract} contract")]
fn remove_from_whitelist(world: &mut DaoWorld, caller: Account, user: Account, contract: Contract) {
    let _ = world.remove_from_whitelist(&contract, &caller, &user);
}

#[then(expr = "{account} is not whitelisted in {contract} contract")]
fn assert_not_whitelisted(world: &mut DaoWorld, account: Account, contract: Contract) {
    assert!(!world.is_whitelisted(&contract, &account));
}

#[then(expr = "{account} is whitelisted in {contract} contract")]
fn assert_whitelisted(world: &mut DaoWorld, account: Account, contract: Contract) {
    assert!(world.is_whitelisted(&contract, &account));
}

#[then(expr = "{account} is the owner of {contract} contract")]
fn assert_ownership(world: &mut DaoWorld, user: Account, contract: Contract) {
    let user_address = world.get_address(&user);
    let owner = world.get_owner(&contract);

    assert_eq!(owner, Some(user_address));
}

#[then(expr = "{account} is not the owner of {contract} contract")]
fn assert_ne_ownership(world: &mut DaoWorld, user: Account, contract: Contract) {
    let user_address = world.get_address(&user);
    let owner = world.get_owner(&contract);

    assert_ne!(owner, Some(user_address));
}
