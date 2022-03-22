use casper_types::bytesrepr::{Bytes, FromBytes, ToBytes};

pub trait BytesConversion {
    fn convert_to_bytes(&self) -> Bytes;
    fn convert_from_bytes(bytes: Bytes) -> Self;
}

impl<T: ToBytes + FromBytes> BytesConversion for T {
    fn convert_to_bytes(&self) -> Bytes {
        Bytes::from(self.to_bytes().unwrap())
    }

    fn convert_from_bytes(bytes: Bytes) -> Self {
        let (v, rest) = T::from_bytes(&bytes).unwrap();
        assert!(rest.is_empty());
        v
    }
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
        let bytes = value.convert_to_bytes();
        let recovered = T::convert_from_bytes(bytes);
        assert_eq!(recovered, value);
    }
}
