use casper_dao_macros::casper_contract_interface;
use casper_dao_utils::TestEnv;
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
    let mut contract_test = ImportantContractTest {
        env: TestEnv::new(),
        package_hash: ContractPackageHash::new([0; 32]),
        data: ImportantContract {},
    };

    let address = casper_dao_utils::Address::Account(AccountHash::new([0; 32]));

    contract_test.init();
    contract_test.mint(address, casper_types::U256::one());
    contract_test.burn(address, casper_types::U256::one());
}
