use casper_types::ApiError;

pub enum Error {
    NotAnOwner,
    OwnerIsNotInitialized,
    NotWhitelisted,
}

impl From<Error> for ApiError {
    fn from(val: Error) -> Self {
        let id = match val {
            Error::NotAnOwner => 1000,
            Error::OwnerIsNotInitialized => 1001,
            Error::NotWhitelisted => 1002,
        };
        ApiError::User(id)
    }
}
