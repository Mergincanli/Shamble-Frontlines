//! A recorded match (ADR 0004 part 9; ADR 0002 part 8): what the host handed the simulation, call for call, so that
//! the same calls can be made again on any target through the players' own entry points, and the hashes each tick
//! must then give. Two files per recording, both committed under `recordings/`:
//!
//! - `<name>.rec`, the inputs, in format version 1 (all integers little-endian): `SHMBLREC`; the version as a
//!   `u16`; the 32-byte seed; the start tick (`u32`); the first command sequence (`u16`); the tick rate (`u16`);
//!   the 32-byte hash of the content the match ran on (all zeros: none); the map's id (a length byte, at most 64,
//!   then UTF-8); the number of steps (`u32`); then 68 bytes per step, mirroring the host's two calls per tick:
//!   the host's time as the bits of an `f64`, the post's number, the held buttons, the four words of press
//!   counts, the move, the cursor ray's origin and direction as the bits of three `f32` each, and the view time
//!   as a tick and a fraction. Nothing follows the last step.
//! - `<name>.hashes`, text: `shamble-hashes 1 <sha256 of the .rec> <steps>`, then per step the world's tick after
//!   it, the shared state's hash and the whole state's hash, each as 16 hex digits.
//!
//! The seed is a public test seed and nothing else: it starts with `TESTSEED`, the writer refuses any other and so
//! does the reader, so a real match's seed can never sit in a recording or be replayed from one (ADR 0004 part 8).
//! The reader takes any bytes: it checks every count against the bytes it was given before it takes memory for
//! them, and it never panics.

#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::arithmetic_side_effects
    )
)]

use std::fmt;

use sim::{SEED_BYTES, TICK_HZ};

use crate::sha256::{hex, sha256};

pub const MAGIC: [u8; 8] = *b"SHMBLREC";
pub const VERSION: u16 = 1;
/// What a recording's seed starts with.
pub const TEST_SEED_MARK: [u8; 8] = *b"TESTSEED";
/// One hour at 30 ticks a second.
pub const MAX_STEPS: u32 = 108_000;
pub const MAX_MAP_ID_BYTES: usize = 64;
pub const STEP_BYTES: usize = 68;
/// The first line of a hashes file starts with this.
pub const HASHES_MARK: &str = "shamble-hashes 1";

/// What a recording says about its match. Its `Debug` never shows the seed (ADR 0004 part 8): the type cannot tell
/// a test seed from a match's own.
#[derive(Clone, PartialEq, Eq)]
pub struct Header {
    pub seed: [u8; SEED_BYTES],
    pub start_tick: u32,
    pub first_sequence: u16,
    pub tick_rate: u16,
    pub content_hash: [u8; 32],
    pub map_id: String,
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut shown = f.debug_struct("Header");
        shown.field("start_tick", &self.start_tick);
        shown.field("first_sequence", &self.first_sequence);
        shown.field("tick_rate", &self.tick_rate);
        shown.field("content_hash", &self.content_hash);
        shown.field("map_id", &self.map_id);
        shown.finish_non_exhaustive()
    }
}

/// One step: the numbers of the host's `input` call and the time its `step` call got.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Step {
    /// The host's milliseconds: finite, and never before the step before.
    pub host_ms: f64,
    pub post: u32,
    /// The held buttons, in the low 16 bits; the rest is zero.
    pub held: u32,
    /// Button `k`'s running press count in byte `k % 4` of word `k / 4`.
    pub presses: [u32; 4],
    /// The move: x in bits 0 to 7 and y in bits 8 to 15, each an `i8`; the rest is zero.
    pub movement: u32,
    /// The cursor ray, as the bits of its `f32` numbers. The input call that takes it turns the bits back into `f32`
    /// before anything reads them: the format holds no wider number.
    pub ray_origin: [u32; 3],
    pub ray_direction: [u32; 3],
    /// The view time: a tick, and a fraction of a tick in 1/65,536 in the low 16 bits; the rest is zero.
    pub view_tick: u32,
    pub view_fraction: u32,
}

impl Step {
    /// A step at `host_ms` with nothing held, pressed, moved or aimed.
    #[must_use]
    pub const fn at(host_ms: f64) -> Self {
        Self {
            host_ms,
            post: 0,
            held: 0,
            presses: [0; 4],
            movement: 0,
            ray_origin: [0; 3],
            ray_direction: [0; 3],
            view_tick: 0,
            view_fraction: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Recording {
    pub header: Header,
    pub steps: Vec<Step>,
}

/// Why bytes are no recording, or why a recording is not written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordingError {
    /// The bytes end inside the header.
    HeaderCut,
    Magic,
    Version(u16),
    /// The seed does not start with `TESTSEED`.
    NotATestSeed,
    TickRate(u16),
    MapIdLength(usize),
    MapIdText,
    Steps(u64),
    /// After the header, this many bytes should follow, and that many do.
    Length {
        expected: u64,
        found: u64,
    },
    /// The step with this index (from 0) has a host time that is not finite or lies before the step before.
    HostTime(u32),
    /// The step with this index has a bit set that the format keeps at zero, in this field.
    SpareBits(u32, &'static str),
}

impl fmt::Display for RecordingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HeaderCut => write!(f, "the bytes end inside the recording's header"),
            Self::Magic => write!(
                f,
                "the bytes do not start with SHMBLREC: this is no recording"
            ),
            Self::Version(found) => write!(
                f,
                "the recording has format version {found}; this build reads version {VERSION} only"
            ),
            Self::NotATestSeed => write!(
                f,
                "the seed does not start with TESTSEED: a recording holds a public test seed and never a match's own"
            ),
            Self::TickRate(found) => write!(
                f,
                "the recording was made at {found} ticks a second; this build runs at {TICK_HZ}"
            ),
            Self::MapIdLength(found) => write!(
                f,
                "the map's id is {found} bytes long; the most is {MAX_MAP_ID_BYTES}"
            ),
            Self::MapIdText => write!(f, "the map's id is not UTF-8 text"),
            Self::Steps(found) => {
                write!(
                    f,
                    "the recording has {found} steps; the most is {MAX_STEPS}"
                )
            }
            Self::Length { expected, found } => write!(
                f,
                "after the header {expected} bytes of steps should follow, and {found} do"
            ),
            Self::HostTime(step) => write!(
                f,
                "step {step}: the host's time is not a finite number or lies before the step before"
            ),
            Self::SpareBits(step, field) => write!(
                f,
                "step {step}: {field} has a bit set that the format keeps at zero"
            ),
        }
    }
}

/// Reads from the front of some bytes; a read past the end takes nothing.
struct Front<'a>(&'a [u8]);

impl Front<'_> {
    fn take<const N: usize>(&mut self) -> Option<[u8; N]> {
        let (taken, rest) = self.0.split_first_chunk::<N>()?;
        self.0 = rest;
        Some(*taken)
    }

    fn u16(&mut self) -> Option<u16> {
        self.take().map(u16::from_le_bytes)
    }

    fn u32(&mut self) -> Option<u32> {
        self.take().map(u32::from_le_bytes)
    }

    fn words<const N: usize>(&mut self) -> Option<[u32; N]> {
        let mut words = [0; N];
        for word in &mut words {
            *word = self.u32()?;
        }
        Some(words)
    }
}

impl Recording {
    /// The checks the writer and the reader share: everything that is not the bytes' own shape.
    fn check(&self) -> Result<(), RecordingError> {
        let header = &self.header;
        if !header.seed.starts_with(&TEST_SEED_MARK) {
            return Err(RecordingError::NotATestSeed);
        }
        if u32::from(header.tick_rate) != TICK_HZ {
            return Err(RecordingError::TickRate(header.tick_rate));
        }
        if header.map_id.len() > MAX_MAP_ID_BYTES {
            return Err(RecordingError::MapIdLength(header.map_id.len()));
        }
        if self.steps.len() > MAX_STEPS as usize {
            return Err(RecordingError::Steps(self.steps.len() as u64));
        }
        let mut before = f64::NEG_INFINITY;
        for (index, step) in (0u32..).zip(&self.steps) {
            if !step.host_ms.is_finite() || step.host_ms < before {
                return Err(RecordingError::HostTime(index));
            }
            before = step.host_ms;
            let spare = [
                ("the held buttons", step.held),
                ("the move", step.movement),
                ("the view time's fraction", step.view_fraction),
            ];
            if let Some((field, _)) = spare.into_iter().find(|(_, word)| *word > 0xFFFF) {
                return Err(RecordingError::SpareBits(index, field));
            }
        }
        Ok(())
    }

    /// The recording's bytes.
    ///
    /// # Errors
    /// What the format does not hold: a seed that is no test seed, another tick rate, a map id or a step count past
    /// its cap, a host time that is not finite or runs backwards, a spare bit set.
    pub fn encode(&self) -> Result<Vec<u8>, RecordingError> {
        self.check()?;
        let Header {
            seed,
            start_tick,
            first_sequence,
            tick_rate,
            content_hash,
            map_id,
        } = &self.header;
        let mut bytes = Vec::with_capacity(self.steps.len().saturating_mul(STEP_BYTES));
        bytes.extend_from_slice(&MAGIC);
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        bytes.extend_from_slice(seed);
        bytes.extend_from_slice(&start_tick.to_le_bytes());
        bytes.extend_from_slice(&first_sequence.to_le_bytes());
        bytes.extend_from_slice(&tick_rate.to_le_bytes());
        bytes.extend_from_slice(content_hash);
        // Both lengths were checked against their caps above.
        bytes.push(map_id.len() as u8);
        bytes.extend_from_slice(map_id.as_bytes());
        bytes.extend_from_slice(&(self.steps.len() as u32).to_le_bytes());
        for step in &self.steps {
            bytes.extend_from_slice(&step.host_ms.to_bits().to_le_bytes());
            let words = [&[step.post, step.held][..], &step.presses, &[step.movement]];
            let ray = [&step.ray_origin[..], &step.ray_direction];
            let view = [step.view_tick, step.view_fraction];
            for word in words.into_iter().chain(ray).flatten().chain(&view) {
                bytes.extend_from_slice(&word.to_le_bytes());
            }
        }
        Ok(bytes)
    }

    /// Reads a recording from any bytes.
    ///
    /// # Errors
    /// Whatever makes the bytes no recording of this version, and what `encode` refuses.
    pub fn decode(bytes: &[u8]) -> Result<Self, RecordingError> {
        let cut = RecordingError::HeaderCut;
        let mut front = Front(bytes);
        if front.take::<8>().ok_or(cut.clone())? != MAGIC {
            return Err(RecordingError::Magic);
        }
        let version = front.u16().ok_or(cut.clone())?;
        if version != VERSION {
            return Err(RecordingError::Version(version));
        }
        let seed = front.take().ok_or(cut.clone())?;
        let start_tick = front.u32().ok_or(cut.clone())?;
        let first_sequence = front.u16().ok_or(cut.clone())?;
        let tick_rate = front.u16().ok_or(cut.clone())?;
        let content_hash = front.take().ok_or(cut.clone())?;
        let [id_length] = front.take().ok_or(cut.clone())?;
        if usize::from(id_length) > MAX_MAP_ID_BYTES {
            return Err(RecordingError::MapIdLength(usize::from(id_length)));
        }
        let (id, rest) = front
            .0
            .split_at_checked(usize::from(id_length))
            .ok_or(cut.clone())?;
        front.0 = rest;
        let map_id = String::from_utf8(id.to_vec()).map_err(|_| RecordingError::MapIdText)?;
        let steps = front.u32().ok_or(cut)?;
        // The count is judged against its cap and against the bytes that are there before any memory is taken.
        if steps > MAX_STEPS {
            return Err(RecordingError::Steps(u64::from(steps)));
        }
        let expected = u64::from(steps).saturating_mul(STEP_BYTES as u64);
        let found = front.0.len() as u64;
        if expected != found {
            return Err(RecordingError::Length { expected, found });
        }
        let mut read = Vec::with_capacity(steps as usize);
        while let Some(time) = front.take::<8>() {
            // Each `?` below cannot fail: the length was checked to be whole steps. It fails closed if it ever does.
            let short = RecordingError::Length { expected, found };
            let [post, held] = front.words().ok_or(short.clone())?;
            let presses = front.words().ok_or(short.clone())?;
            let [movement] = front.words().ok_or(short.clone())?;
            let ray_origin = front.words().ok_or(short.clone())?;
            let ray_direction = front.words().ok_or(short.clone())?;
            let [view_tick, view_fraction] = front.words().ok_or(short)?;
            read.push(Step {
                host_ms: f64::from_bits(u64::from_le_bytes(time)),
                post,
                held,
                presses,
                movement,
                ray_origin,
                ray_direction,
                view_tick,
                view_fraction,
            });
        }
        let recording = Self {
            header: Header {
                seed,
                start_tick,
                first_sequence,
                tick_rate,
                content_hash,
                map_id,
            },
            steps: read,
        };
        recording.check()?;
        Ok(recording)
    }
}

/// One line of a hashes file: the world's tick after a step, and the two hashes of its state then.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickHashes {
    pub tick: u32,
    /// The shared state's hash: the one every target can give.
    pub shared: u64,
    /// The whole state's hash, the seed included: given natively only.
    pub whole: u64,
}

/// The text of the hashes file for the recording with these bytes.
#[must_use]
pub fn hashes_text(recording: &[u8], lines: &[TickHashes]) -> String {
    use fmt::Write;
    let mut text = format!(
        "{HASHES_MARK} {} {}\n",
        hex(&sha256(recording)),
        lines.len()
    );
    for line in lines {
        // Writing into a `String` cannot fail.
        let _ = writeln!(
            text,
            "{} {:016x} {:016x}",
            line.tick, line.shared, line.whole
        );
    }
    text
}

/// Reads a hashes file that belongs to the recording with these bytes.
///
/// # Errors
/// In plain words: a first line that is not this format's or names another recording, a line that is not a tick and
/// two hashes of 16 hex digits, or another number of lines than the first line says.
pub fn parse_hashes(recording: &[u8], text: &str) -> Result<Vec<TickHashes>, String> {
    let mut lines = text.split_inclusive('\n');
    let first = lines.next().unwrap_or_default();
    let expected_start = format!("{HASHES_MARK} {} ", hex(&sha256(recording)));
    let count = first
        .strip_prefix(&expected_start)
        .and_then(|rest| rest.strip_suffix('\n'))
        .and_then(|count| count.parse::<usize>().ok().filter(|_| digits(count)))
        .ok_or_else(|| {
            format!(
                "the first line is not \"{expected_start}<steps>\": the file is not this recording's hashes"
            )
        })?;
    let hash = |text: &str| {
        let lower_hex = text.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));
        (text.len() == 16 && lower_hex).then(|| u64::from_str_radix(text, 16).ok())?
    };
    let mut read = Vec::new();
    for (number, line) in (2usize..).zip(lines) {
        let parts = line.strip_suffix('\n').map(|line| line.split(' '));
        let fields = parts.map(Iterator::collect::<Vec<_>>);
        let parsed = match fields.as_deref() {
            Some([tick, shared, whole]) if digits(tick) => {
                tick.parse().ok().zip(hash(shared)).zip(hash(whole)).map(
                    |((tick, shared), whole)| TickHashes {
                        tick,
                        shared,
                        whole,
                    },
                )
            }
            _ => None,
        };
        read.push(parsed.ok_or_else(|| {
            format!("line {number} is not a tick and two hashes of 16 hex digits")
        })?);
    }
    if read.len() != count {
        return Err(format!(
            "the first line says {count} steps and {} lines follow",
            read.len()
        ));
    }
    Ok(read)
}

/// Decimal digits only, and at least one: no sign, no space.
fn digits(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit())
}

/// One line of the recordings' manifest: a file's name, its length and its SHA-256.
#[must_use]
pub fn manifest_line(name: &str, bytes: &[u8]) -> String {
    format!(
        "{name}: bytes {}  sha256 {}\n",
        bytes.len(),
        hex(&sha256(bytes))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed() -> [u8; SEED_BYTES] {
        let mut seed = [0x11; SEED_BYTES];
        seed[..8].copy_from_slice(&TEST_SEED_MARK);
        seed
    }

    /// A recording with every field set to its own value, floats that are no plain numbers among them.
    fn full() -> Recording {
        let step = |index: u32| Step {
            host_ms: 1000.0 + f64::from(index) * 33.25,
            post: 0xA000_0000 + index,
            held: 0x8001,
            presses: [0x0403_0201 + index, 0x0807_0605, 0x0C0B_0A09, 0xFF0F_0E0D],
            movement: 0x81_7F,
            ray_origin: [1.5f32.to_bits(), (-0.0f32).to_bits(), f32::NAN.to_bits()],
            ray_direction: [f32::INFINITY.to_bits(), 0x0000_0001, f32::MIN.to_bits()],
            view_tick: u32::MAX - index,
            view_fraction: 0xFFFF,
        };
        Recording {
            header: Header {
                seed: seed(),
                start_tick: 131_069,
                first_sequence: 65_534,
                tick_rate: 30,
                content_hash: [0xC4; 32],
                map_id: "test-scene/yard-é".to_owned(),
            },
            steps: (0..5).map(step).collect(),
        }
    }

    #[test]
    fn a_recording_reads_back_as_it_was_written_byte_for_byte() {
        let recording = full();
        let bytes = recording.encode().expect("it has a file form");
        // The header's fixed parts, the map's id, the count, then 68 bytes a step, and nothing more.
        let id = recording.header.map_id.len();
        assert_eq!(
            bytes.len(),
            8 + 2 + 32 + 4 + 2 + 2 + 32 + 1 + id + 4 + 5 * STEP_BYTES
        );
        assert_eq!(bytes[..10], *b"SHMBLREC\x01\x00");
        let read = Recording::decode(&bytes).expect("what was written reads back");
        // Compared through the bytes, so that a NaN's own bits count.
        assert_eq!(read.encode().as_deref(), Ok(&bytes[..]));
        assert_eq!(read.header, recording.header);
        assert_eq!(read.steps[4].post, 0xA000_0004);
        assert_eq!(read.steps[1].ray_origin[2], f32::NAN.to_bits());
        // The emptiest one: no map, no step.
        let empty = Recording {
            header: Header {
                map_id: String::new(),
                ..recording.header
            },
            steps: Vec::new(),
        };
        let bytes = empty.encode().expect("it has a file form");
        assert_eq!(bytes.len(), 87);
        assert_eq!(Recording::decode(&bytes), Ok(empty));
    }

    /// Each thing the format does not hold is refused by name, by the writer and by the reader alike.
    #[test]
    fn what_the_format_does_not_hold_is_refused_by_name() {
        let good = full();
        let refused = |change: &dyn Fn(&mut Recording)| {
            let mut recording = good.clone();
            change(&mut recording);
            recording.encode().expect_err("the writer refuses it")
        };
        // The writer refuses a real seed, and so does the reader when the bytes hold one.
        assert_eq!(
            refused(&|r| r.header.seed = [7; SEED_BYTES]),
            RecordingError::NotATestSeed
        );
        let mut real = good.encode().expect("it has a file form");
        real[10..18].copy_from_slice(b"REALSEED");
        assert_eq!(Recording::decode(&real), Err(RecordingError::NotATestSeed));
        assert_eq!(
            refused(&|r| r.header.tick_rate = 60),
            RecordingError::TickRate(60)
        );
        assert_eq!(
            refused(&|r| r.header.map_id = "m".repeat(65)),
            RecordingError::MapIdLength(65)
        );
        assert_eq!(
            refused(&|r| r.steps = vec![Step::at(0.0); MAX_STEPS as usize + 1]),
            RecordingError::Steps(108_001)
        );
        assert_eq!(
            refused(&|r| r.steps[3].host_ms = f64::NAN),
            RecordingError::HostTime(3)
        );
        assert_eq!(
            refused(&|r| r.steps[0].host_ms = f64::INFINITY),
            RecordingError::HostTime(0)
        );
        assert_eq!(
            refused(&|r| r.steps[2].host_ms = 1000.0),
            RecordingError::HostTime(2)
        );
        assert_eq!(
            refused(&|r| r.steps[1].held = 0x1_0000),
            RecordingError::SpareBits(1, "the held buttons")
        );
        assert_eq!(
            refused(&|r| r.steps[4].movement = 0x1_0000),
            RecordingError::SpareBits(4, "the move")
        );
        assert_eq!(
            refused(&|r| r.steps[0].view_fraction = 0x1_0000),
            RecordingError::SpareBits(0, "the view time's fraction")
        );
        // The same host time twice is no step backwards.
        let mut same = good.clone();
        same.steps[2].host_ms = same.steps[1].host_ms;
        assert!(same.encode().is_ok());
        // A recording's own print names its fields and never its seed (84, 69, 83, 84 is "TEST").
        let shown = format!("{good:?}");
        assert!(
            shown.contains("map_id")
                && !shown.contains("seed")
                && !shown.contains("84, 69, 83, 84")
        );
        // Every reason reads as a plain sentence, and none prints a seed byte.
        for error in [
            RecordingError::HeaderCut,
            RecordingError::Magic,
            RecordingError::Version(2),
            RecordingError::NotATestSeed,
            RecordingError::MapIdText,
            RecordingError::Length {
                expected: 68,
                found: 67,
            },
        ] {
            let text = error.to_string();
            assert!(text.len() > 20 && !text.contains("0x11") && !text.contains("17, 17"));
        }
    }

    #[test]
    fn bytes_of_the_wrong_shape_are_refused_by_name() {
        let good = full().encode().expect("it has a file form");
        let decode = Recording::decode;
        assert_eq!(decode(b""), Err(RecordingError::HeaderCut));
        assert_eq!(decode(b"SHMBLRE"), Err(RecordingError::HeaderCut));
        assert_eq!(decode(b"SHMBLREX\x01\x00"), Err(RecordingError::Magic));
        assert_eq!(decode(b"SHMBLREC\x02\x00"), Err(RecordingError::Version(2)));
        assert_eq!(decode(b"SHMBLREC\x00\x00"), Err(RecordingError::Version(0)));
        // Cut anywhere inside the header.
        let header = good.len() - 5 * STEP_BYTES;
        for cut in 10..header {
            assert_eq!(
                decode(&good[..cut]),
                Err(RecordingError::HeaderCut),
                "cut at {cut}"
            );
        }
        // One byte short, one byte more, one step short, one step more: each is a wrong length, with both numbers.
        let length = |found: usize| {
            Err(RecordingError::Length {
                expected: 340,
                found: found as u64,
            })
        };
        assert_eq!(decode(&good[..good.len() - 1]), length(339));
        assert_eq!(decode(&[&good[..], &[0]].concat()), length(341));
        assert_eq!(decode(&good[..good.len() - STEP_BYTES]), length(272));
        assert_eq!(decode(&good[..header]), length(0));
        // A count far past the bytes that are there is refused by the count's cap or by the length, never trusted.
        let count_at = header - 4;
        let mut huge = good.clone();
        huge[count_at..header].copy_from_slice(&u32::MAX.to_le_bytes());
        assert_eq!(
            decode(&huge),
            Err(RecordingError::Steps(u64::from(u32::MAX)))
        );
        huge[count_at..header].copy_from_slice(&MAX_STEPS.to_le_bytes());
        assert_eq!(
            decode(&huge),
            Err(RecordingError::Length {
                expected: 108_000 * 68,
                found: 340
            })
        );
        // A map id that says it is longer than the cap, and one that is no text.
        let id_at = 8 + 2 + 32 + 4 + 2 + 2 + 32;
        let mut long_id = good.clone();
        long_id[id_at] = 65;
        assert_eq!(decode(&long_id), Err(RecordingError::MapIdLength(65)));
        let mut no_text = good.clone();
        no_text[id_at + 1] = 0xFF;
        assert_eq!(decode(&no_text), Err(RecordingError::MapIdText));
    }

    /// A small generator for the test below: xorshift64.
    fn next(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    /// 100,000 inputs, random and made from a good recording by changing bytes and lengths: the reader never
    /// panics, and whatever it takes writes back to exactly the bytes it was given.
    #[test]
    fn the_reader_takes_any_bytes_without_a_panic_and_only_what_writes_back_the_same() {
        let good = full().encode().expect("it has a file form");
        let mut state = 0x9E37_79B9_7F4A_7C15u64;
        let (mut taken, mut refused) = (0u32, 0u32);
        for round in 0..100_000u32 {
            let mut bytes = good.clone();
            match round % 4 {
                // One to three bytes changed anywhere.
                0 | 1 => {
                    for _ in 0..=next(&mut state) % 3 {
                        let at = next(&mut state) as usize % bytes.len();
                        bytes[at] = next(&mut state) as u8;
                    }
                }
                // Cut short or made longer.
                2 => {
                    let length = next(&mut state) as usize % (bytes.len() + 80);
                    bytes.resize(length, next(&mut state) as u8);
                }
                // Nothing of the good one but its first bytes.
                _ => {
                    let keep = next(&mut state) as usize % 20;
                    for byte in &mut bytes[keep..] {
                        *byte = next(&mut state) as u8;
                    }
                }
            }
            match Recording::decode(&bytes) {
                Ok(recording) => {
                    assert_eq!(
                        recording.encode().as_deref(),
                        Ok(&bytes[..]),
                        "round {round}"
                    );
                    taken += 1;
                }
                Err(_) => refused += 1,
            }
        }
        // Both happen: a changed press count is still a recording, a changed length is not.
        assert!(
            taken > 10_000 && refused > 10_000,
            "{taken} taken, {refused} refused"
        );
    }

    #[test]
    fn a_hashes_file_reads_back_and_belongs_to_one_recording() {
        let recording = full().encode().expect("it has a file form");
        let lines = [
            TickHashes {
                tick: 1,
                shared: 0x0123_4567_89ab_cdef,
                whole: 0,
            },
            TickHashes {
                tick: u32::MAX,
                shared: u64::MAX,
                whole: 0x00ff_0000_0000_0001,
            },
        ];
        let text = hashes_text(&recording, &lines);
        let digest = hex(&sha256(&recording));
        assert_eq!(
            text,
            format!(
                "shamble-hashes 1 {digest} 2\n1 0123456789abcdef 0000000000000000\n4294967295 ffffffffffffffff 00ff000000000001\n"
            )
        );
        assert_eq!(parse_hashes(&recording, &text).as_deref(), Ok(&lines[..]));
        assert_eq!(
            parse_hashes(&recording, &hashes_text(&recording, &[])),
            Ok(Vec::new())
        );
        // Another recording's hashes are not this one's.
        let mut other = recording.clone();
        *other.last_mut().expect("it has bytes") ^= 1;
        let wrong = parse_hashes(&other, &text).expect_err("another recording");
        assert!(wrong.contains("not this recording's hashes"), "{wrong}");
        // Each malformed text is refused with its line.
        let refused = |text: &str| parse_hashes(&recording, text).expect_err("refused");
        let head = format!("shamble-hashes 1 {digest}");
        assert!(refused("").contains("first line"));
        assert!(refused(&format!("{head} 2")).contains("first line"));
        assert!(refused(&format!("{head} +2\n")).contains("first line"));
        assert!(refused(&format!("shamble-hashes 2 {digest} 0\n")).contains("first line"));
        assert!(refused(&format!("{head} 1\n")).contains("says 1 steps and 0 lines"));
        assert!(
            refused(&format!("{head} 0\n1 0123456789abcdef 0000000000000000\n")).contains("says 0")
        );
        for line in [
            "1 0123456789abcdef 0000000000000000",
            "1 0123456789abcde 0000000000000000\n",
            "1 0123456789ABCDEF 0000000000000000\n",
            "1 0123456789abcdef 0000000000000000 \n",
            "1  0123456789abcdef 0000000000000000\n",
            "-1 0123456789abcdef 0000000000000000\n",
            "4294967296 0123456789abcdef 0000000000000000\n",
            "1 0123456789abcdef +000000000000000\n",
            "1 0123456789abcdef\n",
            "\n",
        ] {
            let why = refused(&format!("{head} 1\n{line}"));
            assert!(
                why.contains("line 2 is not a tick and two hashes"),
                "{line:?}: {why}"
            );
        }
    }

    #[test]
    fn a_manifest_line_names_the_file_its_length_and_its_digest() {
        assert_eq!(
            manifest_line("first-loop.rec", b"abc"),
            "first-loop.rec: bytes 3  sha256 ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad\n"
        );
    }
}
