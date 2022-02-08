use casper_types::U256;

use crate::{Address, Mapping, Variable};

pub const EP_INIT: &str = "init";
pub const EP_MINT: &str = "mint";
pub const EP_BURN: &str = "burn";
pub const EP_TRANSFER_FROM: &str = "transfer_from";
pub const EP_STAKE: &str = "stake";
pub const EP_UNSTAKE: &str = "unstake";
pub const EP_REMOVE_FROM_WHITELIST: &str = "remove_from_whitelist";
pub const EP_ADD_TO_WHITELIST: &str = "add_to_whitelist";
pub const EP_CHANGE_OWNERSHIP: &str = "change_ownership";

pub const PARAM_RECIPIENT: &str = "recipient";
pub const PARAM_AMOUNT: &str = "amount";
pub const PARAM_OWNER: &str = "owner";
pub const PARAM_ADDRESS: &str = "address";

pub const NAME_OWNER: &str = "owner";
pub const NAME_STAKES: &str = "stakes";
pub const NAME_TOTAL_SUPPLY: &str = "total_supply";
pub const NAME_BALANCES: &str = "balances";
pub const NAME_WHITELIST: &str = "whitelist";

impl From<&str> for Variable<Option<Address>> {
    fn from(name: &str) -> Self {
        Variable::new(name.to_string())
    }
}

impl From<&str> for Variable<U256> {
    fn from(name: &str) -> Self {
        Variable::new(name.to_string())
    }
}

impl From<&str> for Mapping<Address, U256> {
    fn from(name: &str) -> Self {
        Mapping::new(name.to_string())
    }
}

impl From<&str> for Mapping<Address, bool> {
    fn from(name: &str) -> Self {
        Mapping::new(name.to_string())
    }
}
