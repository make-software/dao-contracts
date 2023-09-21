use crate::actions;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dao-client")]
#[command(about = "Interact with the DAO easily.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    DeployAll,
    Whitelist,
    SetupSlashingVoter,
    PrintAddresses,
    SetupVA {
        account_hash: String,
        reputation_amount: u64,
    },
    PrintVariables,
    SetVariable {
        name: String,
        value: String,
    },
    BalanceOf {
        account_hash: String,
    },
    StakeOf {
        account_hash: String,
    },
    GetVoting {
        voting_id: String,
    }
}

pub fn parse() {
    use Commands::*;
    match Cli::parse().command {
        DeployAll => actions::deploy_all(),
        Whitelist => actions::whitelist(),
        SetupSlashingVoter => actions::setup_slashing_voter(),
        PrintAddresses => actions::print_addresses(),
        SetupVA {
            account_hash,
            reputation_amount,
        } => actions::setup_va(&account_hash, reputation_amount),
        PrintVariables => actions::print_variables(),
        SetVariable { name, value } => actions::set_variable(&name, &value),
        BalanceOf { account_hash } => actions::balance_of(&account_hash),
        StakeOf { account_hash } => actions::stake_of(&account_hash),
        GetVoting { voting_id } => actions::get_voting(&voting_id),
    }
}
