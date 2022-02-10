use casper_types::{account::AccountHash, ContractPackageHash};
use macros::{generate_contract, Contract};

#[derive(Contract)]
pub struct ImportantContract {}

generate_contract!(
    trait ImportantContractInterface {
        fn init(&mut self);
        fn mint(&mut self, recipient: utils::Address, amount: casper_types::U256);
        fn burn(&mut self, owner: utils::Address, amount: casper_types::U256);
    }
);

fn main() {
    ImportantContract::install();
    ImportantContract::entry_points();

    let mut caller = ImportantContractInterfaceCaller {
        contract_package_hash: ContractPackageHash::new([0; 32]),
    };
    let address = utils::Address::Account(casper_types::account::AccountHash::new([0; 32]));

    caller.init();
    caller.mint(address, casper_types::U256::one());
    caller.burn(address, casper_types::U256::one());
}
