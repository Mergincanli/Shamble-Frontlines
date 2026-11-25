//! Replays a recording natively: `cargo run -p devtools --bin replay -- <what> <recording.rec> [...]`.
//!
//! - `hashes <rec>` prints the hashes file this build gives for it;
//! - `check <rec> <hashes>` compares this build with a hashes file and exits 1 at the first step that differs;
//! - `state <rec> <tick>` prints the state at that tick, one named field a line: two targets' prints of one tick,
//!   compared line by line, show which fields differ. The seed is never printed.

use std::process::ExitCode;

use devtools::compare::{Column, first_difference};
use devtools::recording::{Recording, hashes_text, parse_hashes};
use devtools::replay::{replay, state_at};

fn run(arguments: &[&str]) -> Result<String, String> {
    let read = |path: &str| std::fs::read(path).map_err(|error| format!("{path}: {error}"));
    let recording = |bytes: &[u8]| Recording::decode(bytes).map_err(|error| error.to_string());
    match *arguments {
        ["hashes", rec] => {
            let bytes = read(rec)?;
            let lines = replay(&recording(&bytes)?).map_err(|error| error.to_string())?;
            Ok(hashes_text(&bytes, &lines))
        }
        ["check", rec, hashes] => {
            let bytes = read(rec)?;
            let expected = parse_hashes(&bytes, &String::from_utf8_lossy(&read(hashes)?))?;
            let got = replay(&recording(&bytes)?).map_err(|error| error.to_string())?;
            let shared = first_difference(&expected, &got, Column::Shared);
            match shared.or_else(|| first_difference(&expected, &got, Column::Whole)) {
                None => Ok(format!("{} steps, every hash as in the file\n", got.len())),
                Some(difference) => Err(format!("the file against this build, {difference}")),
            }
        }
        ["state", rec, tick] => {
            let tick = tick.parse().map_err(|_| format!("{tick} is no tick"))?;
            let lines =
                state_at(&recording(&read(rec)?)?, tick).map_err(|error| error.to_string())?;
            lines.ok_or_else(|| format!("the recording never reaches tick {tick}"))
        }
        _ => {
            Err("usage: replay hashes <rec> | check <rec> <hashes> | state <rec> <tick>".to_owned())
        }
    }
}

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match run(&arguments.iter().map(String::as_str).collect::<Vec<_>>()) {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(why) => {
            eprintln!("{why}");
            ExitCode::FAILURE
        }
    }
}
