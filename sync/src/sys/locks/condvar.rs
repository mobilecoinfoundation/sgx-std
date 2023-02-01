// Copyright (c) 2023 The MobileCoin Foundation

//! Rust condition variable implementation used in SGX environments
#![allow(dead_code)]
use crate::sys::locks::Mutex;
use mc_sgx_tstdc::Condvar as SgxCondvar;

/// The condition variable backend to use with the common Rust std lib Condvar
/// interface
pub(crate) struct Condvar {
    inner: SgxCondvar,
}

unsafe impl Send for Condvar {}
unsafe impl Sync for Condvar {}

impl Condvar {
    pub const fn new() -> Self {
        Self {
            inner: SgxCondvar::new(),
        }
    }

    /// Wait on the condition variable
    ///
    /// # Arguments
    /// * `mutex` - The mutex to paired with the current [`Condvar`]
    ///
    /// # Panics
    /// If:
    /// - the condition variable got into an invalid state
    /// - the [`Mutex`] is not locked by the current thread
    pub fn wait(&self, mutex: &Mutex) {
        self.inner
            .wait(mutex.raw())
            .expect("Condition variable is invalid or mutex is not locked by current thread");
    }

    /// Notify the next waiting thread (if any) of the condition variable event
    ///
    /// Returns when there are no waiting threads.
    ///
    /// # Panics
    /// If the condition variable got into an invalid state
    pub fn notify_one(&self) {
        self.inner
            .notify_one()
            .expect("Condition variable is in an invalid state");
    }

    /// Notify *all* waiting threads of the condition variable event
    ///
    /// Returns when there are no waiting threads.
    ///
    /// # Panics
    /// If:
    /// - the condition variable got into an invalid state
    /// - ran out of memory while notifying other threads.
    pub fn notify_all(&self) {
        // For the `expect()` message, out of memory could be a reason for
        // failing, but it is unlikely given that the memory allocation is a
        // `void *` per waiting thread.
        self.inner
            .notify_all()
            .expect("Condition variable is in an invalid state");
    }
}
