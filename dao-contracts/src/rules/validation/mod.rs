use casper_dao_utils::Error;

pub trait Validation {
    fn validate(&self) -> Result<(), Error>;
}
