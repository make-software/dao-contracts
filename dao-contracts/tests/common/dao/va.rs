use crate::common::{params::Account, DaoWorld};

#[allow(dead_code)]
impl DaoWorld {
    pub fn is_va_account(&self, account: &Account) -> bool {
        let address = self.get_address(account);
        dbg!(address);
        let res = !self.va_token.balance_of(address).is_zero();
        dbg!(res);
        res
    }
}
