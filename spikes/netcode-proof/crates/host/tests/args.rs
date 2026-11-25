//! npt-host's command line (u6c-brief.md C1 (a), (f)): `issue` takes exactly one word and no path, `serve` one settings
//! file; anything else, and a start that cannot read its settings, exits 2 before any network or file write.

use std::process::Command;

#[test]
fn u6c_npt_host_takes_issue_with_one_word_and_serve_with_one_file() {
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_npt-host"))
            .args(args)
            .output()
            .expect("runs")
    };
    for args in [
        &[][..],
        &["issue"],
        &["issue", "staging", "/tmp"],
        &["issue", "prod"],
        &["serve"],
        &["serve", "/nonexistent/host.json"],
        &["revoke", "production"],
    ] {
        let out = run(args);
        assert_eq!(out.status.code(), Some(2), "{args:?}");
        assert!(out.stdout.is_empty(), "{args:?}: nothing on stdout");
    }
}
