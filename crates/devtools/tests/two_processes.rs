//! The same build reproduces itself in another process (ADR 0001's replay line; PM-0004-18): the test starts its own
//! program twice more, each child replays every committed recording's script and a 36,000-tick scripted match, and
//! prints the hashes of every step. The parent compares the two children with each other and with the files, and
//! names the first step that differs.
//!
//! Two processes differ in what one process cannot vary: the random state of std's hash maps. Each child prints a
//! fingerprint of its own; equal fingerprints would mean the two runs did not differ in it, and the test then fails
//! as not valid, never as passed.

use std::hash::{BuildHasher, RandomState};
use std::process::Command;

use devtools::compare::{Column, first_difference};
use devtools::recording::{Recording, TickHashes, parse_hashes};
use devtools::replay::replay;
use devtools::scripts::{RECORDINGS, looped};

/// The variable that, with the parent's own arguments, makes a run of this program a child.
const ROLE: &str = "SHAMBLE_TWO_PROCESSES_CHILD";
const NAME: &str = "two_more_processes_give_the_same_hashes_as_each_other_and_as_the_files";
const ARGUMENTS: [&str; 4] = ["--exact", NAME, "--nocapture", "--test-threads=1"];
const LONG: (&str, u32) = ("long-loop", 36_000);
/// What each of a child's lines starts with: nothing the test harness prints does.
const LINE: &str = "two-processes ";

/// What a child prints: its hash-map fingerprint, then one line per step of every replay, the long match last.
fn child_report() -> String {
    use std::fmt::Write;
    let fingerprint = RandomState::new().hash_one(0x5348_4D42u32);
    let mut report = format!("\n{LINE}fingerprint {fingerprint:016x}\n");
    let long = looped(LONG.0, LONG.1, 9);
    let scripted = RECORDINGS.iter().map(|(name, script)| (*name, script()));
    for (name, recording) in scripted.chain([(LONG.0, long)]) {
        for step in replay(&recording).expect("it replays") {
            let (tick, shared, whole) = (step.tick, step.shared, step.whole);
            // Writing into a `String` cannot fail.
            let _ = writeln!(report, "{LINE}{name} {tick} {shared:016x} {whole:016x}");
        }
    }
    report
}

/// A child's report, read back: its fingerprint, and the steps of the replay with this name.
fn steps_of(report: &str, name: &str) -> Vec<TickHashes> {
    let lines = report.lines().filter_map(|line| {
        let rest = line
            .strip_prefix(LINE)?
            .strip_prefix(name)?
            .strip_prefix(' ')?;
        let mut parts = rest.split(' ');
        let hex = |text: &str| u64::from_str_radix(text, 16).ok();
        let (tick, shared, whole) = (parts.next()?, parts.next()?, parts.next()?);
        Some(TickHashes {
            tick: tick.parse().ok()?,
            shared: hex(shared)?,
            whole: hex(whole)?,
        })
    });
    lines.collect()
}

#[test]
fn two_more_processes_give_the_same_hashes_as_each_other_and_as_the_files() {
    // A child only with the parent's own arguments too: the variable alone, left in someone's environment, must not
    // turn the test into one that compares nothing.
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    if std::env::var_os(ROLE).is_some() && arguments == ARGUMENTS {
        print!("{}", child_report());
        return;
    }
    let child = || {
        let program = std::env::current_exe().expect("this test's own program");
        let run = Command::new(program)
            .args(ARGUMENTS)
            .env(ROLE, "1")
            .output();
        let run = run.expect("the child runs");
        let failed = String::from_utf8_lossy(&run.stderr);
        assert!(run.status.success(), "the child failed: {failed}");
        String::from_utf8(run.stdout).expect("text")
    };
    let (one, two) = (child(), child());
    let fingerprint = |report: &str| {
        let line = report
            .lines()
            .find_map(|line| line.strip_prefix(LINE)?.strip_prefix("fingerprint "));
        line.expect("a fingerprint line").to_owned()
    };
    let (print_one, print_two) = (fingerprint(&one), fingerprint(&two));
    println!("the two processes' hash-map fingerprints: {print_one} and {print_two}");
    assert_ne!(
        print_one, print_two,
        "not valid: the two processes did not differ in their hash-map state"
    );
    // Step for step, in both columns: the two children agree, and the first step that differs is named.
    let names = RECORDINGS.iter().map(|(name, _)| *name).chain([LONG.0]);
    for name in names {
        let (a, b) = (steps_of(&one, name), steps_of(&two, name));
        for column in [Column::Shared, Column::Whole] {
            let difference = first_difference(&a, &b, column);
            assert_eq!(
                difference, None,
                "{name}, {column:?}: the two processes differ"
            );
        }
        if name == LONG.0 {
            assert_eq!(a.len(), LONG.1 as usize);
            continue;
        }
        // What the children got is what the committed files hold.
        let path =
            |suffix: &str| format!("{}/recordings/{name}.{suffix}", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(path("rec")).expect("the committed recording");
        assert!(Recording::decode(&bytes).is_ok());
        let hashes = std::fs::read_to_string(path("hashes")).expect("its hashes file");
        let file = parse_hashes(&bytes, &hashes).expect("its hashes file reads");
        assert_eq!(
            first_difference(&file, &a, Column::Whole),
            None,
            "{name}: the file against a child"
        );
        assert_eq!(file, a);
    }
}
