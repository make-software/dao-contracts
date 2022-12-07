use casper_dao_utils::BlockTime;
use cucumber::{gherkin::Step, given, when};

use crate::common::{
    helpers::{to_seconds, value_to_bytes},
    params::TimeUnit,
    DaoWorld,
};

#[when(expr = "{int} {time_unit} passed")]
fn advance_time(w: &mut DaoWorld, amount: BlockTime, unit: TimeUnit) {
    w.advance_time(to_seconds(amount, unit));
}

#[given(expr = "following configuration")]
fn configuration(w: &mut DaoWorld, step: &Step) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let variable = row[0].as_str();
        let value = row[1].as_str();
        w.set_variable(variable.to_string(), value_to_bytes(value, variable));
    }
}
