//! The world: the whole state of one match, and the step that advances it (ADR 0004 parts 1, 3, 4, 8 and 9). So far
//! it holds the tick counter, the one player's applied command and the match seed. The step reads no clock and takes
//! no time: one call is one tick.

use crate::state_hash::fnv1a_64;

/// Ticks per second: the one build-time constant, the same on the server and the client (ADR 0004 part 4; TICK-30).
pub const TICK_HZ: u32 = 30;

/// The length of a match seed in bytes.
pub const SEED_BYTES: usize = 32;

/// The buttons a player's command has room for.
pub const BUTTONS: usize = 16;

/// The length of the shared state's canonical bytes: the tick (4), then the player's record (2 and 16). It is the
/// part a client holds too; what follows it is the authority's alone.
pub const SHARED_STATE_BYTES: usize = 4 + 2 + BUTTONS;

/// The length of the whole state's canonical bytes: the shared state, then the seed.
const STATE_BYTES: usize = SHARED_STATE_BYTES + SEED_BYTES;

/// What the authority starts a match from. There is no `Debug`: the seed is never printed or logged (ADR 0004 part
/// 8).
pub struct MatchSetup {
    /// From whoever starts the match; the simulation draws none itself.
    pub seed: [u8; SEED_BYTES],
    /// The tick the match starts at: 0 in play, and near a counter's wrap in the tests of the wraps.
    pub start_tick: u32,
}

/// What the simulation reads of one player's command: the buttons held and a press counter per button, each
/// wrapping. A command's move, aim and view time join when a system reads them.
///
/// The counters are running totals, so one command can bring many presses of one button at once: an honest client
/// after an outage of 255 ticks brings up to 255. A system that acts on presses applies at most what its own rule
/// allows per tick (one jump, one shot per cooldown) and never one action per press it finds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerCommand {
    pub buttons: u16,
    pub presses: [u8; BUTTONS],
}

impl PlayerCommand {
    /// Nothing held and no press yet: the record before the first command.
    pub const NONE: Self = Self {
        buttons: 0,
        presses: [0; BUTTONS],
    };
}

/// What reads the state field by field, in its one fixed order (ADR 0004 part 9). Every writing of the state goes
/// through it: the canonical bytes the hashes cover are one reader, and a dump that names each field (the replay
/// tools') is another, so a name and its bytes cannot drift apart.
pub trait StateSink {
    fn u16(&mut self, name: &'static str, value: u16);
    fn u32(&mut self, name: &'static str, value: u32);
    /// A field that is a run of bytes, each its own value (the press counters).
    fn bytes(&mut self, name: &'static str, value: &[u8]);
    /// Everything before this call is the shared state; what follows is the authority's alone.
    fn authority(&mut self);
    /// Bytes that are never printed or logged: the match seed (ADR 0004 part 8). A reader may hash or count them.
    fn secret(&mut self, name: &'static str, value: &[u8]);
}

/// The canonical bytes: every integer at a fixed width and little-endian, no `usize`, nothing between the fields.
struct Canonical<'a>(&'a mut Vec<u8>);

impl StateSink for Canonical<'_> {
    // The path form on the integer's own type: the rule scan (tests/rules_scan.rs) reads text and cannot tell
    // `.to_le_bytes()` on a tick from the same call on a float, which part 7 does not allow.
    fn u16(&mut self, _: &'static str, value: u16) {
        self.0.extend_from_slice(&u16::to_le_bytes(value));
    }

    fn u32(&mut self, _: &'static str, value: u32) {
        self.0.extend_from_slice(&u32::to_le_bytes(value));
    }

    fn bytes(&mut self, _: &'static str, value: &[u8]) {
        self.0.extend_from_slice(value);
    }

    fn authority(&mut self) {}

    fn secret(&mut self, _: &'static str, value: &[u8]) {
        self.0.extend_from_slice(value);
    }
}

/// The whole state of one match. There is no `Debug`: the seed is never printed or logged (ADR 0004 part 8).
pub struct World {
    tick: u32,
    /// The command last applied for the one player there is so far; the soldier table takes its place.
    player: PlayerCommand,
    seed: [u8; SEED_BYTES],
}

impl World {
    /// A new match at the setup's start tick, with nothing held.
    #[must_use]
    pub const fn new(setup: &MatchSetup) -> Self {
        Self {
            tick: setup.start_tick,
            player: PlayerCommand::NONE,
            seed: setup.seed,
        }
    }

    /// The number of ticks run so far, counted from the start tick. It is the simulation's only clock: sim time is
    /// `tick / TICK_HZ` seconds.
    #[must_use]
    pub const fn tick(&self) -> u32 {
        self.tick
    }

    /// The command last applied for the player.
    #[must_use]
    pub const fn player(&self) -> &PlayerCommand {
        &self.player
    }

    /// Runs one tick of the authority's phases with the player's commands for it, the oldest first: the newest
    /// becomes the player's record. With none the record holds, so a held button stays held.
    ///
    /// # Panics
    /// When the counter is at `u32::MAX`, which is 4.5 years at 30 Hz: it stops loudly and never wraps (ADR 0004 part
    /// 4: counters use explicit operations).
    pub fn authority_step(&mut self, commands: &[PlayerCommand]) {
        if let Some(newest) = commands.last() {
            self.player = *newest;
        }
        self.tick = self
            .tick
            .checked_add(1)
            .expect("the tick counter is at u32::MAX and cannot advance");
    }

    /// Hands the whole state to `sink`, field by field in its one fixed order (ADR 0004 part 9). The shared state
    /// comes first (the tick, then the player's buttons and the 16 press counters), then `authority()`, then the
    /// authority's own part (the seed, as a secret; the random streams' positions join it when a system first
    /// draws). Each field that joins the state joins here or in `visit_shared`, in the same change.
    pub fn visit(&self, sink: &mut impl StateSink) {
        self.visit_shared(sink);
        sink.authority();
        sink.secret("seed", &self.seed);
    }

    /// Hands the shared state to `sink` and nothing of the authority's part: it never touches the seed.
    pub fn visit_shared(&self, sink: &mut impl StateSink) {
        // Every field is named in this pattern: one added to the struct later does not compile until it is handed
        // over here or in `visit`, or left out here in plain sight.
        let Self {
            tick,
            player: PlayerCommand { buttons, presses },
            seed: _,
        } = self;
        sink.u32("tick", *tick);
        sink.u16("player.buttons", *buttons);
        sink.bytes("player.presses", presses);
    }

    /// Writes the canonical bytes of the whole state into `out`, replacing what it held: the shared state's bytes,
    /// then the authority's, so the shared state is a prefix.
    pub fn write_state(&self, out: &mut Vec<u8>) {
        out.clear();
        out.reserve(STATE_BYTES);
        self.visit(&mut Canonical(out));
    }

    /// Writes the canonical bytes of the shared state into `out`, replacing what it held. No seed byte is ever
    /// written into `out`, so a caller that keeps or hands on the buffer keeps and hands on no seed.
    pub fn write_shared_state(&self, out: &mut Vec<u8>) {
        out.clear();
        out.reserve(SHARED_STATE_BYTES);
        self.visit_shared(&mut Canonical(out));
    }

    /// The state hash: `fnv1a_64` over the canonical bytes. `scratch` holds those bytes afterwards; passing the same
    /// buffer every tick means no allocation after the first call (counted in `tests/state_hash_alloc.rs`).
    #[must_use]
    pub fn state_hash(&self, scratch: &mut Vec<u8>) -> u64 {
        self.write_state(scratch);
        fnv1a_64(scratch)
    }

    /// The hash of the shared state only: the one a client's state can be compared with, because it covers no
    /// seed byte (ADR 0004 part 9). `scratch` holds the shared bytes afterwards.
    #[must_use]
    pub fn shared_hash(&self, scratch: &mut Vec<u8>) -> u64 {
        self.write_shared_state(scratch);
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

    fn world(first: u8, start_tick: u32) -> World {
        World::new(&MatchSetup {
            seed: counting_seed(first),
            start_tick,
        })
    }

    /// Buttons 0x8001 and the counters 1, 2, ..., 16.
    fn pressed() -> PlayerCommand {
        let mut presses = [0; BUTTONS];
        for (counter, value) in presses.iter_mut().zip(1..) {
            *counter = value;
        }
        PlayerCommand {
            buttons: 0x8001,
            presses,
        }
    }

    #[test]
    fn a_new_world_is_at_its_start_tick_and_each_step_is_one_tick() {
        let mut at_zero = world(0, 0);
        assert_eq!(at_zero.tick(), 0);
        assert_eq!(*at_zero.player(), PlayerCommand::NONE);
        at_zero.authority_step(&[]);
        at_zero.authority_step(&[]);
        assert_eq!(at_zero.tick(), 2);
        // A match can start anywhere, for the tests of the counters' wraps.
        let mut late = world(0, 131_069);
        late.authority_step(&[]);
        assert_eq!(late.tick(), 131_070);
    }

    #[test]
    fn the_newest_command_of_a_tick_becomes_the_record_and_none_holds_it() {
        let mut world = world(0, 0);
        let older = PlayerCommand {
            buttons: 2,
            presses: [9; BUTTONS],
        };
        world.authority_step(&[older, pressed()]);
        assert_eq!(*world.player(), pressed());
        // No command this tick: what was held stays held, and no counter moves.
        world.authority_step(&[]);
        assert_eq!(*world.player(), pressed());
        world.authority_step(&[older]);
        assert_eq!(*world.player(), older);
        assert_eq!(world.tick(), 3);
    }

    #[test]
    fn state_bytes_have_a_fixed_order_and_width() {
        let mut world = world(0, 0);
        for _ in 0..0x0101 {
            world.authority_step(&[]);
        }
        world.authority_step(&[pressed()]);
        // What was in the buffer before does not survive.
        let mut bytes = vec![0xEE; 100];
        world.write_state(&mut bytes);
        assert_eq!(bytes.len(), STATE_BYTES);
        assert_eq!((SHARED_STATE_BYTES, bytes.len()), (22, 54));
        // The tick 0x0000_0102, little-endian; the buttons 0x8001; the counters; then the seed as it was given.
        assert_eq!(bytes[..4], [0x02, 0x01, 0x00, 0x00]);
        assert_eq!(bytes[4..6], [0x01, 0x80]);
        assert_eq!(bytes[6..22], pressed().presses);
        assert_eq!(bytes[22..], counting_seed(0));
    }

    /// A reader that notes each field's name and width, and where the authority's part begins.
    #[derive(Default)]
    struct Names(Vec<(&'static str, usize)>);

    impl StateSink for Names {
        fn u16(&mut self, name: &'static str, _: u16) {
            self.0.push((name, 2));
        }

        fn u32(&mut self, name: &'static str, _: u32) {
            self.0.push((name, 4));
        }

        fn bytes(&mut self, name: &'static str, value: &[u8]) {
            self.0.push((name, value.len()));
        }

        fn authority(&mut self) {
            self.0.push(("authority", 0));
        }

        fn secret(&mut self, name: &'static str, value: &[u8]) {
            self.0.push((name, value.len()));
        }
    }

    #[test]
    fn every_reader_of_the_state_gets_the_same_fields_in_the_same_order() {
        let world = world(0, 0);
        let mut names = Names::default();
        world.visit(&mut names);
        let fields = [
            ("tick", 4),
            ("player.buttons", 2),
            ("player.presses", 16),
            ("authority", 0),
            ("seed", 32),
        ];
        assert_eq!(names.0, fields);
        // The widths are the canonical lengths: the shared state's up to the mark, the whole state's in all.
        let width = |fields: &[(&str, usize)]| fields.iter().map(|field| field.1).sum::<usize>();
        assert_eq!(width(&fields[..3]), SHARED_STATE_BYTES);
        assert_eq!(width(&fields), STATE_BYTES);
        let mut shared = Names::default();
        world.visit_shared(&mut shared);
        assert_eq!(shared.0, fields[..3]);
    }

    /// A reader of the shared state that stops the test when it is handed anything of the authority's part.
    struct SharedOnly;

    impl StateSink for SharedOnly {
        fn u16(&mut self, _: &'static str, _: u16) {}

        fn u32(&mut self, _: &'static str, _: u32) {}

        fn bytes(&mut self, _: &'static str, _: &[u8]) {}

        fn authority(&mut self) {
            panic!("the shared state's reader was handed the authority's part");
        }

        fn secret(&mut self, name: &'static str, _: &[u8]) {
            panic!("the shared state's reader was handed the secret {name}");
        }
    }

    /// The shared bytes are the whole state's first bytes, and writing them never puts a seed byte in the buffer:
    /// this seed's bytes are 0xA0 to 0xBF, and none of them is among the shared bytes.
    #[test]
    fn the_shared_bytes_are_a_prefix_and_hold_no_seed_byte() {
        let mut world = world(0xA0, 7);
        world.authority_step(&[pressed()]);
        world.visit_shared(&mut SharedOnly);
        let (mut whole, mut shared) = (Vec::new(), vec![0xEE; 100]);
        world.write_state(&mut whole);
        world.write_shared_state(&mut shared);
        assert_eq!(shared.len(), SHARED_STATE_BYTES);
        assert_eq!(shared, whole[..SHARED_STATE_BYTES]);
        assert_eq!(whole[SHARED_STATE_BYTES..], counting_seed(0xA0));
        assert!(shared.iter().all(|byte| !(0xA0..=0xBF).contains(byte)));
    }

    /// Known answers from a second implementation (a script, not this code): they hold the byte order and the hash
    /// still, on every target the tests run on.
    #[test]
    fn state_hash_matches_known_answers() {
        let mut scratch = Vec::new();
        let mut stepped = world(0, 0);
        assert_eq!(stepped.state_hash(&mut scratch), 0x9c1e_17d4_0d7a_14dd);
        for _ in 0..3 {
            stepped.authority_step(&[]);
        }
        assert_eq!(stepped.state_hash(&mut scratch), 0x1117_0104_4a87_a676);
        assert_eq!(world(1, 0).state_hash(&mut scratch), 0x6f2d_d929_11a2_615d);
        assert_eq!(
            world(0, 131_069).state_hash(&mut scratch),
            0x7d30_b610_41b0_af34
        );
        assert_eq!(
            world(0, u32::MAX).state_hash(&mut scratch),
            0xaea1_07e6_21c4_c5d9
        );
        let mut played = world(0, 0);
        played.authority_step(&[]);
        played.authority_step(&[pressed()]);
        assert_eq!(played.state_hash(&mut scratch), 0xd01f_fe7c_1064_6f36);
        assert_eq!(played.shared_hash(&mut scratch), 0x3f6e_d6ed_be36_8db6);
    }

    /// The shared hash covers the tick and the player's record and no seed byte: two matches with different seeds
    /// agree on it, and differ on the whole state's hash.
    #[test]
    fn the_shared_hash_covers_no_seed_byte() {
        let mut scratch = Vec::new();
        let (mut a, mut b) = (world(0, 0), world(1, 0));
        assert_eq!(a.shared_hash(&mut scratch), 0xf848_4165_2498_787d);
        for step in 0..50 {
            let commands = [pressed()];
            let commands = if step % 2 == 0 { &commands[..] } else { &[] };
            a.authority_step(commands);
            b.authority_step(commands);
            assert_eq!(a.shared_hash(&mut scratch), b.shared_hash(&mut scratch));
            assert_ne!(a.state_hash(&mut scratch), b.state_hash(&mut scratch));
        }
        // The tick and each part of the record count. The buffer holds the shared bytes afterwards and no more.
        let base = a.shared_hash(&mut scratch);
        assert_eq!(scratch.len(), SHARED_STATE_BYTES);
        a.authority_step(&[]);
        assert_ne!(a.shared_hash(&mut scratch), base);
        let (mut held, mut counted) = (world(0, 9), world(0, 9));
        held.authority_step(&[PlayerCommand {
            buttons: 1,
            ..PlayerCommand::NONE
        }]);
        let mut presses = [0; BUTTONS];
        presses[BUTTONS - 1] = 1;
        counted.authority_step(&[PlayerCommand {
            buttons: 0,
            presses,
        }]);
        let mut plain = world(0, 9);
        plain.authority_step(&[]);
        let plain = plain.shared_hash(&mut scratch);
        assert_ne!(held.shared_hash(&mut scratch), plain);
        assert_ne!(counted.shared_hash(&mut scratch), plain);
    }

    #[test]
    fn the_same_seed_gives_the_same_hashes_and_every_field_counts() {
        let (mut scratch_a, mut scratch_b) = (Vec::new(), Vec::new());
        let mut a = world(7, 0);
        let mut b = world(7, 0);
        let mut other_seed = world(8, 0);
        let mut seen = Vec::new();
        for _ in 0..100 {
            let hash = a.state_hash(&mut scratch_a);
            assert_eq!(hash, b.state_hash(&mut scratch_b));
            assert_ne!(hash, other_seed.state_hash(&mut scratch_b));
            assert!(!seen.contains(&hash), "each tick has its own hash");
            seen.push(hash);
            a.authority_step(&[]);
            b.authority_step(&[]);
            other_seed.authority_step(&[]);
        }
    }

    #[test]
    #[should_panic(expected = "the tick counter is at u32::MAX")]
    fn the_tick_counter_stops_loudly_and_never_wraps() {
        world(0, u32::MAX).authority_step(&[]);
    }
}
