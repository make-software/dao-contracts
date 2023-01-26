//! Contains CSPR Rate Provider Contract definition and related abstractions.
//! 
//! TODO: short desc
use casper_dao_modules::Owner;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
    Variable,
};
use casper_types::U512;

#[casper_contract_interface]
pub trait CSPRRateProviderContractInterface {
    ///  Contract constructor.
    /// 
    ///  * sets the initial CSPR:Fiat rate.
    ///  * sets the deployer as the owner.
    ///
    ///  [Read more](Owner::init())
    fn init(&mut self, rate: U512);
    /// Gets the current CSPR:Fiat rate.
    fn get_rate(&self) -> U512;
    /// Updates the current CSPR:Fiat rate.
    ///
    /// # Errors
    /// * [`NotAnOwner`](casper_dao_utils::Error::NotAnOwner) if the caller is not the contract owner.
    fn set_rate(&mut self, rate: U512);
    /// Returns the address of the current owner.
    /// [`Read more`](Owner::get_owner()).
    fn get_owner(&self) -> Option<Address>;
}

/// CSPR Rate provider contract allows to read and write the current CSPR:Fiat rate.
/// Only the owner is eligible to update the rate, but any account can read the current value.
///
/// For details see [CSPRRateProviderContractInterface](CSPRRateProviderContractInterface).
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

    fn get_owner(&self) -> Option<Address> {
        self.owner.get_owner()
    }
}
