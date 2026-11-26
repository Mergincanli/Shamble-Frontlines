//! The committed recordings, natively (ADR 0004's milestone and its foundation line; PM-0004-19): every file is
//! what its script makes, and every recording replays to its hashes file, in both columns. The same check is
//! `make_recordings --check`. The files are read from disk when the test runs, never built into the program.

use std::path::{Path, PathBuf};

use devtools::compare::{Column, first_difference};
use devtools::files::{MANIFEST, check, made, vector_text, write};
use devtools::recording::{Recording, parse_hashes};
use devtools::replay::Replay;
use devtools::scripts::{RECORDINGS, VECTOR_TICK};

fn folder() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("recordings")
}

fn read(name: &str) -> Vec<u8> {
    std::fs::read(folder().join(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

#[test]
fn every_committed_file_is_what_its_script_makes() {
    assert_eq!(check(&folder()), Ok(()));
}

#[test]
fn every_recording_replays_to_its_hashes() {
    let manifest = String::from_utf8(read(MANIFEST)).expect("the manifest is text");
    for (name, _) in RECORDINGS {
        // The manifest lists the recording's three files.
        for suffix in ["rec", "hashes", "vector.txt"] {
            let line = format!("{name}.{suffix}: bytes ");
            assert_eq!(manifest.matches(&line).count(), 1, "{line}");
        }
        let bytes = read(&format!("{name}.rec"));
        let recording = Recording::decode(&bytes).expect("a committed recording reads");
        let hashes = String::from_utf8(read(&format!("{name}.hashes"))).expect("text");
        let expected = parse_hashes(&bytes, &hashes).expect("its hashes file reads");
        assert_eq!(expected.len(), recording.steps.len());
        let mut replay = Replay::new(&recording).expect("this build replays it");
        let got: Vec<_> = replay.by_ref().collect();
        for column in [Column::Shared, Column::Whole] {
            let difference = first_difference(&expected, &got, column);
            assert_eq!(difference, None, "{name}, {column:?}");
        }
        // The loop stayed clean to its end: nothing refused, late or missing, and a command ran in every step
        // after the one that carried the join.
        let counters = *replay.sim().local().room().counters();
        let unclean = [counters.refused(), counters.late, counters.missing];
        assert_eq!(unclean, [0; 3], "{name}: {counters:?}");
        assert_eq!(counters.commands_run as usize, recording.steps.len() - 1);
        assert_eq!(
            counters.duplicates + counters.stale + counters.queue_full,
            0
        );
        // The word-size vector: the shared bytes at its tick, the same on every target.
        let vector = String::from_utf8(read(&format!("{name}.vector.txt"))).expect("text");
        assert_eq!(vector_text(&recording, VECTOR_TICK), Ok(vector));
    }
}

/// A copy of the recordings' folder with one thing done to it, in the test run's own scratch folder.
fn changed(case: &str, change: impl Fn(&Path)) -> Result<(), String> {
    let copy = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("recordings-{case}"));
    let _ = std::fs::remove_dir_all(&copy);
    std::fs::create_dir_all(&copy).expect("a scratch folder");
    for (name, bytes) in made().expect("the scripts make their files") {
        std::fs::write(copy.join(name), bytes).expect("a scratch file");
    }
    change(&copy);
    let result = check(&copy);
    std::fs::remove_dir_all(&copy).expect("the scratch folder goes");
    result
}

/// The check fails, naming the file, on each way a folder can differ from what the scripts make; for a hashes file
/// it names the step.
#[test]
fn the_check_names_each_file_that_differs_is_missing_or_is_not_listed() {
    assert_eq!(changed("untouched", |_| ()), Ok(()));
    let flip = |name: &'static str, at: usize| {
        move |folder: &Path| {
            let mut bytes = std::fs::read(folder.join(name)).expect("the file");
            bytes[at] ^= 1;
            std::fs::write(folder.join(name), bytes).expect("the file");
        }
    };
    // One bit of one step's input.
    let rec =
        changed("rec", flip("first-loop.rec", 87 + 900 * 68 + 9)).expect_err("a changed .rec");
    assert_eq!(rec, "first-loop.rec differs");
    // One hex digit of the shared hash of step 900: the line is "900 <16 hex> <16 hex>\n".
    let hashes = String::from_utf8(read("first-loop.hashes")).expect("text");
    let line = hashes.find("\n900 ").expect("step 900's line") + 1;
    let digit = changed("hashes", |folder| {
        let mut bytes = std::fs::read(folder.join("first-loop.hashes")).expect("the file");
        bytes[line + 4] = if bytes[line + 4] == b'0' { b'1' } else { b'0' };
        std::fs::write(folder.join("first-loop.hashes"), bytes).expect("the file");
    })
    .expect_err("a changed digit");
    assert!(
        digit.starts_with(
            "first-loop.hashes differs: the file against this build, step 900: expected tick 900 "
        ),
        "{digit}"
    );
    // A file gone, and a file nobody makes.
    let gone = changed("gone", |folder| {
        std::fs::remove_file(folder.join("first-loop.vector.txt")).expect("the file goes");
    });
    assert!(
        gone.expect_err("a missing file")
            .starts_with("first-loop.vector.txt: ")
    );
    let extra = changed("extra", |folder| {
        std::fs::write(folder.join("stray.rec"), b"SHMBLREC").expect("a scratch file");
    });
    assert_eq!(
        extra.expect_err("an unlisted file"),
        "\"stray.rec\" is in the folder and no script makes it"
    );
    // A link in a file's place is refused by the check and by the writer, which writes nothing through it.
    #[cfg(unix)]
    {
        let outside = Path::new(env!("CARGO_TARGET_TMPDIR")).join("recordings-outside.txt");
        std::fs::write(&outside, b"not to be touched").expect("a scratch file");
        let link = changed("link", |folder| {
            std::fs::remove_file(folder.join("first-loop.rec")).expect("the file goes");
            std::os::unix::fs::symlink(&outside, folder.join("first-loop.rec")).expect("a link");
            let written = write(folder).expect_err("the writer refuses a link");
            assert!(
                written.ends_with("first-loop.rec\" is a link or not a plain file"),
                "{written}"
            );
        });
        let refused = link.expect_err("a link in a file's place");
        assert!(
            refused.contains("first-loop.rec\" is a link or not a plain file"),
            "{refused}"
        );
        assert_eq!(
            std::fs::read(&outside).expect("the scratch file"),
            b"not to be touched"
        );
        std::fs::remove_file(&outside).expect("the scratch file goes");
    }
    // The manifest itself is checked like every other file.
    let manifest = changed("manifest", flip(MANIFEST, 30)).expect_err("a changed manifest");
    assert_eq!(manifest, "manifest.txt differs");
}
