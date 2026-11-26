//! The committed recordings, natively (ADR 0004's milestone and its foundation line; PM-0004-19): every file is
//! what its script makes, and every recording replays to its hashes file, in both columns. The same check is
//! `make_recordings --check`. The files are read from disk when the test runs, never built into the program.

use std::path::{Path, PathBuf};

use devtools::compare::{Column, first_difference};
use devtools::files::{MANIFEST, check, made, read_plain, vector_text, write};
use devtools::recording::{Recording, parse_hashes};
use devtools::replay::Replay;
use devtools::scripts::{RECORDINGS, VECTOR_TICK};

fn folder() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("recordings")
}

fn read(name: &str) -> Vec<u8> {
    read_plain(&folder(), name).unwrap_or_else(|why| panic!("{why}"))
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

/// Mustafa's LINKS-CHECK (c), 2026-11-26: the tests that read the recordings refuse anything that is not a plain
/// file. The links are planted in a scratch folder; no file they point at is opened.
#[cfg(unix)]
#[test]
fn the_plain_read_refuses_a_link_a_folder_and_a_name_that_is_a_path() {
    let scratch = Path::new(env!("CARGO_TARGET_TMPDIR")).join("plain-read");
    let _ = std::fs::remove_dir_all(&scratch);
    let folder = scratch.join("recordings");
    std::fs::create_dir_all(folder.join("sub.rec")).expect("scratch folders");
    std::fs::write(folder.join("a.rec"), b"the recording").expect("a scratch file");
    std::fs::write(scratch.join("outside.txt"), b"not a recording").expect("a scratch file");
    let link = |to: &Path, at: PathBuf| std::os::unix::fs::symlink(to, at).expect("a link");
    link(&scratch.join("outside.txt"), folder.join("link.rec"));
    link(Path::new("/dev/zero"), folder.join("zero.rec"));
    link(&folder, scratch.join("linked"));
    assert_eq!(read_plain(&folder, "a.rec"), Ok(b"the recording".to_vec()));
    let refused = |folder: &Path, name: &str| read_plain(folder, name).expect_err(name);
    // A link to a file outside, and a link to a file that never ends: refused unopened.
    for name in ["link.rec", "zero.rec", "sub.rec"] {
        let why = refused(&folder, name);
        assert!(
            why.ends_with(&format!("{name}\" is a link or not a plain file")),
            "{why}"
        );
    }
    assert!(refused(&folder, "missing.rec").starts_with("missing.rec: "));
    for name in ["", ".", "..", "../outside.txt", "sub.rec/a", "a\\b"] {
        assert!(
            refused(&folder, name).ends_with("is not a file's name"),
            "{name}"
        );
    }
    // The same file through a folder that is a link, with and without a separator at the path's end.
    for linked in ["linked", "linked/"] {
        let why = refused(&scratch.join(linked), "a.rec");
        assert!(
            why.ends_with("is a link or not a plain folder"),
            "{linked}: {why}"
        );
    }
    std::fs::remove_dir_all(&scratch).expect("the scratch folder goes");
}

/// No reader of a committed recording goes around the plain read: the two tests that only read make no file-system
/// call of their own; this file reads the committed folder (`folder()`) through `read_plain` alone; the library opens
/// a file in one place; and the two readers under Node go through their own helper, every other read there being of
/// a WebAssembly file. A text scan, so it holds what the sources say, not what a new reader elsewhere might do.
#[test]
fn the_committed_recordings_are_read_only_through_the_plain_read() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source =
        |path: &str| String::from_utf8(std::fs::read(root.join(path)).expect(path)).expect(path);
    // The words looked for are put together here, so that this test does not find itself.
    let (call, read, committed) = (
        ["fs", "::"].concat(),
        ["fs", "::read"].concat(),
        ["folder", "()"].concat(),
    );
    for path in ["tests/two_instances.rs", "tests/two_processes.rs"] {
        let text = source(path);
        assert!(
            !text.contains(&call) && text.contains("read_plain(&folder, "),
            "{path}"
        );
    }
    let own = source("tests/recordings.rs");
    let around = own
        .lines()
        .find(|line| line.contains(&read) && line.contains(&committed));
    assert_eq!(around, None);
    assert!(own.contains("read_plain(&folder(), name)"));
    let library = source("src/files.rs");
    let plain_read = library
        .split_once("pub fn read_plain")
        .and_then(|(_, rest)| rest.split_once("\n}\n"));
    // One place opens a file, inside `read_plain`; nothing reads a path whole.
    let (open, whole) = (["File", "::open("].concat(), format!("{read}("));
    let opened = plain_read.map(|(body, _)| body.matches(&open).count());
    let counts = (
        library.matches(&open).count(),
        library.matches(&whole).count(),
    );
    assert_eq!((counts, opened), ((1, 0), Some(1)));
    for path in [
        "../../packages/sim-worker/boundary/replay.test.mjs",
        "../../packages/e2e/tests/replay/determinism.spec.ts",
    ] {
        let text = source(path);
        assert!(text.contains("readPlainFile(recordings, "), "{path}");
        for line in text.lines().filter(|line| line.contains("readFileSync(")) {
            assert!(
                line.contains(".wasm"),
                "{path} reads a file that is no WebAssembly file: {line}"
            );
        }
    }
}
