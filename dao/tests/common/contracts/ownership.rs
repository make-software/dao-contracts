use dao::utils::Error::NoProposedOwner;
use odra::test_env;
use odra::types::address::OdraAddress;
use odra::types::Address;

use crate::common::{params::Account, DaoWorld};

#[odra::external_contract]
trait AccessControl {
    fn propose_new_owner(&mut self, owner: Address);
    fn accept_new_owner(&mut self);
    fn is_whitelisted(&self, address: Address) -> bool;
    fn remove_from_whitelist(&mut self, address: Address);
    fn add_to_whitelist(&mut self, address: Address);
}

#[odra::external_contract]
pub trait OwnerContract {
    fn accept_ownership(&mut self, contract_address: &Address);
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
        let new_owner_address = self.get_address(new_owner);
        let contract_address = self.get_address(contract);
        self.set_caller(caller);
        AccessControlRef::at(&contract_address).propose_new_owner(new_owner_address);
        if new_owner_address.is_contract() {
            self.accept_new_owner_as_contract(contract, new_owner);
        } else {
            self.set_caller(new_owner);
            self.accept_new_owner(contract);
        }
    }

    pub fn propose_new_owner(&mut self, contract: &Account, new_owner: &Account) {
        let new_owner = self.get_address(new_owner);
        let contract = self.get_address(contract);
        AccessControlRef::at(&contract).propose_new_owner(new_owner);
    }

    pub fn accept_new_owner(&mut self, contract: &Account) {
        let contract = self.get_address(contract);
        AccessControlRef::at(&contract).accept_new_owner();
    }

    pub fn accept_new_owner_fails(&mut self, contract: &Account) {
        let contract = self.get_address(contract);
        test_env::assert_exception(NoProposedOwner, || {
            AccessControlRef::at(&contract).accept_new_owner()
        });
    }

    pub fn accept_new_owner_as_contract(&mut self, contract: &Account, new_owner: &Account) {
        let new_owner = self.get_address(new_owner);
        let contract = self.get_address(contract);
        OwnerContractRef::at(&new_owner).accept_ownership(&contract);
    }

    pub fn is_whitelisted(&mut self, contract: &Account, account: &Account) -> bool {
        let account = self.get_address(account);
        let contract = self.get_address(contract);
        AccessControlRef::at(&contract).is_whitelisted(account)
    }
}
