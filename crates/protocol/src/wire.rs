//! The wire's reader and writers (ADR 0001 part 6): whole bytes, little-endian. The reader never panics on any
//! bytes: every read asks first whether that many bytes are left, and a message that ends early is an error, not an
//! index out of range. No field of the first messages is narrower than a byte; a bit reader joins with the first
//! field that is (the snapshot's entity sections).

use crate::codec::DecodeError;

/// Reads a message front to back. A read that fails takes nothing.
pub struct Reader<'a> {
    left: &'a [u8],
}

impl<'a> Reader<'a> {
    #[must_use]
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { left: bytes }
    }

    /// How many bytes have not been read yet.
    #[must_use]
    pub const fn left(&self) -> usize {
        self.left.len()
    }

    /// The next `count` bytes; `Truncated` when fewer are left, as for every read below.
    pub fn bytes(&mut self, count: usize) -> Result<&'a [u8], DecodeError> {
        let (taken, left) = self
            .left
            .split_at_checked(count)
            .ok_or(DecodeError::Truncated)?;
        self.left = left;
        Ok(taken)
    }

    /// The next `N` bytes, as an array.
    pub fn array<const N: usize>(&mut self) -> Result<[u8; N], DecodeError> {
        let (taken, left) = self
            .left
            .split_first_chunk::<N>()
            .ok_or(DecodeError::Truncated)?;
        self.left = left;
        Ok(*taken)
    }

    pub fn u8(&mut self) -> Result<u8, DecodeError> {
        self.array().map(u8::from_le_bytes)
    }

    pub fn u16(&mut self) -> Result<u16, DecodeError> {
        self.array().map(u16::from_le_bytes)
    }

    pub fn u32(&mut self) -> Result<u32, DecodeError> {
        self.array().map(u32::from_le_bytes)
    }

    pub fn i8(&mut self) -> Result<i8, DecodeError> {
        self.array().map(i8::from_le_bytes)
    }

    pub fn i16(&mut self) -> Result<i16, DecodeError> {
        self.array().map(i16::from_le_bytes)
    }

    /// Ends the read, with `Trailing` when bytes are left: a message is taken whole or not at all (ADR 0001 part
    /// 5: "trailing bytes or unknown fields" drop the message).
    pub const fn finish(self) -> Result<(), DecodeError> {
        if self.left.is_empty() {
            Ok(())
        } else {
            Err(DecodeError::Trailing)
        }
    }
}

pub fn put_u8(out: &mut Vec<u8>, value: u8) {
    out.push(value);
}

pub fn put_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub fn put_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub fn put_i8(out: &mut Vec<u8>, value: i8) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub fn put_i16(out: &mut Vec<u8>, value: i16) {
    out.extend_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_width_is_read_as_it_was_written_little_endian() {
        let mut bytes = Vec::new();
        put_u8(&mut bytes, 0xAB);
        put_u16(&mut bytes, 0x1234);
        put_u32(&mut bytes, 0xDEAD_BEEF);
        put_i8(&mut bytes, -2);
        put_i16(&mut bytes, -300);
        bytes.extend_from_slice(&[7, 8, 9]);
        assert_eq!(
            bytes,
            [
                0xAB, 0x34, 0x12, 0xEF, 0xBE, 0xAD, 0xDE, 0xFE, 0xD4, 0xFE, 7, 8, 9
            ]
        );
        let mut reader = Reader::new(&bytes);
        assert_eq!(reader.left(), 13);
        assert_eq!(reader.u8(), Ok(0xAB));
        assert_eq!(reader.u16(), Ok(0x1234));
        assert_eq!(reader.u32(), Ok(0xDEAD_BEEF));
        assert_eq!(reader.i8(), Ok(-2));
        assert_eq!(reader.i16(), Ok(-300));
        assert_eq!(reader.array::<2>(), Ok([7, 8]));
        assert_eq!(reader.bytes(1), Ok(&[9][..]));
        assert_eq!(reader.left(), 0);
        assert_eq!(reader.finish(), Ok(()));
    }

    #[test]
    fn a_read_past_the_end_is_an_error_and_takes_nothing() {
        let bytes = [1, 2, 3];
        let mut reader = Reader::new(&bytes);
        assert_eq!(reader.u32(), Err(DecodeError::Truncated));
        assert_eq!(reader.bytes(4), Err(DecodeError::Truncated));
        assert_eq!(reader.bytes(usize::MAX), Err(DecodeError::Truncated));
        assert_eq!(reader.array::<4>(), Err(DecodeError::Truncated));
        // The three bytes are still there, and a read of nothing always works.
        assert_eq!(reader.bytes(0), Ok(&[][..]));
        assert_eq!(reader.u16(), Ok(0x0201));
        assert_eq!(reader.i16(), Err(DecodeError::Truncated));
        assert_eq!(reader.u8(), Ok(3));
        assert_eq!(reader.u8(), Err(DecodeError::Truncated));
        assert_eq!(reader.i8(), Err(DecodeError::Truncated));
        assert_eq!(Reader::new(&[]).u8(), Err(DecodeError::Truncated));
    }

    #[test]
    fn bytes_left_at_the_end_are_an_error() {
        assert_eq!(Reader::new(&[]).finish(), Ok(()));
        assert_eq!(Reader::new(&[0]).finish(), Err(DecodeError::Trailing));
        let mut reader = Reader::new(&[5, 6]);
        assert_eq!(reader.u8(), Ok(5));
        assert_eq!(reader.finish(), Err(DecodeError::Trailing));
    }
}
