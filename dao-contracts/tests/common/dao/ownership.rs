use casper_dao_utils::{Address, TestContract};

use crate::{
    common::{
        params::{Account, Contract},
        DaoWorld,
    },
    on_contract,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn whitelist_account(
        &mut self,
        contract: &Contract,
        caller: &Account,
        user: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let user = self.get_address(user);
        let caller = self.get_address(caller);

        self.whitelist(contract, caller, user)
    }

    pub fn remove_from_whitelist(
        &mut self,
        contract: &Contract,
        caller: &Account,
        user: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let user = self.get_address(user);
        let caller = self.get_address(caller);

        on_contract!(self, caller, contract, remove_from_whitelist(user))
    }

    pub fn get_owner(&mut self, contract: &Contract) -> Option<Address> {
        if let Contract::CSPRRateProvider = contract {
            self.rate_provider.get_owner()
        } else {
            on_contract!(self, contract, get_owner())
        }
    }

    pub fn change_ownership(
        &mut self,
        contract: &Contract,
        caller: &Account,
        new_owner: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let new_owner = self.get_address(new_owner);
        let caller = self.get_address(caller);

        on_contract!(self, caller, contract, change_ownership(new_owner))
    }

    pub fn is_whitelisted(&mut self, contract: &Contract, account: &Account) -> bool {
        let account = self.get_address(account);
        on_contract!(self, contract, is_whitelisted(account))
    }

    fn whitelist(
        &mut self,
        contract: &Contract,
        caller: Address,
        address: Address,
    ) -> Result<(), casper_dao_utils::Error> {
        on_contract!(self, caller, contract, add_to_whitelist(address))
    }
}
