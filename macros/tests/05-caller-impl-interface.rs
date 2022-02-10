use casper_dao_macros::{generate_contract, Contract};
use casper_types::{account::AccountHash, ContractPackageHash};

#[derive(Contract)]
pub struct ImportantContract {}

generate_contract!(
    trait ImportantContractInterface {
        fn init(&mut self);
        fn mint(&mut self, recipient: casper_dao_utils::Address, amount: casper_types::U256);
        fn burn(&mut self, owner: casper_dao_utils::Address, amount: casper_types::U256);
    }
);

fn main() {
    ImportantContract::install();
    ImportantContract::entry_points();

    let mut caller = ImportantContractInterfaceCaller {
        contract_package_hash: ContractPackageHash::new([0; 32]),
    };
    let address = casper_dao_utils::Address::Account(AccountHash::new([0; 32]));

    caller.init();
    caller.mint(address, casper_types::U256::one());
    caller.burn(address, casper_types::U256::one());
}
