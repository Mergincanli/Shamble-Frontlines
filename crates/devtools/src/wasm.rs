//! The test-only WebAssembly module's exports (ADR 0002 part 8): the rule tests' runners, a planted counter, and,
//! linked in from `sim-wasm`, the panic hook as players get it. The development build makes this module beside the
//! players' Wasm, into a folder of its own; no page and no Worker loads it, and no production build makes it.

use std::hint::black_box;

use wasm_bindgen::prelude::*;

// Linked whole, so that its start function installs the real hook and `panic_buffer` is an export of this module.
use sim_wasm as _;

use crate::rules::{f32_ops, stream};

/// The float vectors (PM-0004-22): an empty text when every operation gives the exact reference's results, or what
/// differs.
#[wasm_bindgen]
#[must_use]
pub fn f32_vectors(inputs: &[u8], expected: &str) -> String {
    f32_ops::check(inputs, expected).err().unwrap_or_default()
}

/// The random stream's vectors (PM-0004-27): an empty text when the generator gives the file's words, or the first
/// draw that differs.
#[wasm_bindgen]
#[must_use]
pub fn chacha8_vectors(file: &[u8]) -> String {
    stream::check(file).err().unwrap_or_default()
}

/// PM-0002-53's planted counter: a plain `+`, which panics past the counter's end in every build.
#[wasm_bindgen]
#[must_use]
pub fn plant_u16_add(counter: u16, step: u16) -> u16 {
    black_box(counter) + black_box(step)
}

/// The same counter where wrapping is meant and written.
#[wasm_bindgen]
#[must_use]
pub fn plant_u16_wrapping_add(counter: u16, step: u16) -> u16 {
    black_box(counter).wrapping_add(black_box(step))
}
