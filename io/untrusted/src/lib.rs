// Copyright (c) 2022 The MobileCoin Foundation
#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations)]

//! Module for providing stderr IO from an enclave.
//!
//! By default stderr from an enclave will be directed to the untrusted (host)
//! stderr. Consumers can redirect this stream by providing a [`WriteAll`]
//! function via [`stderr_sink`].

use once_cell::sync::Lazy;
use std::ffi::c_void;
use std::io::Write;
use std::slice;
use std::sync::Mutex;

/// A function that that writes the entire contents of the provided buffer.
///
/// This is meant be a stand alone version of [`std::io::Write::write_all`].
pub type WriteAll = dyn Fn(&[u8]);

/// Specify the function to use for stderr messages
///
/// # Arguments
/// * `write_all` - The function to use for writing stderr from the enclave
pub fn stderr_sink(write_all: &'static WriteAll) {
    let mut stderr = STDERR.lock().expect("Mutex has been poisoned");
    stderr.write_all = write_all;
}

/// Wraps the `write_all` function in a struct so that we can implement the
/// `Send` trait.
struct Stream {
    write_all: &'static dyn Fn(&[u8]),
}

/// SAFETY: The [`Stream`] is local to this crate and will be enclosed in a
/// Mutex so is safe to make `Send`.
unsafe impl Send for Stream {}

/// The stderr stream to use for the `ocall_stderr`
static STDERR: Lazy<Mutex<Stream>> = Lazy::new(|| {
    Mutex::new(Stream {
        write_all: &default_stderr_write_all,
    })
});

/// A ['WriteAll] function that directs to [`std::io::stderr`]
fn default_stderr_write_all(buf: &[u8]) {
    std::io::stderr()
        .write_all(buf)
        .expect("Failed writing to stderr");
}

#[no_mangle]
/// The ocall that will take in stderr messages from the enclave.
extern "C" fn ocall_stderr(input: *const c_void, len: usize) {
    // SAFETY: Converting from C interface to Rust. We must rely on the enclave
    // side of the implementation to provide the correct length for the input
    // buffer
    let bytes = unsafe { slice::from_raw_parts(input as *const u8, len) };
    let stderr = STDERR.lock().expect("Mutex has been poisoned");
    (stderr.write_all)(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mc_sgx_urts::EnclaveBuilder;
    use mc_sgx_util::ResultInto;
    use serial_test::serial;
    use test_enclave::{ecall_round_trip_to_stderr, ENCLAVE};

    static TEST_STREAM: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
    fn test_stream_write_all(message: &[u8]) {
        let mut error_string = TEST_STREAM.lock().expect("Mutex has been poisoned");
        error_string.clear();
        error_string.push_str(std::str::from_utf8(message).unwrap());
    }

    fn test_stream_contents() -> String {
        TEST_STREAM.lock().expect("Mutex has been poisoned").clone()
    }

    #[test]
    #[serial]
    fn one_line_error_message() {
        stderr_sink(&test_stream_write_all);
        let enclave = EnclaveBuilder::from(ENCLAVE).create().unwrap();
        let id = enclave.id();

        let message = b"a one liner";
        unsafe {
            ecall_round_trip_to_stderr(*id, message.as_ptr() as *const c_void, message.len())
        }
        .into_result()
        .unwrap();
        let output = test_stream_contents();
        assert_eq!(output.as_str(), "a one liner");
    }

    #[test]
    #[serial]
    fn multi_line_error_message() {
        stderr_sink(&test_stream_write_all);
        let enclave = EnclaveBuilder::from(ENCLAVE).create().unwrap();
        let id = enclave.id();

        let message = b"this is\nmulti line\n";
        unsafe {
            ecall_round_trip_to_stderr(*id, message.as_ptr() as *const c_void, message.len())
        }
        .into_result()
        .unwrap();
        let output = test_stream_contents();
        assert_eq!(output.as_str(), "this is\nmulti line\n");
    }
}
