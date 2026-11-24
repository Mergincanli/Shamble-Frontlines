//! The float vectors (PM-0004-22): every float operation ADR 0004 part 7 allows, on the fixed inputs of
//! `vectors/f32-inputs.bin`, against the digests an exact reference wrote into `vectors/f32-expected.txt`. The
//! script `vectors/make_f32_vectors.py` makes both files and explains them; the operations here are that script's,
//! in its order. The inputs are read when a test runs, never built in, so no compiler works a result out ahead.

use sim::state_hash::fnv1a_64;

/// Cases in the input file: three `f32` bit patterns each, little-endian.
pub const CASES: usize = 10_000;

/// A float result as bits; a NaN as the one pattern the reference writes: which cases give a NaN must agree on
/// every target, a NaN's own bits need not.
fn f(value: f32) -> u64 {
    u64::from(if value.is_nan() {
        0x7FC0_0000
    } else {
        value.to_bits()
    })
}

/// Two inputs' bits as one 64-bit pattern, the first one low.
fn wide(a: f32, b: f32) -> u64 {
    u64::from(a.to_bits()) | (u64::from(b.to_bits()) << 32)
}

/// An operation: its name, the bytes of one result, and the result as bits (the low bytes count).
pub type Op = (&'static str, usize, fn(f32, f32, f32) -> u64);

#[rustfmt::skip]
pub const OPS: [Op; 38] = [
    ("add", 4, |a, b, _| f(a + b)),
    ("sub", 4, |a, b, _| f(a - b)),
    ("mul", 4, |a, b, _| f(a * b)),
    ("div", 4, |a, b, _| f(a / b)),
    ("neg", 4, |a, _, _| f(-a)),
    ("lt", 1, |a, b, _| u64::from(a < b)),
    ("le", 1, |a, b, _| u64::from(a <= b)),
    ("gt", 1, |a, b, _| u64::from(a > b)),
    ("ge", 1, |a, b, _| u64::from(a >= b)),
    ("eq", 1, |a, b, _| u64::from(a == b)),
    ("ne", 1, |a, b, _| u64::from(a != b)),
    ("total_cmp", 1, |a, b, _| a.total_cmp(&b) as i8 as u8 as u64),
    ("sqrt", 4, |a, _, _| f(a.sqrt())),
    ("sqrt_abs", 4, |a, _, _| f(a.abs().sqrt())),
    ("floor", 4, |a, _, _| f(a.floor())),
    ("ceil", 4, |a, _, _| f(a.ceil())),
    ("round", 4, |a, _, _| f(a.round())),
    ("trunc", 4, |a, _, _| f(a.trunc())),
    ("fract", 4, |a, _, _| f(a.fract())),
    ("mul_add", 4, |a, b, c| f(a.mul_add(b, c))),
    ("abs", 4, |a, _, _| f(a.abs())),
    ("copysign", 4, |a, b, _| f(a.copysign(b))),
    ("to_i8", 1, |a, _, _| a as i8 as u8 as u64),
    ("to_u8", 1, |a, _, _| a as u8 as u64),
    ("to_i16", 2, |a, _, _| a as i16 as u16 as u64),
    ("to_u16", 2, |a, _, _| a as u16 as u64),
    ("to_i32", 4, |a, _, _| a as i32 as u32 as u64),
    ("to_u32", 4, |a, _, _| a as u32 as u64),
    ("to_i64", 8, |a, _, _| a as i64 as u64),
    ("to_u64", 8, |a, _, _| a as u64),
    ("from_i32", 4, |a, _, _| f(a.to_bits() as i32 as f32)),
    ("from_u32", 4, |a, _, _| f(a.to_bits() as f32)),
    ("from_i64", 4, |a, b, _| f(wide(a, b) as i64 as f32)),
    ("from_u64", 4, |a, b, _| f(wide(a, b) as f32)),
    ("to_f64", 8, |a, _, _| f64::from(a).to_bits()),
    // The product of two `f32` is exact as an `f64`, so the cast rounds an `f64` that no `f32` holds.
    ("from_f64", 4, |a, b, _| f((f64::from(a) * f64::from(b)) as f32)),
    ("is_finite", 1, |a, _, _| u64::from(a.is_finite())),
    ("is_nan", 1, |a, b, _| u64::from((a * b).is_nan())),
];

/// Runs every operation on every case and compares its digest with its line of the expected file.
///
/// # Errors
/// Files of the wrong shape, an input that is a NaN, or the operations whose results are not the reference's.
pub fn check(inputs: &[u8], expected: &str) -> Result<(), String> {
    let lines: Vec<&str> = expected
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect();
    if inputs.len() != 12 * CASES || lines.len() != OPS.len() {
        return Err(format!(
            "{} input bytes and {} expected lines, not {} and {}",
            inputs.len(),
            lines.len(),
            12 * CASES,
            OPS.len()
        ));
    }
    let value = |at: usize| {
        f32::from_bits(u32::from_le_bytes([
            inputs[at],
            inputs[at + 1],
            inputs[at + 2],
            inputs[at + 3],
        ]))
    };
    if let Some(at) = (0..3 * CASES).find(|&index| value(4 * index).is_nan()) {
        return Err(format!(
            "input {at} is a NaN: no input may be one (ADR 0004 part 7)"
        ));
    }
    let mut results = Vec::with_capacity(8 * CASES);
    let mut differing = Vec::new();
    for ((name, width, op), line) in OPS.iter().zip(lines) {
        results.clear();
        for case in 0..CASES {
            let bits = op(value(12 * case), value(12 * case + 4), value(12 * case + 8));
            results.extend_from_slice(&u64::to_le_bytes(bits)[..*width]);
        }
        let got = format!("{name} {width} {:016x}", fnv1a_64(&results));
        if got != line {
            differing.push(format!("{got} (the reference: {line})"));
        }
    }
    if differing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "not the exact reference's results: {}",
            differing.join("; ")
        ))
    }
}
