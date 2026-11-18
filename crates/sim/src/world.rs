//! The world: the whole state of one match, and the step that advances it (ADR 0004 parts 1, 3, 4, 8 and 9). So far
//! it holds the tick counter and the match seed. The step reads no clock and takes no time: one call is one tick.

use crate::state_hash::fnv1a_64;

/// Ticks per second: the one build-time constant, the same on the server and the client (ADR 0004 part 4; TICK-30).
pub const TICK_HZ: u32 = 30;

/// The length of a match seed in bytes.
pub const SEED_BYTES: usize = 32;

/// The length of the canonical state bytes: the tick (4), then the seed (32).
const STATE_BYTES: usize = 4 + SEED_BYTES;

/// The whole state of one match. There is no `Debug`: the seed is never printed or logged (ADR 0004 part 8).
pub struct World {
    tick: u32,
    seed: [u8; SEED_BYTES],
}

impl World {
    /// A new match at tick 0. The seed comes from whoever starts the match; the simulation draws none itself.
    #[must_use]
    pub const fn new(seed: [u8; SEED_BYTES]) -> Self {
        Self { tick: 0, seed }
    }

    /// The number of ticks run so far. It is the simulation's only clock: sim time is `tick / TICK_HZ` seconds.
    #[must_use]
    pub const fn tick(&self) -> u32 {
        self.tick
    }

    /// Runs one tick of the authority's phases. So far the only state that moves is the tick counter.
    ///
    /// # Panics
    /// When the counter is at `u32::MAX`, which is 4.5 years at 30 Hz: it stops loudly and never wraps (ADR 0004 part
    /// 4: counters use explicit operations).
    pub fn authority_step(&mut self) {
        self.tick = self
            .tick
            .checked_add(1)
            .expect("the tick counter is at u32::MAX and cannot advance");
    }

    /// Writes the canonical bytes of the whole state into `out`, replacing what it held (ADR 0004 part 9): field by
    /// field in a fixed order, every integer at a fixed width and little-endian, no `usize`. The order is the tick as
    /// a `u32`, then the 32 seed bytes. Each field that joins the state joins here, in the same change.
    pub fn write_state(&self, out: &mut Vec<u8>) {
        // Every field is named in this pattern: one added to the struct later does not compile until it is written
        // here, or left out here in plain sight.
        let Self { tick, seed } = self;
        out.clear();
        out.reserve(STATE_BYTES);
        out.extend_from_slice(&tick.to_le_bytes());
        out.extend_from_slice(seed);
    }

    /// The state hash: `fnv1a_64` over the canonical bytes. `scratch` holds those bytes afterwards; passing the same
    /// buffer every tick means no allocation after the first call.
    #[must_use]
    pub fn state_hash(&self, scratch: &mut Vec<u8>) -> u64 {
        self.write_state(scratch);
        fnv1a_64(scratch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The seed 0, 1, ..., 31.
    fn counting_seed(first: u8) -> [u8; SEED_BYTES] {
        let mut seed = [0; SEED_BYTES];
        for (byte, value) in seed.iter_mut().zip(first..) {
            *byte = value;
        }
        seed
    }

    #[test]
    fn a_new_world_is_at_tick_zero_and_each_step_is_one_tick() {
        let mut world = World::new(counting_seed(0));
        assert_eq!(world.tick(), 0);
        world.authority_step();
        world.authority_step();
        assert_eq!(world.tick(), 2);
    }

    #[test]
    fn state_bytes_have_a_fixed_order_and_width() {
        let mut world = World::new(counting_seed(0));
        for _ in 0..0x0102 {
            world.authority_step();
        }
        // What was in the buffer before does not survive.
        let mut bytes = vec![0xEE; 100];
        world.write_state(&mut bytes);
        assert_eq!(bytes.len(), STATE_BYTES);
        assert_eq!(bytes.len(), 36);
        // The tick 0x0000_0102, little-endian, then the seed as it was given.
        assert_eq!(bytes[..4], [0x02, 0x01, 0x00, 0x00]);
        assert_eq!(bytes[4..], counting_seed(0));
    }

    /// Known answers from a second implementation (a script, not this code): they hold the byte order and the hash
    /// still, on every target the tests run on.
    #[test]
    fn state_hash_matches_known_answers() {
        let mut scratch = Vec::new();
        let mut world = World::new(counting_seed(0));
        assert_eq!(world.state_hash(&mut scratch), 0x56a0_1268_e3dd_0d95);
        for _ in 0..3 {
            world.authority_step();
        }
        assert_eq!(world.state_hash(&mut scratch), 0xd172_6136_40ae_3806);
        assert_eq!(
            World::new(counting_seed(1)).state_hash(&mut scratch),
            0x30ba_2d6a_3b49_4f55
        );
        let last = World {
            tick: u32::MAX,
            seed: counting_seed(0),
        };
        assert_eq!(last.state_hash(&mut scratch), 0xd578_405a_3b5b_8ab1);
    }

    #[test]
    fn the_same_seed_gives_the_same_hashes_and_every_field_counts() {
        let (mut scratch_a, mut scratch_b) = (Vec::new(), Vec::new());
        let mut a = World::new(counting_seed(7));
        let mut b = World::new(counting_seed(7));
        let mut other_seed = World::new(counting_seed(8));
        let mut seen = Vec::new();
        for _ in 0..100 {
            let hash = a.state_hash(&mut scratch_a);
            assert_eq!(hash, b.state_hash(&mut scratch_b));
            assert_ne!(hash, other_seed.state_hash(&mut scratch_b));
            assert!(!seen.contains(&hash), "each tick has its own hash");
            seen.push(hash);
            a.authority_step();
            b.authority_step();
            other_seed.authority_step();
        }
    }

    #[test]
    fn the_scratch_buffer_is_reused() {
        let world = World::new(counting_seed(0));
        let mut scratch = Vec::new();
        let _ = world.state_hash(&mut scratch);
        let (pointer, capacity) = (scratch.as_ptr(), scratch.capacity());
        let _ = world.state_hash(&mut scratch);
        assert_eq!((scratch.as_ptr(), scratch.capacity()), (pointer, capacity));
    }

    #[test]
    #[should_panic(expected = "the tick counter is at u32::MAX")]
    fn the_tick_counter_stops_loudly_and_never_wraps() {
        let mut world = World {
            tick: u32::MAX,
            seed: counting_seed(0),
        };
        world.authority_step();
    }
}
