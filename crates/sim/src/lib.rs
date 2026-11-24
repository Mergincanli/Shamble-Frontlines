//! The simulation: state, phases, tick and the navigation runtime (ADR 0002 part 3; ADR 0004). It depends on
//! `content` only, with no transport, no async runtime and no clock. So far it holds the world (a tick counter and
//! the match seed), its step and its state hash (foundation step 2.5, first half).

pub mod state_hash;
pub mod world;

pub use world::{SEED_BYTES, TICK_HZ, World};

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
