#[derive(Debug, Default, Parameter, Clone)]
#[param(name = "account", regex = ".+")]
pub enum Account {
    Alice,
    Bob,
    Owner,
    Deployer,
    Holder,
    #[default]
    Any,
}

use std::str::FromStr;

use casper_dao_utils::Address;
use cucumber::Parameter;

use crate::common::{helpers, DaoWorld};

impl Account {
    pub fn get_address(&self, world: &DaoWorld) -> Address {
        let idx = match self {
            Account::Owner => 0,
            Account::Deployer => 0,
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
            "Deployer" => Self::Deployer,
            "Holder" => Self::Holder,
            _ => Self::Any,
        })
    }
}

impl From<Option<&String>> for Account {
    fn from(value: Option<&String>) -> Self {
        helpers::parse(value, "Couldn't parse Account")
    }
}
