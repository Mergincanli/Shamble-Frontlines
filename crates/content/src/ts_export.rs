//! The export test of the content kinds' types (ADR 0002 part 5; ADR 0005 PM-0005-8's types half): ts-rs writes
//! their TypeScript into a new scratch folder, and the test fails when `packages/types/src/generated/content` does
//! not hold the same files. To write that folder again: `SHAMBLE_WRITE_TYPES=1 cargo test -p content`. Both folders
//! are named here: no `#[ts(export)]`, no `TS_RS_EXPORT_DIR` and no build file decides where ts-rs writes.
//!
//! The folder rules are the protocol crate's export test's, copied (`crates/protocol/src/ts_export.rs`): the two
//! crates share no code, because this crate depends on nothing and its one test dependency is the type writer.
//! The types say what a content file holds beside its `format` and `version`: a file's own keys, by their names.

#![cfg(test)]

use crate::map::MapDocument;
use crate::movement::Movement;
use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use ts_rs::{Config, TS};

const GENERATED_FOLDER: &str = "packages/types/src/generated/content";

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
        let folder = base.join(format!("shamble-types-content-{}-{n}", std::process::id()));
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

/// Files by name, to their bytes.
type Files = BTreeMap<String, Vec<u8>>;

/// The one folder the generated folder may hold: the schema files' (`schema.rs`).
const SCHEMA_FOLDER: &str = "schema";

/// The generated folder as committed: its own plain files, and the plain `.json` files of its one folder `schema`.
/// Every entry is judged by its own type, never through a link: a link (named `schema` or not), any other folder, a
/// folder inside `schema`, and a file there that is no `.json` file are each refused, by name.
fn committed_in(folder: &Path) -> Result<(Files, Files), String> {
    let shown = folder.display();
    let read =
        |entry: &fs::DirEntry| fs::read(entry.path()).map_err(|error| format!("{shown}: {error}"));
    let mut types = Files::new();
    let mut schemas = Files::new();
    for entry in fs::read_dir(folder).map_err(|error| format!("{shown} cannot be read: {error}"))? {
        let entry = entry.map_err(|error| format!("{shown}: {error}"))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let kind = entry
            .file_type()
            .map_err(|error| format!("{name}: {error}"))?;
        if kind.is_dir() && name == SCHEMA_FOLDER {
            for inner in fs::read_dir(entry.path()).map_err(|error| format!("{name}: {error}"))? {
                let inner = inner.map_err(|error| format!("{name}: {error}"))?;
                let file = inner.file_name().to_string_lossy().into_owned();
                let plain = inner.file_type().is_ok_and(|kind| kind.is_file());
                if !plain || !file.ends_with(".json") {
                    return Err(format!(
                        "{file} in {shown}/{SCHEMA_FOLDER} is no plain .json file"
                    ));
                }
                schemas.insert(file, read(&inner)?);
            }
        } else if kind.is_file() {
            types.insert(name, read(&entry)?);
        } else {
            return Err(format!(
                "{name} in {shown} is no plain file, nor the one folder {SCHEMA_FOLDER}"
            ));
        }
    }
    Ok((types, schemas))
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

/// Write mode: deletes the generated folder's `.ts` entries and its `schema` folder's `.json` entries (only those
/// directly in each; a link is deleted, never followed) and writes the new files. It makes those two folders if
/// they are missing, and nothing above them; it refuses a path with a link in it, so it never writes outside the
/// repository.
fn write_generated(root: &Path, files: &Files, schemas: &Files) {
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
    let schema_folder = folder.join(SCHEMA_FOLDER);
    if fs::symlink_metadata(&schema_folder).is_err() {
        fs::create_dir(&schema_folder).expect("the schema folder can be made");
    }
    assert!(
        unlinked(&schema_folder),
        "write mode refuses {GENERATED_FOLDER}/{SCHEMA_FOLDER}: it is a link"
    );
    for entry in fs::read_dir(&schema_folder).expect("the schema folder can be read") {
        let entry = entry.expect("its entries can be read");
        let is_folder = entry.file_type().is_ok_and(|kind| kind.is_dir());
        if !is_folder && entry.path().extension().is_some_and(|ext| ext == "json") {
            fs::remove_file(entry.path()).expect("an old schema file can be deleted");
        }
    }
    for (name, bytes) in schemas {
        fs::write(schema_folder.join(name), bytes).expect("a schema file can be written");
    }
}

/// The generated folder's one folder is a real folder named `schema`, of plain `.json` files: a link named `schema`,
/// any other folder, a folder inside `schema`, a `.json` link and a file there that is no `.json` file are each
/// refused, by name (the advisor agent's ruling of 2026-11-27, Q3 C6; S-should 18).
#[cfg(unix)]
#[test]
fn the_generated_folder_admits_one_schema_folder_of_plain_json_files() {
    use std::os::unix::fs::symlink;
    let elsewhere = new_scratch_folder();
    fs::write(elsewhere.join("outside.json"), "{}\n").expect("a file elsewhere");
    let verdict = |make: &dyn Fn(&Path)| {
        let folder = new_scratch_folder();
        fs::write(folder.join("Movement.ts"), "type").expect("a type file");
        make(&folder);
        let verdict = committed_in(&folder);
        fs::remove_dir_all(&folder).expect("the case's folder goes");
        verdict
    };
    let schema = |folder: &Path| {
        let inner = folder.join(SCHEMA_FOLDER);
        fs::create_dir(&inner).expect("the schema folder");
        inner
    };
    let good = verdict(&|folder| {
        fs::write(schema(folder).join("map.schema.json"), "{}\n").expect("a schema file");
    })
    .expect("a schema folder of plain .json files is taken");
    let names = |files: &Files| {
        files
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(" ")
    };
    assert_eq!(
        (names(&good.0), names(&good.1)),
        ("Movement.ts".to_owned(), "map.schema.json".to_owned())
    );
    let refused = |make: &dyn Fn(&Path)| verdict(make).expect_err("refused");
    assert!(
        refused(&|folder| symlink(&elsewhere, folder.join(SCHEMA_FOLDER)).expect("a link"))
            .starts_with("schema in ")
    );
    assert!(
        refused(&|folder| fs::create_dir(folder.join("other")).expect("a folder"))
            .starts_with("other in ")
    );
    assert!(
        refused(&|folder| fs::create_dir(schema(folder).join("inner")).expect("a folder"))
            .starts_with("inner in ")
    );
    assert!(
        refused(&|folder| symlink(
            elsewhere.join("outside.json"),
            schema(folder).join("map.schema.json")
        )
        .expect("a link"))
        .starts_with("map.schema.json in ")
    );
    assert!(
        refused(&|folder| fs::write(schema(folder).join("notes.txt"), "x").expect("a file"))
            .starts_with("notes.txt in ")
    );
    fs::remove_dir_all(&elsewhere).expect("the folder elsewhere goes");
}

#[test]
fn generated_types_are_current() {
    let root = repository_root();
    let scratch = new_scratch_folder();
    let config = Config::new()
        .with_out_dir(&scratch)
        .with_import_extension(Some("ts"));
    // Each call also writes the types its kind holds: `Bounds`, `Spawn`, `Shape`, `Form`, `Toward`, `Region`,
    // `Ground`.
    Movement::export_all(&config).expect("ts-rs can write Movement");
    MapDocument::export_all(&config).expect("ts-rs can write MapDocument");
    let generated = files_in(&scratch, "the scratch folder is this test's own");
    for (name, bytes) in &generated {
        // ts-rs copies a doc comment as it is: one that holds "*/" would end the comment and spill into the code.
        // In its output a comment ends on a line of its own, so "*/" anywhere else comes from a doc comment.
        let text = String::from_utf8_lossy(bytes);
        let spills = text
            .lines()
            .any(|line| line.contains("*/") && line.trim() != "*/");
        assert!(
            !spills,
            "{name}: a doc comment of its Rust type holds \"*/\"; reword it"
        );
    }

    // The schema files (`schema.rs`): the canonical writer's text of each kind's schema.
    let schemas: Files = crate::schema::schema_files()
        .into_iter()
        .map(|(name, text)| (name.to_owned(), text.into_bytes()))
        .collect();

    if std::env::var_os("SHAMBLE_WRITE_TYPES").is_some_and(|value| value == "1") {
        write_generated(&root, &generated, &schemas);
    }
    let how = "delete by hand what does not belong there, then write the folder again with \
               SHAMBLE_WRITE_TYPES=1 cargo test -p content";
    let (committed, committed_schemas) =
        committed_in(&root.join(GENERATED_FOLDER)).unwrap_or_else(|why| panic!("{why}; {how}"));
    for (folder, made, found_files) in [
        (GENERATED_FOLDER.to_owned(), &generated, &committed),
        (
            format!("{GENERATED_FOLDER}/{SCHEMA_FOLDER}"),
            &schemas,
            &committed_schemas,
        ),
    ] {
        for (name, bytes) in made {
            let found = found_files
                .get(name)
                .unwrap_or_else(|| panic!("{folder}/{name} is missing; {how}"));
            assert!(
                found == bytes,
                "{folder}/{name} is not what the Rust code gives; {how}\n\
                 --- the committed file\n{}\n--- from the Rust code\n{}",
                String::from_utf8_lossy(found),
                String::from_utf8_lossy(bytes)
            );
        }
        for name in found_files.keys() {
            assert!(
                made.contains_key(name),
                "{folder}/{name} is not a generated file; {how}"
            );
        }
    }

    // Passed: the scratch folder goes. After a failure it stays, to be looked at.
    for name in generated.keys() {
        fs::remove_file(scratch.join(name)).expect("a scratch file can be deleted");
    }
    fs::remove_dir(&scratch).expect("the scratch folder can be removed");
}
