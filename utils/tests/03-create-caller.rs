use casper_dao_macros::casper_contract_interface;
use casper_types::ContractPackageHash;

#[derive(Default)]
pub struct ImportantContract {}

#[casper_contract_interface]
trait ImportantContractInterface {
    fn init(&mut self);
    fn mint(&mut self, recipient: casper_dao_utils::Address, amount: casper_types::U256);
    fn burn(&mut self, owner: casper_dao_utils::Address, amount: casper_types::U256);
}

fn main() {
    let _caller = ImportantContractCaller {
        contract_package_hash: ContractPackageHash::new([0; 32]),
    };
}
