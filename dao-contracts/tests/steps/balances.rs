use casper_types::U256;
use cucumber::{gherkin::Step, given, then};

use crate::common::{
    helpers::{self, is_cspr_close_enough, is_rep_close_enough, to_cspr, to_rep},
    params::{Account, Balance, U512},
    DaoWorld,
};

#[then(expr = "{account} is a VA")]
fn is_va(w: &mut DaoWorld, va: Account) {
    let va = w.get_address(&va);
    assert!(w.is_va(va));
}

#[then(expr = "{account} is not a VA")]
fn is_not_va(w: &mut DaoWorld, va: Account) {
    let va = w.get_address(&va);
    assert!(!w.is_va(va));
}

#[then(expr = "total reputation is {int}")]
fn total_reputation(w: &mut DaoWorld, total_reputation_expected: u64) {
    let total_reputation = w.reputation_token.total_supply();
    let expected = U256::from(total_reputation_expected) * 1_000_000_000;
    assert!(
        is_rep_close_enough(total_reputation, expected),
        "REP total supply should be {:?} but is {:?}",
        expected,
        total_reputation
    );
}

#[then(expr = "balances are")]
#[then(expr = "users balances are")]
fn assert_balances(world: &mut DaoWorld, step: &Step) {
    let labels = step
        .table
        .as_ref()
        .unwrap()
        .rows
        .first()
        .expect("Missing labels");

    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let account = helpers::parse::<Account>(row.get(0), "Could't parse account");
        for (idx, label) in labels.iter().enumerate() {
            match label.as_str() {
                "REP balance" => {
                    let expected_reputation_balance = helpers::parse_or_default::<Balance>(row.get(idx));
                    let real_reputation_balance = world.reputation_balance(&account);

                    assert!(
                        is_rep_close_enough(*expected_reputation_balance, *real_reputation_balance),
                        "For account {:?} CSPR balance should be {:?} but is {:?}",
                        account,
                        expected_reputation_balance,
                        real_reputation_balance
                    );
                }
                "REP stake" => {
                    let expected_reputation_stake = helpers::parse_or_default::<Balance>(row.get(idx));
                    let real_reputation_stake = world.staked_reputation(&account);

                    assert!(
                        is_rep_close_enough(*expected_reputation_stake, *real_reputation_stake),
                        "For account {:?} CSPR balance should be {:?} but is {:?}",
                        account,
                        expected_reputation_stake,
                        real_reputation_stake
                    );
                }
                "CSPR balance" => {
                    let expected_cspr_balance = helpers::parse_or_default::<U512>(row.get(idx));
                    let real_cspr_balance = world.get_cspr_balance2(&account);

                    assert!(
                        is_cspr_close_enough(*expected_cspr_balance, real_cspr_balance),
                        "For account {:?} CSPR balance should be {:?} but is {:?}",
                        account,
                        expected_cspr_balance,
                        real_cspr_balance
                    );
                }
                _ => {}
            }
        }
    }
}

#[then(expr = "{account} is a VA account")]
fn assert_account_is_va(world: &mut DaoWorld, account: Account) {
    assert!(world.is_va_account(&account));
}
