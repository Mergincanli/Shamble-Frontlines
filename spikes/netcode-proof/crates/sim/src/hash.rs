//! The canonical serialization, its hash and its dump (plan §4.5: "computed by canonical serialization: field by field,
//! fixed-width little-endian, no `usize`, floats by `to_bits`, a fixed unseeded hash, never std's `Hasher`"; the hash
//! function is spike-local: the sim core spike picks the real one, ADR 0004:350-351).
//!
//! `World` writes its state as 32-bit words to a [`Sink`], each word named by its section, its index there and its
//! field. Three sinks read the one walk: the hash (FNV-1a-64 over the words' little-endian bytes), the dump (the words
//! and their names) and the finite check (D-4: every float, read by `to_bits` as the hash reads it). So the dump holds
//! exactly what the hash hashes (carry 38), and nothing else: never [`crate::InputReports`], never the commands' claimed
//! press counters.

/// Where the canonical walk writes: one call per 32-bit word, in order.
pub(crate) trait Sink {
    /// One word: `section` (`"soldier"`), the entity's `index` in it, the `field` (`"pos.x"`).
    fn word(&mut self, section: &'static str, index: u32, field: &'static str, w: u32);

    /// A float, by its bits.
    fn float(&mut self, section: &'static str, index: u32, field: &'static str, x: f32) {
        self.word(section, index, field, x.to_bits());
    }

    /// A 64-bit value, low word first.
    fn wide(&mut self, section: &'static str, index: u32, field: &'static str, v: u64) {
        self.word(section, index, field, v as u32);
        self.word(section, index, field, (v >> 32) as u32);
    }
}

/// FNV-1a-64's offset basis and prime.
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// FNV-1a-64 over the words' little-endian bytes.
pub(crate) struct Fnv(pub(crate) u64);

impl Fnv {
    pub(crate) fn new() -> Fnv {
        Fnv(FNV_OFFSET)
    }

    pub(crate) fn bytes(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.0 = (self.0 ^ u64::from(*b)).wrapping_mul(FNV_PRIME);
        }
    }
}

impl Sink for Fnv {
    fn word(&mut self, _: &'static str, _: u32, _: &'static str, w: u32) {
        self.bytes(&w.to_le_bytes());
    }
}

/// One word's name: section, index, field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Name {
    section: &'static str,
    index: u32,
    field: &'static str,
}

/// A world's state as the hash reads it (plan §4.5's "field diff"; u4-build-plan.md §4.15): the canonical words, each
/// named. Its words are exactly what `state_hash` hashes, so [`StateDump::hash`] equals the world's state hash, and a
/// diff between two dumps names the first field that differs. Another process's or target's dump arrives as words
/// alone (npt-hash-wasm's `dump_word`), and this dump names them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateDump {
    words: Vec<u32>,
    names: Vec<Name>,
}

/// The first word two dumps differ at (u4-build-plan.md §5 U4d, D-2: "a mismatch names the first tick and the field").
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldDiff {
    /// The word's index in the dump.
    pub word: usize,
    /// Its field, as `section[index].field` (`soldier[3].pos.x`), or `end` where one dump is longer.
    pub field: String,
    /// This dump's word (0 past its end).
    pub ours: u32,
    /// The other's (0 past its end).
    pub theirs: u32,
}

impl StateDump {
    pub(crate) fn new() -> StateDump {
        StateDump {
            words: Vec::new(),
            names: Vec::new(),
        }
    }

    /// The words, in canonical order: what npt-hash-wasm's `dump_word` exports one by one.
    pub fn words(&self) -> &[u32] {
        &self.words
    }

    /// The hash of the words: the world's state hash at that tick.
    pub fn hash(&self) -> u64 {
        let mut h = Fnv::new();
        for w in &self.words {
            h.bytes(&w.to_le_bytes());
        }
        h.0
    }

    /// Word `i`'s field, as `section[index].field`, or `None` past the end.
    pub fn field(&self, i: usize) -> Option<String> {
        self.names
            .get(i)
            .map(|n| format!("{}[{}].{}", n.section, n.index, n.field))
    }

    /// The first word where this dump and `theirs` (another dump's words) differ, named by this dump; `None` when
    /// they are equal word for word.
    pub fn first_difference(&self, theirs: &[u32]) -> Option<FieldDiff> {
        let n = self.words.len().max(theirs.len());
        (0..n).find_map(|i| {
            let (a, b) = (self.words.get(i), theirs.get(i));
            (a != b).then(|| FieldDiff {
                word: i,
                field: self.field(i).unwrap_or_else(|| "end".to_string()),
                ours: a.copied().unwrap_or(0),
                theirs: b.copied().unwrap_or(0),
            })
        })
    }
}

impl Sink for StateDump {
    fn word(&mut self, section: &'static str, index: u32, field: &'static str, w: u32) {
        self.words.push(w);
        self.names.push(Name {
            section,
            index,
            field,
        });
    }
}

/// The finite check (D-4): the first float that is NaN or infinite, by name.
pub(crate) struct Finite(pub(crate) Option<String>);

impl Sink for Finite {
    fn word(&mut self, _: &'static str, _: u32, _: &'static str, _: u32) {}

    fn float(&mut self, section: &'static str, index: u32, field: &'static str, x: f32) {
        // By its bits, as the hash reads it: every exponent bit set is an infinity or a NaN.
        if self.0.is_none() && x.to_bits() & 0x7f80_0000 == 0x7f80_0000 {
            self.0 = Some(format!("{section}[{index}].{field}"));
        }
    }
}
