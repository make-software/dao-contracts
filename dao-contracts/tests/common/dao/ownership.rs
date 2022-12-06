use casper_dao_utils::{Address, TestContract};

use crate::common::{
    params::{Account, Contract},
    DaoWorld,
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

    pub fn whitelist_contract(
        &mut self,
        contract: &Contract,
        caller: &Account,
        contract_to_whitelist: &Contract,
    ) -> Result<(), casper_dao_utils::Error> {
        let address = match contract_to_whitelist {
            Contract::KycToken => self.kyc_token.address(),
            Contract::VaToken => self.va_token.address(),
            Contract::ReputationToken => self.reputation_token.address(),
            Contract::VariableRepository => self.variable_repo.address(),
            Contract::KycVoter => self.kyc_voter.address(),
            Contract::BidEscrow => self.bid_escrow.address(),
            Contract::SlashingVoter => self.slashing_voter.address(),
        };
        let caller = self.get_address(caller);

        self.whitelist(contract, caller, address)
    }

    pub fn remove_from_whitelist(
        &mut self,
        contract: &Contract,
        caller: &Account,
        user: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let user = self.get_address(user);
        let caller = self.get_address(caller);

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
        let new_owner = self.get_address(new_owner);
        let caller = self.get_address(caller);

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
        let account = self.get_address(account);

        match contract {
            Contract::KycToken => self.kyc_token.is_whitelisted(account),
            Contract::VaToken => self.va_token.is_whitelisted(account),
            Contract::ReputationToken => self.reputation_token.is_whitelisted(account),
            Contract::VariableRepository => self.variable_repo.is_whitelisted(account),
            _ => false,
        }
    }

    fn whitelist(
        &mut self,
        contract: &Contract,
        caller: Address,
        address: Address,
    ) -> Result<(), casper_dao_utils::Error> {
        match contract {
            Contract::KycToken => self.kyc_token.as_account(caller).add_to_whitelist(address),
            Contract::VaToken => self.va_token.as_account(caller).add_to_whitelist(address),
            Contract::ReputationToken => self
                .reputation_token
                .as_account(caller)
                .add_to_whitelist(address),
            Contract::VariableRepository => self
                .variable_repo
                .as_account(caller)
                .add_to_whitelist(address),
            _ => Err(casper_dao_utils::Error::Unknown),
        }
    }
}
