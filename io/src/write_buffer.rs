// Copyright (c) 2023 The MobileCoin Foundation

//! Provides a sized buffer which implements the [`fmt::Write`] trait.

use core::fmt;

/// Byte size of [`WriteBuffer`].
///
/// Attempting to write more than this many bytes to the [`WriteBuffer`] will
/// result in an error.
pub const BUFFER_SIZE: usize = 4096;

/// A buffer which implements the [`fmt::Write`] trait.
#[derive(Debug)]
pub struct WriteBuffer {
    buf: [u8; BUFFER_SIZE],
    pos: usize,
}

impl WriteBuffer {
    /// Create a new empty [`WriteBuffer`]
    pub const fn new() -> Self {
        WriteBuffer {
            buf: [0; BUFFER_SIZE],
            pos: 0,
        }
    }

    /// Clear the contents in the [`WriteBuffer`]
    pub fn clear(&mut self) {
        self.pos = 0;
    }
}

impl AsRef<str> for WriteBuffer {
    fn as_ref(&self) -> &str {
        // Shouldn't fail because [`Write::write_str()`] is the only public way
        // to add content. [`Write::write_str()`] takes a `&str` so for this to
        // fail someone must have coerced invalid UTF-8 to a string prior to
        // this method invocation.
        core::str::from_utf8(&self.buf[..self.pos])
            .expect("`WriteBuffer` is not valid UTF-8. It should have only been given `&str`s")
    }
}

impl AsRef<[u8]> for WriteBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.buf[..self.pos]
    }
}

impl fmt::Write for WriteBuffer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        let bytes = string.as_bytes();

        let remaining = &mut self.buf[self.pos..];
        if remaining.len() < bytes.len() {
            return Err(fmt::Error);
        }

        let new_contents = &mut remaining[..bytes.len()];
        new_contents.copy_from_slice(bytes);

        self.pos += bytes.len();

        Ok(())
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use super::*;
    use core::fmt::Write;
    use std::vec;
    use yare::parameterized;

    #[test]
    fn new_write_buffer_is_empty() {
        let buffer = WriteBuffer::new();

        let contents: &str = buffer.as_ref();
        assert_eq!(contents, "");
    }

    #[parameterized(
    what = {&["what"], "what"},
    hello = {&["hello"], "hello"},
    good_bye = {&["good", " ", "bye"], "good bye"},
    i_like_long_strings = {&["i", " ", "like", " ", "long", " ", "strings"], "i like long strings"},
    no_spaces = {&["no", "spaces"], "nospaces"},
    )]
    fn write_buffer_contains_the_written_contents(messages: &[&str], expected: &str) {
        let mut buffer = WriteBuffer::new();

        for message in messages {
            buffer
                .write_str(message)
                .expect("Shouldn't fail to write message");
        }

        let contents: &str = buffer.as_ref();
        assert_eq!(contents, expected);
    }

    #[parameterized(
    why_not = {b"why not"},
    you_bet = {b"you_bet"},
    )]
    fn write_buffer_as_bytes(message: &[u8]) {
        let mut buffer = WriteBuffer::new();
        let message_str = core::str::from_utf8(message).expect("Message should be valid UTF-8");

        buffer
            .write_str(message_str)
            .expect("Shouldn't fail to write message");

        let contents: &[u8] = buffer.as_ref();
        assert_eq!(contents, message);
    }

    #[test]
    fn write_buffer_can_hold_4096_bytes() {
        let mut buffer = WriteBuffer::new();
        // 66 == 'B'
        let mut message = vec![66u8; BUFFER_SIZE - 1];
        let message_str = core::str::from_utf8(&message).expect("Message should be valid UTF-8");

        buffer
            .write_str(message_str)
            .expect("Shouldn't fail to write message");
        buffer
            .write_str("C")
            .expect("Shouldn't fail to write last byte");

        let contents: &[u8] = buffer.as_ref();
        message.push(67);
        assert_eq!(contents, message);
    }

    #[test]
    fn write_buffer_errors_at_4097_bytes() {
        let mut buffer = WriteBuffer::new();
        let message = [66u8; BUFFER_SIZE - 1];
        let message_str = core::str::from_utf8(&message).expect("Message should be valid UTF-8");

        buffer
            .write_str(message_str)
            .expect("Shouldn't fail to write message");
        buffer
            .write_str("C")
            .expect("Shouldn't fail to write last byte");

        assert!(buffer.write_str("D").is_err());
    }
}
