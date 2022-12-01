mod common;
mod steps;

use common::DaoWorld;
use cucumber::World as _;

fn main() {
    let runner = DaoWorld::cucumber().run_and_exit("tests/features/variables/");
    // let runner = DaoWorld::cucumber()
    //     .run_and_exit("tests/features/variables/voting_clearness_delta.feature");
    futures::executor::block_on(runner);
}
