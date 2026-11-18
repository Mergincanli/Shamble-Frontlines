//! The export test of the message types (ADR 0002 part 5): ts-rs writes their TypeScript into a new scratch folder,
//! and the test fails when `packages/types/src/generated/protocol` does not hold the same files. To write that
//! folder again: `SHAMBLE_WRITE_TYPES=1 cargo test -p protocol`. Both folders are named here: no `#[ts(export)]`,
//! no `TS_RS_EXPORT_DIR` and no build file decides where ts-rs writes.

use crate::messages::{Input, Join, JoinAccepted, Snapshot};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use ts_rs::{Config, TS};

const GENERATED_FOLDER: &str = "packages/types/src/generated/protocol";

/// A new, empty folder for this run. It is always made here and never reused: a name that exists is skipped.
fn new_scratch_folder() -> PathBuf {
    let base = std::env::temp_dir();
    let pid = std::process::id();
    (0..100)
        .map(|n| base.join(format!("shamble-types-protocol-{pid}-{n}")))
        .find(|folder| fs::create_dir(folder).is_ok())
        .expect("a scratch folder can be made in the temporary folder")
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

/// Write mode: clears the generated folder of its `.ts` files (only those directly in it) and writes the new ones.
fn write_generated(folder: &Path, files: &BTreeMap<String, Vec<u8>>) {
    assert!(
        folder.ends_with(GENERATED_FOLDER),
        "write mode only writes into {GENERATED_FOLDER}"
    );
    fs::create_dir_all(folder).expect("the generated folder can be made");
    for entry in fs::read_dir(folder).expect("the generated folder can be read") {
        let path = entry.expect("its entries can be read").path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "ts") {
            fs::remove_file(&path).expect("an old generated file can be deleted");
        }
    }
    for (name, bytes) in files {
        fs::write(folder.join(name), bytes).expect("a generated file can be written");
    }
}

#[test]
fn generated_types_are_current() {
    let how = "write the folder again with SHAMBLE_WRITE_TYPES=1 cargo test -p protocol";
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

    let folder = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(GENERATED_FOLDER);
    if std::env::var_os("SHAMBLE_WRITE_TYPES").is_some_and(|value| value == "1") {
        write_generated(&folder, &generated);
    }
    let committed = files_in(&folder, how);
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
