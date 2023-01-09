// Copyright (c) 2022 The MobileCoin Foundation

#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations)]
#![no_std]

use core::ffi::c_void;
use mc_sgx_core_sys_types::sgx_status_t;
use mc_sgx_core_types::Error;
use mc_sgx_util::ResultInto;

/// Write the entire `buffer` into the hosts stderr sink.
///
/// # Arguments
/// * `buffer` - The buffer to write.
///
/// # Errors
/// When not all of `buffer` could be written to the hosts stderr sink.
///
/// If there is an error, no assumptions should be made about the amount of
/// `buffer` that was written.
pub fn stderr_write_all(buffer: &[u8]) -> Result<(), Error> {
    unsafe { ocall_stderr(buffer.as_ptr() as *const c_void, buffer.len()) }.into_result()
}

extern "C" {
    /// The ocall to send stderr messages to
    ///
    /// # Arguments
    /// * `input` - The input buffer/stream. Should be u8/bytes
    /// * `len` - The byte length of `input`
    ///
    /// # Returns
    /// `sgx_status_t::SGX_SUCCESS` when all of input was successfully written
    /// to the untrusted stderr sink.
    /// An error status if not all of the data could be written to the sink. No
    /// assumptions are made about how much data was written on error.
    fn ocall_stderr(input: *const c_void, len: usize) -> sgx_status_t;
}

// Done out here so that `serial_test` works, since it uses "::std" in the macro
#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::*;
    use core::slice;
    use once_cell::sync::Lazy;
    use serial_test::serial;
    use std::string::String;
    use std::sync::Mutex;

    static TEST_STREAM: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
    static TEST_STREAM_RESULT: Lazy<Mutex<sgx_status_t>> =
        Lazy::new(|| Mutex::new(sgx_status_t::SGX_SUCCESS));

    fn reset_test_stream() {
        let mut stream = TEST_STREAM.lock().expect("Mutex has been poisoned");
        stream.clear();
        let mut status = TEST_STREAM_RESULT.lock().expect("Mutex has been poisoned");
        *status = sgx_status_t::SGX_SUCCESS;
    }

    #[no_mangle]
    extern "C" fn ocall_stderr(input: *const c_void, len: usize) -> sgx_status_t {
        let bytes = unsafe { slice::from_raw_parts(input as *const u8, len) };
        let message =
            std::str::from_utf8(bytes).expect("Expected valid UTF8 from stderr in enclave");
        let mut stream = TEST_STREAM.lock().expect("Mutex has been poisoned");
        stream.clear();
        stream.push_str(message);
        let status = TEST_STREAM_RESULT.lock().expect("Mutex has been poisoned");
        *status
    }

    #[test]
    #[serial]
    fn single_line_output_to_stderr() {
        reset_test_stream();
        let test_message = b"what";
        stderr_write_all(test_message).expect("Expected the write to succeed");

        let written = TEST_STREAM.lock().expect("Mutex has been poisoned");
        assert_eq!(written.as_bytes(), test_message);
    }

    #[test]
    #[serial]
    fn multi_line_output_to_stderr() {
        reset_test_stream();
        let test_message = b"this\nhas\nmultiple\nlines";
        stderr_write_all(test_message).expect("Expected the write to succeed");

        let written = TEST_STREAM.lock().expect("Mutex has been poisoned");
        assert_eq!(written.as_bytes(), test_message);
    }

    #[test]
    #[serial]
    fn error_when_outputting_to_stderr() {
        reset_test_stream();
        let expected_error = sgx_status_t::SGX_ERROR_FILE_BAD_STATUS;
        {
            let mut status = TEST_STREAM_RESULT.lock().expect("Mutex has been poisoned");
            *status = expected_error;
        }

        let error = stderr_write_all(b"what").unwrap_err();
        assert_eq!(error, Error::FileBadStatus);
    }
}
