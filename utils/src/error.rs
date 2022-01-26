use casper_types::ApiError;

pub enum Error {
    NotAnOwner,
    OwnerIsNotInitialized,
    NotOnTheWhietlist,
}

impl From<Error> for ApiError {
    fn from(val: Error) -> Self {
        let id = match val {
            Error::NotAnOwner => 1000,
            Error::OwnerIsNotInitialized => 1001,
            Error::NotOnTheWhietlist => 1002,
        };
        ApiError::User(id)
    }
}
