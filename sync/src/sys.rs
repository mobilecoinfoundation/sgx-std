// Copyright (c) 2023 The MobileCoin Foundation

//! Platform specific abstraction.
//!
//! Provides an abstracted interface over the SGX interface.
//! This follows the pattern of the
//! [`std::sys`](https://github.com/rust-lang/rust/blob/master/library/std/src/sys/mod.rs)
//! module. The difference being this _only_ supports the SGX interface.
//! Having this allows us to re-use the higher level code modules from
//! [`::std`](https://github.com/rust-lang/rust/tree/master/library/std/src)

mod mutex;
