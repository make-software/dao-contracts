mod common;
mod steps;

use common::DaoWorld;
use cucumber::writer::Libtest;
use cucumber::World as _;

fn main() {
    let runner = DaoWorld::cucumber()
        .with_writer(Libtest::or_basic())
        .with_runner(
            cucumber_runner::SyncRunner::default()
                .with_before_scenario(|scenario| {
                    println!("Before scenario: \"{}\"", scenario.name);
                })
                .with_after_scenario(|scenario| {
                    println!("After scenario: \"{}\"", scenario.name);
                }),
        )
        .run_and_exit("tests/features/bid_escrow/");
    futures::executor::block_on(runner);
}
