// Copyright (c) 2022-2023 The MobileCoin Foundation
#![feature(thread_local)]
#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations, unsafe_code)]
#![no_std]

mod panicking;
pub mod thread;
pub use panicking::panic_count;
