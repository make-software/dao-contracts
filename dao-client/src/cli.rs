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
    }
}
