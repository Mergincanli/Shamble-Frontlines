//! The bundle: every content file of a match in one document, and the loader that takes it or refuses it whole.
//!
//! A bundle is `{"files": [{"path": "movement/soldier.json", "text": "<the file's exact text>"}, ...], "format":
//! "shamble-content-bundle", "version": 1}`, itself in canonical form, its files in path order. Each file's own text
//! sits inside as a string, so a refusal names the file's own line and column, and a script can put a bundle
//! together byte for byte as the tool does.
//!
//! The loader's stages, each of which refuses by name: the byte caps; the grammar (`json.rs`); the canonical form;
//! the bundle's own rules (a path is `<kind's folder>/<id>.json`, each once, in order); each file's format and
//! version; its fields and ranges (`read.rs`); the whole set (a file's id is its file name, so one definition per id
//! and kind). There is no fallback anywhere: a bundle is taken whole or not at all.

use crate::error::{Echo, Error, Kind};
use crate::json::{self, Member, Node, Value, is_id};
use crate::map::{self, MapDocument};
use crate::movement::{self, Movement};
use crate::read::Object;
use crate::write::{canonical, first_difference};

/// The most bytes a bundle may have, and the most one file's text may have.
pub const MAX_BUNDLE_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_FILE_BYTES: usize = 1024 * 1024;
/// The most files one bundle may hold.
pub const MAX_FILES: u32 = 16;
/// The `format` of a bundle, and the newest `version` this build reads.
pub const FORMAT: &str = "shamble-content-bundle";
pub const VERSION: u32 = 1;

/// The kinds of content, by the folder their files lie in. A path names its kind; no other folder is a path. (The
/// tool that reads the files under `content/` walks these folders and no other.)
pub const FOLDERS: [&str; 2] = ["maps", "movement"];

/// One content file, read and checked.
#[derive(Debug, Clone, PartialEq)]
pub enum Document {
    Map(MapDocument),
    Movement(Movement),
}

impl Document {
    pub fn id(&self) -> &str {
        match self {
            Self::Map(map) => &map.id,
            Self::Movement(movement) => &movement.id,
        }
    }
}

/// Every content file of a match, read, checked and never changed afterwards.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ContentSet {
    maps: Vec<MapDocument>,
    movements: Vec<Movement>,
}

impl ContentSet {
    pub fn maps(&self) -> &[MapDocument] {
        &self.maps
    }

    pub fn map(&self, id: &str) -> Option<&MapDocument> {
        self.maps.iter().find(|map| map.id == id)
    }

    pub fn movements(&self) -> &[Movement] {
        &self.movements
    }

    pub fn movement(&self, id: &str) -> Option<&Movement> {
        self.movements.iter().find(|movement| movement.id == id)
    }
}

/// The line and column (in bytes, from 1) of the byte at `offset` in `text`.
fn place(text: &[u8], offset: usize) -> (u32, u32) {
    let before = text.get(..offset).unwrap_or(text);
    let line = before.iter().filter(|&&byte| byte == b'\n').count();
    let column = before
        .iter()
        .rev()
        .take_while(|&&byte| byte != b'\n')
        .count();
    let from_one = |count: usize| u32::try_from(count).unwrap_or(u32::MAX).saturating_add(1);
    (from_one(line), from_one(column))
}

/// One document as the loader takes it: within `cap` bytes, in the grammar, and byte for byte in canonical form.
pub fn document(bytes: &[u8], cap: usize) -> Result<Node, Error> {
    if bytes.len() > cap {
        let count = |bytes: usize| u64::try_from(bytes).unwrap_or(u64::MAX);
        return Err(Error::whole(Kind::TooLarge {
            bytes: count(bytes.len()),
            cap: count(cap),
        }));
    }
    let top = json::parse(bytes)?;
    match first_difference(&top, bytes) {
        None => Ok(top),
        Some(difference) => {
            let (line, col) = place(bytes, difference.offset);
            let kind = Kind::NotCanonical {
                expected: Echo::of(&difference.expected),
                found: Echo::of(&difference.found),
            };
            Err(Error::at(line, col, "", kind))
        }
    }
}

/// A path's folder and id: `<folder>/<id>.json`, the folder a kind's own.
fn folder_and_id(path: &str) -> Option<(&'static str, &str)> {
    let (folder, file) = path.split_once('/')?;
    let folder = FOLDERS.into_iter().find(|known| *known == folder)?;
    let id = file.strip_suffix(".json")?;
    is_id(id).then_some((folder, id))
}

/// One content file as the loader reads it: its path, its text's form, its kind's fields, and its id against its
/// file name. The tool that checks the files under `content/` calls this, so it and the loader cannot disagree.
pub fn load_file(path: &str, text: &[u8]) -> Result<Document, Error> {
    let Some((folder, id)) = folder_and_id(path) else {
        return Err(Error::whole(Kind::BadPath {
            path: Echo::of(path.as_bytes()),
        }));
    };
    let read = || -> Result<Document, Error> {
        let top = document(text, MAX_FILE_BYTES)?;
        // One arm per folder of `FOLDERS`.
        let document = match folder {
            "maps" => Document::Map(map::read(&top)?),
            _ => Document::Movement(movement::read(&top)?),
        };
        if document.id() != id {
            return Err(Error::at(
                0,
                0,
                "id",
                Kind::IdNotFileName {
                    id: Echo::of(document.id().as_bytes()),
                },
            ));
        }
        Ok(document)
    };
    read().map_err(|error| error.in_file(path))
}

/// The whole loader: a bundle's bytes to the match's content, or the first refusal.
pub fn load(bytes: &[u8]) -> Result<ContentSet, Error> {
    let top = document(bytes, MAX_BUNDLE_BYTES)?;
    let mut bundle = Object::of(&top, "")?;
    let (files, files_path) = bundle.array("files", u32::MAX)?;
    let found = bundle.text("format")?;
    if found != FORMAT {
        return Err(Error::at(
            0,
            0,
            "format",
            Kind::WrongFormat {
                found: Echo::of(found.as_bytes()),
                expected: FORMAT,
            },
        ));
    }
    let (line, col) = (top.line, top.col);
    bundle
        .whole("version", 1, VERSION)
        .map_err(|error| match error.kind {
            Kind::NotAWholeNumber { bits } => Error::at(
                error.line,
                error.col,
                "version",
                Kind::WrongVersion {
                    bits,
                    newest: VERSION,
                },
            ),
            _ => error,
        })?;
    bundle.finish()?;
    let len = u32::try_from(files.len()).unwrap_or(u32::MAX);
    if len > MAX_FILES {
        return Err(Error::at(
            line,
            col,
            &files_path,
            Kind::TooManyFiles {
                len,
                cap: MAX_FILES,
            },
        ));
    }
    let mut set = ContentSet::default();
    let mut last: Option<&str> = None;
    for (at, entry) in files.iter().enumerate() {
        let entry_path = format!("{files_path}[{at}]");
        let mut object = Object::of(entry, &entry_path)?;
        let path = object.text("path")?;
        let text = object.text("text")?;
        object.finish()?;
        // In path order, each path once: a second file for one path could never be told from the first.
        if last.is_some_and(|last| last.as_bytes() >= path.as_bytes()) {
            return Err(Error::at(
                entry.line,
                entry.col,
                &entry_path,
                Kind::PathOrder {
                    path: Echo::of(path.as_bytes()),
                },
            ));
        }
        last = Some(path);
        match load_file(path, text.as_bytes())? {
            Document::Map(map) => set.maps.push(map),
            Document::Movement(movement) => set.movements.push(movement),
        }
    }
    Ok(set)
}

/// A bundle's text from its files (`(path, text)` pairs, in any order): what the tool writes into `content.json`.
/// The text is canonical by construction; whether the files are content is [`load`]'s to say.
pub fn write(files: &[(&str, &str)]) -> String {
    let mut sorted: Vec<&(&str, &str)> = files.iter().collect();
    sorted.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    let node = |value| Node {
        line: 0,
        col: 0,
        value,
    };
    let member = |key: &str, value| Member {
        key: key.to_owned(),
        line: 0,
        col: 0,
        node: node(value),
    };
    let entries = sorted
        .into_iter()
        .map(|(path, text)| {
            node(Value::Object(vec![
                member("path", Value::Text((*path).to_owned())),
                member("text", Value::Text((*text).to_owned())),
            ]))
        })
        .collect();
    canonical(&node(Value::Object(vec![
        member("files", Value::Array(entries)),
        member("format", Value::Text(FORMAT.to_owned())),
        member("version", Value::Number(f64::from(VERSION))),
    ])))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::movement::TEST_TEXT;

    fn kind_of(bundle: &str) -> Kind {
        load(bundle.as_bytes()).expect_err("refused").kind
    }

    #[test]
    fn a_bundle_written_by_the_writer_loads_and_holds_its_files() {
        let bundle = write(&[("movement/test-soldier.json", TEST_TEXT)]);
        assert!(bundle.starts_with("{\n  \"files\": [\n    {\n      \"path\": \"movement/test-soldier.json\",\n      \"text\": \"{\\n  \\\"ads_speed_multiplier\\\": 0.5,"));
        assert!(
            bundle.ends_with(
                "\n  ],\n  \"format\": \"shamble-content-bundle\",\n  \"version\": 1\n}\n"
            )
        );
        let set = load(bundle.as_bytes()).expect("a valid bundle");
        assert_eq!(set.movements().len(), 1);
        assert_eq!(
            set.movement("test-soldier")
                .map(|movement| movement.walk_speed_m_per_s),
            Some(6.0)
        );
        assert_eq!(set.movement("soldier"), None);
        // An empty bundle is a bundle: which files a match needs is the match's to say, not the loader's.
        assert_eq!(load(write(&[]).as_bytes()), Ok(ContentSet::default()));
        // The writer sorts by path, whatever order it is given.
        let second = TEST_TEXT.replace("test-soldier", "a");
        assert_eq!(
            write(&[
                ("movement/test-soldier.json", TEST_TEXT),
                ("movement/a.json", &second)
            ]),
            write(&[
                ("movement/a.json", &second),
                ("movement/test-soldier.json", TEST_TEXT)
            ])
        );
    }

    #[test]
    fn a_bundle_holds_each_kind_in_its_own_folder() {
        let map =
            canonical(&json::parse(crate::map::tests::TEXT.as_bytes()).expect("the test map"));
        let files = [
            ("movement/test-soldier.json", TEST_TEXT),
            ("maps/yard.json", map.as_str()),
        ];
        let set = load(write(&files).as_bytes()).expect("a valid bundle");
        assert_eq!((set.maps().len(), set.movements().len()), (1, 1));
        assert_eq!(set.map("yard").map(|map| map.shapes.len()), Some(2));
        assert_eq!(set.map("test-soldier"), None);
        // A file in another kind's folder is read as that kind, and refused by its format.
        let moved = [
            write(&[("maps/test-soldier.json", TEST_TEXT)]),
            write(&[("movement/yard.json", &map)]),
        ];
        let expected = moved.map(|bundle| match kind_of(&bundle) {
            Kind::WrongFormat { expected, .. } => expected,
            _ => "another refusal",
        });
        assert_eq!(expected, ["shamble-map", "shamble-movement"]);
        // A map's own rule refuses the whole bundle, with the file and the field: bounds 400 m out on x.
        let far = map.replacen("      50,\n", "      400,\n", 1);
        let error =
            load(write(&[("maps/yard.json", &far)]).as_bytes()).expect_err("beyond the aim");
        let beyond = Kind::BeyondAim {
            bits: 400.0_f64.to_bits(),
            reach_cm: 32_767,
        };
        assert_eq!(
            (error.file.as_deref(), error.path.as_str(), error.kind),
            (Some("maps/yard.json"), "bounds.max[0]", beyond)
        );
    }

    #[test]
    fn the_bundles_own_rules_refuse_by_name() {
        let with = |path: &str, text: &str| write(&[(path, text)]);
        for (path, kind) in [
            ("movement/../movement/test-soldier.json", "BadPath"),
            ("../test-soldier.json", "BadPath"),
            ("source/testlines-export.json", "BadPath"),
            ("weapons/test-soldier.json", "BadPath"),
            ("map/test-soldier.json", "BadPath"),
            ("Movement/test-soldier.json", "BadPath"),
            // A backslash cannot stand in a content string at all: the reader refuses the escape itself.
            ("movement\\test-soldier.json", "RefusedEscape { byte: 92 }"),
            ("movement/test-soldier.JSON", "BadPath"),
            ("movement/test-soldier", "BadPath"),
            ("movement/sub/test-soldier.json", "BadPath"),
            ("/movement/test-soldier.json", "BadPath"),
            ("movement/-soldier.json", "BadPath"),
            ("movement/.json", "BadPath"),
            ("movement/other.json", "IdNotFileName"),
        ] {
            let found = kind_of(&with(path, TEST_TEXT));
            assert!(format!("{found:?}").starts_with(kind), "{path}: {found:?}");
        }
        // A text that is itself a bundle, and a text of another kind: refused by the format the folder asks for.
        assert!(matches!(
            kind_of(&with("movement/test-soldier.json", &write(&[]))),
            Kind::MissingKey { .. } | Kind::WrongFormat { .. }
        ));
        // The same path twice, and two paths out of order: the writer sorts, so these are written by hand.
        let entry = |path: &str| {
            format!(
                "    {{\n      \"path\": \"{path}\",\n      \"text\": {}\n    }}",
                canonical_string(
                    &TEST_TEXT.replace(
                        "test-soldier",
                        path.trim_start_matches("movement/")
                            .trim_end_matches(".json")
                    )
                )
            )
        };
        let by_hand = |entries: &[String]| {
            format!(
                "{{\n  \"files\": [\n{}\n  ],\n  \"format\": \"shamble-content-bundle\",\n  \"version\": 1\n}}\n",
                entries.join(",\n")
            )
        };
        assert!(
            load(by_hand(&[entry("movement/a.json"), entry("movement/b.json")]).as_bytes()).is_ok()
        );
        assert!(matches!(
            kind_of(&by_hand(&[
                entry("movement/a.json"),
                entry("movement/a.json")
            ])),
            Kind::PathOrder { .. }
        ));
        assert!(matches!(
            kind_of(&by_hand(&[
                entry("movement/b.json"),
                entry("movement/a.json")
            ])),
            Kind::PathOrder { .. }
        ));
        // An extra key in the bundle or in an entry; another format; another version.
        let good = with("movement/test-soldier.json", TEST_TEXT);
        assert!(matches!(
            kind_of(&good.replace("  \"format\"", "  \"fog\": 1,\n  \"format\"")),
            Kind::UnknownKey { .. }
        ));
        assert!(matches!(
            kind_of(&good.replace("      \"text\"", "      \"size\": 1,\n      \"text\"")),
            Kind::UnknownKey { .. }
        ));
        assert!(matches!(
            kind_of(&good.replace("shamble-content-bundle", "shamble-movement")),
            Kind::WrongFormat { .. }
        ));
        assert!(matches!(
            kind_of(&good.replace("\"version\": 1\n}", "\"version\": 2\n}")),
            Kind::WrongVersion { newest: 1, .. }
        ));
        // More files than the cap.
        let many: Vec<String> = (0..=MAX_FILES)
            .map(|at| entry(&format!("movement/f{at:02}.json")))
            .collect();
        assert_eq!(
            kind_of(&by_hand(&many)),
            Kind::TooManyFiles {
                len: MAX_FILES + 1,
                cap: MAX_FILES
            }
        );
        assert!(load(by_hand(many.get(1..).expect("17 entries")).as_bytes()).is_ok());
    }

    /// A text as a JSON string in canonical form (for bundles written by hand in these tests).
    fn canonical_string(text: &str) -> String {
        format!("\"{}\"", text.replace('"', "\\\"").replace('\n', "\\n"))
    }

    #[test]
    fn a_refusal_inside_a_file_names_the_file_and_its_own_line() {
        let broken = TEST_TEXT.replace("\"step_up_m\": 0.5", "\"step_up_m\": 1.6");
        let error = load(write(&[("movement/test-soldier.json", &broken)]).as_bytes())
            .expect_err("out of range");
        assert_eq!(error.file.as_deref(), Some("movement/test-soldier.json"));
        assert_eq!(
            (error.line, error.col, error.path.as_str()),
            (16, 16, "step_up_m")
        );
        // A file that is valid but not canonical: refused, with the place of its first wrong byte.
        let loose = TEST_TEXT.replace("\"version\": 1,", "\"version\": 1 ,");
        let error = load_file("movement/test-soldier.json", loose.as_bytes())
            .expect_err("a space before a comma");
        assert!(matches!(error.kind, Kind::NotCanonical { .. }));
        assert_eq!((error.line, error.col), (18, 15));
        assert_eq!(error.file.as_deref(), Some("movement/test-soldier.json"));
    }

    #[test]
    fn the_byte_caps_refuse_before_anything_is_read() {
        // One byte over a file's cap, and a bundle one byte over its own: neither is parsed (the text is not even
        // a document).
        let file = vec![b'x'; MAX_FILE_BYTES + 1];
        assert_eq!(
            load_file("movement/a.json", &file)
                .expect_err("too large")
                .kind,
            Kind::TooLarge {
                bytes: MAX_FILE_BYTES as u64 + 1,
                cap: MAX_FILE_BYTES as u64
            }
        );
        assert!(matches!(
            load_file("movement/a.json", &file[1..])
                .expect_err("not a document")
                .kind,
            Kind::UnexpectedByte { .. }
        ));
        let bundle = vec![b'x'; MAX_BUNDLE_BYTES + 1];
        assert_eq!(
            load(&bundle).expect_err("too large").kind,
            Kind::TooLarge {
                bytes: MAX_BUNDLE_BYTES as u64 + 1,
                cap: MAX_BUNDLE_BYTES as u64
            }
        );
        assert!(matches!(
            load(&bundle[1..]).expect_err("not a document").kind,
            Kind::UnexpectedByte { .. }
        ));
    }

    #[test]
    fn a_place_is_a_line_and_a_column_from_one() {
        assert_eq!(place(b"ab\ncd", 0), (1, 1));
        assert_eq!(place(b"ab\ncd", 2), (1, 3));
        assert_eq!(place(b"ab\ncd", 3), (2, 1));
        assert_eq!(place(b"ab\ncd", 5), (2, 3));
        assert_eq!(place(b"ab\ncd", 99), (2, 3));
    }
}
