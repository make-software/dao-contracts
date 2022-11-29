use std::str::FromStr;

use cucumber::Parameter;

#[derive(Debug, Parameter, PartialEq, Eq, PartialOrd, Ord)]
#[param(name = "contract", regex = ".+")]
pub enum Contract {
    KycToken,
    KycVoter,
    VaToken,
    ReputationToken,
    BidEscrow,
    VariableRepository,
    SlashingVoter,
}

impl FromStr for Contract {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let contract = match s {
            "KycToken" => Self::KycToken,
            "KycVoter" => Self::KycVoter,
            "VaToken" => Self::VaToken,
            "ReputationToken" => Self::ReputationToken,
            "BidEscrow" => Self::BidEscrow,
            "VariableRepository" => Self::VariableRepository,
            "SlashingVoter" => Self::SlashingVoter,
            invalid => return Err(format!("Unknown contract {}", invalid)),
        };
        Ok(contract)
    }
}
