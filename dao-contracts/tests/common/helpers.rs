use casper_types::bytesrepr::{Bytes, ToBytes};
use casper_types::U256;

/// Converts a string value from Gherkin scenario to a `Bytes` representation of the value
pub fn value_to_bytes(value: &str) -> Bytes {
    match value {
        "true" => true.to_bytes().unwrap().into(),
        "false" => false.to_bytes().unwrap().into(),
        _ => {
            let value: f64 = value.parse().unwrap();
            let value = (value * 1000f64) as u64;
            U256::from(value).to_bytes().unwrap().into()
        }
    }
}
