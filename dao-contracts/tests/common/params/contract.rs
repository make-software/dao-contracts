use std::str::FromStr;

use cucumber::Parameter;

#[derive(Debug, Parameter)]
#[param(name = "contract", regex = ".+")]
pub enum Contract {
    KycToken,
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
