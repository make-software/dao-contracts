#![allow(dead_code)]
#![allow(unused_variables)]

use casper_dao_utils::casper_dao_macros::{casper_contract_interface, Instance};

#[casper_contract_interface]
pub trait ImportantContractInterface {
    fn init(&mut self, first_arg: casper_types::U512, second_arg: casper_types::U512);
    fn mint(&mut self, recipient: casper_dao_utils::Address, amount: casper_types::U512);
    fn balance_of(&self, to: casper_dao_utils::Address) -> casper_types::U512;
    fn argless(&mut self);
}

#[derive(Instance)]
pub struct ImportantContract {}

impl ImportantContractInterface for ImportantContract {
    fn init(&mut self, first_arg: casper_types::U512, second_arg: casper_types::U512) {}

    fn mint(&mut self, recipient: casper_dao_utils::Address, amount: casper_types::U512) {}

    fn balance_of(&self, to: casper_dao_utils::Address) -> casper_types::U512 {
        casper_types::U512::default()
    }

    fn argless(&mut self) {}
}
