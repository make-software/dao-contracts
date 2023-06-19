use crate::common::{
    params::{Account, CsprBalance},
    DaoWorld,
};

impl DaoWorld {
    pub fn set_cspr_rate(&mut self, rate: CsprBalance) {
        self.rate_provider.set_rate(rate.0);
    }

    pub fn set_cspr_rate_by(&mut self, rate: CsprBalance, account: &Account) {
        self.set_caller(account);
        self.rate_provider.set_rate(rate.0);
    }

    pub fn get_cspr_rate(&self) -> CsprBalance {
        CsprBalance(self.rate_provider.get_rate())
    }
}
