//! The wasm-bindgen bindings the Worker loads, and nothing else (ADR 0002 parts 3 and 5): the small control API over
//! local play's one step (`local_match`). The page never calls it. Per tick the Worker's host makes two calls,
//! `input` and `step`; both take and return plain numbers, so neither makes the glue call back into JavaScript. A
//! replay of a recorded match also reads `shared_hash`, one plain number; no live tick does.
//!
//! What crosses this boundary is kept this small on purpose. Mustafa, 2026-11-18, on WB-HEALTH: "Keep what crosses
//! it as small as the plan has it (one type, plain numbers on every tick): that keeps a later move off it cheap."
//! The checks live in plain Rust (`Sim::create`), tested natively; the bindings below are a thin wrapper, because
//! the glue's own types only work inside Wasm.

pub mod local_match;
pub mod panic;

use client_endpoint::post::Post;
use local_match::{LocalMatch, loopback};
use sim::collision::{StaticWorld, WorldError};
use sim::{MatchSetup, SEED_BYTES, TICK_HZ};
use wasm_bindgen::prelude::*;

/// Local play: the client endpoint and a local server in one Worker (ADR 0002 part 6). The only mode so far.
pub const MODE_LOCAL: u32 = 0;

/// The map a local match is played on, by its id in the content: the one map the game's content holds so far (the
/// first recording's header names the same id). A mode that names its map comes with a later change-set.
pub const LOCAL_MAP: &str = "test-scene";

/// Ticks per second, for the host's readout: the simulation's own build-time constant, asked once.
#[wasm_bindgen]
#[must_use]
pub fn tick_hz() -> u32 {
    TICK_HZ
}

/// Why a `Sim` was not created.
#[derive(Debug, Clone, PartialEq)]
pub enum CreateError {
    /// The seed is not `SEED_BYTES` long; it holds this many bytes.
    SeedLength(usize),
    /// A mode that does not exist yet.
    Mode(u32),
    /// The content bundle is not one the game's loader takes: its refusal, with the file and the place in it.
    Content(content::Error),
    /// The content holds no map with the id `LOCAL_MAP`.
    MapMissing,
    /// The map makes no static collision world: why.
    World(WorldError),
    /// More padding than the block writer takes.
    PadBytes(u32),
}

impl CreateError {
    /// The reason in plain words, for the Worker's error message. It never holds the seed.
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::SeedLength(found) => {
                format!("the match seed must be {SEED_BYTES} bytes and is {found}")
            }
            Self::Mode(mode) => {
                format!("mode {mode} does not exist: only local play ({MODE_LOCAL})")
            }
            // The loader's own sentence, whole: it begins "the content is refused: " and is at most 512 bytes.
            Self::Content(error) => error.message(),
            Self::MapMissing => {
                format!(
                    "the content holds no map \"{LOCAL_MAP}\", the map a local match is played on"
                )
            }
            Self::World(error) => {
                format!(
                    "the map \"{LOCAL_MAP}\" makes no collision world: {}",
                    error.message()
                )
            }
            Self::PadBytes(pad) => {
                format!("{pad} bytes of block padding is above the writer's cap")
            }
        }
    }
}

/// The static collision world of the local match's map, built once from the content the loader took.
fn local_world(set: &content::ContentSet) -> Result<StaticWorld, CreateError> {
    let map = set.map(LOCAL_MAP).ok_or(CreateError::MapMissing)?;
    StaticWorld::build(map).map_err(CreateError::World)
}

/// One local match in the Worker. Its state is the local match's (`local_match`): the client endpoint, the local
/// server and the loopback between them; this type is the bindings around it.
#[wasm_bindgen]
pub struct Sim {
    local: LocalMatch,
}

impl Sim {
    /// Checks what the host passed and builds the match at tick 0, its join already on its way. The block is whole
    /// from the start; the host's first `rebase` says when the first tick is due. `bundle` is the match's content
    /// (`content.json`): it crosses once, here, so no call can swap it mid-match; the game's own loader judges it.
    /// The match holds the static collision world of the content's map `LOCAL_MAP`; nothing of the content enters
    /// the match's state, and no step queries the world yet (change-set 4c).
    ///
    /// # Errors
    /// A seed of the wrong length, a mode that does not exist, a bundle the loader refuses, a bundle without the map,
    /// a map that makes no world, or too much padding, checked in that order.
    pub fn create(
        seed: &[u8],
        mode: u32,
        pad_bytes: u32,
        bundle: &[u8],
    ) -> Result<Self, CreateError> {
        let seed =
            <[u8; SEED_BYTES]>::try_from(seed).map_err(|_| CreateError::SeedLength(seed.len()))?;
        if mode != MODE_LOCAL {
            return Err(CreateError::Mode(mode));
        }
        let set = content::load(bundle).map_err(CreateError::Content)?;
        let world = local_world(&set)?;
        let setup = MatchSetup {
            seed,
            start_tick: 0,
        };
        let local = LocalMatch::new(&setup, 0, pad_bytes, loopback(), world);
        local
            .map(|local| Self { local })
            .ok_or(CreateError::PadBytes(pad_bytes))
    }

    /// The local match, for native tests; the Worker reads only the block.
    #[must_use]
    pub const fn local(&self) -> &LocalMatch {
        &self.local
    }
}

#[wasm_bindgen]
impl Sim {
    /// Creates the match. `seed` is 32 bytes the Worker drew; `mode` is `MODE_LOCAL` (a `u32`: a whole number below
    /// 2^32 arrives as the host passed it, and JavaScript turns anything else into one first, so the host passes the
    /// constant); `pad_bytes` pads the block for the hand-off readings (the game passes 0); `bundle` is the content
    /// the page fetched, which the Worker has already held to its size limit.
    ///
    /// # Errors
    /// As `Sim::create`, as a JavaScript `Error` with the reason in plain words.
    #[wasm_bindgen(constructor)]
    pub fn new(seed: &[u8], mode: u32, pad_bytes: u32, bundle: &[u8]) -> Result<Sim, JsError> {
        Self::create(seed, mode, pad_bytes, bundle).map_err(|error| JsError::new(&error.message()))
    }

    /// Where the block lies in linear memory. The buffer never moves, so the host asks once.
    #[must_use]
    pub fn block_ptr(&self) -> *const u8 {
        self.local.block().as_ptr()
    }

    /// The block's length in bytes; it never changes.
    #[must_use]
    pub fn block_len(&self) -> u32 {
        self.local.block_len()
    }

    /// The page's newest input, once per tick before `step`, as six plain numbers: the post's number, the held
    /// buttons in the low 16 bits, and the 16 running press counts packed four to a word (button `k` in byte
    /// `k % 4` of word `k / 4`). Any six numbers are taken: nothing here can panic on what the page sent. The
    /// post handed in last before a step is the one that step uses, whatever its number: keeping the newest by
    /// number is the host's work (`packages/sim-worker/src/host.ts`).
    pub fn input(&mut self, post: u32, held: u32, p0: u32, p1: u32, p2: u32, p3: u32) {
        self.local
            .set_post(Post::from_words(post, held, [p0, p1, p2, p3]));
    }

    /// Runs one tick, writes its block, and returns the time (the host's milliseconds) at which the next tick is
    /// due. `now_ms` is the host's time now.
    pub fn step(&mut self, now_ms: f64) -> f64 {
        self.local.step(now_ms)
    }

    /// Drops the time up to `now_ms` (at the start, after a pause, or after a stall longer than one wake may catch
    /// up) and returns when the next tick is due: one tick period later.
    pub fn rebase(&mut self, now_ms: f64) -> f64 {
        self.local.rebase(now_ms)
    }

    /// The hash of the shared state as it stands, as one plain number (a JavaScript `BigInt`): what a replay of a
    /// recorded match compares tick by tick, the same natively and in WebAssembly. It covers no byte of the match
    /// seed, changes nothing and hands out no buffer. Only a replay calls it: a live tick stays at its two calls,
    /// `input` and `step`.
    #[must_use]
    pub fn shared_hash(&self) -> u64 {
        self.local.shared_hash()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client_endpoint::layout::LAYOUT_HASH;

    const SEED: [u8; SEED_BYTES] = [7; SEED_BYTES];

    /// The test bundle (`content::bundle::test_bundle`): the content every test match here is made on.
    fn bundle() -> Vec<u8> {
        content::bundle::test_bundle().into_bytes()
    }

    /// The block's words: the tick, the layout's hash, then the echo and the link's counts.
    fn words(sim: &Sim) -> Vec<u32> {
        let fours = sim.local().block().as_chunks::<4>().0;
        fours.iter().map(|four| u32::from_le_bytes(*four)).collect()
    }

    #[test]
    fn a_new_match_holds_a_valid_block_at_tick_zero() {
        let sim = Sim::create(&SEED, MODE_LOCAL, 0, &bundle()).expect("a good seed and mode");
        assert_eq!(sim.block_len(), 44);
        assert_eq!(words(&sim), [&[0, LAYOUT_HASH][..], &[0; 9]].concat());
        assert_eq!(sim.block_ptr(), sim.local().block().as_ptr());
        assert_eq!(tick_hz(), 30);
    }

    #[test]
    fn each_step_is_one_tick_in_the_block_and_gives_the_next_due_time() {
        let mut sim = Sim::create(&SEED, MODE_LOCAL, 0, &bundle()).expect("a good seed and mode");
        let place = sim.block_ptr();
        let period = 1000.0 / f64::from(TICK_HZ);
        assert_eq!(sim.rebase(1000.0), 1000.0 + period);
        assert_eq!(sim.step(1000.0 + period), 1000.0 + 2.0 * period);
        assert_eq!(words(&sim)[..2], [1, LAYOUT_HASH]);
        assert_eq!(sim.step(1000.0 + 2.0 * period), 1000.0 + 3.0 * period);
        assert_eq!(words(&sim)[..2], [2, LAYOUT_HASH]);
        assert_eq!(sim.block_ptr(), place);
        // A rebase drops the gap and keeps the tick.
        assert_eq!(sim.rebase(60_000.0), 60_000.0 + period);
        sim.step(60_000.0 + period);
        assert_eq!(words(&sim)[0], 3);
    }

    /// The six numbers of `input` reach the local server and come back in the block: each button's count in its
    /// own byte, and only the low 16 held bits.
    #[test]
    fn what_the_page_posts_comes_back_in_the_block_s_echo() {
        let mut sim = Sim::create(&SEED, MODE_LOCAL, 0, &bundle()).expect("a good seed and mode");
        sim.step(0.0);
        for button in 0..16u32 {
            // One press of this button, on top of the ones before it, with only its own bit held.
            let pressed = |word: u32| match (button / 4).cmp(&word) {
                std::cmp::Ordering::Greater => 0x0101_0101,
                std::cmp::Ordering::Equal => 0x0101_0101 >> (8 * (3 - button % 4)),
                std::cmp::Ordering::Less => 0,
            };
            let presses = [0, 1, 2, 3].map(pressed);
            let [p0, p1, p2, p3] = presses;
            sim.input(button + 1, 0xFFFF_0000 | 1 << button, p0, p1, p2, p3);
            sim.step(0.0);
            let echo = [&[1 << button][..], &presses].concat();
            assert_eq!(words(&sim)[2..7], echo, "button {button}");
            // Sent and received climb with the ticks, nothing is refused, and the post used is this step's.
            assert_eq!(words(&sim)[7..], [button + 1, button + 2, 0, button + 1]);
        }
    }

    /// The one number a replay reads is the local server's shared hash: two matches whose seeds differ agree on it
    /// tick by tick, and one button held differently makes it differ.
    #[test]
    fn the_shared_hash_is_the_local_server_s_and_covers_no_seed() {
        let mut a = Sim::create(&SEED, MODE_LOCAL, 0, &bundle()).expect("a good seed and mode");
        let mut b =
            Sim::create(&[9; SEED_BYTES], MODE_LOCAL, 0, &bundle()).expect("a good seed and mode");
        let mut held_otherwise =
            Sim::create(&SEED, MODE_LOCAL, 0, &bundle()).expect("a good seed and mode");
        for tick in 0..20u32 {
            let hash = a.shared_hash();
            assert_eq!(hash, b.shared_hash());
            let world = a.local().room().world();
            assert_eq!(hash, world.shared_hash());
            // Button 0 is pressed once per tick; the third match also holds button 4 on the last tick.
            let last = u32::from(tick == 19);
            a.input(tick + 1, tick % 4, tick + 1, 0, 0, 0);
            b.input(tick + 1, tick % 4, tick + 1, 0, 0, 0);
            held_otherwise.input(tick + 1, (tick % 4) | (last << 4), tick + 1, 0, 0, 0);
            a.step(0.0);
            b.step(0.0);
            held_otherwise.step(0.0);
            assert_eq!(held_otherwise.shared_hash() == a.shared_hash(), tick < 19);
        }
        for sim in [&a, &b, &held_otherwise] {
            assert_eq!(sim.local().room().counters().refused(), 0);
        }
    }

    #[test]
    fn padding_makes_the_block_longer_and_stays_zero() {
        let mut sim =
            Sim::create(&SEED, MODE_LOCAL, 4096, &bundle()).expect("4 KiB of padding is allowed");
        assert_eq!(sim.block_len(), 44 + 4096);
        sim.input(1, 0xFFFF, u32::MAX, u32::MAX, u32::MAX, u32::MAX);
        sim.step(0.0);
        sim.step(0.0);
        assert!(sim.local().block()[44..].iter().all(|byte| *byte == 0));
    }

    #[test]
    fn a_bad_seed_mode_or_padding_is_refused_with_a_plain_reason() {
        let refused = |seed: &[u8], mode, pad| Sim::create(seed, mode, pad, &bundle()).err();
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
        // The mode is compared as the whole number the host passed: 256 is not local play.
        assert_eq!(refused(&SEED, 256, 0), Some(CreateError::Mode(256)));
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

    /// The content crosses here once, and the game's own loader judges it: a bundle it refuses makes no match, and
    /// the reason is the loader's own sentence, with the file and the place in it.
    #[test]
    fn a_bundle_the_loader_refuses_makes_no_match_and_says_the_loader_s_words() {
        let refused = |bundle: &[u8]| Sim::create(&SEED, MODE_LOCAL, 0, bundle).err();
        for bundle in [&b""[..], b"{}", b"not a bundle", &[0xff, 0xfe]] {
            let error = refused(bundle).expect("refused");
            let CreateError::Content(why) = &error else {
                panic!("{error:?}");
            };
            assert_eq!(error.message(), why.message());
            assert!(error.message().starts_with("the content is refused: "));
        }
        // A movement file that is not in canonical form, inside a bundle the writer made: named by its file.
        let text = content::movement::TEST_TEXT.replace(": 100,", ": 100.0,");
        let unformed = content::bundle::write(&[("movement/test-soldier.json", &text)]);
        let message = refused(unformed.as_bytes()).expect("refused").message();
        assert!(message.contains("movement/test-soldier.json"), "{message}");
        assert!(refused(&bundle()).is_none());
    }

    /// The four checks run in a fixed order, seed, mode, content, padding: each case below fails all the ones after
    /// its own, and only its own is named.
    #[test]
    fn the_checks_run_in_the_order_seed_mode_content_map_padding() {
        let bad = b"{}";
        let first = |seed: &[u8], mode, pad, bundle: &[u8]| {
            Sim::create(seed, mode, pad, bundle)
                .err()
                .map(|error| std::mem::discriminant(&error))
        };
        let kind = |error: CreateError| Some(std::mem::discriminant(&error));
        assert_eq!(
            first(&[7; 31], 1, u32::MAX, bad),
            kind(CreateError::SeedLength(0))
        );
        assert_eq!(first(&SEED, 1, u32::MAX, bad), kind(CreateError::Mode(0)));
        let loaded = content::load(bad).expect_err("refused");
        assert_eq!(
            first(&SEED, MODE_LOCAL, u32::MAX, bad),
            kind(CreateError::Content(loaded))
        );
        // A bundle the loader takes, with no map: the map's check comes before the padding's. (The world's check
        // stands between the two; no map made here reaches it: the world's own tests make its refusals.)
        let mapless =
            content::bundle::write(&[("movement/test-soldier.json", content::movement::TEST_TEXT)]);
        assert_eq!(
            first(&SEED, MODE_LOCAL, u32::MAX, mapless.as_bytes()),
            kind(CreateError::MapMissing)
        );
        assert_eq!(
            first(&SEED, MODE_LOCAL, u32::MAX, &bundle()),
            kind(CreateError::PadBytes(0))
        );
    }

    /// The match holds the static world of the content's map `LOCAL_MAP`: the same world the map builds on its own.
    #[test]
    fn the_match_holds_the_static_world_of_the_local_map() {
        let sim = Sim::create(&SEED, MODE_LOCAL, 0, &bundle()).expect("a good seed and mode");
        let set = content::load(&bundle()).expect("the test bundle loads");
        let built =
            StaticWorld::build(set.map(LOCAL_MAP).expect("the test map")).expect("it builds");
        let held = sim.local().static_world();
        assert_eq!(held.solids().len(), built.solids().len());
        assert_eq!(held.tree_depth(), built.tree_depth());
        assert_eq!(held.tree_bytes(), built.tree_bytes());
        assert!(!held.solids().is_empty());
    }

    /// A bundle whose maps have other ids makes no match, and says which map it looked for; a map that makes no
    /// world says the world's own reason.
    #[test]
    fn a_bundle_without_the_local_map_makes_no_match() {
        let other = content::map::TEST_TEXT.replace("\"id\": \"test-scene\"", "\"id\": \"other\"");
        assert_ne!(other, content::map::TEST_TEXT);
        let bundle = content::bundle::write(&[
            ("maps/other.json", &other),
            ("movement/test-soldier.json", content::movement::TEST_TEXT),
        ]);
        assert!(content::load(bundle.as_bytes()).is_ok());
        assert_eq!(
            Sim::create(&SEED, MODE_LOCAL, 0, bundle.as_bytes()).err(),
            Some(CreateError::MapMissing)
        );
        assert_eq!(
            CreateError::MapMissing.message(),
            "the content holds no map \"test-scene\", the map a local match is played on"
        );
        let deep = WorldError::TooDeep(32);
        assert_eq!(
            CreateError::World(deep).message(),
            format!(
                "the map \"test-scene\" makes no collision world: {}",
                deep.message()
            )
        );
    }
}
