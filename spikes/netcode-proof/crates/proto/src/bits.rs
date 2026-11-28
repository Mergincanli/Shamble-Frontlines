//! Bit packing, least significant bit first: a value's low bit goes into the lowest free bit of the current byte, so a
//! byte-aligned 16-, 32- or 64-bit value lands little-endian. Every message body is read and written through these
//! two, so a byte-aligned field and a packed one follow one rule.

use crate::error::DecodeError;

/// Writes values of 1 to 64 bits into a byte buffer.
pub(crate) struct BitWriter<'a> {
    out: &'a mut Vec<u8>,
    /// Bits already used in the last byte of `out` (0 when it is full or `out` holds nothing of ours yet).
    used: u32,
}

impl<'a> BitWriter<'a> {
    /// Appends to `out`, starting at a byte boundary.
    pub(crate) fn new(out: &'a mut Vec<u8>) -> BitWriter<'a> {
        BitWriter { out, used: 0 }
    }

    /// The low `bits` bits of `value` (1 to 64; the caller has checked that `value` fits).
    pub(crate) fn put(&mut self, value: u64, bits: u32) {
        debug_assert!((1..=64).contains(&bits));
        debug_assert!(
            bits == 64 || value >> bits == 0,
            "{value} does not fit in {bits} bits"
        );
        let mut left = bits;
        let mut v = value;
        while left > 0 {
            if self.used == 0 {
                self.out.push(0);
            }
            let take = (8 - self.used).min(left);
            let part = (v & ((1u64 << take) - 1)) as u8;
            if let Some(last) = self.out.last_mut() {
                *last |= part << self.used;
            }
            self.used = (self.used + take) % 8;
            v >>= take;
            left -= take;
        }
    }

    /// A byte-aligned run of bytes (the writer must be at a byte boundary).
    pub(crate) fn bytes(&mut self, b: &[u8]) {
        debug_assert_eq!(self.used, 0);
        self.out.extend_from_slice(b);
    }

    /// Ends the current byte with zero bits, so the next value starts at a byte boundary.
    pub(crate) fn align(&mut self) {
        self.used = 0;
    }
}

/// Reads values of 1 to 64 bits from a byte slice; reading past its end is [`DecodeError::Truncated`], never a panic.
pub(crate) struct BitReader<'a> {
    bytes: &'a [u8],
    /// The next bit to read.
    pos: usize,
}

impl<'a> BitReader<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> BitReader<'a> {
        BitReader { bytes, pos: 0 }
    }

    /// The next `bits` bits (1 to 64) as an unsigned value.
    pub(crate) fn get(&mut self, bits: u32) -> Result<u64, DecodeError> {
        debug_assert!((1..=64).contains(&bits));
        let end = self
            .pos
            .checked_add(bits as usize)
            .ok_or(DecodeError::Truncated)?;
        if end > self.bytes.len().saturating_mul(8) {
            return Err(DecodeError::Truncated);
        }
        let mut value = 0u64;
        let mut got = 0u32;
        while got < bits {
            let byte = *self.bytes.get(self.pos / 8).ok_or(DecodeError::Truncated)?;
            let off = (self.pos % 8) as u32;
            let take = (8 - off).min(bits - got);
            let part = (u64::from(byte) >> off) & ((1u64 << take) - 1);
            value |= part << got;
            got += take;
            self.pos += take as usize;
        }
        Ok(value)
    }

    pub(crate) fn u8(&mut self) -> Result<u8, DecodeError> {
        Ok(self.get(8)? as u8)
    }

    pub(crate) fn u16(&mut self) -> Result<u16, DecodeError> {
        Ok(self.get(16)? as u16)
    }

    pub(crate) fn u32(&mut self) -> Result<u32, DecodeError> {
        Ok(self.get(32)? as u32)
    }

    pub(crate) fn u64(&mut self) -> Result<u64, DecodeError> {
        self.get(64)
    }

    /// The next `N` bytes (the reader must be at a byte boundary).
    pub(crate) fn array<const N: usize>(&mut self) -> Result<[u8; N], DecodeError> {
        debug_assert_eq!(self.pos % 8, 0);
        let start = self.pos / 8;
        let end = start.checked_add(N).ok_or(DecodeError::Truncated)?;
        let slice = self.bytes.get(start..end).ok_or(DecodeError::Truncated)?;
        self.pos = end * 8;
        let mut a = [0u8; N];
        a.copy_from_slice(slice);
        Ok(a)
    }

    /// The next `n` bytes (the reader must be at a byte boundary).
    pub(crate) fn take(&mut self, n: usize) -> Result<&'a [u8], DecodeError> {
        debug_assert_eq!(self.pos % 8, 0);
        let start = self.pos / 8;
        let end = start.checked_add(n).ok_or(DecodeError::Truncated)?;
        let slice = self.bytes.get(start..end).ok_or(DecodeError::Truncated)?;
        self.pos = end * 8;
        Ok(slice)
    }

    /// Skips to the next byte boundary. With `strict`, the skipped bits must be zero: a set pad bit is a field the
    /// format does not have ([`DecodeError::UnknownField`]).
    pub(crate) fn align(&mut self, strict: bool) -> Result<(), DecodeError> {
        let rest = (8 - self.pos % 8) % 8;
        if rest > 0 {
            let pad = self.get(rest as u32)?;
            if strict && pad != 0 {
                return Err(DecodeError::UnknownField);
            }
        }
        Ok(())
    }

    /// The end of the message: aligned, and, with `strict`, every pad bit zero and no byte left
    /// ([`DecodeError::TrailingBytes`]).
    pub(crate) fn finish(&mut self, strict: bool) -> Result<(), DecodeError> {
        self.align(strict)?;
        if strict && self.pos / 8 != self.bytes.len() {
            return Err(DecodeError::TrailingBytes);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_values_come_back_in_order_and_aligned_ones_are_little_endian() {
        let mut out = Vec::new();
        let mut w = BitWriter::new(&mut out);
        w.put(0b101, 3);
        w.put(0x3ff, 10);
        w.put(u64::MAX, 64);
        w.put(1, 1);
        w.align();
        w.put(0xbeef, 16);
        // 3 + 10 + 64 + 1 = 78 bits in 10 bytes, then the aligned 2.
        assert_eq!(out.len(), 10 + 2);
        assert_eq!(&out[out.len() - 2..], &[0xef, 0xbe]);
        let mut r = BitReader::new(&out);
        assert_eq!(r.get(3), Ok(0b101));
        assert_eq!(r.get(10), Ok(0x3ff));
        assert_eq!(r.get(64), Ok(u64::MAX));
        assert_eq!(r.get(1), Ok(1));
        assert_eq!(r.align(true), Ok(()));
        assert_eq!(r.u16(), Ok(0xbeef));
        assert_eq!(r.finish(true), Ok(()));
        assert_eq!(r.get(1), Err(DecodeError::Truncated));
    }

    #[test]
    fn a_set_pad_bit_and_a_trailing_byte_are_refused_only_when_strict() {
        let bytes = [0b1000_0001u8, 7];
        let mut r = BitReader::new(&bytes);
        assert_eq!(r.get(1), Ok(1));
        assert_eq!(r.align(true), Err(DecodeError::UnknownField));
        let mut r = BitReader::new(&bytes);
        assert_eq!(r.get(1), Ok(1));
        assert_eq!(r.finish(false), Ok(()));
        let mut r = BitReader::new(&bytes);
        assert_eq!(r.u8(), Ok(0b1000_0001));
        assert_eq!(r.finish(true), Err(DecodeError::TrailingBytes));
        assert_eq!(BitReader::new(&bytes).take(3), Err(DecodeError::Truncated));
        assert_eq!(
            BitReader::new(&bytes).array::<3>(),
            Err(DecodeError::Truncated)
        );
    }
}
