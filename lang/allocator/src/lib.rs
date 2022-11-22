// Copyright (c) 2022 The MobileCoin Foundation
#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations, unsafe_code)]
#![no_std]

#[global_allocator]
static ALLOCATOR: mc_sgx_alloc::Allocator = mc_sgx_alloc::Allocator;
