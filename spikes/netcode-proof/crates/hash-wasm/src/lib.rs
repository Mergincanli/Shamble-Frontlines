//! npt-hash-wasm: npt-sim's one replay driver ([`npt_sim::replay::Replayer`]) behind seven integer-only exports, built
//! as a wasm32 cdylib that Node runs for the native-against-Wasm hash check (u4-build-plan.md §5 U4d2; plan §4.5's
//! determinism check). check.mjs's `wasm` step feeds a recording's words to the Wasm build through the Node loader
//! beside this file (loader.mjs) and compares its per-tick hashes with npt-lab's native legs. Nothing depends on this
//! crate; it depends on npt-sim alone; its `bites` feature forwards to npt-sim's.
//!
//! **The exports** (each `extern "C"`, every parameter and result a u32):
//! - `begin(bite, arm_tick) -> u32`: drops any replay begun before and starts a fresh one: for `bite` 0 a plain
//!   [`npt_sim::replay::Replayer::new`]; for `bite` k in 1..=4 (bites builds only) `Replayer::with_bite` of
//!   `World::BITES[k - 1]`, armed at `arm_tick`. Returns `BEGUN` (0), or `REFUSED` (1) for a k outside 1..=4 or any
//!   k above 0 in the measured build, and then changes nothing (a replay begun before stays as it was). The scene and
//!   the seed come from the recording's header, the one reader of which is npt-sim's replay.rs.
//! - `feed(word)`: the recording's next 4 bytes, little-endian. Before any `begin` it does nothing.
//! - `step() -> u32`: runs the next tick if the words fed hold it whole: `NEED_MORE` (0), `STEPPED` (1), `DONE` (2)
//!   or `FAILED` (3), the four [`npt_sim::replay::Step`]s. Before any `begin` it returns `FAILED` and changes nothing.
//!   At `NEED_MORE`, feeding more words and stepping again goes on; after `DONE` it returns `DONE` again; after
//!   `FAILED` it returns `FAILED` again (the replayer's own rule), until the next `begin`.
//! - `hash_lo() -> u32`, `hash_hi() -> u32`: the low and the high 32 bits of the last tick's state hash; 0 before the
//!   header has been read (no world yet) and before any `begin`.
//! - `dump_len() -> u32`: takes the last tick's dump once, keeps it, and returns its length in words; 0 (and an empty
//!   kept dump) before the header and before any `begin`. `dump_word(i) -> u32`: word i of the dump `dump_len` kept;
//!   0 at or past its end. A `begin` empties the kept dump.
//!
//! The state lives in a `thread_local!` cell, never a `static mut`: on wasm32 (no atomics) it is one static; natively
//! each test thread has its own, so the unit tests share none.
//!
//! **The spike's third `unsafe` exception** (the first two are npt-client-wasm's and npt-tcpinfo's), granted by the
//! advisor under ADR 0002:264-265's rule: "an exception needs a written invariant, a test and an `#[allow]` with its
//! reason" (the advisor's condition 7; plan.md §4.2). In edition 2024 an export's `#[no_mangle]` takes the
//! `#[unsafe(no_mangle)]` form, which the unsafe_code lint fires on; the workspace forbids that lint, which no allow can
//! relax, so this crate's manifest has its own lint tables (unsafe_code denied, every other lint the workspace's), and
//! each of the seven exports carries one `#[allow(unsafe_code, reason = "...")]` directly on it. No `unsafe` block,
//! unsafe fn, impl, trait or extern block exists here.
//! - **The written invariant**, which each allow's reason repeats in short: no exported name collides with another
//!   symbol, and every export has integer signatures only (u32 parameters and results).
//! - **What it was checked against** (the advisor's U4D2-Q8 ruling): on wasm32-unknown-unknown, the module's only other
//!   exports are the linker's (loader.mjs lists them); natively, this container's glibc 2.39 has one symbol of these
//!   names, the compat `step@GLIBC_2.2.5` (`nm -D`), a non-default version that no new link binds, and the native
//!   build is never loaded as a library (its unit tests call the functions in-process).
//! - **The test**: loader.mjs lists the module's exports with `WebAssembly.Module.exports` and refuses, before
//!   instantiating, any name not on its list (check.test.mjs pins the list and the refusal); check.mjs's deps step
//!   refuses an export whose parameters or result are not u32 or i32 (the advisor's U4D2-Q7 ruling), any `unsafe` but
//!   a listed export's `#[unsafe(no_mangle)]`, and any relaxing of the lint but the seven placed allows.

use std::cell::RefCell;

use npt_sim::replay::{Replayer, Step};

/// `begin`'s results.
const BEGUN: u32 = 0;
const REFUSED: u32 = 1;
/// `step`'s results, one per [`Step`].
const NEED_MORE: u32 = 0;
const STEPPED: u32 = 1;
const DONE: u32 = 2;
const FAILED: u32 = 3;

/// The replay begun last, and the dump `dump_len` kept.
struct State {
    replayer: Option<Replayer>,
    dump: Vec<u32>,
}

thread_local! {
    static STATE: RefCell<State> = const {
        RefCell::new(State {
            replayer: None,
            dump: Vec::new(),
        })
    };
}

/// The replayer `begin` makes for `bite` (0: none; k: `World::BITES[k - 1]`), or `None` to refuse it.
fn replayer_for(bite: u32, arm_tick: u32) -> Option<Replayer> {
    if bite == 0 {
        return Some(Replayer::new());
    }
    with_bite(bite, arm_tick)
}

#[cfg(feature = "bites")]
fn with_bite(bite: u32, arm_tick: u32) -> Option<Replayer> {
    let k = usize::try_from(bite).ok()?.checked_sub(1)?;
    let id = npt_sim::World::BITES.get(k)?;
    Replayer::with_bite(id, arm_tick)
}

#[cfg(not(feature = "bites"))]
fn with_bite(_bite: u32, _arm_tick: u32) -> Option<Replayer> {
    None
}

/// Starts a fresh replay, with no bite (0) or `World::BITES[bite - 1]` armed at `arm_tick` (bites builds only).
#[allow(
    unsafe_code,
    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
              collides with another symbol, and every export has integer signatures only"
)]
#[unsafe(no_mangle)]
pub extern "C" fn begin(bite: u32, arm_tick: u32) -> u32 {
    let Some(replayer) = replayer_for(bite, arm_tick) else {
        return REFUSED;
    };
    STATE.with_borrow_mut(|s| {
        s.replayer = Some(replayer);
        s.dump.clear();
    });
    BEGUN
}

/// Feeds the recording's next 4 bytes, little-endian.
#[allow(
    unsafe_code,
    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
              collides with another symbol, and every export has integer signatures only"
)]
#[unsafe(no_mangle)]
pub extern "C" fn feed(word: u32) {
    STATE.with_borrow_mut(|s| {
        if let Some(r) = &mut s.replayer {
            r.feed(word);
        }
    });
}

/// Runs the next tick if its words are whole: `NEED_MORE`, `STEPPED`, `DONE` or `FAILED`.
#[allow(
    unsafe_code,
    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
              collides with another symbol, and every export has integer signatures only"
)]
#[unsafe(no_mangle)]
pub extern "C" fn step() -> u32 {
    STATE.with_borrow_mut(|s| match s.replayer.as_mut().map(Replayer::step) {
        None | Some(Step::Failed(_)) => FAILED,
        Some(Step::NeedMore) => NEED_MORE,
        Some(Step::Stepped(_)) => STEPPED,
        Some(Step::Done) => DONE,
    })
}

/// The last tick's state hash, `shift` bits down, as a u32 (0 before the header).
fn hash_bits(shift: u32) -> u32 {
    STATE.with_borrow(|s| {
        let h = s
            .replayer
            .as_ref()
            .and_then(Replayer::state_hash)
            .unwrap_or(0);
        (h >> shift) as u32
    })
}

/// The low 32 bits of the last tick's state hash.
#[allow(
    unsafe_code,
    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
              collides with another symbol, and every export has integer signatures only"
)]
#[unsafe(no_mangle)]
pub extern "C" fn hash_lo() -> u32 {
    hash_bits(0)
}

/// The high 32 bits of the last tick's state hash.
#[allow(
    unsafe_code,
    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
              collides with another symbol, and every export has integer signatures only"
)]
#[unsafe(no_mangle)]
pub extern "C" fn hash_hi() -> u32 {
    hash_bits(32)
}

/// Takes the last tick's dump, keeps it, and returns its length in words.
#[allow(
    unsafe_code,
    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
              collides with another symbol, and every export has integer signatures only"
)]
#[unsafe(no_mangle)]
pub extern "C" fn dump_len() -> u32 {
    STATE.with_borrow_mut(|s| {
        s.dump = s
            .replayer
            .as_ref()
            .and_then(Replayer::dump)
            .map(|d| d.words().to_vec())
            .unwrap_or_default();
        u32::try_from(s.dump.len()).unwrap_or(u32::MAX)
    })
}

/// Word `i` of the dump `dump_len` kept; 0 at or past its end.
#[allow(
    unsafe_code,
    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
              collides with another symbol, and every export has integer signatures only"
)]
#[unsafe(no_mangle)]
pub extern "C" fn dump_word(i: u32) -> u32 {
    STATE.with_borrow(|s| {
        usize::try_from(i)
            .ok()
            .and_then(|i| s.dump.get(i).copied())
            .unwrap_or(0)
    })
}

#[cfg(test)]
mod tests {
    //! h2 (i), PM-0004-21's committed vector natively, and h3, the exports against the replayer they wrap
    //! (u4-build-plan.md §5 U4d2; u4d2-brief.md 2.6). Inline, so check.mjs's bite-home scan reads these test-only ids
    //! as tests.

    use super::*;
    use npt_sim::replay::Recording;
    use npt_sim::{Scene, SimCore, World};

    /// The committed vector: CO's initial world (`World::new(&Scene::course(), 1, 0)`, D-7's form), one word a line
    /// as 8 hex digits, below a `digest` line holding its state hash as 16 hex digits.
    const VECTOR: &str = include_str!("../co-initial-dump.txt");

    /// The vector file's digest and words.
    fn vector() -> (u64, Vec<u32>) {
        let mut digest = None;
        let mut words = Vec::new();
        for line in VECTOR.lines().map(str::trim) {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(d) = line.strip_prefix("digest ") {
                digest = Some(u64::from_str_radix(d, 16).expect("16 hex digits"));
            } else {
                assert_eq!(line.len(), 8, "a word is 8 hex digits: {line:?}");
                words.push(u32::from_str_radix(line, 16).expect("8 hex digits"));
            }
        }
        (digest.expect("a digest line"), words)
    }

    /// Bytes as little-endian 32-bit words, the last zero-padded (sim/tests/replay.rs's `words`).
    fn le_words(bytes: &[u8]) -> Vec<u32> {
        bytes
            .chunks(4)
            .map(|c| {
                let mut w = [0u8; 4];
                w[..c.len()].copy_from_slice(c);
                u32::from_le_bytes(w)
            })
            .collect()
    }

    /// The exports' hash, as one u64.
    fn export_hash() -> u64 {
        u64::from(hash_hi()) << 32 | u64::from(hash_lo())
    }

    /// The exports' dump, through `dump_len` and `dump_word`.
    fn export_dump() -> Vec<u32> {
        (0..dump_len()).map(|i| dump_word(i)).collect()
    }

    /// TC's short recording (sim/tests/replay.rs:514-515): 60 ticks across the u32 wrap.
    fn short() -> Recording {
        Recording::synthetic(&Scene::targets(), 8, u32::MAX - 20, 60, 2_000)
    }

    /// A recording with no tick: its header alone, so `step` builds the world and returns `DONE`.
    fn header_only(scene: &Scene) -> Vec<u32> {
        let rec = Recording {
            scene: scene.id(),
            seed: 1,
            first_tick: 0,
            span_ms: 2_000,
            generator: 0,
            ticks: Vec::new(),
        };
        le_words(&rec.encode())
    }

    /// Feeds `words` through the exports in chunks of `chunk`, stepping until `NEED_MORE` between them, and returns
    /// each stepped tick's hash and dump, and the last status.
    fn run_exports(words: &[u32], chunk: usize) -> (Vec<(u64, Vec<u32>)>, u32) {
        let mut out = Vec::new();
        let mut last = NEED_MORE;
        for part in words.chunks(chunk) {
            for w in part {
                feed(*w);
            }
            loop {
                last = step();
                match last {
                    STEPPED => out.push((export_hash(), export_dump())),
                    NEED_MORE | DONE | FAILED => break,
                    other => panic!("a status step does not give: {other}"),
                }
            }
        }
        (out, last)
    }

    /// The same through a `Replayer` fed every word first.
    fn run_reference(mut r: Replayer, words: &[u32]) -> Vec<(u64, Vec<u32>)> {
        for w in words {
            r.feed(*w);
        }
        let mut out = Vec::new();
        while let Step::Stepped(_) = r.step() {
            out.push((
                r.state_hash().expect("a world"),
                r.dump().expect("a world").words().to_vec(),
            ));
        }
        out
    }

    #[test]
    fn h2_cos_initial_dump_is_the_committed_vector_natively_and_through_the_exports() {
        let (digest, words) = vector();
        let world = World::new(&Scene::course(), 1, 0);
        let d = world.dump();
        let shown: Vec<String> = d.words().iter().map(|w| format!("{w:08x}")).collect();
        // The first word that differs, named by its field (h2's shown failure names the word: U4D2-Q11).
        let n = d.words().len().max(words.len());
        if let Some(i) = (0..n).find(|&i| d.words().get(i) != words.get(i)) {
            let show = |w: Option<&u32>| w.map_or("none".to_owned(), |w| format!("{w:08x}"));
            panic!(
                "the committed vector differs at word {i} ({}): {} committed, {} built natively ({} words built, {} \
                 committed); the file should read: digest {:016x}, then\n{}",
                d.field(i).unwrap_or_else(|| "end".to_owned()),
                show(words.get(i)),
                show(d.words().get(i)),
                d.words().len(),
                words.len(),
                d.hash(),
                shown.join("\n")
            );
        }
        assert_eq!(
            (d.hash(), d.words()),
            (digest, words.as_slice()),
            "the committed file should read: digest {:016x}, then\n{}",
            d.hash(),
            shown.join("\n")
        );
        assert_eq!(world.state_hash(), digest);
        assert_eq!(words.len(), 260, "D-7's 260 words");
        // Through the exports, from a header-only recording.
        assert_eq!(begin(0, 0), BEGUN);
        let (stepped, last) = run_exports(&header_only(&Scene::course()), 5);
        assert!(stepped.is_empty());
        assert_eq!(last, DONE);
        assert_eq!((export_hash(), export_dump()), (digest, words));
    }

    #[test]
    fn h3_the_exports_give_the_replayers_own_hashes_and_dumps_tick_by_tick() {
        let words = le_words(&short().encode());
        let reference = run_reference(Replayer::new(), &words);
        assert_eq!(reference.len(), 60);
        // In chunks smaller than a tick and larger than many, so NEED_MORE falls inside ticks and between them.
        for chunk in [7, 1_000, words.len()] {
            assert_eq!(begin(0, 0), BEGUN);
            let (got, last) = run_exports(&words, chunk);
            assert_eq!(last, DONE, "chunk {chunk}");
            assert_eq!(got, reference, "chunk {chunk}");
            assert_eq!(step(), DONE, "DONE again after DONE");
            assert_eq!(export_hash(), reference[59].0, "the last tick's hash stays");
        }
    }

    #[test]
    fn h3_each_call_before_begin_at_need_more_after_done_and_after_failed() {
        // Before any begin: nothing to step, no hash, no dump; feed does nothing.
        feed(1);
        assert_eq!(step(), FAILED);
        assert_eq!(
            (hash_lo(), hash_hi(), dump_len(), dump_word(0)),
            (0, 0, 0, 0)
        );
        // Begun, nothing fed (the word above went nowhere): NEED_MORE, and still no world.
        assert_eq!(begin(0, 0), BEGUN);
        assert_eq!(step(), NEED_MORE);
        let words = le_words(&short().encode());
        for w in &words[..10] {
            feed(*w);
        }
        assert_eq!(step(), NEED_MORE, "10 of the header's 15 words");
        assert_eq!((hash_lo(), hash_hi(), dump_len()), (0, 0, 0));
        // The rest: every tick, then DONE, and DONE again.
        for w in &words[10..] {
            feed(*w);
        }
        let mut stepped = 0;
        loop {
            match step() {
                STEPPED => stepped += 1,
                DONE => break,
                other => panic!("status {other}"),
            }
        }
        assert_eq!(stepped, 60);
        assert_eq!(step(), DONE);
        // A header that is no recording: FAILED, again FAILED, and more words change nothing; begin starts afresh.
        assert_eq!(begin(0, 0), BEGUN);
        assert_eq!(dump_len(), 0, "begin empties the kept dump");
        for _ in 0..15 {
            feed(0xdead_beef);
        }
        assert_eq!(step(), FAILED);
        assert_eq!(step(), FAILED);
        for w in &words {
            feed(*w);
        }
        assert_eq!(step(), FAILED, "after FAILED, FAILED until the next begin");
        assert_eq!(begin(0, 0), BEGUN);
        for w in &words {
            feed(*w);
        }
        assert_eq!(step(), STEPPED);
    }

    #[test]
    fn h3_dump_word_past_dump_len_is_0_and_dump_len_keeps_one_dump() {
        assert_eq!(begin(0, 0), BEGUN);
        for w in header_only(&Scene::targets()) {
            feed(w);
        }
        assert_eq!(step(), DONE);
        let n = dump_len();
        assert_eq!(n, 340, "TC's initial dump: D-7's 340 words");
        assert_ne!(dump_word(n - 1), 0xffff_ffff);
        assert_eq!(
            (dump_word(n), dump_word(n + 1), dump_word(u32::MAX)),
            (0, 0, 0)
        );
        let first = World::new(&Scene::targets(), 1, 0).dump();
        assert_eq!(export_dump(), first.words());
    }

    #[test]
    fn h3_begin_refuses_an_index_outside_world_bites_and_changes_nothing() {
        let words = le_words(&short().encode());
        assert_eq!(begin(0, 0), BEGUN);
        for w in &words {
            feed(*w);
        }
        assert_eq!(step(), STEPPED);
        let before = export_hash();
        #[cfg(feature = "bites")]
        let refused: Vec<u32> = vec![u32::try_from(World::BITES.len()).unwrap() + 1, 99, u32::MAX];
        #[cfg(not(feature = "bites"))]
        let refused: Vec<u32> = vec![1, 2, 3, 4, 5, u32::MAX];
        for k in refused {
            assert_eq!(begin(k, 0), REFUSED, "bite {k}");
            assert_eq!(
                export_hash(),
                before,
                "a refused begin keeps the replay begun before"
            );
        }
        assert_eq!(step(), STEPPED, "the replay goes on");
    }

    #[cfg(feature = "bites")]
    #[test]
    fn h3_every_index_of_world_bites_begins_in_the_bites_build() {
        for k in 1..=World::BITES.len() {
            assert_eq!(begin(u32::try_from(k).unwrap(), 0), BEGUN, "bite {k}");
        }
    }

    #[cfg(feature = "bites")]
    #[test]
    fn h3_an_armed_hash_map_index_matches_a_reference_replayer_less_the_folded_words() {
        let id = "PM-0004-21/hash-map";
        let k = World::BITES
            .iter()
            .position(|b| *b == id)
            .expect("World plants it")
            + 1;
        let rec = short();
        let arm = rec.first_tick.wrapping_add(30);
        let words = le_words(&rec.encode());
        let fold = "bite[0].hash_map_order";
        let mut reference = Replayer::with_bite(id, arm).expect("a bite World plants");
        for w in &words {
            reference.feed(*w);
        }
        assert_eq!(begin(u32::try_from(k).unwrap(), arm), BEGUN);
        for w in &words {
            feed(*w);
        }
        let mut armed_ticks = 0;
        loop {
            let (a, b) = (reference.step(), step());
            let Step::Stepped(tick) = a else {
                assert_eq!((a, b), (Step::Done, DONE));
                break;
            };
            assert_eq!(b, STEPPED, "tick {tick}");
            let ours = STATE.with_borrow(|s| {
                s.replayer
                    .as_ref()
                    .and_then(Replayer::world)
                    .map(World::bite_order_digest)
            });
            let theirs = reference.world().map(World::bite_order_digest);
            // `bite_order_digest` is None before the arming tick and Some from it, on both.
            let from_arm = tick.wrapping_sub(rec.first_tick) >= 30;
            assert_eq!(
                ours.flatten().is_some(),
                from_arm,
                "the export's, tick {tick}"
            );
            assert_eq!(
                theirs.flatten().is_some(),
                from_arm,
                "the reference's, tick {tick}"
            );
            armed_ticks += u32::from(from_arm);
            // The two HashMaps take different RandomState keys: the dumps are compared less the folded words.
            let r = reference.dump().expect("a world");
            let keep: Vec<usize> = (0..r.words().len())
                .filter(|&i| r.field(i).as_deref() != Some(fold))
                .collect();
            assert_eq!(r.words().len() - keep.len(), if from_arm { 2 } else { 0 });
            let mine = export_dump();
            assert_eq!(mine.len(), r.words().len(), "tick {tick}");
            let less = |w: &[u32]| keep.iter().map(|&i| w[i]).collect::<Vec<u32>>();
            assert_eq!(less(&mine), less(r.words()), "tick {tick}");
        }
        assert_eq!(armed_ticks, 30);
    }
}
