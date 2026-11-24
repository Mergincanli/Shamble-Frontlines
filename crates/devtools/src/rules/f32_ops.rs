//! The float vectors (PM-0004-22): every `f32` operation ADR 0004 part 7 allows, on the fixed inputs of
//! `vectors/f32-inputs.bin`, against the digests an exact reference wrote into `vectors/f32-expected.txt`. The
//! script `vectors/make_f32_vectors.py` makes both files and explains them; the operations here are that script's,
//! in its order. The inputs are read when a test runs, never built in, so no compiler works a result out ahead.
//!
//! One operation is not the same on every target (MUL-ADD): `mul_add` is exact natively on a processor with FMA,
//! and in WebAssembly the toolchain's software `fmaf` is not, where the result lies below the smallest normal
//! `f32`. The expected file's `pin` lines hold those cases with the values measured in WebAssembly. There each
//! must still give exactly its measured value, so a toolchain that changes it fails this check and the question
//! is open again; every other case must give the reference's. Natively nothing is pinned: every case must be
//! exact. The pins are a known defect, kept in sight; the simulation's scan refuses `mul_add` meanwhile.

use sim::state_hash::fnv1a_64;

/// Cases in the input file: three `f32` bit patterns each, little-endian. The script's 10,000 and its 48
/// constructed ones.
pub const CASES: usize = 10_048;

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

/// A `pin` line: the case, its three inputs, the exact result, and the result measured in WebAssembly.
struct Pin {
    case: usize,
    inputs: [u32; 3],
    exact: u32,
    measured: u32,
}

fn pin(line: &str) -> Option<Pin> {
    let mut words = line.split(' ');
    let hex = |word: Option<&str>| u32::from_str_radix(word?, 16).ok();
    if (words.next(), words.next()) != (Some("pin"), Some("mul_add")) {
        return None;
    }
    let case = words.next()?.parse().ok()?;
    let inputs = [hex(words.next())?, hex(words.next())?, hex(words.next())?];
    let (exact, measured) = (hex(words.next())?, hex(words.next())?);
    words.next().is_none().then_some(Pin {
        case,
        inputs,
        exact,
        measured,
    })
}

/// Part 7's operations on `f32`, in the script's order: what a case's three inputs give, as bits. The casts are
/// the language's own `as`, which is what the rule allows.
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
    // Two inputs' bits side by side as one `f64`: all 53 bits of it, and far outside `f32`'s range on both sides.
    ("from_f64", 4, |a, b, _| f(f64::from_bits(wide(a, b)) as f32)),
    ("is_finite", 1, |a, _, _| u64::from(a.is_finite())),
    ("is_nan", 1, |a, b, _| u64::from((a * b).is_nan())),
];

/// Runs every operation on every case and compares its digest with its line of the expected file, as this target
/// must give it: in WebAssembly with `mul_add`'s pins, natively with none.
///
/// # Errors
/// Files of the wrong shape, an input that is a NaN, or the operations whose results are not the reference's.
pub fn check(inputs: &[u8], expected: &str) -> Result<(), String> {
    check_as(inputs, expected, cfg!(target_arch = "wasm32"))
}

/// [`check`], told whether the pins count (`pinned`: this is the WebAssembly build). Its own entry so that a
/// native test can show what a toolchain whose `fmaf` changed would meet.
///
/// # Errors
/// As [`check`]; with `pinned`, also each pinned case that no longer gives its measured value.
pub fn check_as(inputs: &[u8], expected: &str, pinned: bool) -> Result<(), String> {
    let lines: Vec<&str> = expected
        .lines()
        .filter(|line| !line.starts_with('#') && !line.starts_with("pin "))
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
    let bits = |at: usize| {
        u32::from_le_bytes([inputs[at], inputs[at + 1], inputs[at + 2], inputs[at + 3]])
    };
    let value = |at: usize| f32::from_bits(bits(at));
    if let Some(at) = (0..3 * CASES).find(|&index| value(4 * index).is_nan()) {
        return Err(format!(
            "input {at} is a NaN: no input may be one (ADR 0004 part 7)"
        ));
    }
    let mut pins = Vec::new();
    for line in expected.lines().filter(|line| line.starts_with("pin ")) {
        let read = pin(line).filter(|pin| {
            pin.case < CASES && (0..3).all(|at| bits(12 * pin.case + 4 * at) == pin.inputs[at])
        });
        pins.push(read.ok_or_else(|| format!("not a pin of these inputs: {line}"))?);
    }
    let mut results = Vec::with_capacity(8 * CASES);
    let mut differing = Vec::new();
    for ((name, width, op), line) in OPS.iter().zip(lines) {
        results.clear();
        let pins: &[Pin] = if pinned && *name == "mul_add" {
            &pins
        } else {
            &[]
        };
        for case in 0..CASES {
            let mut bits = op(value(12 * case), value(12 * case + 4), value(12 * case + 8));
            if let Some(pin) = pins.iter().find(|pin| pin.case == case) {
                if bits != u64::from(pin.measured) {
                    differing.push(format!(
                        "{name} case {case} gives {bits:08x}, not its pinned {:08x} (exact: {:08x}): the toolchain's `fmaf` changed, so MUL-ADD is open again",
                        pin.measured, pin.exact
                    ));
                }
                bits = u64::from(pin.exact);
            }
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
