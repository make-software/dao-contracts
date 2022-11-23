use cucumber::Parameter;

#[derive(Debug, Default, derive_more::FromStr, derive_more::Deref, Parameter, PartialEq, Eq)]
#[param(name = "u256", regex = r"\d+")]
pub struct U256(pub casper_types::U256);

impl U256 {
    pub fn one() -> Self {
        U256(casper_types::U256::one())
    }
}
