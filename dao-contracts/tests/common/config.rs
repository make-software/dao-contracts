use super::{helpers, params::Account};

#[allow(dead_code)]
pub struct UserConfiguration {
    account: Account,
    is_whitelisted: bool,
    is_kyced: bool,
    is_va: bool,
}

#[allow(dead_code)]
impl UserConfiguration {
    pub fn new(data: &Vec<String>) -> Self {
        Self {
            account: helpers::parse(data.get(0), "Invalid config - missing Account"),
            is_whitelisted: helpers::parse_or_default(data.get(1)),
            is_kyced: helpers::parse_or_default(data.get(2)),
            is_va: helpers::parse_or_default(data.get(3)),
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
}

impl From<&Vec<String>> for UserConfiguration {
    fn from(value: &Vec<String>) -> Self {
        UserConfiguration::new(value)
    }
}
