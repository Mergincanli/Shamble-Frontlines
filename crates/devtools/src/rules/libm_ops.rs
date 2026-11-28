//! The maths routines' vectors (PM-0004-22): libm's exponential and sine-and-cosine, as the simulation reaches them
//! (`sim::fmath::exp_of` and `sincos_of`, each handing on the routine's own f64 result), on the committed inputs
//! (`vectors/libm-inputs.bin`, made by `vectors/make_libm_vectors.py`). The expected file holds one FNV-1a 64 a routine
//! over every output's bits in order, under a header naming libm's pinned version and the inputs' byte count and
//! sha256. The native test records it (only with `SHAMBLE_WRITE_LIBM=1`) and every target checks it: natively, and in
//! the test-only module under Node and in each browser engine. libm is not correctly rounded, so its outputs are its
//! own: what this holds is that they are the same bits everywhere (rust-reviewer's block 17, Q1 and Q5; block 18).

use sim::fmath::{exp_of, sincos_of};
use sim::state_hash::fnv1a_64;

use crate::sha256::{hex, sha256};

/// The exponential's inputs, then the sine and cosine's: `make_libm_vectors.py`'s two sets.
pub const EXP_CASES: usize = 33_646;
pub const SINCOS_CASES: usize = 101_017;
/// libm's version as the root manifest pins it (`libm = "=0.2.16"`; `tests/libm_vectors.rs` ties the two): a bump
/// re-records the expected file in a diff a reviewer sees.
pub const LIBM_VERSION: &str = "0.2.16";

/// The expected file's first line for these inputs.
#[must_use]
pub fn header(inputs: &[u8]) -> String {
    format!(
        "shamble-libm-vectors 1 libm ={LIBM_VERSION} {} {}",
        inputs.len(),
        hex(&sha256(inputs))
    )
}

/// The inputs as doubles, the exponential's and then the sine and cosine's, or why they are not the vectors' inputs.
fn doubles(inputs: &[u8]) -> Result<Vec<f64>, String> {
    let want = 8 * (EXP_CASES + SINCOS_CASES);
    if inputs.len() != want {
        return Err(format!(
            "the inputs hold {} bytes, not {want}",
            inputs.len()
        ));
    }
    let (chunks, _) = inputs.as_chunks::<8>();
    let values: Vec<f64> = chunks
        .iter()
        .map(|chunk| f64::from_le_bytes(*chunk))
        .collect();
    if let Some(at) = values.iter().position(|value| value.is_nan()) {
        return Err(format!("input {at} is a NaN"));
    }
    Ok(values)
}

/// The expected file's text for these inputs, worked out on this target: the header, then `exp` and `sincos` with
/// their case counts and digests (the sine and cosine interleaved, as the routine returns them).
///
/// # Errors
/// Inputs of another length, or one that is a NaN.
pub fn expected_text(inputs: &[u8]) -> Result<String, String> {
    let values = doubles(inputs)?;
    let (exp_inputs, sincos_inputs) = values.split_at(EXP_CASES);
    let mut exp_bytes = Vec::with_capacity(8 * EXP_CASES);
    for x in exp_inputs {
        exp_bytes.extend_from_slice(&exp_of(*x).to_bits().to_le_bytes());
    }
    let mut sincos_bytes = Vec::with_capacity(16 * SINCOS_CASES);
    for x in sincos_inputs {
        let (sin, cos) = sincos_of(*x);
        sincos_bytes.extend_from_slice(&sin.to_bits().to_le_bytes());
        sincos_bytes.extend_from_slice(&cos.to_bits().to_le_bytes());
    }
    Ok(format!(
        "{}\nexp {EXP_CASES} {:016x}\nsincos {SINCOS_CASES} {:016x}\n",
        header(inputs),
        fnv1a_64(&exp_bytes),
        fnv1a_64(&sincos_bytes)
    ))
}

/// The committed expected file against this target: nothing when the inputs are the header's (byte count and
/// sha256, checked first, so a changed input file is named as such) and each routine gives the recorded digest; else
/// the first line that differs.
///
/// # Errors
/// The first difference, in plain words.
pub fn check(inputs: &[u8], expected: &str) -> Result<(), String> {
    let first = expected.lines().next().unwrap_or("");
    if first != header(inputs) {
        return Err(format!(
            "the inputs are not the ones the expected file's header names: {} where it has {first}",
            header(inputs)
        ));
    }
    let made = expected_text(inputs)?;
    for (made_line, expected_line) in made.lines().zip(expected.lines()).skip(1) {
        if made_line != expected_line {
            return Err(format!(
                "not the committed outputs: {made_line} where the file has {expected_line}"
            ));
        }
    }
    if made.lines().count() != expected.lines().count() {
        return Err("the expected file has another number of lines".to_owned());
    }
    Ok(())
}
