//! Message types, the codec, decoders that never panic and the loopback transport backend (ADR 0002 part 3;
//! ADR 0001). So far it holds the first message types (`messages`, foundation step 2.4); the codec enters with the
//! protocol's first messages.

pub mod messages;
#[cfg(test)]
mod ts_export;

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
