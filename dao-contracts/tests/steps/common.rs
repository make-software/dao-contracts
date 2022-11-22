use cucumber::when;

use crate::common::DaoWorld;

#[when(expr = "{int} {word} passed")]
fn bid_is_posted(w: &mut DaoWorld, amount: u32, unit: String) {
    let multiplier = match unit.as_str() {
        "seconds" => 1,
        "minutes" => 60,
        "hours" => 60 * 60,
        "days" => 60 * 60 * 24,
        _ => panic!("Unknown unit option - it should be either seconds, minutes, hours or days"),
    };

    w.advance_time(amount * multiplier);
}
