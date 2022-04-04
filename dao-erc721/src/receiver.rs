use casper_dao_utils::{casper_contract::contract_api::runtime, Address};
use casper_types::bytesrepr::Bytes;

use crate::TokenId;

pub trait IERC721Receiver {
    fn on_erc_721_received(
        &self,
        operator: Address,
        from: Address,
        token_id: TokenId,
        data: Option<Bytes>,
    );
}

pub struct ERC721ReceiverCaller {
    contract_package_hash: casper_types::ContractPackageHash,
}

impl IERC721Receiver for ERC721ReceiverCaller {
    fn on_erc_721_received(
        &self,
        operator: Address,
        from: Address,
        token_id: TokenId,
        data: Option<Bytes>,
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

pub mod tests {
    use casper_dao_utils::{
        casper_dao_macros::{casper_contract_interface, Instance},
        Address, Variable,
    };
    use casper_types::bytesrepr::Bytes;

    use crate::TokenId;

    #[casper_contract_interface]
    trait MockERC721ReceiverInterface {
        fn init(&self);
        fn on_erc_721_received(
            &mut self,
            operator: Address,
            from: Address,
            token_id: TokenId,
            data: Option<Bytes>,
        );
        fn get(&self) -> Bytes;
    }

    #[derive(Instance)]
    pub struct MockERC721Receiver {
        var: Variable<Bytes>,
    }

    impl MockERC721ReceiverInterface for MockERC721Receiver {
        fn init(&self) {}

        #[allow(unused_variables)]
        fn on_erc_721_received(
            &mut self,
            operator: Address,
            from: Address,
            token_id: TokenId,
            data: Option<Bytes>,
        ) {
            if let Some(data) = data {
                self.var.set(data)
            }
        }

        fn get(&self) -> Bytes {
            self.var.get()
        }
    }

    #[casper_contract_interface]
    trait MockERC721NonReceiverInterface {
        fn init(&self);
        fn get(&self) -> Bytes;
    }

    #[derive(Instance)]
    pub struct MockERC721NonReceiver {
        var: Variable<Bytes>,
    }

    impl MockERC721NonReceiverInterface for MockERC721NonReceiver {
        fn init(&self) {}

        fn get(&self) -> Bytes {
            self.var.get()
        }
    }
}
