//! The rule tests' runners: pure functions over bytes, with no file and no clock, so each runs natively (the tests in
//! `tests/`) and in WebAssembly (the test-only module) on the same inputs.

pub mod f32_ops;
pub mod stream;
