//! The block writer: the bytes the simulation hands the page each tick (ADR 0002 part 5). One buffer, allocated once
//! and never moved, so the Worker reads its place once; every value is written field by field at the place the
//! layout table gives, at a fixed width and little-endian, with no `unsafe`. So far the block is its header, the
//! server's echo of the player's input and the link's counts.

use crate::endpoint::Readout;
use crate::layout::{self, BLOCK_BYTES, ElementType, LAYOUT_HASH};

/// Where a `u32` field lies in the block: its section's start plus its place in the row. Taken by name, in
/// constants: a name the table does not hold, or a field of another type, stops the build here.
const fn at(section: &str, field: &str) -> usize {
    let (Some(start), Some(field)) = (
        layout::section_offset(section),
        layout::field(section, field),
    ) else {
        panic!("the block's layout lacks a section or a field its writer names");
    };
    assert!(matches!(field.ty, ElementType::U32));
    (start + field.offset) as usize
}

const TICK: usize = at("header", "tick");
const HASH: usize = at("header", "layout_hash");
const HELD: usize = at("echo", "held");
const PRESSES: [usize; 4] = [
    at("echo", "presses_0_3"),
    at("echo", "presses_4_7"),
    at("echo", "presses_8_11"),
    at("echo", "presses_12_15"),
];
const SENT: usize = at("link", "sent");
const RECEIVED: usize = at("link", "received");
const REFUSED: usize = at("link", "refused");
const POST_USED: usize = at("link", "post_used");

/// The most bytes of padding a writer takes: 1 MiB, sixteen times the largest hand-off reading (64 KiB).
pub const MAX_PAD_BYTES: u32 = 1 << 20;

/// Writes the block into its own buffer.
pub struct BlockWriter {
    bytes: Box<[u8]>,
    byte_len: u32,
}

impl BlockWriter {
    /// A writer whose buffer holds the block's sections and `pad_bytes` more bytes, all zero. The padding is for
    /// the hand-off readings, which need blocks of the sizes later sections will have; the game itself passes 0.
    /// `None` when `pad_bytes` is above `MAX_PAD_BYTES`: the number comes from outside, and the buffer is
    /// allocated here.
    #[must_use]
    pub fn new(pad_bytes: u32) -> Option<Self> {
        if pad_bytes > MAX_PAD_BYTES {
            return None;
        }
        let byte_len = BLOCK_BYTES + pad_bytes;
        Some(Self {
            bytes: vec![0; byte_len as usize].into_boxed_slice(),
            byte_len,
        })
    }

    /// Writes the block for this tick from what the endpoint knows: the tick, then the layout's hash, which the
    /// page checks before it trusts any other offset, the server's echo, and the link's counts with `refused` as
    /// the count of refused messages (the caller adds what the other side refused). The padding is never written.
    pub fn write(&mut self, readout: &Readout, refused: u32) {
        let mut put = |at: usize, value: u32| {
            self.bytes[at..at + 4].copy_from_slice(&value.to_le_bytes());
        };
        put(TICK, readout.tick);
        put(HASH, LAYOUT_HASH);
        put(HELD, u32::from(readout.echo_buttons));
        for (at, four) in PRESSES.into_iter().zip(readout.echo_presses.as_chunks().0) {
            put(at, u32::from_le_bytes(*four));
        }
        put(SENT, readout.sent);
        put(RECEIVED, readout.received);
        put(REFUSED, refused);
        put(POST_USED, readout.post_used);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn word(writer: &BlockWriter, at: usize) -> u32 {
        let bytes = writer.bytes();
        u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
    }

    #[test]
    fn every_value_is_written_at_the_table_s_place() {
        let mut writer = BlockWriter::new(0).expect("no padding is allowed");
        assert_eq!(writer.byte_len(), 44);
        assert_eq!(writer.bytes(), [0; 44]);
        let mut echo_presses = [0; 16];
        for (counter, value) in echo_presses.iter_mut().zip(1..) {
            *counter = value;
        }
        let readout = Readout {
            tick: 0x0403_0201,
            echo_buttons: 0x8001,
            echo_presses,
            post_used: 77,
            sent: 5,
            received: 6,
            ..Readout::default()
        };
        writer.write(&readout, 9);
        assert_eq!(writer.bytes()[..4], [0x01, 0x02, 0x03, 0x04]);
        let words: Vec<u32> = (4..44).step_by(4).map(|at| word(&writer, at)).collect();
        // The hash; the held buttons and the counters four to a word, button 0 in the lowest byte; then sent,
        // received, refused and the post used.
        let presses = [0x0403_0201, 0x0807_0605, 0x0C0B_0A09, 0x100F_0E0D];
        let expected = [&[LAYOUT_HASH, 0x8001][..], &presses, &[5, 6, 9, 77]].concat();
        assert_eq!(words, expected);
        // The next tick replaces the last one.
        writer.write(&Readout::default(), 0);
        assert_eq!(word(&writer, 4), LAYOUT_HASH);
        assert!(writer.bytes()[8..].iter().all(|byte| *byte == 0));
    }

    #[test]
    fn the_padding_stays_zero_and_the_buffer_never_moves() {
        let mut writer = BlockWriter::new(1024).expect("1 KiB of padding is allowed");
        assert_eq!(writer.byte_len(), 44 + 1024);
        assert_eq!(writer.bytes().len(), 44 + 1024);
        let place = writer.bytes().as_ptr();
        let full = Readout {
            echo_buttons: u16::MAX,
            echo_presses: [0xFF; 16],
            post_used: u32::MAX,
            sent: u32::MAX,
            received: u32::MAX,
            refused: u32::MAX,
            ..Readout::default()
        };
        for tick in [0, 1, u32::MAX] {
            writer.write(&Readout { tick, ..full }, u32::MAX);
            assert_eq!(word(&writer, 0), tick);
            assert!(writer.bytes()[44..].iter().all(|byte| *byte == 0));
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
