//! npt-load: LG's load (u6c-brief.md Block 4, C1 (c)): `npt-load <settings.json>`, the library's `run_load` (its module
//! text gives the modes, the log, `READY`, `DONE` and the exit codes).

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [path] = args.as_slice() else {
        eprintln!("npt-load: usage: npt-load <settings.json>");
        return ExitCode::from(2);
    };
    match npt_bots::LoadSettings::read(path) {
        Ok(s) => ExitCode::from(npt_bots::run_load(&s, &mut std::io::stdout().lock())),
        Err(e) => {
            eprintln!("npt-load: {e}");
            ExitCode::from(2)
        }
    }
}
