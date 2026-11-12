//! The simulation: state, phases, tick and the navigation runtime (ADR 0002 part 3; ADR 0004). It depends on
//! `content` only, with no transport, no async runtime and no clock. Empty until its first slice.

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
