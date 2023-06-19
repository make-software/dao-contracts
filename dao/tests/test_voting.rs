mod common;
mod steps;

use common::DaoWorld;
use cucumber::World as _;

fn main() {
    let runner = DaoWorld::cucumber()
        // TODO: Make it work with SyncRunner
        .with_runner(cucumber_runner::SyncRunner::default())
        .run_and_exit("tests/features/voting/");
    futures::executor::block_on(runner);
}
