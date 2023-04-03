# Adding a new contract to the DAO
In this tutorial we will show you how to add a new contract to the DAO.

## Flipper Contract
First, create a new contract file in the `dao-contracts/src` folder. We will call it `flipper.rs`.
Add the contract to the `lib.rs` file in the same folder.

```rust
pub mod flipper;
```

Let's put some logic into the `flipper.rs` file and explain its content.

```rust
use casper_dao_utils::casper_dao_macros::{casper_contract_interface, Instance};
use casper_dao_utils::Variable;

#[casper_contract_interface]
pub trait FlipperContractInterface { // This trait is used to generate the contract interface - the contract public API.
    fn init(
        &mut self,
    );

    fn flip(&mut self);

    fn get(&self) -> bool;
}

#[derive(Instance)]
pub struct FlipperContract {
    value: Variable<bool>, // To use contract's storage, we need to wrap variables in the Variable struct.
}

impl FlipperContractInterface for FlipperContract {
    fn init(&mut self) {
        self.value.set(true); // Values stored in the variables can be accessed via set/get methods.
    }

    fn flip(&mut self) {
        self.value.set(!self.value.get_or_revert());
    }

    fn get(&self) -> bool {
        self.value.get_or_revert()
    }
}
```

To let know cargo that a new wasm file should be generated, we need to add the following line to the `Cargo.toml` file.

```toml
[[bin]]
name = "flipper_contract"
path = "bin/flipper_contract.rs"
bench = false
doctest = false
test = false
doc = false
```

and create the `bin/flipper_contract.rs` file:

```rust
use casper_dao_contracts::flipper::{FlipperContract, FlipperContractInterface};
casper_dao_contracts::flipper_contract!();

fn main() {}
```

To create a wasm file run the following command:

```bash
make build-dao-contracts
```

The contract will be located in the `target/wasm32-unknown-unknown/release` folder.

## Calling other contracts
To call other contracts, we need to have their addresses. Let's have a look at the bid_escrow contract - 
we pass the addresses during the deployment using the init method and store them in the Variables.

```rust
fn init(
    &mut self,
    variable_repository: Address,
    reputation_token: Address,
    kyc_token: Address,
    va_token: Address,
) {
    self.refs
        .init(variable_repository, reputation_token, va_token, kyc_token);
    self.access_control.init(caller());
}   
```

To prove that in the end the storage is used, have a look at the `ContractRefStorage` implementation:

```rust
/// A module that stores addresses to common contracts that are used by most of the voting contracts.
#[derive(Instance)]
pub struct ContractRefsStorage {
    variable_repository: Variable<Address>,
    reputation_token: Variable<Address>,
    va_token: Variable<Address>,
}

impl ContractRefsStorage {
    pub fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        va_token: Address,
    ) {
        self.variable_repository.set(variable_repository);
        self.reputation_token.set(reputation_token);
        self.va_token.set(va_token);
    }
    ...
```

`casper_contract_interface` macro generates a lot of code for us. Among others is the Caller struct that eases the process
of calling the contract. Each contract has a caller struct that we can use like this:

```rust
ReputationContractCaller::at(address).unstake_bid(bid);
// ^ Caller Struct        ^ Address  ^ Contract method
```
The caller even exposes public interfaces as easy to use methods! You can also take a look at the ContractRefStorage further
and contracts using it, to see how this code can be simplified even more.

## Advanced storage
Variables are pretty powerful, as they can store any CLTyped data, but they can be not efficient, especially when we want to store
large structs with nesting. In such cases you can use following structs, similar to those used in Solidity:

- `Mapping<K, V>` - a map that stores values of type `V` under keys of type `K`.
- `IndexedMapping<V>` - similar to mapping, but keys are `u32`
- `VecMapping<K, V>` - mapping used to efficiently store vectors
