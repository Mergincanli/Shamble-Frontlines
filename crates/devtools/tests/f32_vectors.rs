//! The float rules, bit for bit (PM-0004-22), natively: every allowed operation against the exact reference's digests.
//! The same check runs in WebAssembly through the test-only module.

use devtools::rules::f32_ops::{CASES, OPS, check, check_as};

/// Whether this processor has the fused multiply-add instruction. The toolchain's `fmaf` asks the same question
/// when it runs; without the instruction it takes its software path, which is not exact for the pinned cases.
fn fma() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    if !std::is_x86_feature_detected!("fma") {
        return "this processor reports no FMA: unless it is an AMD one with FMA4, `mul_add` takes the toolchain's software path, which is not exact (MUL-ADD)";
    }
    #[cfg(target_arch = "x86_64")]
    return "this processor has FMA";
    #[cfg(not(target_arch = "x86_64"))]
    "not x86-64: whether `mul_add` is one instruction here was not asked"
}

fn files() -> (Vec<u8>, String) {
    // Read when the test runs, never built into the program.
    let folder = concat!(env!("CARGO_MANIFEST_DIR"), "/vectors/");
    let inputs = std::fs::read(format!("{folder}f32-inputs.bin")).expect("the input file");
    let expected =
        std::fs::read_to_string(format!("{folder}f32-expected.txt")).expect("the expected file");
    (inputs, expected)
}

#[test]
fn every_allowed_operation_gives_the_exact_reference_s_results() {
    let (inputs, expected) = files();
    assert_eq!((inputs.len(), OPS.len()), (12 * CASES, 38));
    println!("{}", fma());
    assert_eq!(check(&inputs, &expected), Ok(()), "{}", fma());
}

#[test]
fn a_pinned_case_that_no_longer_gives_its_measured_value_fails_by_name() {
    // Natively `mul_add` is exact, which is what a toolchain with a corrected `fmaf` would give in WebAssembly:
    // every pin then fails, each by its case, and nothing else does.
    let (inputs, expected) = files();
    let pins = expected.lines().filter(|l| l.starts_with("pin ")).count();
    assert_eq!(pins, 16);
    let found = check_as(&inputs, &expected, true).expect_err(fma());
    assert_eq!(
        found.matches("MUL-ADD is open again").count(),
        pins,
        "{found}"
    );
    assert!(
        found.contains(
            "mul_add case 10024 gives 007fffff, not its pinned 00800000 (exact: 007fffff)"
        ),
        "{found}"
    );
    assert_eq!(found.matches("the reference:").count(), 0, "{found}");
    // A pin that is not of these inputs, or not a pin at all, is refused before anything is compared.
    let moved = expected.replace("pin mul_add 10024 ", "pin mul_add 10023 ");
    assert!(
        check(&inputs, &moved)
            .expect_err("a pin on another case")
            .contains("not a pin of these inputs: pin mul_add 10023 01000001")
    );
    let cut = expected.replace(" 007fffff 00800000\n", " 007fffff\n");
    assert!(
        check(&inputs, &cut)
            .expect_err("a pin with a word missing")
            .contains("not a pin of these inputs")
    );
}

#[test]
fn a_changed_file_is_found_and_named() {
    let (inputs, expected) = files();
    // One digit of one digest: that operation is named, and no other.
    let sqrt_line = expected
        .lines()
        .find(|line| line.starts_with("sqrt "))
        .expect("the sqrt line");
    let digit = if sqrt_line.ends_with('0') { "1" } else { "0" };
    let changed = format!("{}{digit}", &sqrt_line[..sqrt_line.len() - 1]);
    let found =
        check(&inputs, &expected.replace(sqrt_line, &changed)).expect_err("a changed digest");
    assert!(
        found.contains("sqrt 4 ") && found.matches("the reference:").count() == 1,
        "{found}"
    );
    // One bit of the first input (a zero becomes a small number): the operations that tell the two apart differ.
    let mut bad = inputs.clone();
    bad[1] ^= 0x40;
    let found = check(&bad, &expected).expect_err("a changed input");
    assert!(
        found.contains("add 4 ") && found.contains("sqrt_abs 4 ") && found.contains("to_f64 8 "),
        "{found}"
    );
    // A NaN among the inputs, a short input file and a missing line are refused before anything is compared.
    bad[..4].copy_from_slice(&f32::NAN.to_bits().to_le_bytes());
    assert!(
        check(&bad, &expected)
            .expect_err("a NaN input")
            .contains("input 0 is a NaN")
    );
    assert!(
        check(&inputs[4..], &expected)
            .expect_err("a short file")
            .contains("120812 input bytes")
    );
    let short = expected.replace(sqrt_line, "#");
    assert!(
        check(&inputs, &short)
            .expect_err("a missing line")
            .contains("37 expected lines")
    );
}
