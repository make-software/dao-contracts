use casper_types::{bytesrepr::FromBytes, ContractPackageHash};
use std::fmt::Debug;

use crate::Address;

pub trait TestContract {
    fn address(&self) -> Address;
    fn as_account(&mut self, account: Address) -> &mut Self;
    fn assert_event_at<T: FromBytes + PartialEq + Debug>(&self, index: i32, event: T);
    fn assert_last_event<T: FromBytes + PartialEq + Debug>(&self, event: T);
    fn event<T: FromBytes>(&self, index: u32) -> T;
    fn get_package_hash(&self) -> ContractPackageHash;
}
