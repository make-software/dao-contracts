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
    /// Deploys all DAO contracts
    DeployAll,
    /// Configures whitelists of all contracts
    Whitelist,
    /// Sets up a slashing voter
    SetupSlashingVoter,
    /// Prints addresses of all contracts
    PrintAddresses,
    /// Sets up a VA account
    SetupVA {
        /// Account hash of the address in a form "account-hash-..."
        account_hash: String,
        /// Amount of reputation to be minted
        reputation_amount: u64,
    },
    /// Prints all variables stored in the variable repository
    PrintVariables,
    /// Sets a variable value in the variable repository
    SetVariable {
        /// Name of the variable
        name: String,
        /// Value of the variable
        value: String,
    },
    /// Prints balance of an account
    BalanceOf {
        /// Account hash of the address in a form "account-hash-..."
        account_hash: String,
    },
    /// Prints stake of an account
    StakeOf {
        /// Account hash of the address in a form "account-hash-..."
        account_hash: String,
    },
    /// Get voting information
    GetVoting {
        /// Voting id
        voting_id: String,
        /// Voting contract name
        /// Possible values: kyc_voter, repo_voter, reputation_voter, admin, slashing_voter, simple_voter, bid_escrow
        contract: String,
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
        PrintVariables => actions::print_variables(),
        SetVariable { name, value } => actions::set_variable(&name, &value),
        BalanceOf { account_hash } => actions::balance_of(&account_hash),
        StakeOf { account_hash } => actions::stake_of(&account_hash),
        GetVoting {
            voting_id,
            contract,
        } => actions::get_voting(&voting_id, &contract),
    }
}
