use casper_types::bytesrepr::{Bytes, ToBytes};
use casper_types::{U256, U512};

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

pub fn to_rep(v: &str) -> U256 {
    U256::from((v.parse::<f32>().unwrap() * 1_000f32) as u32) * 1_000_000
}

pub fn to_cspr(v: &str) -> U512 {
    U512::from((v.parse::<f32>().unwrap() * 1_000f32) as u32) * 1_000_000
}

pub fn is_cspr_close_enough(a: U512, b: U512) -> bool {
    let diff = if a > b { a - b } else { b - a };
    diff < U512::from(10_000_000)
}

pub fn is_rep_close_enough(a: U256, b: U256) -> bool {
    let diff = if a > b { a - b } else { b - a };
    diff < U256::from(10_000_000)
}
