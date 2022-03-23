#![no_std]
#![no_main]

extern crate alloc;

use core::mem::MaybeUninit;

use alloc::{string::String, vec::Vec};
use casper_contract::{
    contract_api::{self, runtime},
    ext_ffi,
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    api_error,
    bytesrepr::{Bytes, FromBytes, ToBytes},
    ApiError, ContractPackageHash, ContractVersion, RuntimeArgs,
};

#[no_mangle]
fn call() {
    let contract_package_hash: ContractPackageHash =
        runtime::get_named_arg("contract_package_hash");
    let entry_point: String = runtime::get_named_arg("entry_point");
    let args_bytes: Bytes = runtime::get_named_arg("args");
    let (args, _) = RuntimeArgs::from_bytes(&args_bytes).unwrap_or_revert();
    let has_return: bool = runtime::get_named_arg("has_return");
    if has_return {
        let result: Vec<u8> =
            call_versioned_contract(contract_package_hash, None, &entry_point, args);
        casper_dao_utils::casper_env::set_key("result", Bytes::from(result));
    } else {
        let _: () =
            runtime::call_versioned_contract(contract_package_hash, None, &entry_point, args);
    }
}

fn call_versioned_contract(
    contract_package_hash: ContractPackageHash,
    contract_version: Option<ContractVersion>,
    entry_point_name: &str,
    runtime_args: RuntimeArgs,
) -> Vec<u8> {
    let (contract_package_hash_ptr, contract_package_hash_size, _bytes) =
        to_ptr(contract_package_hash);
    let (contract_version_ptr, contract_version_size, _bytes) = to_ptr(contract_version);
    let (entry_point_name_ptr, entry_point_name_size, _bytes) = to_ptr(entry_point_name);
    let (runtime_args_ptr, runtime_args_size, _bytes) = to_ptr(runtime_args);

    let bytes_written = {
        let mut bytes_written = MaybeUninit::uninit();
        let ret = unsafe {
            ext_ffi::casper_call_versioned_contract(
                contract_package_hash_ptr,
                contract_package_hash_size,
                contract_version_ptr,
                contract_version_size,
                entry_point_name_ptr,
                entry_point_name_size,
                runtime_args_ptr,
                runtime_args_size,
                bytes_written.as_mut_ptr(),
            )
        };
        api_error::result_from(ret).unwrap_or_revert();
        unsafe { bytes_written.assume_init() }
    };
    deserialize_contract_result(bytes_written)
}

fn deserialize_contract_result(bytes_written: usize) -> Vec<u8> {
    if bytes_written == 0 {
        // If no bytes were written, the host buffer hasn't been set and hence shouldn't be read.
        Vec::new()
    } else {
        // NOTE: this is a copy of the contents of `read_host_buffer()`.  Calling that directly from
        // here causes several contracts to fail with a Wasmi `Unreachable` error.
        let bytes_non_null_ptr = contract_api::alloc_bytes(bytes_written);
        let mut dest: Vec<u8> = unsafe {
            Vec::from_raw_parts(bytes_non_null_ptr.as_ptr(), bytes_written, bytes_written)
        };
        read_host_buffer_into(&mut dest).unwrap_or_revert();
        dest
    }
}

fn read_host_buffer_into(dest: &mut [u8]) -> Result<usize, ApiError> {
    let mut bytes_written = MaybeUninit::uninit();
    let ret = unsafe {
        ext_ffi::casper_read_host_buffer(dest.as_mut_ptr(), dest.len(), bytes_written.as_mut_ptr())
    };
    // NOTE: When rewriting below expression as `result_from(ret).map(|_| unsafe { ... })`, and the
    // caller ignores the return value, execution of the contract becomes unstable and ultimately
    // leads to `Unreachable` error.
    api_error::result_from(ret)?;
    Ok(unsafe { bytes_written.assume_init() })
}

fn to_ptr<T: ToBytes>(t: T) -> (*const u8, usize, Vec<u8>) {
    let bytes = t.into_bytes().unwrap_or_revert();
    let ptr = bytes.as_ptr();
    let size = bytes.len();
    (ptr, size, bytes)
}
