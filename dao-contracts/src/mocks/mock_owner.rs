use casper_dao_modules::Owner;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env, Address,
};

use delegate::delegate;

#[casper_contract_interface]
trait MockOwnerInterface {
    fn init(&mut self);
    fn change_ownership(&mut self, owner: Address);
    fn ensure_owner(&self);
    fn get_owner(&self) -> Option<Address>;
}

#[derive(Instance)]
pub struct MockOwner {
    owner: Owner,
}

impl MockOwnerInterface for MockOwner {
    fn init(&mut self) {
        self.owner.init(casper_env::caller());
    }

    delegate! {
        to self.owner {
            fn change_ownership(&mut self, owner: Address);
            fn ensure_owner(&self);
            fn get_owner(&self) -> Option<Address>;
        }
    }
}
