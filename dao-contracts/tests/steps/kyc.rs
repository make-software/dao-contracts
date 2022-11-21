use casper_types::U256;
use cucumber::then;

use crate::common::DaoWorld;

#[then(expr = "total supply is {int} tokens")]
fn total_reputation(w: &mut DaoWorld, expected_total_supply: u32) {
    let total_supply = w.kyc_token.total_supply();
    assert_eq!(
        total_supply,
        U256::from(expected_total_supply)
    );
}