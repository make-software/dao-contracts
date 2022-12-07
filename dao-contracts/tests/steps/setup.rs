use cucumber::{gherkin::Step, given};

use crate::common::{config::UserConfiguration, params::Account, DaoWorld};

#[given(expr = "users")]
#[given(expr = "accounts")]
#[given(expr = "following balances")]
fn users_setup(world: &mut DaoWorld, step: &Step) {
    let labels = step
        .table
        .as_ref()
        .unwrap()
        .rows
        .first()
        .expect("User configuration is missing");
    let users_iter = step.table.as_ref().unwrap().rows.iter().skip(1);

    for row in users_iter {
        let config = UserConfiguration::from_labeled_data(labels, row);

        let account = config.account();
        let owner = Account::Owner;
        let reputation_balance = config.reputation_balance();
        let cspr_balance = config.cspr_balance();

        for contract in config.get_contracts_to_be_whitelisted_in() {
            world.whitelist_account(contract, &owner, account).unwrap();
        }

        if config.is_kyced() {
            world.mint_kyc_token(&owner, account);
        }

        if config.is_va() {
            world.mint_va_token(&owner, account);
        }

        if !reputation_balance.is_zero() {
            world.mint_reputation(&Account::Owner, account, reputation_balance);
        }

        world.set_cspr_balance(account, cspr_balance);
    }
}
