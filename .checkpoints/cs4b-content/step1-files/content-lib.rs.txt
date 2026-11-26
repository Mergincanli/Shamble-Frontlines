//! The game's content: the files under `content/`, read by one loader of our own (ADR 0005; Mustafa's
//! CONTENT-SOURCE (c), 2026-11-26: "all our own. Keep the reader to what the content files need and refuse everything
//! else, duplicate keys included"). This crate has no dependency, and the simulation depends on it.
//!
//! Nothing here may panic on bytes: every path returns a `Result`, and the lints below hold that from the first line.

#![forbid(unsafe_code)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::unreachable,
    clippy::todo,
    clippy::arithmetic_side_effects
)]
