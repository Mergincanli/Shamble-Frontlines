//! The block writer: the bytes the simulation hands the page each tick (ADR 0002 part 5). One buffer, allocated once
//! and never moved, so the Worker reads its place once; every value is written field by field at the offset the
//! layout table gives, at a fixed width and little-endian, with no `unsafe`. So far the block is its header.

use crate::layout::{self, ElementType, Field, LAYOUT_HASH, Section};

// Taken by name, in constants: a name the table does not hold stops the build (`layout::section`, `layout::field`).
const HEADER: Section = layout::section("header");
const TICK: Field = layout::field("header", "tick");
const HASH: Field = layout::field("header", "layout_hash");

// `put_u32` writes four bytes: a field of another type at one of these names stops the build too.
const _: () = assert!(matches!(TICK.ty, ElementType::U32));
const _: () = assert!(matches!(HASH.ty, ElementType::U32));

/// The most bytes of padding a writer takes: 1 MiB, sixteen times the largest hand-off reading (64 KiB).
pub const MAX_PAD_BYTES: u32 = 1 << 20;

/// Writes the block into its own buffer.
pub struct BlockWriter {
    bytes: Box<[u8]>,
    byte_len: u32,
}

impl BlockWriter {
    /// A writer whose buffer holds the header and `pad_bytes` more bytes, all zero. The padding is for the hand-off
    /// readings, which need blocks of the sizes later sections will have; the game itself passes 0. `None` when
    /// `pad_bytes` is above `MAX_PAD_BYTES`: the number comes from outside, and the buffer is allocated here.
    #[must_use]
    pub fn new(pad_bytes: u32) -> Option<Self> {
        if pad_bytes > MAX_PAD_BYTES {
            return None;
        }
        let byte_len = HEADER.stride + pad_bytes;
        Some(Self {
            bytes: vec![0; byte_len as usize].into_boxed_slice(),
            byte_len,
        })
    }

    /// Writes the header for this tick: the tick, then the layout's hash, which the page checks before it trusts
    /// any other offset. The padding is never written.
    pub fn write_header(&mut self, tick: u32) {
        put_u32(&mut self.bytes, TICK, tick);
        put_u32(&mut self.bytes, HASH, LAYOUT_HASH);
    }

    /// The block's bytes. Their address does not change for the life of the writer.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The block's length in bytes, as a fixed-width number for the Worker.
    #[must_use]
    pub const fn byte_len(&self) -> u32 {
        self.byte_len
    }
}

/// Writes `value` at the field's offset, little-endian. The offset lies inside the header's row (the layout's rules),
/// and the buffer is never shorter than that row.
fn put_u32(bytes: &mut [u8], field: Field, value: u32) {
    let at = field.offset as usize;
    bytes[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_header_is_written_at_the_table_s_offsets() {
        let mut writer = BlockWriter::new(0).expect("no padding is allowed");
        assert_eq!(writer.byte_len(), 8);
        assert_eq!(writer.bytes(), [0; 8]);
        writer.write_header(0x0403_0201);
        assert_eq!(writer.bytes()[..4], [0x01, 0x02, 0x03, 0x04]);
        assert_eq!(writer.bytes()[4..8], LAYOUT_HASH.to_le_bytes());
        // The next tick replaces the last one.
        writer.write_header(7);
        assert_eq!(writer.bytes()[..4], [7, 0, 0, 0]);
        assert_eq!(writer.bytes()[4..8], LAYOUT_HASH.to_le_bytes());
    }

    #[test]
    fn the_padding_stays_zero_and_the_buffer_never_moves() {
        let mut writer = BlockWriter::new(1024).expect("1 KiB of padding is allowed");
        assert_eq!(writer.byte_len(), 8 + 1024);
        assert_eq!(writer.bytes().len(), 8 + 1024);
        let place = writer.bytes().as_ptr();
        for tick in [0, 1, u32::MAX] {
            writer.write_header(tick);
            assert_eq!(writer.bytes()[..4], tick.to_le_bytes());
            assert!(writer.bytes()[8..].iter().all(|byte| *byte == 0));
            assert_eq!(writer.bytes().as_ptr(), place);
        }
    }

    #[test]
    fn padding_above_the_cap_is_refused() {
        assert!(BlockWriter::new(MAX_PAD_BYTES).is_some());
        assert!(BlockWriter::new(MAX_PAD_BYTES + 1).is_none());
        assert!(BlockWriter::new(u32::MAX).is_none());
    }
}
