use super::{
    helpers,
    params::{Account, CsprBalance},
};
use crate::common::params::ReputationBalance;

#[allow(dead_code)]
pub struct UserConfiguration {
    account: Account,
    whitelisted_in: Vec<Account>,
    is_kyced: bool,
    is_va: bool,
    reputation_balance: ReputationBalance,
    reputation_staked: ReputationBalance,
    cspr_balance: CsprBalance,
}

#[allow(dead_code)]
impl UserConfiguration {
    pub fn from_labeled_data(labels: &[String], data: &[String]) -> Self {
        let mut whitelisted_in = vec![];
        let mut is_kyced = false;
        let mut is_va = false;
        let mut reputation_balance = ReputationBalance::zero();
        let mut reputation_staked = ReputationBalance::zero();
        let mut cspr_balance = CsprBalance::zero();
        let mut account = None::<Account>;

        for (idx, label) in labels.iter().enumerate() {
            match label.as_str() {
                "whitelisted_in" => {
                    let contracts_string = data.get(idx).map(|s| s.to_owned()).unwrap_or_default();
                    let contracts_names = contracts_string.split(',');
                    whitelisted_in = contracts_names
                        .filter(|s| !s.is_empty())
                        .map(|name| {
                            helpers::parse::<Account>(
                                Some(&name.to_owned()),
                                "Couldn't parse contract",
                            )
                        })
                        .collect();
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
                "REP stake" => {
                    reputation_staked = helpers::parse_or_default(data.get(idx));
                }
                "CSPR balance" => {
                    cspr_balance = helpers::parse_or_default(data.get(idx));
                }
                "user" => {
                    account = helpers::parse_or_none(data.get(idx));
                }
                "account" => {
                    account = helpers::parse_or_none(data.get(idx));
                }
                unknown => {
                    dbg!("Unknown label {} found", unknown);
                }
            }
        }

        Self {
            account: account.expect("Invalid config - `user` label is missing"),
            whitelisted_in,
            is_kyced,
            is_va,
            reputation_balance,
            reputation_staked,
            cspr_balance,
        }
    }

    pub fn account(&self) -> &Account {
        &self.account
    }

    pub fn get_contracts_to_be_whitelisted_in(&self) -> &Vec<Account> {
        &self.whitelisted_in
    }

    pub fn is_kyced(&self) -> bool {
        self.is_kyced
    }

    pub fn is_va(&self) -> bool {
        self.is_va
    }

    pub fn reputation_balance(&self) -> ReputationBalance {
        self.reputation_balance
    }

    pub fn cspr_balance(&self) -> CsprBalance {
        self.cspr_balance
    }
}
