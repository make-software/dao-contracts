use casper_dao_utils::TestContract;

use crate::common::{
    params::{Account, U256},
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn is_va_account(&self, account: &Account) -> bool {
        let address = self.get_address(account);
        !self.va_token.balance_of(address).is_zero()
    }

    pub fn checked_mint_va_token(
        &mut self,
        minter: &Account,
        recipient: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let minter = self.get_address(minter);
        let recipient = self.get_address(recipient);

        self.va_token.as_account(minter).mint(recipient)
    }

    pub fn mint_va_token(&mut self, minter: &Account, recipient: &Account) {
        self.checked_mint_va_token(minter, recipient)
            .expect("A VA Token should be minted successfully");
    }

    pub fn checked_burn_va_token(
        &mut self,
        burner: &Account,
        holder: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let token_id = self.get_va_token_id(holder);
        let burner = self.get_address(burner);

        self.va_token.as_account(burner).burn(*token_id)
    }

    pub fn burn_va_token(&mut self, burner: &Account, holder: &Account) {
        self.checked_burn_va_token(burner, holder)
            .expect("VA Token should burned successfully");
    }

    pub fn va_token_balance_of(&self, account: &Account) -> U256 {
        let address = self.get_address(account);

        U256(self.va_token.balance_of(address))
    }

    pub fn get_va_token_id(&self, holder: &Account) -> U256 {
        let holder = self.get_address(holder);
        let id = self
            .va_token
            .token_id(holder)
            .expect("Holder should own a token");
        U256(id)
    }

    pub fn is_va(&self, account: &Account) -> bool {
        let address = self.get_address(account);
        !self.va_token.balance_of(address).is_zero()
    }
}
