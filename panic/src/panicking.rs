// Copyright (c) 2023 The MobileCoin Foundation

#[allow(dead_code)]

/// Common backend panic implementation
///
/// This is a subset of the functionality available in Rust's std
/// [panicking.rs](https://github.com/rust-lang/rust/blob/master/library/std/src/panicking.rs)
/// module.

pub(crate) mod panic_count {
    //! Number of panics that are currently being handled on the current thread
    //!
    //! This deviates from
    //! [panicking.rs](https://github.com/rust-lang/rust/blob/master/library/std/src/panicking.rs)
    //! It does not:
    //! - use `GLOBAL_PANIC_COUNT` count. The global count is meant to provide
    //!   an optimization on some platforms. It's not clear if SGX benefits from
    //!   this optimization and thus needing this complexity
    //! - no `set_always_abort()`. The docs say this is for `libc::fork`.
    //!   Enclaves do not support forking or spawning child threads or processes
    //! - use the `thread_local!` macro or the `::std::thread::LocalKey` type.
    //!   These are generic over any type that `needs_drop`, but
    //!   `usize` does not need drop
    use core::cell::Cell;

    /// Panic count for the current thread.
    #[thread_local]
    static LOCAL_PANIC_COUNT: Cell<usize> = Cell::new(0);

    /// Increase the number of panics that are currently happening
    ///
    /// # Returns
    /// The number of panics that are currently happening, including this
    /// increase.
    pub fn increase() -> usize {
        let panics = LOCAL_PANIC_COUNT.get() + 1;
        LOCAL_PANIC_COUNT.set(panics);
        panics
    }

    /// Decrease the number of panics that are currently happening
    pub fn decrease() {
        let panics = LOCAL_PANIC_COUNT.get() - 1;
        LOCAL_PANIC_COUNT.set(panics);
    }

    /// Get the current number of panics that are in process
    #[must_use]
    pub fn get_count() -> usize {
        LOCAL_PANIC_COUNT.get()
    }

    /// Returns `true` when the there are no panics currently being handled
    #[must_use]
    pub fn count_is_zero() -> bool {
        get_count() == 0
    }

    #[cfg(test)]
    mod test {
        use super::*;

        /// Reset the panic count back to 0
        ///
        /// By default each test runs in a separate thread thus having their own
        /// thread local copy of `LOCAL_PANIC_COUNT`. However per
        /// https://github.com/rust-lang/rust/issues/58907 when running single
        /// threaded the thread is re-used for each test execution so the value
        /// should be reset to a known value on each test. This also means that
        /// there isn't a reliable way to test that 0 is the true initial value.
        fn reset_panic_count() {
            LOCAL_PANIC_COUNT.set(0);
        }

        #[test]
        fn count_is_zero_is_false_when_not_zero() {
            reset_panic_count();
            increase();
            assert!(!count_is_zero());
        }

        #[test]
        fn count_is_zero_is_true_when_zero() {
            reset_panic_count();
            assert!(count_is_zero());
            increase();
            assert!(!count_is_zero());
            decrease();
            assert!(count_is_zero());
        }

        #[test]
        fn incrementing_one_at_a_time() {
            reset_panic_count();
            assert_eq!(increase(), 1);
            assert_eq!(increase(), 2);
            assert_eq!(increase(), 3);
            assert_eq!(increase(), 4);
            assert_eq!(get_count(), 4);
        }

        #[test]
        fn decrementing_one_at_a_time() {
            LOCAL_PANIC_COUNT.set(4);
            assert_eq!(get_count(), 4);
            decrease();
            assert_eq!(get_count(), 3);
            decrease();
            assert_eq!(get_count(), 2);
            decrease();
            assert_eq!(get_count(), 1);
            decrease();
            assert_eq!(get_count(), 0);
        }
    }
}
