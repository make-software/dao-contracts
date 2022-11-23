use crate::common::{params::{common::Contract, nft::Account}, DaoWorld};
use cucumber::{gherkin::Step, given};


#[given(expr = "users in {contract} contract")]
fn users_setup(world: &mut DaoWorld, step: &Step, contract: Contract) {
    let users_iter = step.table.as_ref().unwrap().rows.iter().skip(1);

    // rows: account, is_whitelisted
    for row in users_iter {
        let account: Account = row[0].parse().unwrap();
        let should_whitelist: bool = row[1].parse().unwrap();

        if should_whitelist {
            world.whitelist(&contract, &Account::Owner, &account);
        }
    }
}