use casper_dao_utils::{Address, TestContract};

use crate::common::{
    params::{Account, Contract},
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn whitelist(
        &mut self,
        contract: &Contract,
        caller: &Account,
        user: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let user = user.get_address(self);
        let caller = caller.get_address(self);

        match contract {
            Contract::KycToken => self.kyc_token.as_account(caller).add_to_whitelist(user),
            Contract::VaToken => self.va_token.as_account(caller).add_to_whitelist(user),
            Contract::ReputationToken => self
                .reputation_token
                .as_account(caller)
                .add_to_whitelist(user),
            Contract::VariableRepository => {
                self.variable_repo.as_account(caller).add_to_whitelist(user)
            }
            _ => Err(casper_dao_utils::Error::Unknown),
        }
    }

    pub fn remove_from_whitelist(
        &mut self,
        contract: &Contract,
        caller: &Account,
        user: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let user = user.get_address(self);
        let caller = caller.get_address(self);

        match contract {
            Contract::KycToken => self
                .kyc_token
                .as_account(caller)
                .remove_from_whitelist(user),
            Contract::VaToken => self.va_token.as_account(caller).remove_from_whitelist(user),
            Contract::ReputationToken => self
                .reputation_token
                .as_account(caller)
                .remove_from_whitelist(user),
            Contract::VariableRepository => self
                .variable_repo
                .as_account(caller)
                .remove_from_whitelist(user),
            _ => Err(casper_dao_utils::Error::Unknown),
        }
    }

    pub fn get_owner(&mut self, contract: &Contract) -> Option<Address> {
        match contract {
            Contract::KycToken => self.kyc_token.get_owner(),
            Contract::VaToken => self.va_token.get_owner(),
            Contract::ReputationToken => self.reputation_token.get_owner(),
            Contract::VariableRepository => self.variable_repo.get_owner(),
            _ => None,
        }
    }

    pub fn change_ownership(
        &mut self,
        contract: &Contract,
        caller: &Account,
        new_owner: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let new_owner = new_owner.get_address(self);
        let caller = caller.get_address(self);

        match contract {
            Contract::KycToken => self
                .kyc_token
                .as_account(caller)
                .change_ownership(new_owner),
            Contract::VaToken => self.va_token.as_account(caller).change_ownership(new_owner),
            Contract::ReputationToken => self
                .reputation_token
                .as_account(caller)
                .change_ownership(new_owner),
            Contract::VariableRepository => self
                .variable_repo
                .as_account(caller)
                .change_ownership(new_owner),
            _ => Err(casper_dao_utils::Error::Unknown),
        }
    }

    pub fn is_whitelisted(&mut self, contract: &Contract, account: &Account) -> bool {
        let account = account.get_address(self);

        match contract {
            Contract::KycToken => self.kyc_token.is_whitelisted(account),
            Contract::VaToken => self.va_token.is_whitelisted(account),
            Contract::ReputationToken => self.reputation_token.is_whitelisted(account),
            Contract::VariableRepository => self.variable_repo.is_whitelisted(account),
            _ => false,
        }
    }
}
