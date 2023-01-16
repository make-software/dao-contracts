use casper_types::U512;
use cucumber::{given, then};

use crate::common::{
    params::{Account, Balance},
    DaoWorld,
};

#[then(expr = "{account} Bid {word} posted")]
fn bid_is_posted(world: &mut DaoWorld, account: Account, is_posted: String) {
    let is_posted = match is_posted.as_str() {
        "is" => true,
        "isn't" => false,
        _ => panic!("Unknown is_posted option - it should be either is or isn't"),
    };
    let bid = world.get_bid(0, account);

    assert_eq!(bid.is_some(), is_posted);
}

#[then(expr = "value of {word} is {word}")]
fn assert_variable(world: &mut DaoWorld, key: String, value: String) {
    let current_value: U512 = world.get_variable(key);
    let expected = U512::from_dec_str(&value).unwrap();
    assert_eq!(current_value, expected);
}

#[given(expr = "the price of USDT is {balance} CSPR")]
fn set_cspr_rate(world: &mut DaoWorld, rate: Balance) {
    let _ = world.rate_provider.set_rate(*rate);
}
