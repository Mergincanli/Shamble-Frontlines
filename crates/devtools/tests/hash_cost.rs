//! What the state hash costs per tick (PM-0004-21's report): a reading, printed with `--nocapture`, and no pass
//! mark on time. It times the hash's function on the canonical bytes of today's state (22 shared bytes, 54 in all)
//! and on states of the sizes to come (4 KiB, 16 KiB), and the whole call on today's world. What it does assert
//! holds on any machine: the function gives the same digest every time.

use std::time::Instant;

use sim::state_hash::fnv1a_64;
use sim::{MatchSetup, SEED_BYTES, World};

/// The mean and the 99th percentile of `rounds` timed calls, in nanoseconds.
fn timed(rounds: usize, mut call: impl FnMut() -> u64) -> (f64, u128, u64) {
    let mut nanos = Vec::with_capacity(rounds);
    let mut folded = 0;
    for _ in 0..rounds {
        let start = Instant::now();
        folded ^= std::hint::black_box(call());
        nanos.push(start.elapsed().as_nanos());
    }
    nanos.sort_unstable();
    let mean = nanos.iter().sum::<u128>() as f64 / rounds as f64;
    (mean, nanos[rounds * 99 / 100], folded)
}

#[test]
fn the_state_hash_s_cost_per_tick_is_read_and_printed() {
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    println!("\nstate hash cost, native, {profile} build (a reading):");
    for length in [22usize, 54, 4_096, 16_384] {
        let bytes: Vec<u8> = (0..length).map(|at| (at * 31 % 251) as u8).collect();
        let digest = fnv1a_64(&bytes);
        let (mean, p99, _) = timed(20_000, || fnv1a_64(std::hint::black_box(&bytes)));
        println!("  fnv1a_64 over {length:>6} bytes: mean {mean:>9.1} ns, p99 {p99:>7} ns");
        assert_eq!(fnv1a_64(&bytes), digest);
    }
    let mut world = World::new(&MatchSetup {
        seed: [7; SEED_BYTES],
        start_tick: 0,
    });
    for (name, shared) in [("shared_hash", true), ("state_hash", false)] {
        let (mean, p99, _) = timed(20_000, || {
            world.authority_step(&[]);
            if shared {
                world.shared_hash()
            } else {
                world.state_hash()
            }
        });
        println!(
            "  one step and World::{name} on today's world: mean {mean:>7.1} ns, p99 {p99:>6} ns"
        );
    }
}
