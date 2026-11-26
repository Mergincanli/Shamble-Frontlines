//! The content bundle: `cargo run -p devtools --bin content_bundle -- <folder>` reads the files under `content/`,
//! asks the loader about each and about the whole, and writes `content.json` into `<folder>` (the build names it;
//! it must exist and lie outside `content/`). With `--check` it writes nothing and exits 1 when a file is not what
//! the loader takes or the folder holds anything that is no content file. The plain test run makes the same check
//! (`tests/content_files.rs`). It reads one folder, a constant of this file, and takes no path from the environment.

use std::path::Path;
use std::process::ExitCode;

use devtools::content_files::run;

fn main() -> ExitCode {
    let content = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content");
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let result = match arguments.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        ["--check"] => run(&content, None),
        [out] if !out.starts_with('-') => run(&content, Some(Path::new(out))),
        _ => Err(
            "usage: content_bundle --check | content_bundle <folder for content.json>".to_owned(),
        ),
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
