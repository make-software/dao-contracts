use casper_dao_modules::{Record, Repository};
use casper_dao_utils::casper_dao_macros::{casper_contract_interface, Instance};

use casper_types::bytesrepr::Bytes;
use delegate::delegate;

#[casper_contract_interface]
trait MockRepositoryContractInterface {
    fn init(&mut self) {}
    fn intialize_module(&mut self);
    fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>);
    fn get(&self, key: String) -> Option<Bytes>;
    fn get_full_value(&self, key: String) -> Option<Record>;
}

#[derive(Instance)]
pub struct MockRepositoryContract {
    repository: Repository,
}

impl MockRepositoryContractInterface for MockRepositoryContract {
    delegate! {
        to self.repository {
            #[call(init)]
            fn intialize_module(&mut self);
            fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>);
            fn get(&self, key: String) -> Option<Bytes>;
            fn get_full_value(&self, key: String) -> Option<Record>;
        }
    }
}
