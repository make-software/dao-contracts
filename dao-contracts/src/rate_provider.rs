use casper_dao_modules::Owner;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Variable,
};
use casper_types::U512;

#[casper_contract_interface]
pub trait CSPRRateProviderContractInterface {
    fn init(&mut self, rate: U512);
    fn get_rate(&self) -> U512;
    fn set_rate(&mut self, rate: U512);
}

#[derive(Instance)]
pub struct CSPRRateProviderContract {
    owner: Owner,
    rate: Variable<U512>,
}

impl CSPRRateProviderContractInterface for CSPRRateProviderContract {
    fn init(&mut self, rate: U512) {
        let deployer = caller();
        self.owner.init(deployer);
        self.set_rate(rate);
    }

    fn get_rate(&self) -> U512 {
        self.rate.get().unwrap_or_default()
    }

    fn set_rate(&mut self, rate: U512) {
        self.owner.ensure_owner();

        self.rate.set(rate);
    }
}
