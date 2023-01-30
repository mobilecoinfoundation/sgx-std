// Copyright (c) 2023 The MobileCoin Foundation

//! This is a subset of the functionality available in Rust's std
//! [thread/mod.rs](https://github.com/rust-lang/rust/blob/master/library/std/src/thread/mod.rs)
//! module.
//!
//! SGX enclaves don't support many of the threading primitives so this is a
//! very small subset of std::thread

use crate::panicking;

/// Determines whether the current thread is unwinding because of panic.
///
/// A common use of this feature is to poison shared resources when writing
/// unsafe code, by checking `panicking` when the `drop` is called.
///
/// This is usually not needed when writing safe code, as [`Mutex`es][Mutex]
/// already poison themselves when a thread panics while holding the lock.
///
/// This can also be used in multithreaded applications, in order to send a
/// message to other threads warning that a thread has panicked (e.g., for
/// monitoring purposes).
///
/// # Examples
///
/// ```should_panic
/// use mc_sgx_panic_sys::thread;
///
/// struct SomeStruct;
///
/// impl Drop for SomeStruct {
///     fn drop(&mut self) {
///         if thread::panicking() {
///             println!("dropped while unwinding");
///         } else {
///             println!("dropped while not unwinding");
///         }
///     }
/// }
///
/// {
///     print!("a: ");
///     let a = SomeStruct;
/// }
///
/// {
///     print!("b: ");
///     let b = SomeStruct;
///     panic!()
/// }
/// ```
///
/// [Mutex]: mc-sgx-sync::Mutex
#[inline]
#[must_use]
pub fn panicking() -> bool {
    panicking::panicking()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::panicking::panic_count;

    /// Sets panic count is 0
    /// Similar to the tests in mod `panicking::panic_count` tests need to call
    /// this prior to testing to ensure correct behavior
    fn clear_panic_count() {
        while panic_count::get_count() != 0 {
            panic_count::decrease();
        }
    }

    #[test]
    fn is_panicking_false_when_panic_count_zero() {
        clear_panic_count();
        assert!(!panicking());
    }

    #[test]
    fn is_panicking_true_when_panic_count_gt_zero() {
        clear_panic_count();

        // First panic
        panic_count::increase();
        assert!(panicking());

        // Second panic
        panic_count::increase();
        assert!(panicking());

        // finished second panic
        panic_count::decrease();
        assert!(panicking());

        // finished first panic
        // NB that this one is *not* panicking since we've decreased back down
        // to zero
        panic_count::decrease();
        assert!(!panicking());
    }
}
