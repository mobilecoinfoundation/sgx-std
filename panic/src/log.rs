// Copyright (c) 2022-2023 The MobileCoin Foundation

//! Logging utilities used during panic handling

use core::fmt::Write;
use core::panic::PanicInfo;
use mc_sgx_io::WriteBuffer;
use mc_sgx_sync::Mutex;

/// A buffer for building up the panic message.
/// We avoid allocating when handling the panic as failure to allocate could be
/// the cause of the panic.
static MESSAGE_BUFFER: Mutex<WriteBuffer> = Mutex::new(WriteBuffer::new());

/// Log information during a panic
///
/// If for some reason the `info` exceeds the size of the [`MESSAGE_BUFFER`]
/// then this will log a default message.
///
/// # Arguments:
/// * `info` - The panic information to log
pub(crate) fn log_panic_info(info: &PanicInfo) {
    if let Ok(mut buffer) = MESSAGE_BUFFER.lock() {
        buffer.clear();
        let message = match write!(buffer, "{info}") {
            Ok(()) => buffer.as_ref(),
            _ => "Failed to format panic log info.",
        };

        // Ignore the result, we're already panicking we can't really do much if
        // `stderr_write_all()` fails
        let _ = mc_sgx_io::stderr_write_all(message.as_bytes());
    } else {
        let _ = mc_sgx_io::stderr_write_all(b"Mutex for panic logging has been poisoned.");
    }
}
