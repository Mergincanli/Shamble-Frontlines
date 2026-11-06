//! npt-judge's command line.
//!
//! - `npt-judge --marks <marks.json> --logs <folder> [--out <verdicts.json>] [--only <id,id,...>]` judges every log
//!   under the folder, prints one line per mark and writes the full verdicts as JSON;
//! - `npt-judge --marks <marks.json> --check-marks` only reads and checks `marks.json`.
//!
//! Exit codes: 0 when the verdicts were written (whatever they are), 1 when an input is malformed, 2 on bad usage.

use std::path::PathBuf;
use std::process::ExitCode;

struct Args {
    marks: PathBuf,
    logs: Option<PathBuf>,
    out: Option<PathBuf>,
    only: Vec<String>,
    check_marks: bool,
}

const USAGE: &str = "usage: npt-judge --marks <marks.json> (--logs <folder> [--out <verdicts.json>] [--only <id,...>] | --check-marks)";

fn parse_args() -> Result<Args, String> {
    let mut args = Args {
        marks: PathBuf::new(),
        logs: None,
        out: None,
        only: Vec::new(),
        check_marks: false,
    };
    let mut have_marks = false;
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        let mut value = || it.next().ok_or(format!("{a} needs a value"));
        match a.as_str() {
            "--marks" => {
                args.marks = PathBuf::from(value()?);
                have_marks = true;
            }
            "--logs" => args.logs = Some(PathBuf::from(value()?)),
            "--out" => args.out = Some(PathBuf::from(value()?)),
            "--only" => args.only = value()?.split(',').map(str::to_owned).collect(),
            "--check-marks" => args.check_marks = true,
            other => return Err(format!("unknown argument {other:?}")),
        }
    }
    if !have_marks || (args.logs.is_none() != args.check_marks) {
        return Err("give --marks, and either --logs or --check-marks".to_owned());
    }
    Ok(args)
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("npt-judge: {e}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let text = match std::fs::read_to_string(&args.marks) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("npt-judge: {}: {e}", args.marks.display());
            return ExitCode::from(1);
        }
    };
    let mut marks = match npt_judge::parse_marks(&text) {
        Ok(m) => m,
        Err(errors) => {
            for e in errors {
                eprintln!("npt-judge: {e}");
            }
            return ExitCode::from(1);
        }
    };
    if args.check_marks {
        println!(
            "npt-judge: {} marks read and checked (drafts at {})",
            marks.marks.len(),
            marks.drafts_commit
        );
        return ExitCode::SUCCESS;
    }
    if !args.only.is_empty() {
        let unknown: Vec<&String> = args
            .only
            .iter()
            .filter(|id| !marks.marks.iter().any(|m| &m.id == *id))
            .collect();
        if !unknown.is_empty() {
            eprintln!("npt-judge: --only names marks not in marks.json: {unknown:?}");
            return ExitCode::from(2);
        }
        marks.marks.retain(|m| args.only.contains(&m.id));
    }
    let Some(dir) = args.logs else {
        return ExitCode::from(2);
    };
    let logs = match npt_judge::read_logs(&dir) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("npt-judge: {e}");
            return ExitCode::from(1);
        }
    };
    let verdicts = npt_judge::judge(&marks, &logs);
    for v in &verdicts {
        println!("{}", npt_judge::summary_line(v));
    }
    if let Some(out) = args.out {
        let json = match serde_json::to_string_pretty(&verdicts) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("npt-judge: {e}");
                return ExitCode::from(1);
            }
        };
        if let Err(e) = std::fs::write(&out, json + "\n") {
            eprintln!("npt-judge: {}: {e}", out.display());
            return ExitCode::from(1);
        }
    }
    ExitCode::SUCCESS
}
