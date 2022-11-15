use casper_types::{
    bytesrepr::{Bytes, FromBytes, ToBytes},
    U256,
    U512,
};

use crate::Error;

pub trait BytesConversion: Sized {
    fn convert_to_bytes(&self) -> Result<Bytes, Error>;
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

pub fn u512_to_u256(u512: U512) -> Result<U256, Error> {
    U256::convert_from_bytes(u512.convert_to_bytes()?)
}

pub fn u256_to_512(u256: U256) -> Result<U512, Error> {
    U512::convert_from_bytes(u256.convert_to_bytes()?)
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
