use casper_types::U256;
use macros::{generate_contract, Contract};
use utils::Address;

#[derive(Contract)]
struct ImportantContract {}

generate_contract!(
    trait ReputationContractInterface {
        fn init(&mut self);
        fn mint(&mut self, recipient: Address, amount: U256);
        fn burn(&mut self, owner: Address, amount: U256);
    }
);

fn main() {
    ImportantContract::install();
    ImportantContract::entry_points();
}
