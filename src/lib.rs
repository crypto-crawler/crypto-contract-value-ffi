#![allow(clippy::not_unsafe_ptr_arg_deref)]
mod market_type;
pub use market_type::*;

use std::{
    ffi::CStr,
    os::raw::{c_char, c_double},
};

/// Get the unit value of a contract, 0 means not found.
#[no_mangle]
pub extern "C" fn get_contract_value(
    exchange: *const c_char,
    market_type: MarketType,
    pair: *const c_char,
) -> c_double {
    let exchange = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };

    let market_type = market_type.to_rust();

    let pair = unsafe {
        debug_assert!(!pair.is_null());
        CStr::from_ptr(pair).to_str().unwrap()
    };

    let result = std::panic::catch_unwind(|| {
        if let Some(contract_value) =
            crypto_contract_value::get_contract_value(exchange, market_type, pair)
        {
            contract_value
        } else {
            0.0 // 0 means not found
        }
    });
    match result {
        Ok(contract_value) => contract_value,
        Err(err) => {
            eprintln!("{:?}", err);
            0.0
        }
    }
}
