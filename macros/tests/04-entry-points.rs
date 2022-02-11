use casper_dao_macros::{casper_contract_interface, CasperContract};
use casper_types::EntryPoints;

#[derive(Default, CasperContract)]
pub struct ImportantContract {}

#[casper_contract_interface]
trait ImportantContractInterface {
    fn init(&mut self);
    fn mint(&mut self, recipient: casper_dao_utils::Address, amount: casper_types::U256);
    fn burn(&mut self, owner: casper_dao_utils::Address, amount: casper_types::U256);
}

fn main() {
    let ep: EntryPoints = ImportantContract::entry_points();

    assert_eq!(ep.keys().count(), 3);
    assert!(ep.has_entry_point("init"));
    assert!(ep.has_entry_point("mint"));
    assert!(ep.has_entry_point("burn"));
}
