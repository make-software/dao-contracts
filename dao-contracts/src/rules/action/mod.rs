use casper_dao_utils::Error;

pub trait Action {
    fn execute(&self) -> Result<(), Error>;
}
