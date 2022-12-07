mod common;
mod steps;

use common::DaoWorld;
use cucumber::World as _;

fn main() {
    // let runner = DaoWorld::cucumber().run_and_exit("tests/features/slashing/");
    let runner = DaoWorld::cucumber().run_and_exit("tests/features/slashing/complex_full_voting_creator.feature");
    futures::executor::block_on(runner);
}
