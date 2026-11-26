//! The committed recordings as files (`crates/devtools/recordings/`): what the scripts make, written there by
//! `make_recordings` and compared with what is there by `make_recordings --check` and by the plain test run, byte
//! for byte. Per recording: `<name>.rec`, `<name>.hashes`, and `<name>.vector.txt`, the state at one tick field by
//! field and its shared bytes (the word-size vector: the same text on a 32-bit and a 64-bit target). `manifest.txt` lists
//! every file with its length and SHA-256.

use std::path::Path;

use sim::state_hash::fnv1a_64;

use crate::compare::{Column, first_difference};
use crate::recording::{Recording, hashes_text, manifest_line, parse_hashes};
use crate::replay::{replay, shared_bytes_at, state_at};
use crate::scripts::{RECORDINGS, VECTOR_TICK};
use crate::sha256::hex;

pub const MANIFEST: &str = "manifest.txt";

/// The vector file's text: the state at `tick` as lines, one named field each, then the shared state's canonical
/// bytes there as hex and their hash. The same text must come out natively and in WebAssembly.
///
/// # Errors
/// A recording that does not replay or never reaches the tick, in plain words.
pub fn vector_text(recording: &Recording, tick: u32) -> Result<String, String> {
    let never = || format!("the recording never reaches tick {tick}");
    let state = state_at(recording, tick).map_err(|error| error.to_string())?;
    let bytes = shared_bytes_at(recording, tick).map_err(|error| error.to_string())?;
    let (state, bytes) = (state.ok_or_else(never)?, bytes.ok_or_else(never)?);
    Ok(format!(
        "{state}tick {tick} shared bytes {} fnv1a64 {:016x}\n",
        hex(&bytes),
        fnv1a_64(&bytes)
    ))
}

/// Every file the scripts make, by name, in the manifest's order, the manifest last.
///
/// # Errors
/// A script whose recording has no file form or does not replay: a bug in the script, said in plain words.
pub fn made() -> Result<Vec<(String, Vec<u8>)>, String> {
    let mut files = Vec::new();
    for (name, script) in RECORDINGS {
        let recording = script();
        let bytes = recording
            .encode()
            .map_err(|error| format!("{name}: {error}"))?;
        let lines = replay(&recording).map_err(|error| format!("{name}: {error}"))?;
        let hashes = hashes_text(&bytes, &lines);
        let vector =
            vector_text(&recording, VECTOR_TICK).map_err(|error| format!("{name}: {error}"))?;
        files.push((format!("{name}.rec"), bytes));
        files.push((format!("{name}.hashes"), hashes.into_bytes()));
        files.push((format!("{name}.vector.txt"), vector.into_bytes()));
    }
    let manifest: String = files
        .iter()
        .map(|(name, bytes)| manifest_line(name, bytes))
        .collect();
    files.push((MANIFEST.to_owned(), manifest.into_bytes()));
    Ok(files)
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

/// A committed file's bytes, read only when the folder and the file are what they are by their own type: a plain
/// folder and a plain file in it, never a link (Mustafa's LINKS-CHECK (c), 2026-11-26: the tests that read the
/// recordings refuse anything that is not a plain file, as the folder check does). Every test that reads a recording
/// reads it through here; `tests/recordings.rs` scans their sources for another way in.
///
/// # Errors
/// A name that is a path, a folder or a file that is a link or of another type, a file that is not there or is
/// larger than a read takes, each by name.
pub fn read_plain(folder: &Path, name: &str) -> Result<Vec<u8>, String> {
    if name.is_empty() || name == "." || name == ".." || name.contains(['/', '\\']) {
        return Err(format!("{name:?} is not a file's name"));
    }
    plain(folder, "")?;
    plain(folder, name)?;
    let path = folder.join(name);
    let found = std::fs::symlink_metadata(&path).map_err(|error| format!("{name}: {error}"))?;
    if found.len() > MAX_READ_BYTES {
        return Err(format!(
            "{name}: {} bytes, more than a read takes",
            found.len()
        ));
    }
    std::fs::read(&path).map_err(|error| format!("{name}: {error}"))
}

/// Writes every file into `folder`, which must exist. Making the recordings again is deliberate: it is done in the
/// commit that changes the simulation on purpose, after `--check` has named the first step that moved, and the
/// change-set's report says why; a change-set that means no change leaves every file byte for byte as it was.
///
/// # Errors
/// What `made` gives, a path that is a link or no plain file, or a file that could not be written.
pub fn write(folder: &Path) -> Result<Vec<String>, String> {
    let files = made()?;
    plain(folder, "")?;
    for (name, bytes) in &files {
        plain(folder, name)?;
        std::fs::write(folder.join(name), bytes).map_err(|error| format!("{name}: {error}"))?;
    }
    Ok(files.into_iter().map(|(name, _)| name).collect())
}

/// Compares `folder` with what the scripts make and writes nothing: every file there, byte for byte, and no file
/// there that the scripts do not make.
///
/// # Errors
/// Each file that is missing, differs, is not listed or is no plain file, by name; for a hashes file, the first step
/// that differs.
pub fn check(folder: &Path) -> Result<(), String> {
    let files = made()?;
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
    for name in found {
        if !files.iter().any(|(made, _)| *made == name) {
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
