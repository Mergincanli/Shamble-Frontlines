//! The content loader at its caps and under a bounded fuzz run (content item 2; PM-0005-4). Mustafa's words for
//! the reader (2026-11-26): "Fuzz it before the first value rests on it." No content file exists yet when this runs.
//!
//! The run enters through the loader's own two entries (`load` for a bundle, `load_file` for one file, as the tool
//! that checks the files under `content/` will call it). Nine inputs in ten are a valid bundle or file changed the
//! way content gets broken (`devtools::content_proofs::fuzz_input`); one in ten is random. Every input must leave no
//! panic and keep the loader's promises (`fuzz_judge`); each stage of the loader must settle its share of the
//! verdicts, or the run did not reach it; the slowest input is named. Every map of an input the loader takes goes on
//! into the static world's builder (`devtools::world_proofs::fuzz_worlds`): no panic there either, no refusal but a
//! tree's depth, and a world that answers a ray onto each of its shapes as a search of every shape does (4b-2's
//! step 10; the worlds built are counted, so a run that reached none fails). A failure prints the seed, the input's number
//! and its bytes, so it can be made again. The full count runs where debug assertions are off (`cargo test
//! --release`, which keeps overflow checks on); a debug run takes fewer, and each test's name says which it ran.
//! `SHAMBLE_FUZZ_SEED` runs another seed by hand.
//!
//! A test binary of its own, because a program has one global allocator: this one counts the bytes a thread holds,
//! which is what the caps' memory ceilings are held against.

// A global allocator cannot be written without `unsafe`; the workspace denies it everywhere else, and this file is
// a test that ships nowhere.
#![allow(unsafe_code)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::panic::catch_unwind;
use std::time::{Duration, Instant};

use content::{load, load_file};
use devtools::content_proofs::{LIMIT_CASES, STAGES, fuzz_input, fuzz_judge, limit_case};
use devtools::world_proofs::{WORLD_VERDICTS, fuzz_worlds};

thread_local! {
    /// The bytes this thread holds now, and the most it has held since `peak` last started. A constant start
    /// value: the counter itself never allocates.
    static HELD: Cell<(usize, usize)> = const { Cell::new((0, 0)) };
}

fn count(taken: usize, given_back: usize) {
    // A thread that is ending may no longer have the counter; such a call is not a test's.
    let _ = HELD.try_with(|held| {
        let (now, most) = held.get();
        let now = now.saturating_add(taken).saturating_sub(given_back);
        held.set((now, most.max(now)));
    });
}

struct Counting;

// SAFETY: every call is passed to the system allocator unchanged, with the caller's own arguments, so its contract
// is the system allocator's; the only thing added is a thread-local pair of numbers. `count` holds what a global
// allocator must: it never unwinds (it reads and sets a `Cell` through `try_with` and its sums saturate) and it never
// allocates (the thread-local is constant-initialised, `Copy`, and has no destructor).
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count(layout.size(), 0);
        // SAFETY: the caller's contract for `alloc`, passed on.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count(layout.size(), 0);
        // SAFETY: the caller's contract for `alloc_zeroed`, passed on.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        count(new_size, layout.size());
        // SAFETY: the caller's contract for `realloc`, passed on.
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        count(0, layout.size());
        // SAFETY: the caller's contract for `dealloc`, passed on.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// `work`'s result, and the most bytes it held at once above what the thread held when it started.
fn peak<T>(work: impl FnOnce() -> T) -> (T, usize) {
    let start = HELD.get().0;
    HELD.set((start, start));
    let result = work();
    (result, HELD.get().1.saturating_sub(start))
}

#[test]
fn the_counting_allocator_sees_what_is_held_at_once() {
    let ((), held) = peak(|| {
        drop(vec![0_u8; 3_000]);
        drop(vec![0_u8; 5_000]);
    });
    assert!((5_000..8_000).contains(&held), "{held}");
}

/// What a file's tree may hold beyond the text it was read from, and how long one megabyte may take: far above
/// what was measured (the printed line of each case), so that a slow machine does not fail. The time is a backstop
/// against a loader that hangs: it is too wide to see a search that walks every pair of keys (1.8 s under a 20 s
/// limit, planted by test-engineer), which the test of the two key shapes below holds.
const TREE_BYTES: usize = 4 << 20;
const PER_MEBIBYTE: Duration = Duration::from_secs(if cfg!(debug_assertions) { 20 } else { 2 });

#[test]
fn at_its_caps_the_loader_holds_memory_in_step_with_the_input_and_answers_in_time() {
    assert!(
        limit_case(LIMIT_CASES.len()).is_none(),
        "a case the list does not name"
    );
    for (at, name) in LIMIT_CASES.into_iter().enumerate() {
        let case = limit_case(at).expect("every name of the list has its case");
        let started = Instant::now();
        let (verdict, held) = peak(|| match &case.path {
            Some(path) => load_file(path, &case.bytes).map(drop),
            None => load(&case.bytes).map(drop),
        });
        let took = started.elapsed();
        let bytes = case.bytes.len();
        println!("{name}: {bytes} bytes in, at most {held} bytes held, {took:?}");
        assert_eq!(
            verdict.map_err(|error| error.record()),
            Err(case.expect),
            "{name}"
        );
        // A bundle's tree holds each file's text once; a file's tree is bounded by the node cap.
        assert!(
            held <= bytes + TREE_BYTES,
            "{name}: {held} bytes held for {bytes}"
        );
        let megabytes = u32::try_from(1 + (bytes >> 20)).expect("a few megabytes");
        assert!(took < PER_MEBIBYTE * megabytes, "{name}: {took:?}");
    }
}

/// The search for a key that is there twice is a sort, never a walk of every pair of keys (the test list's point
/// on its cost). An object of 16,383 different keys is where a walk of every pair has the most to do (134 million
/// compares); an object of one key 16,383 times is where it stops at the second key. With the sort the first takes
/// about 2.4 times the second; with a pairwise search planted, 120 times (test-engineer's two runs). The best of
/// three runs of each, so that one slow moment of the machine decides nothing.
#[test]
fn the_search_for_a_key_that_is_there_twice_does_not_walk_every_pair() {
    let best = |name: &str| {
        let at = LIMIT_CASES.iter().position(|case| *case == name);
        let case = at.and_then(limit_case).expect("a limit case of that name");
        let path = case.path.as_deref().expect("one file");
        let once = |_| {
            let started = Instant::now();
            drop(std::hint::black_box(load_file(path, &case.bytes)));
            started.elapsed()
        };
        (0..3).map(once).min().expect("three runs")
    };
    let different = best("an object of the most keys a document holds");
    let same = best("one key 16,383 times");
    println!("16,383 different keys: {different:?}; one key 16,383 times: {same:?}");
    assert!(
        different < same * 20,
        "16,383 different keys took {different:?}, more than twenty times the {same:?} of one key 16,383 times"
    );
}

/// The share of the verdicts each stage must settle, in ten-thousandths: about half of what the run gave when it was
/// written (its printed histogram: 1,867 taken, 2,131, 10, 1,508, 3,306, 384, 601, 169 and 20 in 10,000). A stage
/// under its share was not reached as it was meant to be. The byte caps settle none: no input here is a megabyte
/// long, and the caps are held by the classes of broken files and by the test above. The map's own rules are a
/// stage of their own (test-engineer): counted with a file's name against its id, which a fixed rename gives, a
/// run with no map in it passed.
const SHARES: [u64; STAGES.len()] = [900, 0, 1_000, 5, 700, 1_600, 150, 300, 80, 10];

/// The worlds built in 10,000 inputs, at the least: about half of what the run gave when the worlds joined it (its
/// printed line). No share for a tree too deep: mutation does not reach one (the chain of cubes among the hostile
/// maps of `tests/static_world.rs` does).
const WORLDS_BUILT: u64 = 250;

type Tally = (
    [u64; STAGES.len()],
    [u64; WORLD_VERDICTS.len()],
    (Duration, u64, usize),
);

/// One worker's inputs, judged: how many verdicts each stage settled, what the world's builder made of the maps
/// that were taken, and the slowest input (its time, its number, its length). The first input that leaves a panic
/// or breaks a promise ends the run, named.
fn judge_all(seed: u64, indices: impl Iterator<Item = u64>) -> Tally {
    let (mut stages, mut worlds, mut slowest) = (
        [0; STAGES.len()],
        [0; WORLD_VERDICTS.len()],
        (Duration::ZERO, 0, 0),
    );
    for index in indices {
        let (path, bytes) = fuzz_input(seed, index);
        let started = Instant::now();
        let judged = catch_unwind(|| {
            let stage = fuzz_judge(path.as_deref(), &bytes)?;
            fuzz_worlds(path.as_deref(), &bytes).map(|built| (stage, built))
        });
        let took = started.elapsed();
        let why = match judged {
            Ok(Ok((stage, built))) => {
                stages[stage] += 1;
                for (sum, more) in worlds.iter_mut().zip(built) {
                    *sum += more;
                }
                slowest = slowest.max((took, index, bytes.len()));
                continue;
            }
            Ok(Err(why)) => why,
            Err(_) => "the loader or the world's builder panicked".to_owned(),
        };
        let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        panic!(
            "seed {seed}, input {index}, path {path:?}: {why}\nits {} bytes: {hex}",
            bytes.len()
        );
    }
    (stages, worlds, slowest)
}

fn run(inputs: u64) {
    let seed = std::env::var("SHAMBLE_FUZZ_SEED").map_or(4, |seed| seed.parse().expect("a seed"));
    // Each input is made from the seed and its own number alone, so the workers share nothing: worker `w` of `n`
    // takes the inputs `w`, `w + n`, `w + 2n`, and the tallies add up to the same run on any machine.
    let workers = std::thread::available_parallelism().map_or(1, |count| count.get().min(8));
    let tallies: Vec<Tally> = std::thread::scope(|scope| {
        let work = |worker: u64| move || judge_all(seed, (worker..inputs).step_by(workers));
        let started: Vec<_> = (0..workers as u64)
            .map(|worker| scope.spawn(work(worker)))
            .collect();
        let joined = started.into_iter().map(|worker| worker.join());
        joined.map(|tally| tally.expect("a worker's run")).collect()
    });
    let (took, index, bytes) = tallies
        .iter()
        .map(|tally| tally.2)
        .max()
        .unwrap_or_default();
    // By the wall clock with the workers side by side: a reading that names an input to look at, not a measure.
    println!(
        "seed {seed}, {inputs} inputs; the slowest by the clock: input {index}, {bytes} bytes, {took:?}"
    );
    for (stage, (name, share)) in STAGES.iter().zip(SHARES).enumerate() {
        let count: u64 = tallies.iter().map(|tally| tally.0[stage]).sum();
        let had = count * 10_000 / inputs;
        println!("  {name}: {count} ({had} in 10,000; at least {share})");
        assert!(
            had >= share,
            "{name} settled {had} in 10,000 verdicts, under its {share}"
        );
    }
    let worlds = |verdict: usize| tallies.iter().map(|tally| tally.1[verdict]).sum::<u64>();
    let built = worlds(0) * 10_000 / inputs;
    println!(
        "  maps sent on to the world's builder: {} gave {}, {} gave {} ({built} built in 10,000 inputs; at least \
         {WORLDS_BUILT})",
        worlds(0),
        WORLD_VERDICTS[0],
        worlds(1),
        WORLD_VERDICTS[1]
    );
    assert!(
        built >= WORLDS_BUILT,
        "{built} worlds built in 10,000 inputs, under {WORLDS_BUILT}"
    );
}

#[cfg(not(debug_assertions))]
#[test]
fn one_million_inputs_leave_no_panic_and_every_stage_settles_its_share() {
    run(1_000_000);
}

#[cfg(debug_assertions)]
#[test]
fn fifty_thousand_inputs_leave_no_panic_and_every_stage_settles_its_share() {
    run(50_000);
}
