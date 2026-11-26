//! The time unit's rounding report (PM-0004-11): `cargo run -p devtools --bin time_report` writes
//! `crates/devtools/vectors/time-report.txt` from the durations extract beside it; with `--check` it writes nothing
//! and exits 1 when the committed report is not what this build makes, when the two conversions disagree on a
//! duration, or when a rule's interval is shorter than one tick. The plain test run makes the same check
//! (`tests/time_unit.rs`).

use std::path::Path;
use std::process::ExitCode;

use devtools::time_report::run;

fn main() -> ExitCode {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("vectors");
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let result = match arguments.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        [] => run(&folder, false),
        ["--check"] => run(&folder, true),
        _ => Err("usage: time_report [--check]".to_owned()),
    };
    match result {
        Ok(done) => {
            println!("{done}");
            ExitCode::SUCCESS
        }
        Err(why) => {
            eprintln!("{why}");
            ExitCode::FAILURE
        }
    }
}
