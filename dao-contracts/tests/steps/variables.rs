use cucumber::then;

use crate::common::DaoWorld;

#[then(expr = "{word} bid is posted")]
fn bid_is_posted(w: &mut DaoWorld, account_name: String) {
    // let account = w.named_address(account_name);
    // let bid = w.bid_escrow.get_bid();
    // assert!(bid.is_some());
}
