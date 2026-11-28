//! The committed recordings, natively (ADR 0004's milestone and its foundation line; PM-0004-19): every file is
//! what its script makes from the frozen content beside it, and every recording replays on that content to its
//! hashes file, in both columns. The same check is `make_recordings --check`. The files are read from disk when the
//! test runs, never built into the program.

use std::path::{Path, PathBuf};

use devtools::compare::{Column, first_difference};
use devtools::files::{MANIFEST, check, frozen, frozen_name, made, read_plain, vector_text, write};
use devtools::recording::{Recording, parse_hashes};
use devtools::replay::Replay;
use devtools::scripts::{RECORDINGS, VECTOR_TICK};
use devtools::sha256::sha256;

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
    for (name, map_id, _) in RECORDINGS {
        // The manifest lists the recording's frozen content and its three files.
        for suffix in ["content.json", "rec", "hashes", "vector.txt"] {
            let line = format!("{name}.{suffix}: bytes ");
            assert_eq!(manifest.matches(&line).count(), 1, "{line}");
        }
        let bytes = read(&format!("{name}.rec"));
        let recording = Recording::decode(&bytes).expect("a committed recording reads");
        // Its header names the frozen content beside it, by its sha256, and the map it is played on.
        let content = frozen(&folder(), name).unwrap_or_else(|why| panic!("{why}"));
        assert_eq!(recording.header.content_hash, sha256(&content));
        assert_eq!(recording.header.map_id, map_id);
        let hashes = String::from_utf8(read(&format!("{name}.hashes"))).expect("text");
        let expected = parse_hashes(&bytes, &hashes).expect("its hashes file reads");
        assert_eq!(expected.len(), recording.steps.len());
        let mut replay = Replay::new(&recording, &content).expect("this build replays it");
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
        assert_eq!(vector_text(&recording, &content, VECTOR_TICK), Ok(vector));
    }
}

/// A copy of the recordings' folder with one thing done to it, in the test run's own scratch folder: the frozen
/// content as it is committed, and every file the scripts make from it.
fn changed(case: &str, change: impl Fn(&Path)) -> Result<(), String> {
    let copy = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("recordings-{case}"));
    let _ = std::fs::remove_dir_all(&copy);
    std::fs::create_dir_all(&copy).expect("a scratch folder");
    for (name, _, _) in RECORDINGS {
        let content = frozen(&folder(), name).unwrap_or_else(|why| panic!("{why}"));
        std::fs::write(copy.join(frozen_name(name)), content).expect("a scratch file");
    }
    for (name, bytes) in made(&folder()).expect("the scripts make their files") {
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
    // One bit of one step's input: the header's length is the file's less its 1,800 steps of 68 bytes.
    let header = read("first-loop.rec").len() - 1_800 * 68;
    let rec =
        changed("rec", flip("first-loop.rec", header + 900 * 68 + 9)).expect_err("a changed .rec");
    assert_eq!(rec, "first-loop.rec differs");
    // The frozen content gone, or one bit of it changed: the scripts cannot make the recording on it, by name.
    let gone = changed("content-gone", |folder| {
        std::fs::remove_file(folder.join("first-loop.content.json")).expect("the file goes");
    })
    .expect_err("no frozen content");
    assert!(
        gone.starts_with("first-loop: its frozen content: first-loop.content.json: "),
        "{gone}"
    );
    let content = changed("content", flip("first-loop.content.json", 40))
        .expect_err("a changed frozen content");
    assert!(
        content.starts_with("first-loop: the content is not the recording's: "),
        "{content}"
    );
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
    // `read_plain` is one call of `read_plain_within` with the folder's own cap; the open is in the second.
    let wrapper = library
        .split_once("pub fn read_plain(")
        .and_then(|(_, rest)| rest.split_once("\n}\n"));
    assert!(wrapper.is_some_and(|(body, _)| {
        body.trim_end()
            .ends_with("read_plain_within(folder, name, MAX_READ_BYTES)")
    }));
    let plain_read = library
        .split_once("pub fn read_plain_within(")
        .and_then(|(_, rest)| rest.split_once("\n}\n"));
    // One place opens a file to read it, inside `read_plain_within`; nothing reads a path whole, or opens one
    // another way. One place makes a file, and only under a name that is not there yet (`replace_plain`).
    let (open, whole) = (["File", "::open("].concat(), format!("{read}("));
    let made = ["File", "::options()"].concat();
    assert_eq!(library.matches(&made).count(), 1);
    assert!(
        library.split_once(&made).is_some_and(
            |(_, rest)| rest.contains(".create_new(true)") && !rest.contains(".read(true)")
        ),
        "a file made any other way than new"
    );
    let others = [
        ["read_to", "_string"].concat(),
        ["Open", "Options"].concat(),
    ];
    assert!(
        others.iter().all(|way| !library.contains(way)),
        "{others:?}"
    );
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
