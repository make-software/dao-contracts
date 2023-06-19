mod balances;
mod bid_escrow;
mod common;
mod kyc;
mod ownership;
mod setup;
mod va;
mod variables;
mod voting;

pub(crate) fn suppress<F: FnOnce() -> R, R>(f: F) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
}
