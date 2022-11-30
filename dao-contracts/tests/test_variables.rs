mod common;
mod steps;

use common::DaoWorld;
use cucumber::World as _;

fn main() {
    // let runner = DaoWorld::cucumber().run_and_exit("tests/features/variables/");
    let runner = DaoWorld::cucumber()
        .run_and_exit("tests/features/variables/va_bid_acceptance_timeout.feature");
    futures::executor::block_on(runner);
}
