// Copyright (c) 2022 The MobileCoin Foundation

#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations)]
#![no_std]

extern crate alloc;
use alloc::string::ToString;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Ignore the result, we're already panicking we can't really do much if
    // `stderr_write_all()` fails
    let _ = mc_sgx_io::stderr_write_all(info.to_string().as_bytes());

    loop {}
}
