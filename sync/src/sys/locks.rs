// Copyright (c) 2023 The MobileCoin Foundation

//! Platform specific concurrency locking primitives.

mod mutex;
pub(crate) use mutex::Mutex;
