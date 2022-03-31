use casper_dao_utils::{casper_contract::contract_api::runtime, Address};
use casper_types::bytesrepr::Bytes;

use crate::TokenId;

pub trait IERC721Receiver {
    fn on_erc_721_received(
        &mut self,
        operator: Address,
        from: Address,
        token_id: TokenId,
        data: Bytes,
    );
}

pub struct ERC721ReceiverCaller {
    contract_package_hash: casper_types::ContractPackageHash,
}

impl IERC721Receiver for ERC721ReceiverCaller {
    fn on_erc_721_received(
        &mut self,
        operator: Address,
        from: Address,
        token_id: TokenId,
        data: Bytes,
    ) {
        runtime::call_versioned_contract(self.contract_package_hash, None, "on_erc_721_received", {
            let mut named_args = casper_types::RuntimeArgs::new();
            named_args.insert("operator", operator).unwrap();
            named_args.insert("from", from).unwrap();
            named_args.insert("token_id", token_id).unwrap();
            named_args.insert("data", data).unwrap();
            named_args
        })
    }
}

impl ERC721ReceiverCaller {
    pub fn at(contract_package_hash: casper_types::ContractPackageHash) -> Self {
        Self {
            contract_package_hash,
        }
    }
}
#[cfg(feature = "test-support")]
pub mod tests {
    use casper_dao_utils::{
        casper_contract::contract_api::runtime,
        casper_dao_macros::{casper_contract_interface, Instance},
        Address, Variable,
    };
    use casper_types::bytesrepr::Bytes;

    use crate::TokenId;

    #[casper_contract_interface]
    trait SampleERC721ReceiverInterface {
        fn init(&self);
        fn on_erc_721_received(
            &mut self,
            operator: Address,
            from: Address,
            token_id: TokenId,
            data: Bytes,
        );
        fn get(&self) -> Bytes;
    }

    #[derive(Instance)]
    struct SampleERC721Receiver {
        var: Variable<Bytes>,
    }

    impl SampleERC721ReceiverInterface for SampleERC721Receiver {
        fn init(&self) {
            runtime::print("init erc721 receiver");
        }

        fn on_erc_721_received(
            &mut self,
            operator: Address,
            from: Address,
            token_id: TokenId,
            data: Bytes,
        ) {
            runtime::print(format!("operator = {:?}", operator).as_str());
            runtime::print(format!("from = {:?}", from).as_str());
            runtime::print(format!("token_id = {:?}", token_id).as_str());

            self.var.set(data)
        }

        fn get(&self) -> Bytes {
            self.var.get()
        }
    }

    #[casper_contract_interface]
    trait SampleInterface {
        fn init(&self);
        fn get(&self) -> Bytes;
    }

    #[derive(Instance)]
    struct Sample {
        var: Variable<Bytes>,
    }

    impl SampleInterface for Sample {
        fn init(&self) {
            runtime::print("init sample contract");
        }

        fn get(&self) -> Bytes {
            self.var.get()
        }
    }
}
