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

use cucumber::Parameter;

use crate::common::helpers;

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
