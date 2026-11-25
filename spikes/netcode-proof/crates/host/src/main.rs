//! npt-host (plan E21, §8.4; u6c-brief.md Blocks 5 and 6): the page host, `npt-host serve <settings.json>` (serve.rs:
//! U7's dist files from a compiled-in allow-list, the per-room hash files, the host-key file, TLS 1.3 on the production
//! certificate), and the DNS-01 issue mode, `npt-host issue staging|production` (issue.rs: one process across both
//! phases, its account key in memory only, its files under /var/lib/npt/tls). A binary only. The cuts (replan-units.md
//! :13-14, :26-27, :32): no connection limits, no `/iso/`, robots or noindex, no HTTP-01 and no `revoke`.

mod issue;
mod serve;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["serve", path] => serve::main(path),
        ["issue", word] => issue::main(word),
        _ => {
            eprintln!(
                "npt-host: usage: npt-host serve <settings.json> | npt-host issue staging|production"
            );
            ExitCode::from(2)
        }
    }
}
