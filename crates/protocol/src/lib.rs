//! Message types, the codec, decoders that never panic and the loopback transport backend (ADR 0002 part 3;
//! ADR 0001): the first messages (`messages`), their wire form (`codec` over `wire`), the compares for counters
//! that wrap (`wrap`), and the transport interface (`transport`) with its first backend, the `loopback`.
//!
//! Nothing here panics on bytes from a link, allocates per message once its buffers are warm, reads a clock or
//! uses a hash map. The lints below turn the first rule into errors wherever a lint step runs; until CI has one,
//! the tests carry the proof.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::arithmetic_side_effects
    )
)]

pub mod codec;
pub mod loopback;
pub mod messages;
pub mod transport;
#[cfg(test)]
mod ts_export;
pub mod wire;
pub mod wrap;

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
