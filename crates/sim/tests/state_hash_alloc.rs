//! "There is no buffer, so it takes no memory" (`World::state_hash`), counted and not inferred: a global allocator
//! that counts, around the system's. The same count holds a ray cast into the static world, on the deepest tree a
//! world may have (`StaticWorld::cast_ray`: "It allocates nothing").
//!
//! It is a test binary of its own because a program has one global allocator, and the count is kept per thread
//! because the test harness allocates on its own threads while a test runs.

// A global allocator cannot be written without `unsafe`; the workspace denies it everywhere else, and this file is
// a test that ships nowhere.
#![allow(unsafe_code)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::hint::black_box;

use content::map::{Bounds, Form, MapDocument, Shape};
use sim::aim::AimCm;
use sim::collision::{MAX_TREE_DEPTH, StaticWorld};
use sim::rules::Rules;
use sim::soldiers::spawn_points;
use sim::vec::V3;
use sim::{BUTTONS, MatchSetup, PlayerCommand, SEED_BYTES, World};

thread_local! {
    /// Calls on this thread that took memory: `alloc`, `alloc_zeroed` and `realloc`. A constant start value, so the
    /// counter itself never allocates and has nothing to tear down.
    static TAKEN: Cell<u64> = const { Cell::new(0) };
}

fn count() {
    // A thread that is ending may no longer have the counter; such a call is not this test's.
    let _ = TAKEN.try_with(|taken| taken.set(taken.get().wrapping_add(1)));
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
fn the_state_hashes_take_no_memory() {
    // A match on the test bundle: his soldier in slot 0, so the hash reads a row that holds a soldier and 47 that
    // hold none.
    let set =
        content::load(content::bundle::test_bundle().as_bytes()).expect("the test bundle loads");
    let rules = Rules::take(&set.movements()[0]).expect("its movement table plays");
    let spawns = spawn_points(set.map("test-scene").expect("its map"));
    let setup = MatchSetup {
        seed: [7; SEED_BYTES],
        start_tick: 0,
    };
    let mut world = World::new(&setup, &rules, &spawns);

    // The control: the counter sees an allocation when there is one.
    let before = taken();
    let seen = black_box(Vec::<u8>::with_capacity(64));
    assert_eq!(
        taken() - before,
        1,
        "the counting allocator does not see an allocation"
    );
    drop(seen);

    // The hash is fed field by field with no buffer: not even the first call takes memory.
    let before = taken();
    let first = world.state_hash();
    assert_eq!(taken() - before, 0, "the first call took memory");

    // Ten minutes of ticks at 30 a second, each stepped with a command (its move and its aim into his row) and hashed
    // twice: not one more.
    let before = taken();
    let mut last = first;
    for tick in 0..18_000_u16 {
        let [low, _] = tick.to_le_bytes();
        world.authority_step(&[PlayerCommand {
            buttons: tick,
            presses: [low; BUTTONS],
            move_x: low as i8 / 2,
            move_y: 50,
            aim: AimCm {
                x: i16::from(low),
                y: tick,
                z: -7,
            },
        }]);
        black_box(world.shared_hash());
        last = black_box(world.state_hash());
    }
    assert_eq!(taken() - before, 0, "a later call took memory");
    assert_ne!(first, last);
    assert_eq!(world.state_hash(), last);
}

/// The low corners, in millimetres, of the first 31 cubes of the chain in `collision.rs`'s own tests: one-centimetre
/// cubes whose tree is as deep as a world's tree may be.
#[rustfmt::skip]
const CHAIN: [[i32; 3]; 31] = [[-5, 327665, -5], [-5, 327666, -5], [-5, 327667, -5], [-5, 327663, -5], [-5, 327671, -5], [-14, 327665, -5], [4, 327665, -5], [-5, 327665, -14], [-5, 327665, 4], [-5, 327648, -5], [-5, 327746, -5], [-5, 327530, -5], [-162, 327665, -5], [-5, 327665, -162], [189, 327665, -5], [-5, 327665, 189], [-5, 328456, -5], [-5, 326420, -5], [-1272, 327665, -5], [-5, 327665, -1272], [1924, 327665, -5], [-5, 327665, 1924], [-5, 335079, -5], [-5, 316071, -5], [-11621, 327665, -5], [-5, 327665, -11621], [18096, 327665, -5], [-5, 327665, 18096], [-5, 396773, -5], [-5, 219673, -5], [-108019, 327665, -5]];

fn document(shapes: Vec<Shape>) -> MapDocument {
    let bounds = Bounds {
        min: [-327.0, 0.0, -327.0],
        max: [327.0, 655.0, 327.0],
    };
    MapDocument {
        id: "test".to_owned(),
        bounds,
        spawns: Vec::new(),
        shapes,
        regions: Vec::new(),
    }
}

#[test]
fn a_ray_cast_takes_no_memory_on_the_deepest_tree_and_on_the_fullest_world() {
    let metres = |millimetres: i32| f64::from(millimetres) / 1000.0;
    let cube = |id: String, low: [f64; 3], side: f64| Shape {
        id,
        min: low,
        max: low.map(|value| value + side),
        form: Form::Box,
    };
    let chain = CHAIN
        .iter()
        .enumerate()
        .map(|(at, low)| cube(format!("cube_{at:02}"), low.map(metres), 0.01))
        .collect();
    let deep = StaticWorld::build(&document(chain)).expect("the deepest world");
    assert_eq!(deep.tree_depth(), MAX_TREE_DEPTH);
    // 512 boxes on a grid of 32 by 16, two metres apart.
    let grid = (0..512_u32)
        .map(|at| {
            let low = [f64::from(at % 32) * 2.0, 0.0, f64::from(at / 32) * 2.0];
            cube(format!("box_{at:03}"), low, 1.0)
        })
        .collect();
    let full = StaticWorld::build(&document(grid)).expect("the fullest world");
    assert_eq!(full.solids().len(), 512);

    let before = taken();
    let mut met = 0_u32;
    for step in 0..4_000_u16 {
        let (a, b) = (f32::from(step % 80), f32::from(step / 80));
        // Down onto the grid, level across it, slanted through it; then at the chain from four sides.
        let rays = [
            (V3::new(a * 0.8, 50.0, b * 0.6), V3::new(0.0, -1.0, 0.0)),
            (V3::new(-5.0, 0.5, b * 0.6), V3::new(1.0, 0.0, 0.0)),
            (V3::new(a * 0.8, 40.0, -5.0), V3::new(0.1, -1.0, 0.7)),
        ];
        for (origin, direction) in rays {
            met += u32::from(black_box(full.cast_ray(origin, direction, 500.0)).is_some());
        }
        let around = [
            (V3::new(-0.001, 600.0, -0.001), V3::new(0.0, -1.0, 0.0)),
            (V3::new(-300.0, 327.67, -0.001), V3::new(1.0, 0.0, 0.0)),
            (V3::new(-0.001, 327.67, 300.0), V3::new(0.0, 0.0, -1.0)),
            (V3::new(a - 40.0, 100.0, b - 25.0), V3::new(0.0, 1.0, 0.0)),
        ];
        for (origin, direction) in around {
            met += u32::from(black_box(deep.cast_ray(origin, direction, 1000.0)).is_some());
        }
    }
    assert_eq!(taken() - before, 0, "a ray cast took memory");
    // The rays did meet shapes: a count of none would be a test of nothing.
    assert!(met > 4_000, "only {met} rays met a shape");
}
