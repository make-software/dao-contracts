use cucumber::then;
use casper_dao_contracts::bid::bid::Bid;

use crate::common::DaoWorld;

#[then(expr = "{word} Bid {word} posted")]
fn bid_is_posted(w: &mut DaoWorld, account_name: String, is_posted: String) {
    let is_posted = match is_posted.as_str() {
        "is" => true,
        "isn't" => false,
        _ => panic!("Unknown is_posted option - it should be either is or isn't"),
    };
    let account = w.named_address(account_name);
    let bid = w.get_bid(0, account);
    let bid_exists = match bid {
        None => false,
        Some(_) => true,
    };

    assert_eq!(bid_exists, is_posted);
}
