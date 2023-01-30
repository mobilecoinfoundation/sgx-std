// Copyright (c) 2023 The MobileCoin Foundation

#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations)]
#![no_std]
#![feature(error_in_core, must_not_suspend, negative_impls)]

mod mutex;
mod poison;
pub use mutex::{Mutex, MutexGuard};
pub use poison::{LockResult, PoisonError, TryLockError, TryLockResult};
mod sys;
