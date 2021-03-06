#![allow(dead_code)]
#![allow(unused_variables)]

use casper_dao_utils::casper_dao_macros::casper_contract_interface;
use casper_dao_utils::casper_dao_macros::Instance;

#[casper_contract_interface]
pub trait ImportantContractInterface {
    fn init(&mut self, first_arg: casper_types::U256, second_arg: casper_types::U256);
    fn mint(&mut self, recipient: casper_dao_utils::Address, amount: casper_types::U256);
    fn balance_of(&self, to: casper_dao_utils::Address) -> casper_types::U256;
    fn argless(&mut self);
}

#[derive(Instance)]
pub struct ImportantContract {}

impl ImportantContractInterface for ImportantContract {
    fn init(&mut self, first_arg: casper_types::U256, second_arg: casper_types::U256) {}

    fn mint(&mut self, recipient: casper_dao_utils::Address, amount: casper_types::U256) {}

    fn balance_of(&self, to: casper_dao_utils::Address) -> casper_types::U256 {
        casper_types::U256::default()
    }

    fn argless(&mut self) {}
}
