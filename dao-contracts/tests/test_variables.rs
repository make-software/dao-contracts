mod common;
mod steps;

use common::DaoWorld;
use cucumber::World as _;

fn main() {
    // let runner = DaoWorld::cucumber().run_and_exit("tests/features/variables/");
    let runner = DaoWorld::cucumber()
        .run_and_exit("tests/features/bid_escrow/variables/time_between_votings.feature");
    futures::executor::block_on(runner);
}
