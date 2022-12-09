mod common;
mod steps;

use common::DaoWorld;
use cucumber::World as _;

fn main() {
    // let runner = DaoWorld::cucumber().run_and_exit("tests/features/slashing/");
    let runner = DaoWorld::cucumber()
        .run_and_exit("tests/features/slashing/slashing_voter_contract.feature");
    futures::executor::block_on(runner);
}
