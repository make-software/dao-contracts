use cucumber::then;

use crate::common::{params::Account, DaoWorld};

#[then(expr = "{account} Bid {word} posted")]
fn bid_is_posted(w: &mut DaoWorld, account: Account, is_posted: String) {
    let is_posted = match is_posted.as_str() {
        "is" => true,
        "isn't" => false,
        _ => panic!("Unknown is_posted option - it should be either is or isn't"),
    };
    let bid = w.get_bid(0, account);

    assert_eq!(bid.is_some(), is_posted);
}
