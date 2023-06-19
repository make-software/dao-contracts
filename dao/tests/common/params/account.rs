#[derive(Debug, Default, Parameter, Clone, Copy)]
#[param(name = "account", regex = ".+")]
pub enum Account {
    Alice,
    Bob,
    Owner,
    Deployer,
    Holder,
    #[default]
    Any,
    VA(usize),
    JobPoster,
    ExternalWorker,
    InternalWorker,
    Contract(Contract),
    MultisigWallet,
}

use std::str::FromStr;

use cucumber::Parameter;

use super::Contract;
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
            "Account" => Self::Any,
            "VA1" => Self::VA(1),
            "VA2" => Self::VA(2),
            "VA3" => Self::VA(3),
            "VA4" => Self::VA(4),
            "VA5" => Self::VA(5),
            "VA6" => Self::VA(6),
            "VA7" => Self::VA(7),
            "VA8" => Self::VA(8),
            "JobPoster" => Self::JobPoster,
            "ExternalWorker" => Self::ExternalWorker,
            "InternalWorker" => Self::InternalWorker,
            "MultisigWallet" => Self::MultisigWallet,
            other => Self::Contract(other.parse().unwrap()),
        })
    }
}

impl From<Option<&String>> for Account {
    fn from(value: Option<&String>) -> Self {
        helpers::parse(value, "Couldn't parse Account")
    }
}
