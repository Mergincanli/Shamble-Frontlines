//! npt-judge: one verdict per mark from the runs' results logs and `marks.json` (plan §6.1): PASS, FAIL, INVALID,
//! STAND-IN, REPORTED or NOT-RUN, with nearest-rank percentiles and exact fractions. `marks/README.md` is the
//! contract it follows. It depends only on npt-log and serde (plan §4.2), so its verdicts never share code with what
//! they judge.

pub mod eval;
pub mod figures;
pub mod marks;
mod practice;
pub mod ratio;

pub use eval::{Verdict, VerdictKind, judge};
pub use marks::{Marks, parse as parse_marks};

use std::io::BufReader;
use std::path::{Path, PathBuf};

/// Every `*.jsonl` file under a folder, in path order.
pub fn log_files(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d)? {
            let path = entry?.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "jsonl") {
                out.push(path);
            }
        }
    }
    out.sort();
    Ok(out)
}

/// Reads every log under a folder. Any malformed log is an error naming its file: a judge never skips one.
pub fn read_logs(dir: &Path) -> Result<Vec<npt_log::Log>, String> {
    let mut logs = Vec::new();
    for path in log_files(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let file = std::fs::File::open(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let log = npt_log::read_log(BufReader::new(file))
            .map_err(|e| format!("{}: {e}", path.display()))?;
        logs.push(log);
    }
    Ok(logs)
}

/// One line per verdict: the mark, the verdict, and its first reason.
pub fn summary_line(v: &Verdict) -> String {
    let first = v.why.first().map(String::as_str).unwrap_or("");
    format!("{:<12} {:<9} {first}", v.mark, v.verdict.to_string())
}
