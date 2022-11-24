use std::{str::FromStr, fmt::Debug};

use super::params::nft::Account;

pub struct UserConfiguration {
    account: Account,
    is_whitelisted: bool,
    is_kyced: bool,
    is_va: bool,
}

impl UserConfiguration {

    pub fn new(data: &Vec<String>) -> Self {
        Self { 
            account: Self::parse(data.get(0), "Invalid config - missing Account"),
            is_whitelisted: Self::parse_or_default(data.get(1)), 
            is_kyced: Self::parse_or_default(data.get(2)), 
            is_va: Self::parse_or_default(data.get(3)),
        }
    }

    pub fn account(&self) -> &Account {
        &self.account
    }

    pub fn is_whitelisted(&self) -> bool {
        self.is_whitelisted
    }

    pub fn is_kyced(&self) -> bool {
        self.is_kyced
    }

    pub fn is_va(&self) -> bool {
        self.is_va
    }

    fn parse<T>(item: Option<&String>, err_msg: &str) -> T
    where T: FromStr, <T as FromStr>::Err: Debug {
        item.expect(err_msg).parse::<T>().expect("Parsing failed.")
    }

    fn parse_or_default<T: FromStr + Default>(item: Option<&String>) -> T {
        match item {
            Some(value) => value.parse::<T>().unwrap_or_default(),
            None => T::default(),
        }
    }
}

impl From<&Vec<String>> for UserConfiguration {
    fn from(value: &Vec<String>) -> Self {
        UserConfiguration::new(value)
    }
}
