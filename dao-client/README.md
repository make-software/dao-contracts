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
