mod common;
mod steps;

use common::DaoWorld;
use cucumber::World as _;

fn main() {
    let runner = DaoWorld::cucumber().run_and_exit("tests/features/bid_escrow/cancel_job_offer.feature");
    futures::executor::block_on(runner);
}
