use casper_dao_macros::casper_contract_interface;
use casper_types::{account::AccountHash, ContractPackageHash};

#[derive(Default)]
pub struct ImportantContract {}

#[casper_contract_interface]
trait ImportantContractInterface {
    fn init(&mut self);
    fn mint(&mut self, recipient: casper_dao_utils::Address, amount: casper_types::U256);
    fn burn(&mut self, owner: casper_dao_utils::Address, amount: casper_types::U256);
}

fn main() {
    ImportantContract::install();
    ImportantContract::entry_points();

    let mut caller = ImportantContractCaller {
        contract_package_hash: ContractPackageHash::new([0; 32]),
    };
    let address = casper_dao_utils::Address::Account(AccountHash::new([0; 32]));

    caller.init();
    caller.mint(address, casper_types::U256::one());
    caller.burn(address, casper_types::U256::one());
}
