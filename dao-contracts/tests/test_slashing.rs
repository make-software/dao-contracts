mod common;
mod steps;

use common::DaoWorld;

use cucumber::World as _;

fn main() {
    // let runner = DaoWorld::cucumber().run_and_exit("tests/features/slashing/");
    let runner = DaoWorld::cucumber()
    .run_and_exit("tests/features/slashing/complex_non_full.feature");
    futures::executor::block_on(runner);
}
