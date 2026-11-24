//! "Passing the same buffer every tick means no allocation after the first call" (`World::state_hash`), counted and
//! not inferred: a global allocator that counts, around the system's. A test of a buffer's address and capacity
//! cannot show it, because an allocator may hand the same block out again after a free.
//!
//! It is a test binary of its own because a program has one global allocator, and the count is kept per thread
//! because the test harness allocates on its own threads while a test runs.

// A global allocator cannot be written without `unsafe`; the workspace denies it everywhere else, and this file is
// a test that ships nowhere.
#![allow(unsafe_code)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::hint::black_box;

use sim::{SEED_BYTES, World};

thread_local! {
    /// Calls on this thread that took memory: `alloc`, `alloc_zeroed` and `realloc`. A constant start value, so the
    /// counter itself never allocates and has nothing to tear down.
    static TAKEN: Cell<u64> = const { Cell::new(0) };
}

fn count() {
    // A thread that is ending may no longer have the counter; such a call is not this test's.
    let _ = TAKEN.try_with(|taken| taken.set(taken.get() + 1));
}

fn taken() -> u64 {
    TAKEN.with(Cell::get)
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
fn the_state_hash_takes_memory_once_and_never_again() {
    let mut world = World::new([7; SEED_BYTES]);
    let mut scratch = Vec::new();

    // The control: the counter sees an allocation when there is one.
    let before = taken();
    let seen = black_box(Vec::<u8>::with_capacity(64));
    assert_eq!(
        taken() - before,
        1,
        "the counting allocator does not see an allocation"
    );
    drop(seen);

    // The first call sizes the buffer: one allocation.
    let before = taken();
    let first = world.state_hash(&mut scratch);
    assert_eq!(
        taken() - before,
        1,
        "the first call sizes the scratch buffer once"
    );

    // Ten minutes of ticks at 30 a second, each stepped and hashed: not one more.
    let before = taken();
    let mut last = first;
    for _ in 0..18_000 {
        world.authority_step();
        last = black_box(world.state_hash(&mut scratch));
    }
    assert_eq!(taken() - before, 0, "a later call took memory");
    assert_ne!(first, last);

    // A buffer that comes back in with other content, or longer, is reused all the same.
    scratch.extend_from_slice(&[0xEE; 200]);
    let before = taken();
    assert_eq!(world.state_hash(&mut scratch), last);
    assert_eq!(taken() - before, 0);
}
