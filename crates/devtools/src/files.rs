//! The committed recordings as files (`crates/devtools/recordings/`): what the scripts make, written there by
//! `make_recordings` and compared with what is there by `make_recordings --check` and by the plain test run, byte
//! for byte. Per recording: `<name>.content.json`, the frozen content its match runs on (the bundle of `content/` as
//! it was when `make_recordings --freeze <name>` wrote it, its one writer: an input of the scripts, never made
//! again by them); `<name>.rec`, whose header names that bundle by its sha256 and the map it is played on;
//! `<name>.hashes`; and `<name>.vector.txt`, the state at one tick field by field and its shared bytes (the
//! word-size vector: the same text on a 32-bit and a 64-bit target). `manifest.txt` lists every file with its length
//! and SHA-256.

use std::io::Read as _;
use std::path::Path;

use sim::state_hash::fnv1a_64;

use crate::compare::{Column, first_difference};
#[cfg(not(target_arch = "wasm32"))]
use crate::content_files::{bundle, walk};
use crate::recording::{Recording, hashes_text, manifest_line, parse_hashes};
use crate::replay::{replay, shared_bytes_at, state_at};
use crate::scripts::{Content, RECORDINGS, VECTOR_TICK};
use crate::sha256::hex;

pub const MANIFEST: &str = "manifest.txt";

/// The name of a recording's frozen content, beside it.
#[must_use]
pub fn frozen_name(recording: &str) -> String {
    format!("{recording}.content.json")
}

/// The frozen content of the committed recording `name` in `folder`, read through the plain read.
///
/// # Errors
/// The file missing, or no plain file, by name.
pub fn frozen(folder: &Path, name: &str) -> Result<Vec<u8>, String> {
    read_plain(folder, &frozen_name(name))
        .map_err(|why| format!("{name}: its frozen content: {why}"))
}

/// The vector file's text: the state at `tick` as lines, one named field each, then the shared state's canonical
/// bytes there as hex and their hash. The same text must come out natively and in WebAssembly.
///
/// # Errors
/// A recording that does not replay on `bundle` or never reaches the tick, in plain words.
pub fn vector_text(recording: &Recording, bundle: &[u8], tick: u32) -> Result<String, String> {
    let never = || format!("the recording never reaches tick {tick}");
    let state = state_at(recording, bundle, tick).map_err(|error| error.to_string())?;
    let bytes = shared_bytes_at(recording, bundle, tick).map_err(|error| error.to_string())?;
    let (state, bytes) = (state.ok_or_else(never)?, bytes.ok_or_else(never)?);
    Ok(format!(
        "{state}tick {tick} shared bytes {} fnv1a64 {:016x}\n",
        hex(&bytes),
        fnv1a_64(&bytes)
    ))
}

/// Every file the scripts make from the frozen content beside them in `folder`, by name, in the manifest's order,
/// the manifest last. The manifest also lists each recording's frozen content (first among its files), which the
/// scripts read and never make.
///
/// # Errors
/// A frozen content file missing; a script whose recording has no file form or does not replay on its content: a
/// bug in the script, or content that is not the recording's, said in plain words.
pub fn made(folder: &Path) -> Result<Vec<(String, Vec<u8>)>, String> {
    let mut files = Vec::new();
    let mut manifest = String::new();
    for (name, map_id, script) in RECORDINGS {
        let content = frozen(folder, name)?;
        let recording = script(&Content::of(&content, map_id));
        let bytes = recording
            .encode()
            .map_err(|error| format!("{name}: {error}"))?;
        let lines = replay(&recording, &content).map_err(|error| format!("{name}: {error}"))?;
        let hashes = hashes_text(&bytes, &lines);
        let vector = vector_text(&recording, &content, VECTOR_TICK)
            .map_err(|error| format!("{name}: {error}"))?;
        manifest.push_str(&manifest_line(&frozen_name(name), &content));
        for (file, made) in [
            (format!("{name}.rec"), bytes),
            (format!("{name}.hashes"), hashes.into_bytes()),
            (format!("{name}.vector.txt"), vector.into_bytes()),
        ] {
            manifest.push_str(&manifest_line(&file, &made));
            files.push((file, made));
        }
    }
    files.push((MANIFEST.to_owned(), manifest.into_bytes()));
    Ok(files)
}

/// Freezes today's content for the committed recording `name`: the bundle of the content folder `content` (read and
/// judged by the game's loader, as the page's build reads it) is written into `folder` as `<name>.content.json`.
/// It is that file's one writer (`make_recordings --freeze <name>`), run when a recording is meant to move to new
/// content, after which `make_recordings` makes the recording again on it (the change-set's report says why).
///
/// Native only, as the content folder's walk is: the test-only WebAssembly module replays, it never freezes.
///
/// # Errors
/// No recording of that name; the content folder as the content tool refuses it; a file that could not be written.
#[cfg(not(target_arch = "wasm32"))]
pub fn freeze(folder: &Path, name: &str, content: &Path) -> Result<String, String> {
    if !RECORDINGS.iter().any(|(recorded, _, _)| *recorded == name) {
        return Err(format!("no committed recording is named {name:?}"));
    }
    let text = bundle(&walk(content)?)?;
    write_plain(folder, &frozen_name(name), text.as_bytes())?;
    Ok(frozen_name(name))
}

/// Refuses what is in the tree and is no plain file: a link there could lead a read or a write anywhere, and the
/// repository's tree is not trusted. Nothing there yet is fine.
fn plain(folder: &Path, name: &str) -> Result<(), String> {
    // The folder is asked for by its own path: a path that ends in a separator would be followed through a link,
    // so the path is put together again from its parts, which leaves a last separator out.
    let is_folder = name.is_empty();
    let path: std::path::PathBuf = if is_folder {
        folder.components().collect()
    } else {
        folder.join(name)
    };
    match std::fs::symlink_metadata(&path) {
        Ok(found) if found.is_dir() == is_folder && found.is_file() != is_folder => Ok(()),
        Ok(_) => Err(format!(
            "{:?} is a link or not a plain {}",
            path.display(),
            if is_folder { "folder" } else { "file" }
        )),
        Err(_) => Ok(()),
    }
}

/// The most bytes one read of a committed file takes: far above any file of the folder.
pub const MAX_READ_BYTES: u64 = 64 * 1024 * 1024;

/// A name that is one file's name and no path.
fn file_name(name: &str) -> Result<(), String> {
    if name.is_empty() || name == "." || name == ".." || name.contains(['/', '\\']) {
        return Err(format!("{name:?} is not a file's name"));
    }
    Ok(())
}

/// A committed file's bytes, read only when the folder and the file are what they are by their own type: a plain
/// folder and a plain file in it, never a link (Mustafa's LINKS-CHECK (c), 2026-11-26: the tests that read the
/// recordings refuse anything that is not a plain file, as the folder check does). The file is opened once, and what
/// is judged again and read is the opened file itself, at most `MAX_READ_BYTES` of it. The tests that read a
/// committed recording or a committed vector read through here; `tests/recordings.rs` scans the sources of the
/// recordings' readers by name, which holds what those files say and not what a new reader elsewhere does. Two
/// things this does not do. The tests that read the repository's own sources (the manifest rule, the two source
/// scans) read them with `std::fs`: a link there could stall such a test, never hand it a recording. And a link
/// put in the file's place between the check and the open is followed: that takes a second writer on the machine
/// while the test runs, where this guards the tree as it was checked out.
///
/// # Errors
/// A name that is a path, a folder or a file that is a link or of another type, a file that is not there or is
/// larger than a read takes, each by name.
pub fn read_plain(folder: &Path, name: &str) -> Result<Vec<u8>, String> {
    read_plain_within(folder, name, MAX_READ_BYTES)
}

/// As [`read_plain`], for a file whose reader has a smaller limit of its own: at most `cap` bytes are taken, and
/// one byte more is read only to say that the file is larger.
///
/// # Errors
/// As [`read_plain`]; "larger than a read takes" is measured against `cap`.
pub fn read_plain_within(folder: &Path, name: &str, cap: u64) -> Result<Vec<u8>, String> {
    file_name(name)?;
    plain(folder, "")?;
    plain(folder, name)?;
    let said = |error: std::io::Error| format!("{name}: {error}");
    let file = std::fs::File::open(folder.join(name)).map_err(said)?;
    if !file.metadata().map_err(said)?.is_file() {
        return Err(format!("{name:?} is not a plain file"));
    }
    let mut bytes = Vec::new();
    let read = file.take(cap.saturating_add(1)).read_to_end(&mut bytes);
    read.map_err(said)?;
    if bytes.len() as u64 > cap {
        return Err(format!("{name}: more than the {cap} bytes a read takes"));
    }
    Ok(bytes)
}

/// Whether `folder` is there as a folder of its own type: not a link to one, and not anything else.
#[must_use]
pub fn is_plain_folder(folder: &Path) -> bool {
    let path: std::path::PathBuf = folder.components().collect();
    std::fs::symlink_metadata(path).is_ok_and(|found| found.is_dir())
}

/// Puts `bytes` in `name`'s place in `folder`, which must exist: written whole into a new file beside it first
/// (made only if no file of that name is there, never through a link), then renamed over `name`. A reader of
/// `name` sees the old file or the new one, never half of either.
///
/// # Errors
/// A name that is a path, a path that is a link or no plain file, a leftover of an earlier write, or a file that
/// could not be written.
pub fn replace_plain(folder: &Path, name: &str, bytes: &[u8]) -> Result<(), String> {
    use std::io::Write as _;
    file_name(name)?;
    if !is_plain_folder(folder) {
        return Err(format!("{:?} is not a plain folder", folder.display()));
    }
    plain(folder, name)?;
    let new = format!(".{name}.new");
    let said = |error: std::io::Error| format!("{new}: {error}");
    let mut options = std::fs::File::options();
    let mut file = options
        .write(true)
        .create_new(true)
        .open(folder.join(&new))
        .map_err(said)?;
    let moved = file
        .write_all(bytes)
        .and_then(|()| file.sync_all())
        .and_then(|()| std::fs::rename(folder.join(&new), folder.join(name)));
    moved.map_err(|error| {
        // The half-written file goes; if even that fails, the next write names it.
        let _ = std::fs::remove_file(folder.join(&new));
        said(error)
    })
}

/// Writes one file into `folder`, which must exist, never through a link standing in the folder's or the file's
/// place.
///
/// # Errors
/// A name that is a path, a path that is a link or no plain file, or a file that could not be written.
pub fn write_plain(folder: &Path, name: &str, bytes: &[u8]) -> Result<(), String> {
    file_name(name)?;
    plain(folder, "")?;
    plain(folder, name)?;
    std::fs::write(folder.join(name), bytes).map_err(|error| format!("{name}: {error}"))
}

/// Writes every file into `folder`, which must exist. Making the recordings again is deliberate: it is done in the
/// commit that changes the simulation on purpose, after `--check` has named the first step that moved, and the
/// change-set's report says why; a change-set that means no change leaves every file byte for byte as it was.
///
/// # Errors
/// What `made` gives, a path that is a link or no plain file, or a file that could not be written.
pub fn write(folder: &Path) -> Result<Vec<String>, String> {
    let files = made(folder)?;
    for (name, bytes) in &files {
        write_plain(folder, name, bytes)?;
    }
    Ok(files.into_iter().map(|(name, _)| name).collect())
}

/// Compares `folder` with what the scripts make from the frozen content there and writes nothing: every file there,
/// byte for byte, and no file there that the scripts do not make or read.
///
/// # Errors
/// Each file that is missing, differs, is not listed or is no plain file, by name; for a hashes file, the first step
/// that differs.
pub fn check(folder: &Path) -> Result<(), String> {
    let files = made(folder)?;
    plain(folder, "")?;
    let mut wrong = Vec::new();
    for (name, bytes) in &files {
        match read_plain(folder, name) {
            Err(why) => wrong.push(why),
            Ok(found) if found == *bytes => {}
            Ok(found) => wrong.push(format!(
                "{name} differs{}",
                where_hashes_differ(name, &found, &files)
            )),
        }
    }
    let listing =
        std::fs::read_dir(folder).map_err(|error| format!("{}: {error}", folder.display()))?;
    let mut found: Vec<String> = listing
        .filter_map(|entry| Some(entry.ok()?.file_name().to_string_lossy().into_owned()))
        .collect();
    found.sort();
    let read = |name: &str| {
        RECORDINGS
            .iter()
            .any(|(recorded, _, _)| frozen_name(recorded) == name)
    };
    for name in found {
        if !files.iter().any(|(made, _)| *made == name) && !read(&name) {
            wrong.push(format!("{name:?} is in the folder and no script makes it"));
        }
    }
    if wrong.is_empty() {
        Ok(())
    } else {
        Err(wrong.join("\n"))
    }
}

/// For a hashes file that differs from the made one: the first step at which it does, as text.
fn where_hashes_differ(name: &str, found: &[u8], files: &[(String, Vec<u8>)]) -> String {
    let Some(stem) = name.strip_suffix(".hashes") else {
        return String::new();
    };
    let file = |suffix: &str| {
        let wanted = format!("{stem}{suffix}");
        files
            .iter()
            .find(|(name, _)| *name == wanted)
            .map(|(_, bytes)| &bytes[..])
    };
    let (Some(recording), Some(made)) = (file(".rec"), file(".hashes")) else {
        return String::new();
    };
    let parse = |bytes: &[u8]| parse_hashes(recording, &String::from_utf8_lossy(bytes));
    match (parse(made), parse(found)) {
        (Ok(made), Ok(found)) => {
            let shared = first_difference(&found, &made, Column::Shared);
            let whole = first_difference(&found, &made, Column::Whole);
            match shared.or(whole) {
                Some(difference) => format!(": the file against this build, {difference}"),
                None => String::new(),
            }
        }
        (_, Err(why)) => format!(": {why}"),
        (Err(why), _) => format!(": the made text does not read back ({why})"),
    }
}
