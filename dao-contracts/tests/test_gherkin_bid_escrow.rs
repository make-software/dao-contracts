use casper_dao_utils::TestContract;
use std::fmt::Debug;
mod common;

use crate::common::DaoWorld;
use cucumber::gherkin::Step;
use cucumber::{given, World as _};

#[given("following starting balances")] // Cucumber Expression
async fn someone_is_hungry(w: &mut DaoWorld, step: &Step) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let name = row.get(0).unwrap();
        let cspr_balance = row.get(1).unwrap();
        let rep_balance = row.get(2).unwrap();

        // set balances

        println!("{} has {} cspr and {} rep", name, cspr_balance, rep_balance);
    }
}

// #[when(regex = r"^(?:he|she|they) eats? (\d+) cucumbers?$")]
// async fn eat_cucumbers(w: &mut World, count: usize) {
//
// }
//
// #[then("she is full")]
// async fn is_full(w: &mut World) {
//
// }

#[tokio::main]
async fn main() {
    DaoWorld::run("tests/features/bid_escrow").await;
}
