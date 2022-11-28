use super::{
    helpers,
    params::{Account, U256},
};

#[allow(dead_code)]
pub struct UserConfiguration {
    account: Account,
    is_whitelisted: bool,
    is_kyced: bool,
    is_va: bool,
    reputation_balance: U256,
}

#[allow(dead_code)]
impl UserConfiguration {
    pub fn from_labeled_data(labels: &Vec<String>, data: &Vec<String>) -> Self {
        let mut is_whitelisted = false;
        let mut is_kyced = false;
        let mut is_va = false;
        let mut reputation_balance = U256::zero();
        let mut account = None::<Account>;

        for (idx, label) in labels.iter().enumerate() {
            match label.as_str() {
                "is_whitelisted" => {
                    is_whitelisted = helpers::parse_or_default(data.get(idx));
                }
                "is_kyced" => {
                    is_kyced = helpers::parse_or_default(data.get(idx));
                }
                "is_va" => {
                    is_va = helpers::parse_or_default(data.get(idx));
                }
                "REP balance" => {
                    reputation_balance = helpers::parse_or_default(data.get(idx));
                }
                "user" => {
                    account = helpers::parse_or_none(data.get(idx));
                }
                unknown => {
                    dbg!("Unknown label {} found", unknown);
                }
            }
        }

        Self {
            account: account.expect("Invalid config - `user` label is missing"),
            is_whitelisted,
            is_kyced,
            is_va,
            reputation_balance,
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

    pub fn reputation_balance(&self) -> U256 {
        self.reputation_balance
    }
}
