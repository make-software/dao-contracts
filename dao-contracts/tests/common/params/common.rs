use cucumber::Parameter;

#[derive(Debug, Default, derive_more::FromStr, derive_more::Deref, Parameter, PartialEq, Eq)]
#[param(name = "u256", regex = r"\d+")]
pub struct U256(pub casper_types::U256);

#[allow(dead_code)]
impl U256 {
    pub fn zero() -> Self {
        U256(casper_types::U256::zero())
    }

    pub fn one() -> Self {
        U256(casper_types::U256::one())
    }
}

#[derive(Debug, Default, derive_more::FromStr, derive_more::Deref, Parameter)]
#[param(name = "token_id", regex = r"\d+")]
pub struct TokenId(pub casper_dao_erc721::TokenId);
