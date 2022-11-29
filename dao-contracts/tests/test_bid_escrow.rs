mod common;
mod steps;

use common::DaoWorld;
use cucumber::World as _;

fn main() {

    // let names = vec![
    //     // OK
    //     "internal_worker.feature",
    //     "internal_worker_failed.feature",
    //     "internal_worker_quorum_not_reached.feature",

    //     "external_to_va_worker.feature",
    //     "external_to_va_worker_failed.feature",
    //     "external_to_va_worker_quorum_not_reached.feature",
        
    //     "external_not_to_va_worker.feature",
    //     "external_not_to_va_worker_quorum_not_reached.feature",
    //     "external_not_to_va_worker_failed.feature",
    // ];


    // for name in names {
    //     let path = format!("tests/features/bid_escrow/{}", name);
    //     let runner = DaoWorld::cucumber().run_and_exit(path);
    //     futures::executor::block_on(runner);
    // }
    
    let runner = DaoWorld::cucumber()
        .run_and_exit("tests/features/bid_escrow/");
    futures::executor::block_on(runner);
}
