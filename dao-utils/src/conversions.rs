//! Types conversion.
use casper_types::bytesrepr::{Bytes, FromBytes, ToBytes};

use crate::Error;

/// Error-safe conversion from/to bytes.
pub trait BytesConversion: Sized {
    /// Converts the struct to [`Bytes`] or returns an [`Error`].
    fn convert_to_bytes(&self) -> Result<Bytes, Error>;
    /// Converts [`Bytes`] to a struct or returns an [`Error`].
    fn convert_from_bytes(bytes: Bytes) -> Result<Self, Error>;
}

impl<T: ToBytes + FromBytes> BytesConversion for T {
    fn convert_to_bytes(&self) -> Result<Bytes, Error> {
        match self.to_bytes() {
            Ok(bytes) => Ok(Bytes::from(bytes)),
            Err(_) => Err(Error::BytesConversionError),
        }
    }

    fn convert_from_bytes(bytes: Bytes) -> Result<Self, Error> {
        let conversion = T::from_bytes(&bytes);
        match conversion {
            Ok((v, rest)) => {
                if !rest.is_empty() {
                    Err(Error::BytesConversionError)
                } else {
                    Ok(v)
                }
            }
            Err(_) => Err(Error::BytesConversionError),
        }
    }
}

pub fn sec_to_milli(sec: u64) -> u64 {
    sec * 1000u64
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use crate::BytesConversion;

    #[test]
    fn test_bytes_conversion() {
        make_a_round(1);
        make_a_round("string".to_string());
        make_a_round((1, "value".to_string()));
    }

    fn make_a_round<T: BytesConversion + Debug + PartialEq>(value: T) {
        let bytes = value.convert_to_bytes().unwrap();
        let recovered = T::convert_from_bytes(bytes).unwrap();
        assert_eq!(recovered, value);
    }
}
