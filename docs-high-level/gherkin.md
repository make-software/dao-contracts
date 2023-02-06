# BDD Testing with Gherkin
DAO Contracts use the cucumber crate to run tests written in [gherkin](https://cucumber.io/docs/gherkin/) language.

Feature files are located in `dao-contracts/tests/features` directory.
Test executables are located in `dao-contracts/tests` directory and need to be registered in the
Cargo.toml file like this:

```toml
[[test]]
name = "test_bid_escrow"
harness = false  # allows Cucumber to print output instead of libtest
required-features = ["test-support"]
```

## Running tests
To run the tests, for example for the `test_bid_escrow` executable, run the following command:

```bash
cargo test -p casper-dao-contracts --test test_bid_escrow
```

Keep in mind that tests require wasm files to be compiled. You can look at, or use the ready
recipe in the Makefile:

```bash
make test-bid-escrow
```

## Writing tests
The executable file is pretty straightforward, here's a boilerplate used for the `test_bid_escrow`:

```rust
mod common;
mod steps;

use common::DaoWorld;
use cucumber::World as _;

fn main() {
    let runner = DaoWorld::cucumber().run_and_exit("tests/features/bid_escrow/"); // path to feature files
    futures::executor::block_on(runner);
}
```

Next, we need to describe in our code what each step in the feature file should do. This is done in files located in
`dao-contracts/tests/steps` directory. For example, here's a code snippet explaining the cucumber what to do with
the `When InternalWorker submits the JobProof of Job 0` line from the feature file:

```rust
#[when(expr = "{account} submits the JobProof of Job {int}")]
fn submit_job_proof(w: &mut DaoWorld, worker: Account, job_id: JobId) {
    let worker = w.get_address(&worker);
    w.bid_escrow
        .as_account(worker)
        .submit_job_proof(job_id, DocumentHash::from("Job Proof"))
        .unwrap();
}
```

To learn more about cucumber, check out its [documentation](https://docs.rs/cucumber/latest/cucumber/).