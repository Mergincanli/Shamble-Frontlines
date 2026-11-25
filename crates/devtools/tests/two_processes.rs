//! The same build reproduces itself in another process (ADR 0001's replay line; PM-0004-18): the test starts its own
//! program twice more, each child replays every committed recording and a 36,000-tick scripted match, and prints a
//! digest of every hash it got. The parent compares the two children with each other and with the files.
//!
//! Two processes differ in what one process cannot vary: the random state of std's hash maps. Each child prints a
//! fingerprint of its own; equal fingerprints would mean the two runs did not differ in it, and the test then fails
//! as not valid, never as passed.

use std::hash::{BuildHasher, RandomState};
use std::process::Command;

use devtools::recording::{Recording, TickHashes, parse_hashes};
use devtools::replay::replay;
use devtools::scripts::{RECORDINGS, looped};
use devtools::sha256::{hex, sha256};

/// The variable that makes a run of this program a child.
const ROLE: &str = "SHAMBLE_TWO_PROCESSES_CHILD";
const LONG_STEPS: u32 = 36_000;

fn digest(lines: &[TickHashes]) -> String {
    let mut bytes = Vec::with_capacity(lines.len() * 20);
    for line in lines {
        bytes.extend_from_slice(&line.tick.to_le_bytes());
        bytes.extend_from_slice(&line.shared.to_le_bytes());
        bytes.extend_from_slice(&line.whole.to_le_bytes());
    }
    hex(&sha256(&bytes))
}

/// What a child prints: its hash-map fingerprint, then a digest per recording, the long scripted match last.
fn child_report() -> String {
    let mut report = format!(
        "fingerprint {:016x}\n",
        RandomState::new().hash_one(0x5348_4D42u32)
    );
    for (name, script) in RECORDINGS {
        let lines = replay(&script()).expect("it replays");
        report.push_str(&format!("{name} {} {}\n", lines.len(), digest(&lines)));
    }
    let long = replay(&looped("long-loop", LONG_STEPS, 9)).expect("it replays");
    report.push_str(&format!("long-loop {} {}\n", long.len(), digest(&long)));
    report
}

#[test]
fn two_more_processes_give_the_same_hashes_as_each_other_and_as_the_files() {
    if std::env::var_os(ROLE).is_some() {
        // On a line of its own: the harness has printed the test's name without ending the line.
        print!("\n{}", child_report());
        return;
    }
    let child = || {
        let program = std::env::current_exe().expect("this test's own program");
        let run = Command::new(program)
            .args([
                "--exact",
                "two_more_processes_give_the_same_hashes_as_each_other_and_as_the_files",
            ])
            .args(["--nocapture", "--test-threads=1"])
            .env(ROLE, "1")
            .output()
            .expect("the child runs");
        assert!(
            run.status.success(),
            "the child failed: {}",
            String::from_utf8_lossy(&run.stderr)
        );
        let printed = String::from_utf8(run.stdout).expect("text");
        // The harness prints its own lines around the test's: keep the report's.
        let keep = |line: &&str| line.starts_with("fingerprint ") || line.contains("-loop ");
        printed
            .lines()
            .filter(keep)
            .map(|line| format!("{line}\n"))
            .collect::<String>()
    };
    let (one, two) = (child(), child());
    let split = |report: &str| {
        let (fingerprint, digests) = report.split_once('\n').expect("a fingerprint line");
        (fingerprint.to_owned(), digests.to_owned())
    };
    let ((print_one, digests_one), (print_two, digests_two)) = (split(&one), split(&two));
    assert!(
        print_one.starts_with("fingerprint ") && print_one.len() == 28,
        "{one}"
    );
    assert_ne!(
        print_one, print_two,
        "not valid: the two processes did not differ in their hash-map state"
    );
    assert_eq!(digests_one, digests_two);
    // What the children got is what the committed files hold, and the long match ran to its end.
    let mut expected = String::new();
    for (name, _) in RECORDINGS {
        let path =
            |suffix: &str| format!("{}/recordings/{name}.{suffix}", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(path("rec")).expect("the committed recording");
        assert!(Recording::decode(&bytes).is_ok());
        let hashes = std::fs::read_to_string(path("hashes")).expect("its hashes file");
        let lines = parse_hashes(&bytes, &hashes).expect("its hashes file reads");
        expected.push_str(&format!("{name} {} {}\n", lines.len(), digest(&lines)));
    }
    assert!(
        digests_one.starts_with(&expected),
        "{digests_one}\nis not\n{expected}"
    );
    let long = digests_one.strip_prefix(&expected).expect("checked above");
    assert!(
        long.starts_with(&format!("long-loop {LONG_STEPS} ")) && long.lines().count() == 1,
        "{long}"
    );
}
