use casper_dao_utils::TestContract;
use cucumber::{gherkin::Step, then};

use crate::common::{
    params::{events::Event, Contract},
    DaoWorld,
};

#[then(expr = "{contract} contract emits events")]
fn assert_event(world: &mut DaoWorld, step: &Step, contract: Contract) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);

    let total_events = world.kyc_token.events_count();
    let expected_events_count = table.len() as i32;
    // In a scenario are given last n events, so we need to skip first #events-n events.
    let skipped_events = total_events - expected_events_count;

    for (idx, row) in table.enumerate() {
        let event: Event = row.into();
        let event_idx = idx as i32 + skipped_events;
        world.assert_event(&contract, event_idx, event);
    }
}
