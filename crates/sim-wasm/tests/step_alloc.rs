//! "Nothing is allocated per tick" for local play's one step, counted and not inferred: a global allocator that
//! counts, around the system's. The step runs the whole exchange (the endpoint's command through the codec and the
//! loopback, the room's checks and queue, the world's step, the snapshot back, the block), so this is the count
//! for all of them together.
//!
//! A test binary of its own, because a program has one global allocator; the count is kept per thread, because the
//! test harness allocates on its own threads while a test runs.

// A global allocator cannot be written without `unsafe`; the workspace denies it everywhere else, and this file is
// a test that ships nowhere.
#![allow(unsafe_code)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use client_endpoint::post::Post;
use sim::collision::StaticWorld;
use sim::{BUTTONS, MatchSetup, SEED_BYTES};
use sim_wasm::local_match::{LocalMatch, loopback};

thread_local! {
    /// Calls on this thread that took memory: `alloc`, `alloc_zeroed` and `realloc`. A constant start value, so the
    /// counter itself never allocates.
    static TAKEN: Cell<u64> = const { Cell::new(0) };
}

fn count() {
    // A thread that is ending may no longer have the counter; such a call is not this test's.
    let _ = TAKEN.try_with(|taken| taken.set(taken.get().wrapping_add(1)));
}

struct Counting;

// SAFETY: every call is passed to the system allocator unchanged, with the caller's own arguments, so its contract
// is the system allocator's; the only thing added is a count in a thread-local number.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count();
        // SAFETY: the caller's contract for `alloc`, passed on.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count();
        // SAFETY: the caller's contract for `alloc_zeroed`, passed on.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        count();
        // SAFETY: the caller's contract for `realloc`, passed on.
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: the caller's contract for `dealloc`, passed on.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

#[test]
fn a_tick_of_the_local_match_takes_no_memory_once_the_match_is_warm() {
    // The control: the counter sees an allocation when there is one.
    let before = TAKEN.get();
    drop(std::hint::black_box(Vec::<u8>::with_capacity(64)));
    assert_eq!(
        TAKEN.get() - before,
        1,
        "the counting allocator does not see an allocation"
    );

    let seed = [7; SEED_BYTES];
    let setup = MatchSetup {
        seed,
        start_tick: 0,
    };
    // The match is made on the content crate's test map, as a local match always holds its map's static world.
    let bundle = content::bundle::test_bundle();
    let set = content::load(bundle.as_bytes()).expect("the test bundle loads");
    let world =
        StaticWorld::build(set.map("test-scene").expect("the test map")).expect("it builds");
    let mut local =
        LocalMatch::new(&setup, 0, 0, loopback(), world).expect("no padding is allowed");
    let mut presses = [0u8; BUTTONS];
    let mut step = |local: &mut LocalMatch, number: u32| {
        // A press every tick, round the buttons, and every bit pattern of the held buttons in turn.
        let button = number as usize % BUTTONS;
        presses[button] = presses[button].wrapping_add(1);
        let buttons = number as u16;
        local.set_post(Post {
            number,
            buttons,
            presses,
        });
        local.step(f64::from(number));
    };
    // The join and the first commands size what is reused: the token and the mode id of the join exchange, and
    // the room's two lists.
    let before = TAKEN.get();
    for number in 0..3 {
        step(&mut local, number);
    }
    let warm_up = TAKEN.get() - before;
    assert!(
        (1..=8).contains(&warm_up),
        "the first three steps took memory {warm_up} times"
    );

    // Ten minutes of ticks at 30 a second: not one more. The shared hash is read after every tick, as a replay
    // reads it: it takes no memory either, from its first call on (it has no buffer).
    let before = TAKEN.get();
    let mut hashes = 0u64;
    for number in 3..18_003 {
        step(&mut local, number);
        hashes ^= local.shared_hash();
    }
    assert_eq!(
        TAKEN.get() - before,
        0,
        "a later step or its hash took memory"
    );
    assert_ne!(std::hint::black_box(hashes), 0);
    assert_eq!(local.room().counters().commands_run, 18_002);
    assert_eq!(local.room().counters().refused(), 0);
}
