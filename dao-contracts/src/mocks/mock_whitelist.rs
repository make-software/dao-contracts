use casper_dao_modules::Whitelist;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    Address,
};

use delegate::delegate;

#[casper_contract_interface]
trait MockWhitelistInterface {
    fn init(&mut self) {}
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
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
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
            fn ensure_whitelisted(&self);
        }
    }

    fn is_whitelisted(&self, address: Address) -> bool {
        self.whitelist.is_whitelisted(&address)
    }
}
