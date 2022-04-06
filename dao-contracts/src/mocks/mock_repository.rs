use casper_dao_modules::{Record, Repository};
use casper_dao_utils::casper_dao_macros::{casper_contract_interface, Instance};

use casper_types::bytesrepr::Bytes;
use delegate::delegate;

#[casper_contract_interface]
trait MockRepositoryInterface {
    fn init(&mut self);
    fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>);
    fn get(&self, key: String) -> Option<Bytes>;
    fn get_full_value(&self, key: String) -> Option<Record>;
}

#[derive(Instance)]
pub struct MockRepository {
    repository: Repository,
}

impl MockRepositoryInterface for MockRepository {
    delegate! {
        to self.repository {
            fn init(&mut self);
            fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>);
            fn get(&self, key: String) -> Option<Bytes>;
            fn get_full_value(&self, key: String) -> Option<Record>;
        }
    }
}
