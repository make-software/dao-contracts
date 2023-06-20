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
}

pub fn parse() {
    let cli = Cli::parse();
    match cli.command {
        Commands::DeployAll => actions::deploy_all(),
        Commands::Whitelist => actions::whitelist(),
        Commands::SetupSlashingVoter => actions::setup_slashing_voter(),
        Commands::PrintAddresses => actions::print_addresses(),
    }
}
