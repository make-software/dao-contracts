use casper_types::U256;
use cucumber::{gherkin::Step, given, then};

use crate::common::{
    helpers::{self, is_cspr_close_enough, is_rep_close_enough, to_cspr, to_rep},
    params::{Account, U256 as pU256},
    DaoWorld,
};

#[given(expr = "following balances")]
fn starting_balances(w: &mut DaoWorld, step: &Step) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let name = row[0].as_str();
        let cspr_balance = to_cspr(&row[1]);
        let rep_balance = to_rep(&row[2]);

        // set balances
        let address = w.named_address(name);
        w.set_cspr_balance(address, cspr_balance);
        w.set_rep_balance(address, rep_balance);
    }
}

#[then(expr = "{word} is a VA")]
fn is_va(w: &mut DaoWorld, va_name: String) {
    let va = w.named_address(va_name);
    assert!(w.is_va(va));
}

#[then(expr = "{word} is not a VA")]
fn is_not_va(w: &mut DaoWorld, va_name: String) {
    let va = w.named_address(va_name);
    assert!(!w.is_va(va));
}

#[then(expr = "total reputation is {int}")]
fn total_reputation(w: &mut DaoWorld, total_reputation_expected: u64) {
    let total_reputation = w.reputation_token.total_supply();
    assert_eq!(
        total_reputation,
        U256::from(total_reputation_expected * 1_000_000_000)
    );
}

#[then(expr = "balances are")]
fn balances(w: &mut DaoWorld, step: &Step) {
    // let (total_rep_supply, all_rep_balances) = w.reputation_token.all_balances();
    // dbg!(total_rep_supply);
    // dbg!(all_rep_balances.balances);

    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let name = row.get(0).unwrap();
        let address = w.named_address(name);

        // Check REP balance.
        let expected_rep_balance = to_rep(&row[2]);
        let real_rep_balance = w.get_rep_balance(address);
        assert!(
            is_rep_close_enough(expected_rep_balance, real_rep_balance),
            "For account {} REP balance should be {:?} but is {:?}",
            name,
            expected_rep_balance,
            real_rep_balance
        );

        // Check CSPR balance
        let expected_cspr_balance = to_cspr(&row[1]);
        let real_cspr_balance = w.get_cspr_balance(address);
        assert!(
            is_cspr_close_enough(expected_cspr_balance, real_cspr_balance),
            "For account {} CSPR balance should be {:?} but is {:?}",
            name,
            expected_cspr_balance,
            real_cspr_balance
        );

        // Check staked REP balance.
        let expected_rep_stake = to_rep(&row[3]);
        let real_rep_stake = w.reputation_token.get_stake(address);
        assert!(
            is_rep_close_enough(expected_rep_stake, real_rep_stake),
            "For account {} REP stake should be {:?} but is {:?}",
            name,
            expected_rep_stake,
            real_rep_stake
        );
    }
}

#[then(expr = "users balances are")]
fn assert_balances(world: &mut DaoWorld, step: &Step) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let account = helpers::parse::<Account>(row.get(0), "Could't parse account");
        let expected_balance = helpers::parse_or_default::<pU256>(row.get(1));
        let expected_stake = helpers::parse_or_default::<pU256>(row.get(2));

        assert_eq!(world.reputation_balance(&account), expected_balance);

        assert_eq!(world.staked_reputation(&account), expected_stake);
    }
}
