use std::str::FromStr;

use casper_dao_utils::Address;
use cucumber::Parameter;

use crate::common::DaoWorld;

#[derive(Debug, Default, derive_more::FromStr, derive_more::Deref, Parameter)]
#[param(name = "token_id", regex = r"\d+")]
pub struct TokenId(pub casper_dao_erc721::TokenId);

#[derive(Debug, Default, Parameter)]
#[param(name = "account", regex = ".+")]
pub enum Account {
    Alice,
    Bob,
    Owner,
    Holder,
    #[default]
    Any,
}

impl Account {
    pub fn get_address(&self, world: &DaoWorld) -> Address {
        let idx = match self {
            Account::Owner => 0,
            Account::Alice => 1,
            Account::Bob => 2,
            Account::Holder => 3,
            Account::Any => 4,
        };
        world.env.get_account(idx)
    }
}

impl FromStr for Account {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Bob" => Self::Bob,
            "Alice" => Self::Alice,
            "Owner" => Self::Owner,
            "Holder" => Self::Holder,
            _ => Self::Any,
        })
    }
}
