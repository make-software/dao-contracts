use std::slice::SliceIndex;

use casper_types::EntryPoints;
use macros::{generate_contract, Contract};

#[derive(Contract)]
pub struct ImportantContract {}

generate_contract!(
    trait ImportantContractInterface {
        fn init(&mut self);
        fn mint(&mut self, recipient: utils::Address, amount: casper_types::U256);
        fn burn(&mut self, owner: utils::Address, amount: casper_types::U256);
    }
);

fn main() {
    let ep: EntryPoints = ImportantContract::entry_points();

    assert_eq!(ep.keys().count(), 3);
    assert!(ep.has_entry_point("init"));
    assert!(ep.has_entry_point("mint"));
    assert!(ep.has_entry_point("burn"));
}
