use casper_types::{ContractPackageHash, U256};
use macros::{generate_contract, Contract};
use utils::Address;

#[derive(Contract)]
pub struct ImportantContract {}

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

    let _caller = ReputationContractInterfaceCaller {
        contract_package_hash: ContractPackageHash::new([0; 32]),
    };
}
