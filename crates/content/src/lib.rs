//! The game's content: the files under `content/`, read by one loader of our own (ADR 0005; Mustafa's
//! CONTENT-SOURCE (c), 2026-11-26: "all our own. Keep the reader to what the content files need and refuse everything
//! else, duplicate keys included"). This crate has no dependency, and the simulation depends on it.
//!
//! - [`bundle`]: the loader ([`load`]) and the bundle's own rules; [`load_file`] reads one file as the loader does.
//! - [`json`]: bytes to a tree, in the one grammar a content file has. [`write`]: the one canonical writer.
//! - [`number`]: how a number is read and written. [`read`]: the typed read, field by field, with ranges.
//! - [`movement`]: the movement table. [`error`]: what a refusal says.
//!
//! Nothing here may panic on bytes: every path returns a `Result`, and the lints below hold that outside the tests.

#![forbid(unsafe_code)]
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::arithmetic_side_effects
    )
)]

pub mod bundle;
pub mod error;
pub mod json;
pub mod map;
pub mod movement;
pub mod number;
pub mod read;
pub mod write;

pub use bundle::{ContentSet, Document, load, load_file};
pub use error::{Error, Kind};
