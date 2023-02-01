// Copyright (c) 2022-2023 The MobileCoin Foundation

#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations)]
#![no_std]

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(feature = "log")]
mod log;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(feature = "log")]
    log::log_panic_info(_info);

    extern "C" {
        fn abort() -> !;
    }

    unsafe { abort() }
}
