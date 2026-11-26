//! Makes the committed recordings from their scripts: `cargo run -p devtools --bin make_recordings` writes every
//! file into `crates/devtools/recordings/`; with `--check` it writes nothing, compares what is there byte for byte
//! and exits 1 naming each file that differs (for a hashes file, the first step that does). The plain test run
//! makes the same check (`tests/recordings.rs`).

use std::path::Path;
use std::process::ExitCode;

use devtools::files::{check, write};

fn main() -> ExitCode {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("recordings");
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let result = match arguments.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        [] => write(&folder)
            .map(|files| format!("written to {}: {}", folder.display(), files.join(", "))),
        ["--check"] => {
            check(&folder).map(|()| "the recordings are what their scripts make".to_owned())
        }
        _ => Err("usage: make_recordings [--check]".to_owned()),
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
