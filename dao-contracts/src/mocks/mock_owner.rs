use casper_dao_modules::Owner;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    Address,
};

use delegate::delegate;

#[casper_contract_interface]
trait MockOwnerContractInterface {
    fn init(&mut self) {}
    fn initialize_module(&mut self, owner: Address);
    fn change_ownership(&mut self, owner: Address);
    fn ensure_owner(&self);
    fn get_owner(&self) -> Option<Address>;
}

#[derive(Instance)]
pub struct MockOwnerContract {
    owner: Owner,
}

impl MockOwnerContractInterface for MockOwnerContract {
    delegate! {
        to self.owner {
            #[call(init)]
            fn initialize_module(&mut self, owner: Address);
            fn change_ownership(&mut self, owner: Address);
            fn ensure_owner(&self);
            fn get_owner(&self) -> Option<Address>;
        }
    }
}
