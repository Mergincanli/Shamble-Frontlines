//! The simulation: state, phases, tick and the navigation runtime (ADR 0002 part 3; ADR 0004). It depends on
//! `content` and on the collision library only, with no transport, no async runtime and no clock. So far it holds the world (a tick counter, the
//! one player's applied command and the match seed), its step, its state hash and the generator of its random
//! streams (foundation step 2.5).

// The simulation's native build runs on the instructions every x86-64 processor has. With the fused multiply-add or
// the wide vector set switched on for a whole build, code in the collision library's tree would compile to other
// instructions than it does in WebAssembly and on the machine beside it (Mustafa's MUL-ADD (a), 2026-11-24; ADR 0004
// part 7). Nothing of ours turns them on: this refuses a build where a flag from outside did, whatever passed it
// (a variable, a cargo config file). The native tests of the crates that do not build this one assert the same
// (`tests/native_target.rs`); the WebAssembly build's vector instructions are the built-file check's to refuse (it
// reads the file).
#[cfg(any(target_feature = "fma", target_feature = "avx2"))]
compile_error!(
    "the simulation is built without the fma and avx2 target features: unset RUSTFLAGS, target-cpu and target-feature"
);

pub mod aim;
pub mod collision;
pub mod fmath;
pub mod ids;
pub mod rng;
pub mod rules;
pub mod soldiers;
pub mod state_hash;
pub mod time;
pub mod vec;
pub mod world;

pub use rng::ChaCha8;
pub use world::{
    BUTTONS, MatchSetup, PlayerCommand, SEED_BYTES, SHARED_STATE_BYTES, StateSink, TICK_HZ, World,
};

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    /// PM-0002-53's planted counter: a `u16` one step past its end panics. It passes only while overflow checks
    /// are on, so `cargo test --release` proves the release profile's setting too (ADR 0002 part 4).
    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn overflow_checks_are_on() {
        let counter = black_box(u16::MAX);
        let _ = black_box(counter + 1);
    }

    /// The same counter where wrapping is meant and written: it wraps to the same value in every build.
    #[test]
    fn a_counter_told_to_wrap_wraps() {
        assert_eq!(black_box(u16::MAX).wrapping_add(black_box(1)), 0);
    }
}
