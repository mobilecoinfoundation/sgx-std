// Copyright (c) 2022-2024 The MobileCoin Foundation

#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations)]
#![no_std]

#[cfg(not(test))]
use core::panic::PanicInfo;
#[cfg(not(test))]
use mc_sgx_panic_sys::panic_count;

#[cfg(feature = "log")]
mod log;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    extern "C" {
        fn abort() -> !;
    }

    let panics = panic_count::increase();

    // If we entered the panic handler more than once then we must have panicked
    // while trying to handle the panic. Fail hard in these instances, nothing
    // more we can do.
    if panics > 1 {
        unsafe { abort() }
    }

    #[cfg(feature = "log")]
    log::log_panic_info(_info);

    unsafe { abort() }
}
