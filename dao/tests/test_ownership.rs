mod common;
mod steps;

use common::DaoWorld;
use cucumber::World as _;
use cucumber_runner::SyncRunner;

fn main() {
    let runner = DaoWorld::cucumber()
        .with_runner(SyncRunner::default())
        .run_and_exit("tests/features/ownership/");
    futures::executor::block_on(runner);
}
