// Copyright (c) 2022-2024 The MobileCoin Foundation

#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations)]
#![no_std]

extern crate alloc;

mod allocator;

pub use crate::allocator::Allocator;

/// Defines a global allocator for use in an SGX enclave.
///
/// This should only be used in one place in the enclave binary.
///
/// The macro takes one argument, the name of the static allocator object.
///
/// # Example
///
/// ```
/// mc_sgx_alloc::allocator!(ALLOCATOR_NAME);
/// ```
#[macro_export]
macro_rules! allocator {
    ($name:ident) => {
        #[global_allocator]
        static $name: $crate::Allocator = $crate::Allocator;
    };
}
