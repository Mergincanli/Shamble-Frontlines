//! The test-only WebAssembly module's exports (ADR 0002 part 8): the rule tests' runners, a planted counter, and,
//! linked in from `sim-wasm`, the panic hook as players get it. The development build makes this module beside the
//! players' Wasm, into a folder of its own; no page and no Worker loads it, and no production build makes it.

use std::hint::black_box;

use wasm_bindgen::prelude::*;

// Linked whole, so that its start function installs the real hook and `panic_buffer` is an export of this module.
use sim_wasm as _;

use crate::content_proofs;
use crate::files::vector_text;
use crate::recording::{Recording, hashes_text};
use crate::replay::replay;
use crate::rules::{f32_ops, stream};
use crate::world_proofs;

/// The float vectors (PM-0004-22): an empty text when every operation gives the exact reference's results and
/// `mul_add`'s pinned cases their measured values (MUL-ADD), or what differs.
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

/// Reads a recording for the two calls below; a refusal comes back as a text that starts with `error: `.
fn read_recording(bytes: &[u8]) -> Result<Recording, String> {
    Recording::decode(bytes).map_err(|error| format!("error: {error}"))
}

/// The hashes file this module gives for a recording on `bundle`, the content its header names, both columns: the
/// same runner as the native one, compiled to WebAssembly. The whole-state column is why it lives here and not on
/// the players' file, which gives the shared hash only.
#[wasm_bindgen]
#[must_use]
pub fn replay_hashes(recording: &[u8], bundle: &[u8]) -> String {
    let replayed = read_recording(recording)
        .and_then(|read| replay(&read, bundle).map_err(|error| format!("error: {error}")));
    match replayed {
        Ok(lines) => hashes_text(recording, &lines),
        Err(why) => why,
    }
}

/// The vector file's text for `tick` as this module gives it (`files::vector_text`), the recording replayed on
/// `bundle`: the state as lines, one named field each (the seed is never among them), then the shared state's bytes
/// and their hash. Compared line by line with the native text, it shows which fields two targets disagree on.
#[wasm_bindgen]
#[must_use]
pub fn replay_state_at(recording: &[u8], bundle: &[u8], tick: u32) -> String {
    let text = read_recording(recording).and_then(|read| {
        vector_text(&read, bundle, tick).map_err(|error| format!("error: {error}"))
    });
    text.unwrap_or_else(|why| why)
}

/// The content reader's proofs (content items 1 and 2), each as the native tests run it, each an empty text when it
/// holds or what differs when it does not. The number corpus: the reader's bits and the writer's text against a
/// browser engine's own, number by number.
#[wasm_bindgen]
#[must_use]
pub fn content_corpus(corpus: &str) -> String {
    content_proofs::corpus_check(corpus)
}

/// The tree corpus: the canonical writer's text against the engine's `JSON.stringify` with sorted keys.
#[wasm_bindgen]
#[must_use]
pub fn content_trees(trees: &str) -> String {
    content_proofs::trees_check(trees)
}

/// The classes of broken files and the fuzz run's first inputs, judged here and compared with the committed
/// digests of the native verdicts: record for record, messages included.
#[wasm_bindgen]
#[must_use]
pub fn content_verdicts(expected: &str) -> String {
    content_proofs::verdicts_check(expected)
}

/// The inputs that lean on the loader's caps, each against its verdict; whoever calls it reads the clock and the
/// module's memory.
#[wasm_bindgen]
#[must_use]
pub fn content_limits() -> String {
    content_proofs::limits_check()
}

/// The static world's proof: the trees this module builds and the rays it casts, against the native build's
/// committed lines.
#[wasm_bindgen]
#[must_use]
pub fn static_world(expected: &str) -> String {
    world_proofs::check(expected)
}

/// The state hash's function over `length` bytes, `rounds` times over: for the reading of its cost in WebAssembly
/// (PM-0004-21's report), timed by whoever calls it. It returns the digests folded together.
#[wasm_bindgen]
#[must_use]
pub fn fnv1a_64_rounds(length: u32, rounds: u32) -> u64 {
    // At most 16 MiB, so no length can ask for more memory than a reading needs, and no product can overflow.
    let bytes: Vec<u8> = (0..length.min(1 << 24))
        .map(|at| (at * 31 % 251) as u8)
        .collect();
    (0..rounds).fold(0, |folded, round| {
        folded.rotate_left(1) ^ sim::state_hash::fnv1a_64(black_box(&bytes)) ^ u64::from(round)
    })
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
