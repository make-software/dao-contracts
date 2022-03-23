use casper_types::ApiError;

/// All possible errors that can be raised by the utils crate.
pub enum Error {
    InsufficientBalance,
}

impl From<Error> for ApiError {
    fn from(val: Error) -> Self {
        let id = match val {
            Error::InsufficientBalance => 1003,
        };
        ApiError::User(id)
    }
}
