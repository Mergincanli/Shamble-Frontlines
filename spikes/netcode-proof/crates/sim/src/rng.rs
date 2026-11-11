//! The sim's random streams (plan §4.5: `ChaCha8Rng`, pinned; the seed passed in by the host; our own sampling
//! functions; the authority-only spread stream kept apart).

use rand_chacha::ChaCha8Rng;
use rand_core::{Rng, SeedableRng};

/// One of the sim's random streams: ChaCha8 keyed by the host's seed, one ChaCha stream number per purpose, so a draw
/// on one never moves another (a soldier's spread never shifts the effect rows' rolls). Every stream's position is
/// state and in the state hash; the client-held hash leaves the authority-only ones out (plan §4.5).
///
/// The sampling functions are the sim's own (integers only, the same on every target), never `rand`'s distributions.
/// The world's alone: nothing outside npt-sim draws from its streams, so it is not public.
#[derive(Clone, Debug)]
pub(crate) struct SimRng {
    rng: ChaCha8Rng,
}

/// The purposes, each its own ChaCha stream number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Stream {
    /// Authority-only: each shot's spread (SEC-SPREAD (a)); the client's prediction never draws from it.
    Spread = 1,
    /// The effect rows' kinds, durations and magnitudes (S1's stand-in).
    Effects = 2,
    /// The stand-in projectiles' starts and fuses.
    Projectiles = 3,
    /// Where a soldier respawns.
    Respawn = 4,
}

/// Every stream, in the state's order.
pub(crate) const STREAMS: [Stream; 4] = [
    Stream::Spread,
    Stream::Effects,
    Stream::Projectiles,
    Stream::Respawn,
];

impl SimRng {
    /// The stream `stream` of the run whose seed is `seed`: the 32-byte ChaCha key is the seed, little-endian, in its
    /// first 8 bytes and a fixed tag after them.
    pub(crate) fn new(seed: u64, stream: Stream) -> SimRng {
        let mut key = *b"npt-sim seed....stand-in sim key";
        key[..8].copy_from_slice(&seed.to_le_bytes());
        let mut rng = ChaCha8Rng::from_seed(key);
        rng.set_stream(stream as u64);
        SimRng { rng }
    }

    /// The next 32 random bits.
    pub(crate) fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    /// A draw in `0..n` (`n` above 0), by the multiply-shift of a 32-bit draw: integers only.
    pub(crate) fn below(&mut self, n: u32) -> u32 {
        debug_assert!(n > 0, "a draw below 0");
        ((u64::from(self.next_u32()) * u64::from(n)) >> 32) as u32
    }

    /// A draw in `lo..=hi` (`lo` at most `hi`).
    pub(crate) fn between(&mut self, lo: i32, hi: i32) -> i32 {
        let span = (i64::from(hi) - i64::from(lo) + 1) as u64;
        let k = (u64::from(self.next_u32()) * span) >> 32;
        (i64::from(lo) + k as i64) as i32
    }

    /// The stream's position in 32-bit words since its start: its whole state, given the key and the stream number.
    pub(crate) fn position(&self) -> u128 {
        self.rng.get_word_pos()
    }
}
