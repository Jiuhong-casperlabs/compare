#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use alloc::string::{String, ToString};

use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPoints},
    CLType, CLTyped, ContractHash, EntryPointAccess, EntryPointType, Key, Parameter,
};

use casper_contract::unwrap_or_revert::UnwrapOrRevert;

use core::convert::TryInto;

const ENTRY_FUNCTION_NAME: &str = "delegate";
const PACKAGE_HASH_KEY_NAME: &str = "do_nothing_package_hash";
const ACCESS_KEY_NAME: &str = "do_nothing_access";
const ARG_PURSE_NAME: &str = "purse_name";

#[no_mangle]
pub extern "C" fn delegate() {}

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();

            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let entry_points = {
        let mut entry_points = EntryPoints::new();
        let entry_point = EntryPoint::new(
            ENTRY_FUNCTION_NAME.to_string(),
            vec![Parameter::new(ARG_PURSE_NAME, String::cl_type())],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(entry_point);
        entry_points
    };

    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        None,
        Some(PACKAGE_HASH_KEY_NAME.to_string()),
        Some(ACCESS_KEY_NAME.to_string()),
    );

    runtime::put_key("tttruntime", contract_hash.into());
    let result1: ContractHash = runtime::get_key("tttruntime")
        .and_then(Key::into_hash)
        .expect("should have key")
        .into();
    runtime::put_key("tttruntime1", result1.into());

    set_key("tttsetkey", contract_hash);

    let result2: ContractHash = get_key("tttsetkey");
    set_key("tttsetkey1", result2);

    //get blocktime
    let actual_block_time: u64 = runtime::get_blocktime().into();
    let uref = storage::new_uref(actual_block_time);
    runtime::put_key("blocktimenew", uref.into());
}
