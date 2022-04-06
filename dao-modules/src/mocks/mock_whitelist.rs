use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    Address,
};

use delegate::delegate;

use crate::Whitelist;

#[casper_contract_interface]
trait MockWhitelistInterface {
    fn init(&mut self) {}
    fn add(&mut self, address: Address);
    fn remove(&mut self, address: Address);
    fn ensure_whitelisted(&self);
    fn is_whitelisted(&self, address: Address) -> bool;
}

#[derive(Instance)]
pub struct MockWhitelist {
    whitelist: Whitelist,
}

impl MockWhitelistInterface for MockWhitelist {
    delegate! {
        to self.whitelist {
            #[call(add_to_whitelist)]
            fn add(&mut self, address: Address);
            #[call(remove_from_whitelist)]
            fn remove(&mut self, address: Address);
            fn ensure_whitelisted(&self);
        }
    }

    fn is_whitelisted(&self, address: Address) -> bool {
        self.whitelist.is_whitelisted(&address)
    }
}
