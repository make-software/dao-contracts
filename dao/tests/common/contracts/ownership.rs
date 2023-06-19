use odra::types::Address;

use crate::common::{params::Account, DaoWorld};

#[odra::external_contract]
trait AccessControl {
    fn change_ownership(&mut self, owner: Address);
    fn is_whitelisted(&self, address: Address) -> bool;
    fn remove_from_whitelist(&mut self, address: Address);
    fn add_to_whitelist(&mut self, address: Address);
}

#[odra::external_contract]
trait Ownable {
    fn get_owner(&self) -> Option<Address>;
}

#[allow(dead_code)]
impl DaoWorld {
    pub fn whitelist_account(&mut self, contract: &Account, caller: &Account, user: &Account) {
        let user = self.get_address(user);
        let contract = self.get_address(contract);

        self.set_caller(caller);
        AccessControlRef::at(&contract).add_to_whitelist(user);
    }

    pub fn remove_from_whitelist(&mut self, contract: &Account, caller: &Account, user: &Account) {
        let user = self.get_address(user);
        let contract = self.get_address(contract);

        self.set_caller(caller);
        AccessControlRef::at(&contract).remove_from_whitelist(user);
    }

    pub fn get_owner(&mut self, contract: &Account) -> Option<Address> {
        let contract = self.get_address(contract);
        OwnableRef::at(&contract).get_owner()
    }

    pub fn change_ownership(&mut self, contract: &Account, caller: &Account, new_owner: &Account) {
        let new_owner = self.get_address(new_owner);
        let contract = self.get_address(contract);

        self.set_caller(caller);
        AccessControlRef::at(&contract).change_ownership(new_owner);
    }

    pub fn is_whitelisted(&mut self, contract: &Account, account: &Account) -> bool {
        let account = self.get_address(account);
        let contract = self.get_address(contract);
        AccessControlRef::at(&contract).is_whitelisted(account)
    }
}
