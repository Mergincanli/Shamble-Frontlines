//! The wasm-bindgen bindings the Worker loads, and nothing else (ADR 0002 parts 3 and 5): the small control API over
//! the simulation and the client endpoint. The page never calls it. Per tick the Worker's host makes two calls,
//! `input` and `step`; both take and return plain numbers, so neither makes the glue call back into JavaScript.
//!
//! What crosses this boundary is kept this small on purpose (Mustafa, 2026-11-18, WB-HEALTH: "one type, plain numbers
//! on every tick: that keeps a later move off it cheap"). The checks live in plain Rust (`Sim::create`), tested
//! natively; the bindings below are a thin wrapper, because the glue's own types only work inside Wasm.

use client_endpoint::block::BlockWriter;
use client_endpoint::schedule::LocalSchedule;
use sim::{SEED_BYTES, TICK_HZ, World};
use wasm_bindgen::prelude::*;

/// Local play: the client endpoint and a local server in one Worker (ADR 0002 part 6). The only mode so far.
pub const MODE_LOCAL: u8 = 0;

/// Why a `Sim` was not created.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateError {
    /// The seed is not `SEED_BYTES` long; it holds this many bytes.
    SeedLength(usize),
    /// A mode that does not exist yet.
    Mode(u8),
    /// More padding than the block writer takes.
    PadBytes(u32),
}

impl CreateError {
    /// The reason in plain words, for the Worker's error message. It never holds the seed.
    #[must_use]
    pub fn message(self) -> String {
        match self {
            Self::SeedLength(found) => {
                format!("the match seed must be {SEED_BYTES} bytes and is {found}")
            }
            Self::Mode(mode) => {
                format!("mode {mode} does not exist: only local play ({MODE_LOCAL})")
            }
            Self::PadBytes(pad) => {
                format!("{pad} bytes of block padding is above the writer's cap")
            }
        }
    }
}

/// One local match in the Worker: the world, the block it writes each tick, and the schedule of its ticks.
#[wasm_bindgen]
pub struct Sim {
    world: World,
    writer: BlockWriter,
    schedule: LocalSchedule,
    newest_input: u32,
}

impl Sim {
    /// Checks what the host passed and builds the match. The block holds tick 0 from the start, and the first tick
    /// is due one tick period after `now_ms`.
    ///
    /// # Errors
    /// A seed of the wrong length, a mode that does not exist, or too much padding.
    pub fn create(seed: &[u8], mode: u8, pad_bytes: u32, now_ms: f64) -> Result<Self, CreateError> {
        let seed =
            <[u8; SEED_BYTES]>::try_from(seed).map_err(|_| CreateError::SeedLength(seed.len()))?;
        if mode != MODE_LOCAL {
            return Err(CreateError::Mode(mode));
        }
        let mut writer = BlockWriter::new(pad_bytes).ok_or(CreateError::PadBytes(pad_bytes))?;
        let world = World::new(seed);
        writer.write_header(world.tick());
        Ok(Self {
            world,
            writer,
            schedule: LocalSchedule::new(now_ms, TICK_HZ),
            newest_input: 0,
        })
    }

    /// The block's bytes, for native tests; the Worker reads them through `block_ptr` and `block_len`.
    #[must_use]
    pub fn block(&self) -> &[u8] {
        self.writer.bytes()
    }
}

#[wasm_bindgen]
impl Sim {
    /// Creates the match. `seed` is 32 bytes the Worker drew; `mode` is `MODE_LOCAL`; `pad_bytes` pads the block for
    /// the hand-off readings (the game passes 0); `now_ms` is the host's time now.
    ///
    /// # Errors
    /// As `Sim::create`, as a JavaScript `Error` with the reason in plain words.
    #[wasm_bindgen(constructor)]
    pub fn new(seed: &[u8], mode: u8, pad_bytes: u32, now_ms: f64) -> Result<Sim, JsError> {
        Self::create(seed, mode, pad_bytes, now_ms).map_err(|error| JsError::new(&error.message()))
    }

    /// Where the block lies in linear memory. The buffer never moves, so the host asks once.
    #[must_use]
    pub fn block_ptr(&self) -> *const u8 {
        self.writer.bytes().as_ptr()
    }

    /// The block's length in bytes; it never changes.
    #[must_use]
    pub fn block_len(&self) -> u32 {
        self.writer.byte_len()
    }

    /// The page's newest input, once per tick before `step`. So far an input is its sequence number.
    pub fn input(&mut self, sequence: u32) {
        self.newest_input = sequence;
    }

    /// The sequence number of the newest input the match has taken.
    #[must_use]
    pub fn newest_input(&self) -> u32 {
        self.newest_input
    }

    /// Runs one tick, writes its block, and returns the time (the host's milliseconds) at which the next tick is due.
    pub fn step(&mut self) -> f64 {
        self.world.authority_step();
        self.writer.write_header(self.world.tick());
        self.schedule.ticked();
        self.schedule.next_due_ms()
    }

    /// Drops the time up to `now_ms` (after a pause, or a stall longer than one wake may catch up) and returns when
    /// the next tick is due: one tick period later.
    pub fn rebase(&mut self, now_ms: f64) -> f64 {
        self.schedule.rebase(now_ms);
        self.schedule.next_due_ms()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client_endpoint::layout::LAYOUT_HASH;

    const SEED: [u8; SEED_BYTES] = [7; SEED_BYTES];

    fn header(sim: &Sim) -> (u32, u32) {
        let block = sim.block();
        let word = |at: usize| {
            u32::from_le_bytes([block[at], block[at + 1], block[at + 2], block[at + 3]])
        };
        (word(0), word(4))
    }

    #[test]
    fn a_new_match_holds_a_valid_block_at_tick_zero() {
        let sim = Sim::create(&SEED, MODE_LOCAL, 0, 1000.0).expect("a good seed and mode");
        assert_eq!(sim.block_len(), 8);
        assert_eq!(sim.block().len(), 8);
        assert_eq!(header(&sim), (0, LAYOUT_HASH));
        assert_eq!(sim.block_ptr(), sim.block().as_ptr());
        assert_eq!(sim.newest_input(), 0);
    }

    #[test]
    fn each_step_is_one_tick_in_the_block_and_gives_the_next_due_time() {
        let mut sim = Sim::create(&SEED, MODE_LOCAL, 0, 1000.0).expect("a good seed and mode");
        let place = sim.block_ptr();
        let period = 1000.0 / f64::from(TICK_HZ);
        let due = sim.step();
        assert_eq!(header(&sim), (1, LAYOUT_HASH));
        assert_eq!(due, 1000.0 + 2.0 * period);
        let due = sim.step();
        assert_eq!(header(&sim), (2, LAYOUT_HASH));
        assert_eq!(due, 1000.0 + 3.0 * period);
        assert_eq!(sim.block_ptr(), place);
    }

    #[test]
    fn a_rebase_drops_the_gap_and_keeps_the_tick() {
        let mut sim = Sim::create(&SEED, MODE_LOCAL, 0, 0.0).expect("a good seed and mode");
        sim.step();
        let due = sim.rebase(60_000.0);
        assert_eq!(due, 60_000.0 + 1000.0 / f64::from(TICK_HZ));
        assert_eq!(header(&sim).0, 1);
        sim.step();
        assert_eq!(header(&sim).0, 2);
    }

    #[test]
    fn the_newest_input_is_kept() {
        let mut sim = Sim::create(&SEED, MODE_LOCAL, 0, 0.0).expect("a good seed and mode");
        sim.input(41);
        sim.input(42);
        assert_eq!(sim.newest_input(), 42);
    }

    #[test]
    fn padding_makes_the_block_longer_and_stays_zero() {
        let mut sim =
            Sim::create(&SEED, MODE_LOCAL, 4096, 0.0).expect("4 KiB of padding is allowed");
        assert_eq!(sim.block_len(), 8 + 4096);
        sim.step();
        assert!(sim.block()[8..].iter().all(|byte| *byte == 0));
    }

    #[test]
    fn a_bad_seed_mode_or_padding_is_refused_with_a_plain_reason() {
        let refused = |seed: &[u8], mode, pad| Sim::create(seed, mode, pad, 0.0).err();
        assert_eq!(
            refused(&[7; 31], MODE_LOCAL, 0),
            Some(CreateError::SeedLength(31))
        );
        assert_eq!(
            refused(&[7; 33], MODE_LOCAL, 0),
            Some(CreateError::SeedLength(33))
        );
        assert_eq!(
            refused(&[], MODE_LOCAL, 0),
            Some(CreateError::SeedLength(0))
        );
        assert_eq!(refused(&SEED, 1, 0), Some(CreateError::Mode(1)));
        assert_eq!(
            refused(&SEED, MODE_LOCAL, u32::MAX),
            Some(CreateError::PadBytes(u32::MAX))
        );
        assert_eq!(
            CreateError::SeedLength(31).message(),
            "the match seed must be 32 bytes and is 31"
        );
        assert!(
            CreateError::Mode(1)
                .message()
                .contains("only local play (0)")
        );
        // No reason quotes the seed's bytes.
        for error in [
            CreateError::SeedLength(31),
            CreateError::Mode(1),
            CreateError::PadBytes(9),
        ] {
            assert!(!error.message().contains("7, 7"));
        }
    }
}
