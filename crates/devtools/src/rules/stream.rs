//! The random stream's vectors (PM-0004-27): the simulation's generator against the words a second implementation
//! wrote into `vectors/chacha8-words.bin` (`vectors/make_chacha8_vectors.py`; the manifest beside it names every
//! section). The layout here is that script's.

use sim::{ChaCha8, SEED_BYTES};

/// The key of every section: the published test key of TC8 (the draft named in `sim::rng`'s tests).
pub const KEY: [u8; SEED_BYTES] = [
    0xc4, 0x6e, 0xc1, 0xb1, 0x8c, 0xe8, 0xa8, 0x78, 0x72, 0x5a, 0x37, 0xe7, 0x80, 0xdf, 0xb7, 0x35,
    0x1f, 0x68, 0xed, 0x2e, 0x19, 0x4c, 0x79, 0xfb, 0xc6, 0xae, 0xbe, 0xe1, 0xa6, 0x67, 0x97, 0x5d,
];
/// The four long sections' stream ids, at the edges of both halves of the id.
pub const STREAMS: [u64; 4] = [0, 1, 1 << 32, u64::MAX];
/// Words in each long section, from the stream's start.
pub const WORDS: usize = 10_000;
/// The last section: this many words of stream 1 from block 2^32 - 3, across the carry of the block counter's low
/// word into its high word.
pub const CARRY_WORDS: usize = 96;
/// The first word of that section: block 2^32 - 3's first.
pub const CARRY_START: u64 = ((1 << 32) - 3) * 16;
/// Where a stream is restored in mid-run, and for how many words it must then go on as the file does.
pub const RESTORE_AT: usize = 1_001;
/// The words a restored stream is followed for.
pub const RESTORE_WORDS: usize = 1_000;
/// The file's length: four bytes a word.
pub const FILE_BYTES: usize = 4 * (STREAMS.len() * WORDS + CARRY_WORDS);

/// Checks the generator against the vector file. `Err` says which draw differs first.
///
/// # Errors
/// A file of the wrong length, or the first word the generator gives that is not the file's.
pub fn check(file: &[u8]) -> Result<(), String> {
    if file.len() != FILE_BYTES {
        return Err(format!(
            "the vector file has {} bytes, not {FILE_BYTES}",
            file.len()
        ));
    }
    let word = |at: usize| {
        u32::from_le_bytes([
            file[4 * at],
            file[4 * at + 1],
            file[4 * at + 2],
            file[4 * at + 3],
        ])
    };
    let same = |at: usize, got: u32, what: &str, index: usize| {
        if got == word(at) {
            Ok(())
        } else {
            Err(format!(
                "{what}, word {index}: the file holds {:#010x}, the generator gave {got:#010x}",
                word(at)
            ))
        }
    };
    for (section, &id) in STREAMS.iter().enumerate() {
        let mut stream = ChaCha8::new(&KEY, id);
        for index in 0..WORDS {
            same(
                section * WORDS + index,
                stream.next_u32(),
                &format!("stream {id}"),
                index,
            )?;
        }
        // A stream put at a position in mid-run goes on as one that was drawn that far.
        let mut restored = ChaCha8::new(&KEY, id);
        restored.set_position(RESTORE_AT as u64);
        for index in RESTORE_AT..RESTORE_AT + RESTORE_WORDS {
            same(
                section * WORDS + index,
                restored.next_u32(),
                &format!("stream {id} restored at word {RESTORE_AT}"),
                index,
            )?;
        }
        // 64 bits are two words in drawing order, the first one low.
        let mut pairs = ChaCha8::new(&KEY, id);
        for index in 0..WORDS / 2 {
            let (low, high) = (
                word(section * WORDS + 2 * index),
                word(section * WORDS + 2 * index + 1),
            );
            let (want, got) = (u64::from(low) | (u64::from(high) << 32), pairs.next_u64());
            if got != want {
                return Err(format!(
                    "stream {id} drawn as 64 bits, draw {index}: the file holds {want:#018x}, the generator gave {got:#018x}"
                ));
            }
        }
    }
    let mut carry = ChaCha8::new(&KEY, 1);
    carry.set_position(CARRY_START);
    for index in 0..CARRY_WORDS {
        same(
            STREAMS.len() * WORDS + index,
            carry.next_u32(),
            "stream 1 across the carry at block 2^32",
            index,
        )?;
    }
    Ok(())
}
