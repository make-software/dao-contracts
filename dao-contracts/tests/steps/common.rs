use cucumber::{gherkin::Step, given, when};

use crate::common::{
    helpers::{multiplier, value_to_bytes},
    DaoWorld,
};

#[when(expr = "{int} {word} passed")]
fn advance_time(w: &mut DaoWorld, amount: u32, unit: String) {
    let multiplier = multiplier(unit);
    w.advance_time(amount * multiplier);
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
