use cucumber::{gherkin::Step, given, when};
use odra::types::BlockTime;

use crate::common::{
    helpers::{to_milliseconds, value_to_bytes},
    params::TimeUnit,
    DaoWorld,
};

#[when(expr = "{int} {time_unit} passed")]
fn advance_time(world: &mut DaoWorld, amount: BlockTime, unit: TimeUnit) {
    world.advance_time(to_milliseconds(amount, unit));
}

#[given(expr = "following configuration")]
fn configuration(world: &mut DaoWorld, step: &Step) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);
    for row in table {
        let variable = row[0].as_str();
        let value = row[1].as_str();
        world.set_variable(variable.to_string(), value_to_bytes(value, variable));
    }
}
