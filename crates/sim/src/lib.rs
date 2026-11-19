//! The simulation: state, phases, tick and the navigation runtime (ADR 0002 part 3; ADR 0004). It depends on
//! `content` only, with no transport, no async runtime and no clock. So far it holds the world (a tick counter and
//! the match seed), its step and its state hash (foundation step 2.5, first half).

pub mod state_hash;
pub mod world;

pub use world::{SEED_BYTES, TICK_HZ, World};

#[cfg(test)]
mod tests {
    /// Passes only while overflow checks are on, so `cargo test --release` proves the release profile's setting
    /// (ADR 0002 part 4).
    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn overflow_checks_are_on() {
        let max = std::hint::black_box(u8::MAX);
        let _ = max + 1;
    }
}
