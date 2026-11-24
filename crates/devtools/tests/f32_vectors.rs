//! The float rules, bit for bit (PM-0004-22), natively: every allowed operation against the exact reference's digests.
//! The same check runs in WebAssembly through the test-only module.

use devtools::rules::f32_ops::{CASES, OPS, check};

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
    assert_eq!(check(&inputs, &expected), Ok(()));
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
            .contains("119996 input bytes")
    );
    let short = expected.replace(sqrt_line, "#");
    assert!(
        check(&inputs, &short)
            .expect_err("a missing line")
            .contains("37 expected lines")
    );
}
