use cucumber::{gherkin::Step, given, when};

use crate::common::{
    config::UserConfiguration,
    params::{Account, Contract},
    DaoWorld,
};

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
            world.whitelist_account(contract, &owner, account);
        }

        if config.is_kyced() {
            world.mint_nft_token(Contract::KycToken, &owner, account);
        }

        if config.is_va() {
            world.mint_nft_token(Contract::VaToken, &owner, account);
        }

        if !reputation_balance.is_zero() {
            world.mint_reputation(&Account::Owner, account, reputation_balance);
        }

        world.set_cspr_balance(account, cspr_balance);
    }
}

#[given(expr = "Admin is the owner of all contracts")]
#[when(expr = "Admin is the owner of all contracts")]
fn admin_is_the_owner_of_all_contracts(world: &mut DaoWorld) {
    let contracts = [
        Contract::BidEscrow,
        Contract::KycToken,
        Contract::KycVoter,
        Contract::Onboarding,
        Contract::RepoVoter,
        Contract::ReputationToken,
        Contract::ReputationVoter,
        Contract::SimpleVoter,
        Contract::SlashingVoter,
        Contract::VaToken,
        Contract::VariableRepository,
    ];

    for contract in contracts {
        world.change_ownership(
            &Account::Contract(contract),
            &Account::Owner,
            &Account::Contract(Contract::Admin),
        );
    }
}
