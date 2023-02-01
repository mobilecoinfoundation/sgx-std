// Copyright (c) 2023 The MobileCoin Foundation

//! Rust RwLock implementation used in SGX environments

#![allow(dead_code)]

use mc_sgx_tstdc::RwLock as SgxRwLock;

/// SGX rwlock backend to use with the common Rust std lib
/// [`RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html) interface.
///
///
/// Read locks are implemented using a reference count. This means that clients
/// are responsible for managing which threads hold the read locks and ensuring
/// threads only unlock if they are currently holding a reader lock. Write locks
/// do track which thread owns them.
pub(crate) struct RwLock {
    inner: SgxRwLock,
}

unsafe impl Send for RwLock {}
unsafe impl Sync for RwLock {}

impl RwLock {
    /// Create a new [`RwLock`]
    pub const fn new() -> RwLock {
        RwLock {
            inner: SgxRwLock::new(),
        }
    }

    /// Acquire a read lock on the [`RwLock`]
    ///
    /// NB: This acquires **a** reader lock on the [`RwLock`] instance, it does
    ///     not keep track of which threads have reader locks.
    ///
    /// # Panics
    /// Panics if the [`RwLock`] got into an invalid state. Invalid states
    /// include:
    /// - Corrupted underlying data
    /// - Trying to obtain a read lock when thread has write lock
    pub fn read(&self) {
        self.inner
            .read()
            .expect("RwLock got into an invalid state.")
    }

    /// Try to acquire a read lock on the [`RwLock`]
    ///
    /// NB: This acquires **a** reader lock on the [`RwLock`] instance, it does
    ///     not keep track of which threads have reader locks.
    ///
    /// # Returns
    /// `true` if a read lock was acquired, `false` otherwise.
    pub fn try_read(&self) -> bool {
        self.inner
            .try_read()
            .expect("RwLock got into an invalid state.")
    }

    /// Acquire a write lock on the [`RwLock`]
    ///
    /// # Panics
    /// Panics if the [`RwLock`] got into an invalid state. Invalid states
    /// include:
    /// - Corrupted underlying data
    /// - Trying to obtain a write lock when the thread already has the write
    ///   lock
    pub fn write(&self) {
        self.inner
            .read()
            .expect("RwLock got into an invalid state.")
    }

    /// Try to acquire a read lock on the [`RwLock`]
    ///
    /// # Returns
    /// `true` if a read lock was acquired, `false` otherwise.
    pub fn try_write(&self) -> bool {
        self.inner
            .try_write()
            .expect("RwLock got into an invalid state.")
    }

    /// Release a read lock on the [`RwLock`]
    ///
    /// NB: This release **a** reader lock on the [`RwLock`] instance, it does
    ///     not validate that the current thread created the reader lock.
    ///
    /// # Panics
    /// Panics if the [`RwLock`] got into an invalid state. Invalid states
    /// include:
    /// - Corrupted underlying data
    /// - Trying to unlock when there are no read locks currently held
    pub fn read_unlock(&self) {
        self.inner
            .read_unlock()
            .expect("RwLock got into an invalid state.")
    }

    /// Release the write lock on the [`RwLock`]
    ///
    /// # Panics
    /// Panics if the [`RwLock`] got into an invalid state. Invalid states
    /// include:
    /// - Corrupted underlying data
    /// - Trying to unlock when the current thread doesn't hold the write lock
    pub fn write_unlock(&self) {
        // Note on the `expect()` statement, though unlock can fail due to out
        // of memory it's unlikely to happen as unlocking only allocates a
        // `void *` per waiting reader thread.
        self.inner
            .write_unlock()
            .expect("RwLock got into an invalid state.")
    }
}
