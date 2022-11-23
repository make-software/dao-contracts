use std::str::FromStr;

use cucumber::Parameter;

#[derive(Debug, Default, derive_more::FromStr, derive_more::Deref, Parameter, PartialEq, Eq)]
#[param(name = "u256", regex = r"\d+")]
pub struct U256(pub casper_types::U256);

impl U256 {
    pub fn zero() -> Self {
        U256(casper_types::U256::zero())
    }

    pub fn one() -> Self {
        U256(casper_types::U256::one())
    }
}

#[derive(Debug, Parameter)]
#[param(name = "contract", regex = ".+")]
pub enum Contract {
    KycToken,
    VaToken,
    ReputationToken,
}

impl FromStr for Contract {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let contract = match s {
            "KycToken" => Self::KycToken,
            "VaToken" => Self::VaToken,
            "ReputationToken" => Self::ReputationToken,
            invalid => return Err(format!("Unknown contract {}", invalid)),
        };
        Ok(contract)
    }
}
