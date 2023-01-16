use cucumber::{gherkin::Step, then};

use crate::common::{
    helpers,
    params::{Account, Balance},
    DaoWorld,
};

#[then(expr = "total reputation is {balance}")]
fn total_reputation(world: &mut DaoWorld, total_reputation_expected: Balance) {
    world.assert_total_supply(total_reputation_expected);
}

#[then(expr = "passive REP of {account} is {balance}")]
fn assert_passive_reputation(world: &mut DaoWorld, account: Account, expected_balance: Balance) {
    world.assert_passive_reputation(&account, expected_balance);
}

#[then(expr = "balance of {account} is {balance}")]
fn assert_balance(world: &mut DaoWorld, account: Account, expected_balance: Balance) {
    world.assert_reputation(&account, expected_balance);
}

#[then(expr = "balances are")]
#[then(expr = "users balances are")]
fn assert_balances(world: &mut DaoWorld, step: &Step) {
    let labels = step
        .table
        .as_ref()
        .expect("Missing labels")
        .rows
        .first()
        .expect("Missing labels");

    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let account = helpers::parse::<Account>(row.get(0), "Could't parse account");
        for (idx, label) in labels.iter().enumerate() {
            match label.as_str() {
                "REP balance" => {
                    let expected_reputation_balance =
                        helpers::parse_or_default::<Balance>(row.get(idx));
                    world.assert_reputation(&account, expected_reputation_balance);
                }
                "REP stake" => {
                    let expected_reputation_stake =
                        helpers::parse_or_default::<Balance>(row.get(idx));
                    world.assert_staked_reputation(&account, expected_reputation_stake)
                }
                "CSPR balance" => {
                    let expected_cspr_balance = helpers::parse_or_default::<Balance>(row.get(idx));
                    world.assert_cspr_balance(&account, expected_cspr_balance);
                }
                _ => {}
            }
        }
    }
}
