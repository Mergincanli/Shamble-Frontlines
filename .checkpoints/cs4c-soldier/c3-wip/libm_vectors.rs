//! The maths routines' vectors natively (PM-0004-22): libm's exponential and sine-and-cosine give the recorded outputs
//! on the committed inputs. The same check runs in WebAssembly through the test-only module, under Node and in each
//! browser engine, so the native build and every engine give the same bits.

use std::path::{Path, PathBuf};

use devtools::files::{read_plain, write_plain};
use devtools::rules::libm_ops::{
    EXP_CASES, LIBM_VERSION, SINCOS_CASES, check, expected_text, header,
};

// PM-0004-22 asks at least 10,000 fixed inputs a routine.
const _: () = assert!(EXP_CASES >= 10_000 && SINCOS_CASES >= 10_000);

fn folder() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("vectors")
}

fn vector(name: &str) -> Vec<u8> {
    // Read when the test runs, never built into the program, and only when it is a plain file in a plain folder.
    read_plain(&folder(), name).unwrap_or_else(|why| panic!("{why}"))
}

#[test]
fn libm_s_two_routines_give_the_recorded_outputs() {
    let inputs = vector("libm-inputs.bin");
    // SHAMBLE_WRITE_LIBM=1, and no other value, records the outputs again (after a change of libm's version or of the
    // inputs, which then shows in the expected file's diff), never through a link in its place; without it the file
    // is only read.
    if std::env::var("SHAMBLE_WRITE_LIBM").as_deref() == Ok("1") {
        let made = expected_text(&inputs).unwrap_or_else(|why| panic!("{why}"));
        write_plain(&folder(), "libm-expected.txt", made.as_bytes()).expect("the expected file");
    }
    let expected = String::from_utf8(vector("libm-expected.txt")).expect("text");
    assert_eq!(expected.lines().count(), 3);
    assert_eq!(check(&inputs, &expected), Ok(()));
    // The header names libm at the version the root manifest pins.
    let root =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml"))
            .expect("the root manifest");
    assert!(
        root.lines()
            .any(|line| line == format!("libm = \"={LIBM_VERSION}\"")),
        "the root Cargo.toml pins libm at {LIBM_VERSION}"
    );
    assert_eq!(expected.lines().next(), Some(header(&inputs).as_str()));
}

#[test]
fn the_check_bites_on_a_changed_input_and_on_a_changed_output() {
    let inputs = vector("libm-inputs.bin");
    let expected = String::from_utf8(vector("libm-expected.txt")).expect("text");
    // One input byte changed: named by the header, before anything is worked out.
    let mut changed = inputs.clone();
    changed[1] ^= 0x40;
    let why = check(&changed, &expected).expect_err("a changed input");
    assert!(
        why.starts_with("the inputs are not the ones the expected file's header names: "),
        "{why}"
    );
    // One digit of one digest changed: named by its routine's line.
    let line = expected.lines().nth(1).expect("the exp line");
    let last = line.chars().last().expect("a digit");
    let moved = expected.replacen(
        line,
        &format!(
            "{}{}",
            &line[..line.len() - 1],
            if last == '0' { '1' } else { '0' }
        ),
        1,
    );
    let why = check(&inputs, &moved).expect_err("a changed digest");
    assert!(
        why.starts_with("not the committed outputs: exp 33646 "),
        "{why}"
    );
    // A file of the wrong length or holding a NaN is refused as such.
    assert!(expected_text(&inputs[..8]).is_err_and(|why| why.contains("not 1077304")));
    let mut nan = inputs.clone();
    nan[..8].copy_from_slice(&f64::NAN.to_le_bytes());
    assert_eq!(expected_text(&nan), Err("input 0 is a NaN".to_owned()));
}
