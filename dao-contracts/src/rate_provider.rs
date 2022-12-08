use casper_dao_modules::Owner;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    Variable, casper_env::caller,
};

#[casper_contract_interface]
pub trait CSPRRateProviderContractInterface {
    fn init(&mut self);
    fn get_rate(&self) -> u32;
    fn set_rate(&mut self, rate: u32);
}

#[derive(Instance)]
pub struct CSPRRateProviderContract {
    owner: Owner,
    rate: Variable<u32>,
}

impl CSPRRateProviderContractInterface for CSPRRateProviderContract {
    fn init(&mut self) {
        let deployer = caller();
        self.owner.init(deployer);
    }

    fn get_rate(&self) -> u32 {
        self.rate.get().unwrap_or_default()
    }

    fn set_rate(&mut self, rate: u32) {
        self.owner.ensure_owner();

        self.rate.set(rate);
    }
}
