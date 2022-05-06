use casper_types::{bytesrepr::FromBytes, ContractPackageHash};
use std::fmt::Debug;

use crate::{Address, TestEnv};

pub trait TestContract {
    fn address(&self) -> Address;
    fn as_account(&mut self, account: Address) -> &mut Self;
    fn as_nth_account(&mut self, account: usize) -> &mut Self;
    fn advance_block_time_by(&mut self, seconds: u64) -> &mut Self;
    fn assert_event_at<T: FromBytes + PartialEq + Debug>(&self, index: i32, event: T);
    fn assert_last_event<T: FromBytes + PartialEq + Debug>(&self, event: T);
    fn event<T: FromBytes>(&self, index: i32) -> T;
    fn events_count(&self) -> i32;
    fn get_package_hash(&self) -> ContractPackageHash;
    fn get_env(&self) -> &TestEnv;
}
