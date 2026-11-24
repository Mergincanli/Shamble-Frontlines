//! The replay runner, the hash comparer, the tick bench, the legacy-golden reader and the headless protocol
//! clients (ADR 0002 parts 3 and 8). So far: the rule tests' runners, which also build as the test-only WebAssembly
//! module. Nothing here is part of what players get.

pub mod rules;
pub mod wasm;
