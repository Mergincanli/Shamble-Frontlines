//! The export test of the message types (ADR 0002 part 5): ts-rs writes their TypeScript into a new scratch folder,
//! and the test fails when `packages/types/src/generated/protocol` does not hold the same files. To write that
//! folder again: `SHAMBLE_WRITE_TYPES=1 cargo test -p protocol`. Both folders are named here: no `#[ts(export)]`,
//! no `TS_RS_EXPORT_DIR` and no build file decides where ts-rs writes.

use crate::messages::{Input, Join, JoinAccepted, Snapshot};
use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use ts_rs::{Config, TS};

const GENERATED_FOLDER: &str = "packages/types/src/generated/protocol";

/// The repository's root, from the folder `cargo test` runs this crate in. A test binary built in another copy of the
/// repository (two copies sharing one target folder) is stopped here: it would check and write that other copy.
fn repository_root() -> PathBuf {
    let built_in = env!("CARGO_MANIFEST_DIR");
    let run_in = std::env::var("CARGO_MANIFEST_DIR").ok();
    assert_eq!(
        run_in.as_deref(),
        Some(built_in),
        "run this test through cargo test, from the copy of the repository it was built in ({built_in}), with a \
         target folder that no other copy shares"
    );
    Path::new(built_in).join("../..")
}

/// A new, empty folder for this run, in the temporary folder. It is always made here and never reused: a name that
/// exists (a folder, a file, a link) is skipped, and any other error stops the test with its reason.
fn new_scratch_folder() -> PathBuf {
    let base = std::env::temp_dir();
    assert!(
        base.is_absolute(),
        "the temporary folder {} is not an absolute path",
        base.display()
    );
    for n in 0..100 {
        let folder = base.join(format!("shamble-types-protocol-{}-{n}", std::process::id()));
        match fs::create_dir(&folder) {
            Ok(()) => return folder,
            Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
            Err(error) => panic!(
                "the scratch folder {} cannot be made: {error}",
                folder.display()
            ),
        }
    }
    panic!(
        "100 scratch folders of this process already exist in {}",
        base.display()
    );
}

/// The files directly in a folder, name to bytes. Anything else in it (a folder, a link) fails the test.
fn files_in(folder: &Path, how: &str) -> BTreeMap<String, Vec<u8>> {
    let shown = folder.display();
    let entries = fs::read_dir(folder)
        .unwrap_or_else(|error| panic!("{shown} cannot be read ({error}); {how}"));
    let file = |entry: fs::DirEntry| {
        let name = entry.file_name().to_string_lossy().into_owned();
        let plain = entry.file_type().is_ok_and(|kind| kind.is_file());
        assert!(plain, "{name} in {shown} is not a plain file; {how}");
        (
            name,
            fs::read(entry.path()).expect("a generated file can be read"),
        )
    };
    entries
        .map(|entry| file(entry.expect("its entries can be read")))
        .collect()
}

/// Write mode: deletes the generated folder's `.ts` entries (only those directly in it; a link is deleted, never
/// followed) and writes the new files. It makes that one folder if it is missing, and nothing above it; it refuses
/// a path with a link in it, so it never writes outside the repository.
fn write_generated(root: &Path, files: &BTreeMap<String, Vec<u8>>) {
    let root = fs::canonicalize(root).expect("the repository's root can be resolved");
    let folder = root.join(GENERATED_FOLDER);
    let parent = folder.parent().expect("the generated folder has a parent");
    let unlinked = |path: &Path| fs::canonicalize(path).is_ok_and(|real| real == path);
    assert!(
        unlinked(parent),
        "write mode needs {} to exist, with no link in its path",
        parent.display()
    );
    if fs::symlink_metadata(&folder).is_err() {
        fs::create_dir(&folder).expect("the generated folder can be made");
    }
    assert!(
        unlinked(&folder),
        "write mode refuses {GENERATED_FOLDER}: it is a link"
    );
    for entry in fs::read_dir(&folder).expect("the generated folder can be read") {
        let entry = entry.expect("its entries can be read");
        let is_folder = entry.file_type().is_ok_and(|kind| kind.is_dir());
        if !is_folder && entry.path().extension().is_some_and(|ext| ext == "ts") {
            fs::remove_file(entry.path()).expect("an old generated file can be deleted");
        }
    }
    for (name, bytes) in files {
        fs::write(folder.join(name), bytes).expect("a generated file can be written");
    }
}

#[test]
fn generated_types_are_current() {
    let root = repository_root();
    let scratch = new_scratch_folder();
    let config = Config::new()
        .with_out_dir(&scratch)
        .with_import_extension(Some("ts"));
    // Each call also writes the types its message holds: `Command`, `SnapshotAck`, `AimPoint`, `ViewTime`.
    Join::export_all(&config).expect("ts-rs can write Join");
    JoinAccepted::export_all(&config).expect("ts-rs can write JoinAccepted");
    Input::export_all(&config).expect("ts-rs can write Input");
    Snapshot::export_all(&config).expect("ts-rs can write Snapshot");
    let generated = files_in(&scratch, "the scratch folder is this test's own");
    for (name, bytes) in &generated {
        // ts-rs copies a doc comment as it is: one that holds "*/" would end the comment and spill into the code.
        let text = String::from_utf8_lossy(bytes);
        let (opened, closed) = (text.matches("/**").count(), text.matches("*/").count());
        assert_eq!(
            opened, closed,
            "{name}: a doc comment of its Rust type holds \"*/\"; reword it"
        );
    }

    if std::env::var_os("SHAMBLE_WRITE_TYPES").is_some_and(|value| value == "1") {
        write_generated(&root, &generated);
    }
    let how = "delete by hand what does not belong there, then write the folder again with \
               SHAMBLE_WRITE_TYPES=1 cargo test -p protocol";
    let committed = files_in(&root.join(GENERATED_FOLDER), how);
    for (name, bytes) in &generated {
        let found = committed
            .get(name)
            .unwrap_or_else(|| panic!("{GENERATED_FOLDER}/{name} is missing; {how}"));
        assert!(
            found == bytes,
            "{GENERATED_FOLDER}/{name} is not what the Rust types give; {how}\n\
             --- the committed file\n{}\n--- from the Rust types\n{}",
            String::from_utf8_lossy(found),
            String::from_utf8_lossy(bytes)
        );
    }
    for name in committed.keys() {
        assert!(
            generated.contains_key(name),
            "{GENERATED_FOLDER}/{name} is not a generated file; {how}"
        );
    }

    // Passed: the scratch folder goes. After a failure it stays, to be looked at.
    for name in generated.keys() {
        fs::remove_file(scratch.join(name)).expect("a scratch file can be deleted");
    }
    fs::remove_dir(&scratch).expect("the scratch folder can be removed");
}
