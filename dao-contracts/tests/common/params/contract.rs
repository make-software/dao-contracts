use std::str::FromStr;

use cucumber::Parameter;

#[derive(Clone, Copy, Debug, Parameter, PartialEq, Eq, PartialOrd, Ord)]
#[param(name = "contract", regex = ".+")]
pub enum Contract {
    Admin,
    KycToken,
    VaToken,
    ReputationToken,
    VariableRepository,
    KycVoter,
    RepoVoter,
    SlashingVoter,
    SimpleVoter,
    ReputationVoter,
    BidEscrow,
    Onboarding,
}

impl FromStr for Contract {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let contract = match s {
            "Admin" => Self::Admin,
            "KycToken" => Self::KycToken,
            "VaToken" => Self::VaToken,
            "ReputationToken" => Self::ReputationToken,
            "VariableRepository" => Self::VariableRepository,
            "KycVoter" => Self::KycVoter,
            "RepoVoter" => Self::RepoVoter,
            "SlashingVoter" => Self::SlashingVoter,
            "SimpleVoter" => Self::SimpleVoter,
            "ReputationVoter" => Self::ReputationVoter,
            "BidEscrow" => Self::BidEscrow,
            "Onboarding" => Self::Onboarding,
            invalid => return Err(format!("Unknown contract {}", invalid)),
        };
        Ok(contract)
    }
}
