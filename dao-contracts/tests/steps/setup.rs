use cucumber::{gherkin::Step, given};

use crate::common::{config::UserConfiguration, params::{Account, Contract}, DaoWorld};

macro_rules! transfer_ownership_to_admin {
    ($world:ident, $contract:expr) => {
        $world.change_ownership(
            &$contract, 
            &Account::Owner, 
            &Account::Contract(Contract::Admin)
        ).unwrap();
    }
}

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

    // A hack - the owner/deployer should be removed from the whitelist but if we do so, 
    // some calls fail (the owner/deployer is the default caller).
    // TestEnv does not allow to set a contract as the call executor, so we need leave the owner/deployer
    // on the whitelist.
    transfer_ownership_to_admin!(world, Contract::BidEscrow);
    transfer_ownership_to_admin!(world, Contract::KycToken);
    transfer_ownership_to_admin!(world, Contract::KycVoter);
    transfer_ownership_to_admin!(world, Contract::Onboarding);
    transfer_ownership_to_admin!(world, Contract::RepoVoter);
    transfer_ownership_to_admin!(world, Contract::ReputationToken);
    transfer_ownership_to_admin!(world, Contract::ReputationVoter);
    transfer_ownership_to_admin!(world, Contract::SimpleVoter);
    transfer_ownership_to_admin!(world, Contract::SlashingVoter);
    transfer_ownership_to_admin!(world, Contract::VaToken);
    transfer_ownership_to_admin!(world, Contract::VariableRepository);
}
