# DAO Client
The CLI DAO Client.

## Deploy Contracts
Make sure `wasm` directory in a root of the repository exists if you want to deploy wasm files.

Then `.env` file should exists under `dao-client/.env`. See `.env.sample` to set it up correctly.

Contracts can be deployed now.

```bash
cargo run -- deploy-all
```

This command will create `deployed_contracts.toml` in your directory.
It contains list of addresses of all contracts.
The file is later used to interact with the DAO by other commands.
At the moment we keep the latest version of it in the repository.
It contains contracts deployed to the Integration Network.

## Setup whitelists
To setup all whitelists run:
```bash
cargo run -- whitelist
```

## Setup Slashing Voter
To setup slashing voter run:
```bash
cargo run -- setup-slashing-voter
```

## Setup VA
Before the DAO starts all VAs should have their accounts created.
That means minting KYC token, VA token and reputation.

Example:
```bash
cargo run -- setup-va account-hash-3b4ffcfb21411ced5fc1560c3f6ffed86f4885e5ea05cde49d90962a48a14d95 1000000000
```

## Print Variables
To print all DAO variables and their values run:
```bash
cargo run -- print-variables
```

## Update Variable
To update DAO variable run:
```bash
cargo run -- set-variable <variable_name> <variable_value>
```

To see available variable names and sample values, use `print-variables` command.

for example:
```bash
cargo run -- set-variable FormalQuorumRatio 100
cargo run -- set-variable ForumKycRequired false
```

## Querying balance and stake

To query balance and stake of a user run respectively:
```bash
cargo run -- balance-of account-hash-<account_hash>
cargo run -- stake-of account-hash-<account_hash>
```

## Showing voting stats

To show stats of a voting run:
```bash
cargo run -- get-voting <voting_id> <contract_name>
```

Possible values for <contract_name> are: `kyc_voter`, `repo_voter`, `reputation_voter`, `admin`, `slashing_voter`, `simple_voter` and `bid_escrow`.
