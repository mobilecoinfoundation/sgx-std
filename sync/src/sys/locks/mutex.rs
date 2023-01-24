// Copyright (c) 2023 The MobileCoin Foundation

//! Rust mutex implementation used in SGX environments
//!
//! Per the docs and discussions Rust mutexes are not re-entrant,
//! <https://github.com/rust-lang/rust/issues/32260>.

use mc_sgx_tstdc::Mutex as SgxMutex;

/// The mutex backend to use with the common Rust std lib Mutex interface
pub(crate) struct Mutex {
    inner: SgxMutex,
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

impl Mutex {
    /// Create a new Mutex
    pub(crate) const fn new() -> Mutex {
        Mutex {
            inner: SgxMutex::new(),
        }
    }

    /// Lock the Mutex
    ///
    /// # Panics
    /// Panics if the mutex got into an invalid state. Invalid states include:
    /// - Corrupted underlying data
    /// - Trying to lock when mutex is already locked
    pub(crate) fn lock(&self) {
        self.inner.lock().expect("Mutex got into an invalid state.")
    }

    /// UnLock the Mutex
    ///
    /// # Panics
    /// Panics if the mutex got into an invalid state. Invalid states include:
    /// - Corrupted underlying data
    /// - Trying to unlock when mutex is already unlocked
    pub(crate) fn unlock(&self) {
        self.inner
            .unlock()
            .expect("Mutex got into an invalid state.")
    }

    /// Try to lock the mutex
    ///
    /// # Returns
    /// `true` if the mutex was locked, `false` otherwise.
    pub(crate) fn try_lock(&self) -> bool {
        self.inner
            .try_lock()
            .expect("Mutex got into an invalid state.")
    }

    pub(crate) fn raw(&self) -> &SgxMutex {
        &self.inner
    }
}
