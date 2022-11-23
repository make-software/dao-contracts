
use casper_dao_utils::TestContract;

use crate::common::{DaoWorld, params::{nft::Account, common::Contract}};

impl DaoWorld {
    pub fn whitelist(&mut self, contract: &Contract, caller: &Account, user: &Account) {
        let user = user.get_address(self);
        let caller = caller.get_address(self);

        match contract {
            Contract::KycToken => self.kyc_token.as_account(caller).add_to_whitelist(user).unwrap(),
            Contract::VaToken => todo!(),
            Contract::ReputationToken => todo!(),
        }
    }
}
