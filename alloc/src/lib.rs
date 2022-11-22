// Copyright (c) 2022 The MobileCoin Foundation

#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations)]
#![no_std]

extern crate alloc;

mod allocator;

pub use crate::allocator::Allocator;
