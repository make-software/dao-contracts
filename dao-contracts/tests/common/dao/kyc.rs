use casper_dao_utils::TestContract;

use crate::common::{
    params::{Account, U256},
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn mint_kyc_token(
        &mut self,
        minter: &Account,
        recipient: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let minter = self.get_address(minter);
        let recipient = self.get_address(recipient);

        let result = self.kyc_token.as_account(minter).mint(recipient);

        if result.is_ok() {
            self.kyc_count += casper_types::U256::one();
        }
        result
    }

    pub fn checked_mint_kyc_token(&mut self, minter: &Account, recipient: &Account) {
        self.mint_kyc_token(minter, recipient)
            .expect("A token should be minted");
    }

    pub fn burn_kyc_token(
        &mut self,
        burner: &Account,
        holder: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let token_id = self.get_kyc_token_id(holder);
        let burner = self.get_address(burner);

        let result = self.kyc_token.as_account(burner).burn(*token_id);

        if result.is_ok() {
            self.kyc_count -= casper_types::U256::one();
        }
        result
    }

    pub fn checked_burn_kyc_token(&mut self, minter: &Account, recipient: &Account) {
        self.burn_kyc_token(minter, recipient)
            .expect("A token should be burned");
    }

    pub fn get_kyc_token_id(&self, holder: &Account) -> U256 {
        let holder = self.get_address(holder);
        let id = self
            .kyc_token
            .token_id(holder)
            .expect("Holder should own a token");
        U256(id)
    }
}