//! The replay runner, the hash comparer, the tick bench, the legacy-golden reader and the headless protocol
//! clients (ADR 0002 parts 3 and 8). So far: the rule tests' runners, and the recorded match (its file format, the
//! replay runner, the comparer and the scripts the recordings are made from); the runners also build as the
//! test-only WebAssembly module. Nothing here is part of what players get.

pub mod compare;
#[cfg(not(target_arch = "wasm32"))]
pub mod content_files;
pub mod content_proofs;
pub mod files;
pub mod recording;
pub mod replay;
pub mod rules;
pub mod scripts;
pub mod sha256;
pub mod time_report;
pub mod wasm;
