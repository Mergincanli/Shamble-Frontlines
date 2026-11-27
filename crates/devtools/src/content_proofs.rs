//! The content reader's proofs (content items 1 and 2; PM-0005-1, -2, -4, -6), written once and run natively and,
//! through the test-only module, in WebAssembly under Node and in the browsers:
//!
//! - the number corpus and the tree corpus ([`corpus_check`], [`trees_check`]): the reader gives the bits, and the
//!   writer the text, that a browser engine's own `JSON.parse` and `JSON.stringify` gave for the same input. Both
//!   corpora are made by `packages/client/scripts/corpus-texts.mjs`, never by the code they test;
//! - the broken files ([`verdicts`]): a seeded generator makes files and bundles class by class, each with the
//!   verdict it must get, worked out by the generator itself: the reason, the file, the line and column, the field.
//!   A class's verdicts and messages are folded into one digest and the digests are a committed file, so the native
//!   and the WebAssembly verdicts are compared record for record;
//! - the inputs that lean on the caps ([`LIMIT_CASES`], [`limit_case`]) and the bounded fuzz run's inputs and judge
//!   ([`fuzz_input`], [`fuzz_judge`]), which `tests/content_fuzz.rs` runs by the million.
//!
//! They live here and not in `content`'s own tests because the WebAssembly side runs the same generator, and
//! `content` depends on nothing.

use std::fmt::Write as _;

use content::bundle::{self, MAX_BUNDLE_BYTES, MAX_FILE_BYTES, MAX_FILES};
use content::error::{Echo, Error, Kind, MESSAGE_BYTES};
use content::json::{self, Value};
use content::map::{
    Bounds, Form, Ground, MAX_REGIONS, MAX_SHAPES, MAX_SPAWNS, MapDocument, Region, Shape, Spawn,
    Toward,
};
use content::movement::{self, FIELDS};
use content::number::{number_text, parse_number};
use content::read::Range;
use content::write::canonical;
use content::{load, load_file};
use sim::state_hash::fnv1a_64;
use sim::{ChaCha8, SEED_BYTES};

pub(crate) fn hold(holds: bool, why: impl FnOnce() -> String) -> Result<(), String> {
    if holds { Ok(()) } else { Err(why()) }
}

/// One line of the number corpus: `<the double's 16 hex digits> <a text of it> <its canonical text>`.
fn corpus_line(line: &str) -> Result<(), String> {
    let mut parts = line.split(' ');
    let (Some(bits), Some(text), Some(expected), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err("not three fields".to_owned());
    };
    let bits = u64::from_str_radix(bits, 16).map_err(|_| "the bits are not hex digits")?;
    let read = |text: &str| {
        parse_number(text.as_bytes()).map_err(|error| format!("{text} is refused: {error:?}"))
    };
    let value = read(text)?;
    hold(value.to_bits() == bits, || {
        format!("the reader gives {:016x}", value.to_bits())
    })?;
    hold(number_text(value) == expected, || {
        format!("the writer writes {}", number_text(value))
    })?;
    // The canonical text reads back as the same number (both zeros are written `0`, which reads as +0).
    let back = read(expected)?;
    hold(back == value && number_text(back) == expected, || {
        format!("the canonical text reads back as {:016x}", back.to_bits())
    })?;
    // And the loader takes the number in its canonical text and in no other.
    let document = |number: &str| -> Result<Option<Value>, Error> {
        let top = bundle::document(format!("{{\n  \"n\": {number}\n}}\n").as_bytes(), 64)?;
        match &top.value {
            Value::Object(members) => Ok(members.first().map(|member| member.node.value.clone())),
            _ => Ok(None),
        }
    };
    hold(document(expected) == Ok(Some(Value::Number(back))), || {
        "the loader does not take the canonical text as that number".to_owned()
    })?;
    let refusal = Kind::NumberNotCanonical {
        text: Echo::of(text.as_bytes()),
        canonical: Echo::of(expected.as_bytes()),
    };
    hold(
        text == expected || document(text).map_err(|error| error.kind) == Err(refusal),
        || "the loader does not refuse the other text by naming the canonical one".to_owned(),
    )
}

/// The first lines of `text`, after its first, that `check` does not pass (five at most), and a first line that is
/// not `<name> 1 <count>`: an empty text when the whole file holds.
fn file_check<'a>(
    name: &str,
    first: &str,
    records: impl Iterator<Item = &'a str>,
    check: impl Fn(&str) -> Result<(), String>,
) -> String {
    let mut wrong = Vec::new();
    let mut count = 0_usize;
    for record in records {
        count += 1;
        if let Err(why) = check(record)
            && wrong.len() < 5
        {
            wrong.push(format!("number {count}, {record:?}: {why}"));
        }
    }
    if first != format!("{name} 1 {count}") {
        wrong.push(format!(
            "the first line is {first:?}, and the file holds {count}"
        ));
    }
    wrong.join("\n")
}

/// The number corpus (`vectors/content-corpus.txt`): an empty text when the reader gives every text the engine's
/// bits and the writer every number the engine's text, or the first lines that differ.
pub fn corpus_check(corpus: &str) -> String {
    let mut lines = corpus.lines();
    let first = lines.next().unwrap_or("");
    file_check("shamble-content-corpus", first, lines, corpus_line)
}

/// The tree corpus (`vectors/content-trees.txt`; records parted by an empty line: a tree as the engine writes it on
/// one line, keys in any order, then as it writes it with sorted keys and two spaces): an empty text when the
/// canonical writer makes the engine's second text from the first, and the loader takes it, or the first that differ.
pub fn trees_check(trees: &str) -> String {
    let (first, rest) = trees.split_once('\n').unwrap_or((trees, ""));
    let records = rest.split("\n\n").filter(|record| !record.is_empty());
    file_check("shamble-content-trees", first, records, |record| {
        let (compact, pretty) = record.split_once('\n').ok_or("no second text")?;
        let pretty = format!("{}\n", pretty.trim_end_matches('\n'));
        let top = json::parse(compact.as_bytes()).map_err(|error| error.record())?;
        hold(canonical(&top) == pretty, || {
            format!("the writer writes {:?}", canonical(&top))
        })?;
        let taken = bundle::document(pretty.as_bytes(), MAX_FILE_BYTES);
        hold(taken.is_ok(), || {
            "the loader refuses the engine's text".to_owned()
        })
    })
}

/// The generator's random numbers: the simulation's own stream type, on a seed and stream numbers of its own.
struct Rng(ChaCha8);

impl Rng {
    fn new(seed: u64, stream: u64) -> Self {
        let mut bytes = [0; SEED_BYTES];
        bytes[..8].copy_from_slice(&seed.to_le_bytes());
        Self(ChaCha8::new(&bytes, stream))
    }

    /// A number below `bound`, which is far below 2^32.
    fn below(&mut self, bound: usize) -> usize {
        ((u64::from(self.0.next_u32()) * bound as u64) >> 32) as usize
    }
}

/// The double next to `value` (never negative here), above it or below it.
fn beside(value: f64, up: bool) -> f64 {
    match (value == 0.0, up) {
        (true, true) => f64::from_bits(1),
        (true, false) => -f64::from_bits(1),
        (false, true) => f64::from_bits(value.to_bits() + 1),
        (false, false) => f64::from_bits(value.to_bits() - 1),
    }
}

/// A number a field takes: anywhere in its range, and often an end of it or the double next to one.
fn in_range(rng: &mut Rng, range: Range) -> f64 {
    match (rng.below(8), range.min_inclusive) {
        (0, _) => range.max,
        (1, true) => range.min,
        (1, false) => beside(range.min, true),
        (2, _) => beside(range.max, false),
        _ => range.min + (range.max - range.min) * (1 + rng.below(4095)) as f64 / 4096.0,
    }
}

/// A piece of input as a refusal shows it, written out again here so that the loader's own echo is held against
/// something: 64 bytes at most, and everything but printable ASCII, the backslash and the quote too, as `\xNN`.
fn echo(bytes: &[u8]) -> String {
    let mut shown = String::new();
    for &byte in bytes.iter().take(64) {
        match byte {
            b'\\' | b'"' | 0..=0x1f | 0x7f.. => {
                let _ = write!(shown, "\\x{byte:02X}");
            }
            _ => shown.push(char::from(byte)),
        }
    }
    if bytes.len() > 64 {
        shown.push_str("...");
    }
    format!("Echo({shown:?})")
}

fn unexpected(byte: u8, expected: &str) -> String {
    format!("UnexpectedByte {{ byte: {byte}, expected: {expected:?} }}")
}

fn text_of(lines: &[String]) -> String {
    lines.join("\n") + "\n"
}

/// Where the byte at `line` and `col` (both from 1) stands in `text`.
fn offset(text: &[u8], line: usize, col: usize) -> usize {
    let before = text.split_inclusive(|&byte| byte == b'\n').take(line - 1);
    before.map(<[u8]>::len).sum::<usize>() + col - 1
}

/// `levels` arrays, one inside the other, as the canonical writer lays them out from a line indented by `indent`.
fn nest(levels: usize, indent: usize) -> String {
    if levels <= 1 {
        return "[]".to_owned();
    }
    let inner = nest(levels - 1, indent + 2);
    format!("[\n{0:1$}{inner}\n{0:2$}]", "", indent + 2, indent)
}

/// Where `text` first leaves `canonical`, as the refusal must say it: the place, and what each has from there.
fn form(canonical: &[u8], text: &[u8]) -> (usize, usize, String) {
    let at = canonical
        .iter()
        .zip(text)
        .take_while(|(a, b)| a == b)
        .count();
    let before = &text[..at];
    let line = 1 + before.iter().filter(|&&byte| byte == b'\n').count();
    let col = 1 + before
        .iter()
        .rev()
        .take_while(|&&byte| byte != b'\n')
        .count();
    let shown = |bytes: &[u8]| echo(&bytes[at.min(bytes.len())..bytes.len().min(at + 24)]);
    let (expected, found) = (shown(canonical), shown(text));
    (
        line,
        col,
        format!("NotCanonical {{ expected: {expected}, found: {found} }}"),
    )
}

/// An id: one to twelve of the characters an id may hold, the first a letter or a digit. A draw that lands on a name
/// Windows keeps for its devices is no id (the committed seed draws `aux` once): it gets one more letter. The rule is
/// the generator's own here, never the loader's `is_id`, which is what the classes judge.
fn random_id(rng: &mut Rng) -> String {
    const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789-_";
    let length = 1 + rng.below(12);
    let letter = |at| char::from(LETTERS[rng.below(if at == 0 { 36 } else { 38 })]);
    let mut id: String = (0..length).map(letter).collect();
    let port = id.len() == 4
        && (id.starts_with("com") || id.starts_with("lpt"))
        && id.ends_with(|last: char| last.is_ascii_digit());
    if port || ["con", "prn", "aux", "nul"].contains(&id.as_str()) {
        id.push('x');
    }
    id
}

/// A file's members, key and value text.
type Members = Vec<(String, String)>;

/// A valid movement file as the generator holds it.
#[derive(Clone)]
struct File {
    path: String,
    /// The members, key and value text, in key order: member `at` stands on line `at + 2`, its key's quote in column
    /// 3 and its value in column `key.len() + 7`.
    members: Members,
    /// What the loader must read from it, as the derived `Debug` form of `Movement` writes it.
    read: String,
}

impl File {
    fn valid(rng: &mut Rng) -> Self {
        let id = random_id(rng);
        let text = |text: &str| format!("\"{text}\"");
        let mut members = vec![
            ("format".to_owned(), text(movement::FORMAT)),
            ("id".to_owned(), text(&id)),
            ("version".to_owned(), "1".to_owned()),
        ];
        let mut read = format!("Movement {{ id: {id:?}");
        for field in &FIELDS {
            let value = in_range(rng, field.range);
            members.push((field.name.to_owned(), number_text(value)));
            let _ = write!(read, ", {}: {value:?}", field.name);
        }
        read.push_str(" }");
        members.sort();
        let path = format!("movement/{id}.json");
        Self {
            path,
            members,
            read,
        }
    }

    fn lines(&self) -> Vec<String> {
        let last = self.members.len().saturating_sub(1);
        let mut lines = vec!["{".to_owned()];
        lines.extend(self.members.iter().enumerate().map(|(at, (key, value))| {
            format!("  \"{key}\": {value}{}", if at == last { "" } else { "," })
        }));
        lines.push("}".to_owned());
        lines
    }

    fn text(&self) -> String {
        text_of(&self.lines())
    }
}

/// `count` valid files with `count` different paths, in path order.
fn distinct(rng: &mut Rng, count: usize) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    while files.len() < count {
        let file = File::valid(rng);
        if files.iter().all(|other| other.path != file.path) {
            files.push(file);
        }
    }
    files.sort_by(|a, b| a.path.cmp(&b.path));
    files
}

fn entries_of(files: &[File]) -> Vec<(String, Vec<u8>)> {
    let entry = |file: &File| (file.path.clone(), file.text().into_bytes());
    files.iter().map(entry).collect()
}

/// A bundle's bytes from its entries, path and text, in the order given: the canonical writer's layout, written out
/// again here (a valid bundle made this way is taken only if it is the writer's own text, byte for byte).
fn bundle_bytes(entries: &[(String, Vec<u8>)]) -> Vec<u8> {
    let escaped = |bytes: &[u8], out: &mut Vec<u8>| {
        for &byte in bytes {
            match byte {
                b'"' => out.extend_from_slice(b"\\\""),
                b'\n' => out.extend_from_slice(b"\\n"),
                b'\\' => out.extend_from_slice(b"\\\\"),
                _ => out.push(byte),
            }
        }
    };
    let mut out = b"{\n  \"files\": [".to_vec();
    for (at, (path, text)) in entries.iter().enumerate() {
        out.extend_from_slice(if at == 0 { b"\n" } else { b",\n" });
        out.extend_from_slice(b"    {\n      \"path\": \"");
        escaped(path.as_bytes(), &mut out);
        out.extend_from_slice(b"\",\n      \"text\": \"");
        escaped(text, &mut out);
        out.extend_from_slice(b"\"\n    }");
    }
    out.extend_from_slice(if entries.is_empty() { b"]" } else { b"\n  ]" });
    out.extend_from_slice(b",\n  \"format\": \"shamble-content-bundle\",\n  \"version\": 1\n}\n");
    out
}

/// Numbers as a list on one line.
fn list(numbers: &[f64]) -> String {
    let texts: Vec<String> = numbers.iter().map(|number| number_text(*number)).collect();
    format!("[{}]", texts.join(", "))
}

/// A shape's entry on one line, with the words given for its `rises_toward` and its `type`.
fn shape_text(shape: &Shape, toward: Option<&str>, kind: &str) -> String {
    let toward = toward.map_or(String::new(), |toward| {
        format!("\"rises_toward\": \"{toward}\", ")
    });
    let (id, max, min) = (&shape.id, list(&shape.max), list(&shape.min));
    format!("{{\"id\": \"{id}\", \"max\": {max}, \"min\": {min}, {toward}\"type\": \"{kind}\"}}")
}

/// The word of a shape's form, if it is a ramp.
fn toward_of(shape: &Shape) -> Option<&'static str> {
    match shape.form {
        Form::Box => None,
        Form::Ramp { rises_toward } => Some(["+x", "-x", "+z", "-z"][rises_toward as usize]),
    }
}

/// A valid map as the generator holds it: the document the loader must read, built here field by field and never
/// read from a text, and each entry's text on one line, so that a class can change one entry's words.
struct Map {
    document: MapDocument,
    spawns: Vec<String>,
    shapes: Vec<String>,
    regions: Vec<String>,
}

#[rustfmt::skip]
impl Map {
    /// A map with at least `least` shapes and regions and one spawn; with `fill`, a list is now and then as long
    /// as it may be. Its bounds are whole metres, or, one time in eight, the aim's own reach on every side; every
    /// corner is a whole number of millimetres inside them.
    fn valid(rng: &mut Rng, least: usize, fill: bool) -> Self {
        let [x, y, z] = [10 + rng.below(300), 5 + rng.below(60), 10 + rng.below(300)].map(|side| side as f64);
        let bounds = match rng.below(8) {
            0 => Bounds { min: [-327.67, 0.0, -327.67], max: [327.67, 655.35, 327.67] },
            _ => Bounds { min: [-x, 0.0, -z], max: [x, y, z] },
        };
        let low = bounds.min.map(|bound| (bound * 1000.0).round() as i64);
        let high = bounds.max.map(|bound| (bound * 1000.0).round() as i64);
        let metres = |millimetres: i64| millimetres as f64 / 1000.0;
        let below = |rng: &mut Rng, bound: i64| rng.below(bound as usize) as i64;
        // A place on an axis, and a span of at least 1 cm on it.
        let place = |rng: &mut Rng, axis: usize| metres(low[axis] + below(rng, high[axis] - low[axis] + 1));
        let span = |rng: &mut Rng, axis: usize| {
            let from = low[axis] + below(rng, high[axis] - low[axis] - 9);
            (metres(from), metres(from + 10 + below(rng, high[axis] - from - 9)))
        };
        let count = |rng: &mut Rng, least: usize, most: u32| match rng.below(16) {
            0 if fill => most as usize,
            _ => least + rng.below(3),
        };
        let id = random_id(rng);
        let spawns = (0..count(rng, 1, MAX_SPAWNS)).map(|at| {
            let at_place = [0, 1, 2].map(|axis| place(rng, axis));
            Spawn { id: format!("{id}-{at}"), at: at_place, facing_deg: rng.below(360_000) as f64 / 1000.0 }
        }).collect();
        let shapes = (0..count(rng, least, MAX_SHAPES)).map(|at| {
            let spans = [0, 1, 2].map(|axis| span(rng, axis));
            let towards = [Toward::PlusX, Toward::MinusX, Toward::PlusZ, Toward::MinusZ];
            let form = towards.get(rng.below(5)).map_or(Form::Box, |toward| Form::Ramp { rises_toward: *toward });
            Shape { id: format!("{id}_{at}"), min: spans.map(|span| span.0), max: spans.map(|span| span.1), form }
        }).collect();
        let regions = (0..count(rng, least, MAX_REGIONS)).map(|at| {
            let spans = [0, 2].map(|axis| span(rng, axis));
            let kind = if rng.below(2) == 0 { Ground::Street } else { Ground::Grass };
            Region { id: format!("r{at}"), kind, min: spans.map(|span| span.0), max: spans.map(|span| span.1) }
        }).collect();
        Self::of(MapDocument { id, bounds, spawns, shapes, regions })
    }

    /// A document with its entries' texts.
    fn of(document: MapDocument) -> Self {
        let spawn = |spawn: &Spawn| format!("{{\"at\": {}, \"facing_deg\": {}, \"id\": \"{}\"}}", list(&spawn.at), number_text(spawn.facing_deg), spawn.id);
        let shape = |shape: &Shape| shape_text(shape, toward_of(shape), if shape.form == Form::Box { "box" } else { "ramp" });
        let region = |region: &Region| format!("{{\"id\": \"{}\", \"kind\": \"{}\", \"max\": {}, \"min\": {}}}", region.id, ["street", "grass"][region.kind as usize], list(&region.max), list(&region.min));
        let (spawns, shapes, regions) = (document.spawns.iter().map(spawn).collect(), document.shapes.iter().map(shape).collect(), document.regions.iter().map(region).collect());
        Self { document, spawns, shapes, regions }
    }

    fn path(&self) -> String {
        format!("maps/{}.json", self.document.id)
    }

    /// The file's text: the map on one line as written here, then laid out by the canonical writer.
    fn text(&self) -> Vec<u8> {
        let (Bounds { min, max }, id) = (self.document.bounds, &self.document.id);
        let (regions, shapes, spawns) = (self.regions.join(", "), self.shapes.join(", "), self.spawns.join(", "));
        let one_line = format!("{{\"bounds\": {{\"max\": {}, \"min\": {}}}, \"format\": \"shamble-map\", \"id\": \"{id}\", \"regions\": [{regions}], \"shapes\": [{shapes}], \"spawns\": [{spawns}], \"version\": 1}}", list(&max), list(&min));
        json::parse(one_line.as_bytes()).map_or(one_line.clone(), |top| canonical(&top)).into_bytes()
    }
}

/// A map document as the file the generator writes for it: its path and its text.
pub(crate) fn map_file(document: &MapDocument) -> (String, Vec<u8>) {
    let map = Map::of(document.clone());
    (map.path(), map.text())
}

/// Where `token` stands in a text, `past` bytes into it: the line and the column, both from 1.
fn find_place(text: &[u8], token: &str, past: usize) -> (usize, usize) {
    let at = text
        .windows(token.len())
        .position(|window| window == token.as_bytes())
        .unwrap_or(0)
        + past;
    let before = &text[..at.min(text.len())];
    let line = 1 + before.iter().filter(|&&byte| byte == b'\n').count();
    (
        line,
        1 + before
            .iter()
            .rev()
            .take_while(|&&byte| byte != b'\n')
            .count(),
    )
}

/// One generated input with the verdict it must get.
pub struct Case {
    /// The file's path, for `load_file`; `None` when the bytes are a bundle, for `load`.
    pub path: Option<String>,
    pub bytes: Vec<u8>,
    /// The refusal's record as the loader must give it, or `ok` and what it must read.
    pub expect: String,
}

/// The classes, each with the share of the per-class count it runs: the three heavy ones (one at the cap on values,
/// two that make megabytes) run a tenth. The
/// first [`FILE_CLASSES`] make one file, the rest a bundle.
#[rustfmt::skip]
pub const CLASSES: [(&str, u32); 37] = [
    ("a valid file", 1), ("a refused word", 1), ("a byte-order mark", 1), ("a forbidden byte between values", 1),
    ("a tab or a carriage return", 1), ("a comment", 1), ("a number that is no number", 1),
    ("a forbidden byte in a string", 1), ("an escape the content does not use", 1), ("a text that ends early", 1),
    ("a comma or a colon out of place", 1), ("brackets that do not match", 1), ("nesting past the cap", 1),
    ("a key that is there twice", 1), ("a key that is no key", 1), ("more values than a document holds", 10),
    ("a file past the byte cap", 10), ("a key the kind does not have", 1), ("a key that is missing", 1),
    ("a value of the wrong type", 1), ("a number outside its range", 1), ("another format or version", 1),
    ("an id that is no id", 1), ("an id that is not the file's name", 1), ("a text not in canonical form", 1),
    ("a valid map", 1), ("a map's list too long or a word it does not know", 1),
    ("a map's box off its grid, too thin or outside the bounds", 1),
    ("a map past the aim's reach, with no spawn or an id twice", 1),
    ("a valid bundle", 1), ("a path that is no content path", 1), ("paths out of order", 1),
    ("more files than a bundle holds", 1), ("a bundle with a key too many or too few", 1),
    ("another bundle format or version", 1), ("a bundle past a byte cap", 10), ("a file refused, in a bundle", 1),
];
pub const FILE_CLASSES: usize = 29;

/// Numbers that are none, or not the canonical text: the text, how far after the value's start the refusal stands
/// (past the start, the reader has left the field), and the refusal.
#[rustfmt::skip]
const BAD_NUMBERS: [(&str, usize, &str); 24] = [
    ("+1", 0, "UnexpectedByte { byte: 43, expected: \"a value\" }"),
    (".5", 0, "UnexpectedByte { byte: 46, expected: \"a value\" }"),
    ("0x1F", 1, "UnexpectedByte { byte: 120, expected: \"a comma or the object's end\" }"),
    ("5.", 0, "NumberGrammar { text: Echo(\"5.\") }"),
    ("1e", 0, "NumberGrammar { text: Echo(\"1e\") }"),
    ("--1", 0, "NumberGrammar { text: Echo(\"--1\") }"),
    ("01", 0, "NumberGrammar { text: Echo(\"01\") }"),
    ("-01", 0, "NumberGrammar { text: Echo(\"-01\") }"),
    ("-Infinity", 0, "NumberGrammar { text: Echo(\"-\") }"),
    ("1e400", 0, "NumberNotFinite { text: Echo(\"1e400\") }"),
    ("-1e400", 0, "NumberNotFinite { text: Echo(\"-1e400\") }"),
    ("1e309", 0, "NumberNotFinite { text: Echo(\"1e309\") }"),
    ("1.0", 0, "NumberNotCanonical { text: Echo(\"1.0\"), canonical: Echo(\"1\") }"),
    ("1e5", 0, "NumberNotCanonical { text: Echo(\"1e5\"), canonical: Echo(\"100000\") }"),
    ("1E-7", 0, "NumberNotCanonical { text: Echo(\"1E-7\"), canonical: Echo(\"1e-7\") }"),
    ("1e21", 0, "NumberNotCanonical { text: Echo(\"1e21\"), canonical: Echo(\"1e+21\") }"),
    ("-0", 0, "NumberNotCanonical { text: Echo(\"-0\"), canonical: Echo(\"0\") }"),
    ("0.10", 0, "NumberNotCanonical { text: Echo(\"0.10\"), canonical: Echo(\"0.1\") }"),
    ("0.123456789012345678", 0, "NumberNotCanonical { text: Echo(\"0.123456789012345678\"), canonical: Echo(\"0.12345678901234568\") }"),
    ("9007199254740993", 0, "NumberNotCanonical { text: Echo(\"9007199254740993\"), canonical: Echo(\"9007199254740992\") }"),
    ("9007199254740993.0", 0, "NumberNotCanonical { text: Echo(\"9007199254740993.0\"), canonical: Echo(\"9007199254740992\") }"),
    ("9.007199254740993e15", 0, "NumberNotCanonical { text: Echo(\"9.007199254740993e15\"), canonical: Echo(\"9007199254740992\") }"),
    ("-9007199254740993", 0, "NumberNotCanonical { text: Echo(\"-9007199254740993\"), canonical: Echo(\"-9007199254740992\") }"),
    ("12345678901234567890123456", 0, "NumberTooLong { len: 26, cap: 25 }"),
];

/// A `version` no build reads, with the double it is.
const VERSIONS: [(&str, f64); 5] = [
    ("2", 2.0),
    ("0", 0.0),
    ("1.5", 1.5),
    ("-1", -1.0),
    ("4294967296", 4_294_967_296.0),
];

/// One case of a class: a valid file, changed in one way, with the verdict that change must get. Laid out as a
/// table, one way of breaking a file to a line where it fits, like the tables above.
#[rustfmt::skip]
fn case(class: &str, rng: &mut Rng) -> Case {
    let file = File::valid(rng);
    let count = file.members.len();
    let at = rng.below(count);
    let (key, value) = file.members[at].clone();
    let (line, col) = (at + 2, key.len() + 7);
    let place_of = |key: &str| file.members.iter().position(|member| member.0 == key);
    let id_at = place_of("id").unwrap_or(0);
    let id = file.members[id_at].1.trim_matches('"').to_owned();
    let good = file.text().into_bytes();
    let lines = file.lines();
    let refuse = |bytes: Vec<u8>, line: usize, col: usize, field: &str, kind: String| {
        let field = if field.is_empty() { "-" } else { field };
        let expect = format!("{}|{line}:{col}|{field}|{kind}", file.path);
        Case { path: Some(file.path.clone()), bytes, expect }
    };
    let bundle =
        |entries: &[(String, Vec<u8>)], expect: String| Case { path: None, bytes: bundle_bytes(entries), expect };
    // The file with its members changed, with one line changed, with bytes put in at a place, and cut at a place.
    let model = |change: &dyn Fn(&mut Members)| {
        let mut changed = file.clone();
        change(&mut changed.members);
        changed.text().into_bytes()
    };
    let put = |at: usize, value: &str| model(&|members| members[at].1 = value.to_owned());
    let with = |key: &str, value: &str| {
        model(&|members| {
            members.push((key.to_owned(), value.to_owned()));
            members.sort();
        })
    };
    let edit = |line: usize, change: &dyn Fn(&str) -> String| {
        let mut lines = lines.clone();
        lines[line - 1] = change(&lines[line - 1]);
        text_of(&lines).into_bytes()
    };
    let insert = |line: usize, col: usize, bytes: &[u8]| {
        let at = offset(&good, line, col);
        [&good[..at], bytes, &good[at..]].concat()
    };
    let cut = |line: usize, col: usize| good[..offset(&good, line, col)].to_vec();
    // A gap between two tokens of member `at`: before its key, after its key, after its value.
    let gaps = [(1, ""), (key.len() + 5, key.as_str()), (col + value.len(), "")];
    let (gap, gap_field) = gaps[rng.below(3)];
    // A key no kind has sorts last: its member stands on line `count + 2`.
    let unknown = |key: &str| format!("UnknownKey {{ key: {} }}", echo(key.as_bytes()));
    let long = "k".repeat(65);
    match class {
        "a valid file" => {
            let expect = format!("ok Movement({})", file.read);
            Case { path: Some(file.path.clone()), bytes: good.clone(), expect }
        }
        "a refused word" => {
            let word = ["null", "true", "false", "NaN", "Infinity"][rng.below(5)];
            let kind = format!("RefusedLiteral {{ word: {word:?} }}");
            refuse(put(at, word), line, col, &key, kind)
        }
        "a byte-order mark" => {
            let bytes = [b"\xef\xbb\xbf", good.as_slice()].concat();
            refuse(bytes, 1, 1, "", "ByteOrderMark".to_owned())
        }
        "a forbidden byte between values" => {
            let byte = [0, 1, 0x1b, 0x1f, 0x7f, 0x80, 0xc3, 0xff][rng.below(8)];
            let kind = format!("ForbiddenByte {{ byte: {byte} }}");
            refuse(insert(line, gap, &[byte]), line, gap, gap_field, kind)
        }
        "a tab or a carriage return" => {
            let byte = b"\t\r"[rng.below(2)];
            let kind = format!("Whitespace {{ byte: {byte} }}");
            refuse(insert(line, gap, &[byte]), line, gap, gap_field, kind)
        }
        "a comment" => {
            let comment = [b"//".as_slice(), b"/* x */", b"// x\n"][rng.below(3)];
            refuse(insert(line, gap, comment), line, gap, gap_field, "Comment".to_owned())
        }
        "a number that is no number" => match BAD_NUMBERS.get(rng.below(BAD_NUMBERS.len() + 2)) {
            Some(&(text, 0, kind)) => refuse(put(at, text), line, col, &key, kind.to_owned()),
            Some(&(text, past, kind)) => refuse(put(at, text), line, col + past, "", kind.to_owned()),
            None => {
                let text = ["9".repeat(10_000), format!("1e{:030}", 1)][rng.below(2)].clone();
                let kind = format!("NumberTooLong {{ len: {}, cap: 25 }}", text.len());
                refuse(put(at, &text), line, col, &key, kind)
            }
        },
        // The id's text starts in column 10 of its line.
        "a forbidden byte in a string" => {
            let (line, col) = (id_at + 2, 10 + rng.below(id.len() + 1));
            let byte = [0, b'\t', b'\n', 0x1f, 0x7f, 0x80, 0xff][rng.below(7)];
            let kind = format!("ForbiddenByte {{ byte: {byte} }}");
            refuse(insert(line, col, &[byte]), line, col, "id", kind)
        }
        "an escape the content does not use" => {
            let (line, col) = (id_at + 2, 10 + rng.below(id.len() + 1));
            let escapes: [&[u8]; 8] = [b"\\t", b"\\r", b"\\b", b"\\f", b"\\/", b"\\u0041", b"\\\\", b"\\'"];
            let escape = escapes[rng.below(8)];
            let kind = format!("RefusedEscape {{ byte: {} }}", escape[1]);
            refuse(insert(line, col, escape), line, col + 1, "id", kind)
        }
        "a text that ends early" => {
            let ended = "Unterminated".to_owned();
            match rng.below(6) {
                0 => refuse(Vec::new(), 1, 1, "", "Empty".to_owned()),
                1 => refuse(b"\n \n".to_vec(), 3, 1, "", "Empty".to_owned()),
                // Inside the key, after the colon, before the closing brace, and after a backslash in the id.
                2 => {
                    let kept = 4 + rng.below(key.len() + 1);
                    refuse(cut(line, kept), line, kept, "", ended)
                }
                3 => refuse(cut(line, key.len() + 6), line, key.len() + 6, &key, ended),
                4 => refuse(cut(count + 2, 1), count + 2, 1, "", ended),
                _ => {
                    let (line, col) = (id_at + 2, 10 + rng.below(id.len() + 1));
                    refuse([cut(line, col), b"\\".to_vec()].concat(), line, col + 1, "id", ended)
                }
            }
        }
        "a comma or a colon out of place" => {
            // A member's line that ends in a comma: every member's but the last one's.
            let comma = 2 + rng.below(count - 1);
            let end = lines[comma - 1].len();
            let more = |line: &str| format!("{line},");
            let after = "a comma or the object's end";
            match rng.below(5) {
                0 => refuse(edit(count + 1, &more), count + 2, 1, "", "TrailingComma".to_owned()),
                1 => {
                    let bytes = edit(comma, &|line| line.trim_end_matches(',').to_owned());
                    refuse(bytes, comma + 1, 3, "", unexpected(b'"', after))
                }
                2 => refuse(edit(comma, &more), comma, end + 1, "", unexpected(b',', "a key")),
                3 => {
                    let bytes = edit(line, &|line| line.replacen("\": ", "\" ", 1));
                    refuse(bytes, line, key.len() + 6, &key, unexpected(value.as_bytes()[0], "a colon"))
                }
                _ => {
                    let bytes = edit(line, &|line| line.replacen("\": ", "\":: ", 1));
                    refuse(bytes, line, key.len() + 6, &key, unexpected(b':', "a value"))
                }
            }
        }
        "brackets that do not match" => {
            let after = "a comma or the object's end";
            let (extra, not_object) = ("TrailingData".to_owned(), "TopLevelNotObject".to_owned());
            match rng.below(6) {
                0 => refuse([good.as_slice(), b"}\n"].concat(), count + 3, 1, "", extra),
                1 => refuse(good.repeat(2), count + 3, 1, "", extra),
                2 => refuse(edit(count + 2, &|_| "]".to_owned()), count + 2, 1, "", unexpected(b']', after)),
                3 => refuse(b"[]\n".to_vec(), 1, 1, "", not_object),
                4 => refuse(b"\"a\"\n".to_vec(), 1, 1, "", not_object),
                _ => refuse(b"1\n".to_vec(), 1, 1, "", not_object),
            }
        }
        // The top object is level 1 and the member's own array level 2: 31 arrays reach the cap of 32 and are read
        // (and then refused as a key the kind does not have); the 32nd, on its own line or the same, is one too many.
        "nesting past the cap" => {
            let field = format!("zz{}", "[0]".repeat(31));
            let too_deep = "TooDeep { cap: 32 }".to_owned();
            match rng.below(3) {
                0 => refuse(with("zz", &nest(31, 2)), count + 2, 3, "", unknown("zz")),
                1 => refuse(with("zz", &nest(32, 2)), count + 33, 65, &field, too_deep),
                _ => refuse(with("zz", &"[".repeat(10_000)), count + 2, 40, &field, too_deep),
            }
        }
        "a key that is there twice" => {
            let twice = |key: &str| format!("DuplicateKey {{ key: {} }}", echo(key.as_bytes()));
            let member = (key.clone(), value.clone());
            match rng.below(3) {
                0 => {
                    let bytes = model(&|members| members.insert(at + 1, member.clone()));
                    refuse(bytes, line + 1, 3, "", twice(&key))
                }
                1 => {
                    let bytes = model(&|members| members.push(member.clone()));
                    refuse(bytes, count + 2, 3, "", twice(&key))
                }
                _ => {
                    let nested = "{\n    \"a\": 1,\n    \"b\": 2,\n    \"a\": 3\n  }";
                    refuse(with("zz", nested), count + 5, 5, "zz", twice("a"))
                }
            }
        }
        "a key that is no key" => {
            let bad = ["", "Walk", "__proto__", "a-b", "9a", long.as_str()][rng.below(6)];
            let bytes = model(&|members| members[at].0 = bad.to_owned());
            refuse(bytes, line, 3, "", format!("BadKey {{ key: {} }}", echo(bad.as_bytes())))
        }
        // The top object, its members' values and the new array are `count + 2` values; the rest are the array's.
        "more values than a document holds" => {
            let over = rng.below(10) != 0;
            let items = 16_384 - (count + 2) + usize::from(over);
            let bytes = with("zz", &format!("[\n{}\n  ]", vec!["    0"; items].join(",\n")));
            if over {
                let field = format!("zz[{}]", items - 1);
                let kind = "TooManyNodes { cap: 16384 }".to_owned();
                refuse(bytes, count + 2 + items, 5, &field, kind)
            } else {
                refuse(bytes, count + 2, 3, "", unknown("zz"))
            }
        }
        // At the cap the text is read, and found to hold nothing; one byte more is refused unread.
        "a file past the byte cap" => {
            if rng.below(10) == 0 {
                return refuse(vec![b' '; 1_048_576], 1, 1_048_577, "", "Empty".to_owned());
            }
            let length = 1_048_577 + rng.below(64);
            let kind = format!("TooLarge {{ bytes: {length}, cap: 1048576 }}");
            refuse(vec![b' '; length], 0, 0, "", kind)
        }
        "a key the kind does not have" => {
            let new = ["fog", "walk_speed", "zz", "a", "speed_kmh", "walk_speed_m_per_s2"][rng.below(6)];
            let line = 2 + file.members.iter().filter(|member| member.0.as_str() < new).count();
            refuse(with(new, "1"), line, 3, "", unknown(new))
        }
        "a key that is missing" => {
            let bytes = model(&|members| drop(members.remove(at)));
            refuse(bytes, 0, 0, "", format!("MissingKey {{ key: {key:?} }}"))
        }
        "a value of the wrong type" => {
            let other = [("\"6\"", "a string"), ("{}", "an object"), ("[]", "an array")][rng.below(3)];
            let (new, expected, found) = match key.as_str() {
                "format" | "id" => ("1", "a string", "a number"),
                "version" => ("\"1\"", "a whole number", "a string"),
                _ => (other.0, "a number", other.1),
            };
            let kind = format!("WrongType {{ expected: {expected:?}, found: {found:?} }}");
            refuse(put(at, new), line, col, &key, kind)
        }
        // One double outside the range, at either end: the end itself where the range leaves it out.
        "a number outside its range" => {
            let field = FIELDS[rng.below(FIELDS.len())];
            let range = field.range;
            let value = match (rng.below(2), range.min_inclusive, range.max_inclusive) {
                (0, true, _) => beside(range.min, false),
                (0, false, _) => range.min,
                (_, _, true) => beside(range.max, true),
                (_, _, false) => range.max,
            };
            let at = place_of(field.name).unwrap_or(0);
            let kind = format!(
                "OutOfRange {:016x} {} {:016x} {:016x} {}",
                value.to_bits(),
                if range.min_inclusive { '[' } else { '(' },
                range.min.to_bits(),
                range.max.to_bits(),
                if range.max_inclusive { ']' } else { ')' }
            );
            refuse(put(at, &number_text(value)), at + 2, field.name.len() + 7, field.name, kind)
        }
        "another format or version" => {
            if rng.below(2) == 0 {
                let other = ["shamble-map", "shamble-movement2", "", "Shamble-Movement"][rng.below(4)];
                let at = place_of("format").unwrap_or(0);
                let found = echo(other.as_bytes());
                let kind = format!("WrongFormat {{ found: {found}, expected: \"shamble-movement\" }}");
                return refuse(put(at, &format!("\"{other}\"")), at + 2, 13, "format", kind);
            }
            let (text, value) = VERSIONS[rng.below(5)];
            let at = place_of("version").unwrap_or(0);
            let kind = format!("WrongVersion {:016x} 1", value.to_bits());
            refuse(put(at, text), at + 2, 14, "version", kind)
        }
        // The text as the file has it, and the id it is read as (the last holds a line feed, written as its escape).
        "an id that is no id" => {
            let long = (long.as_str(), long.as_str());
            // The last three are names Windows keeps for its devices: no file of such a name can be made there.
            let ids = [("Abc", "Abc"), ("a b", "a b"), ("a<b", "a<b"), ("-ab", "-ab"), ("", ""), long, ("con", "con"), ("lpt1", "lpt1"), ("com0", "com0")];
            let (text, read) = *ids.get(rng.below(10)).unwrap_or(&("a\\nb", "a\nb"));
            let kind = format!("BadId {{ id: {} }}", echo(read.as_bytes()));
            refuse(put(id_at, &format!("\"{text}\"")), id_at + 2, 9, "id", kind)
        }
        "an id that is not the file's name" => {
            let path = "movement/another.json".to_owned();
            let expect = format!("{path}|0:0|id|IdNotFileName {{ id: {} }}", echo(id.as_bytes()));
            Case { path: Some(path), bytes: good.clone(), expect }
        }
        "a text not in canonical form" => {
            let swap = rng.below(count - 1);
            let bytes = match rng.below(6) {
                0 => model(&|members| members.swap(swap, swap + 1)),
                1 => edit(line, &|line| format!("  {line}")),
                2 => good[..good.len() - 1].to_vec(),
                3 => [good.as_slice(), b"\n"].concat(),
                4 => edit(line, &|line| line.replacen("\":", "\" :", 1)),
                _ => edit(line, &|line| line.replacen(": ", ":", 1)),
            };
            let (line, col, kind) = form(&good, &bytes);
            refuse(bytes, line, col, "", kind)
        }
        "a valid map" => {
            let map = Map::valid(rng, 0, true);
            Case { path: Some(map.path()), bytes: map.text(), expect: format!("ok Map({:?})", map.document) }
        }
        // The typed read's refusals stand at a place in the laid-out text: where a planted word is found, or a
        // fixed number of lines from one (the bounds' highest corner opens on line 3; a shape's `rises_toward` is
        // eleven lines under its id).
        "a map's list too long or a word it does not know" => {
            let mut map = Map::valid(rng, 1, false);
            let (shape, region) = (rng.below(map.shapes.len()), rng.below(map.regions.len()));
            let (form, id) = (map.document.shapes[shape].clone(), map.document.shapes[shape].id.clone());
            let too_many = |len: usize, max: u32| format!("TooMany {{ len: {len}, max: {max} }}");
            let not_one_of = |value: &str, allowed: &str| format!("NotOneOf {{ value: {}, allowed: {allowed:?} }}", echo(value.as_bytes()));
            let (token, past, lines, field, kind) = match rng.below(9) {
                0 => { map.spawns = vec![map.spawns[0].clone(); 17]; ("\"spawns\": [".to_owned(), 10, 0, "spawns".to_owned(), too_many(17, 16)) }
                1 => { map.shapes = vec![map.shapes[0].clone(); 513]; ("\"shapes\": [".to_owned(), 10, 0, "shapes".to_owned(), too_many(513, 512)) }
                2 => { map.regions = vec![map.regions[0].clone(); 65]; ("\"regions\": [".to_owned(), 11, 0, "regions".to_owned(), too_many(65, 64)) }
                3 => {
                    map.shapes[shape] = shape_text(&form, toward_of(&form), "sphere");
                    ("\"type\": \"sphere\"".to_owned(), 8, 0, format!("shapes[{shape}].type"), not_one_of("sphere", "box or ramp"))
                }
                4 => {
                    map.shapes[shape] = shape_text(&form, Some("up"), "ramp");
                    ("\"rises_toward\": \"up\"".to_owned(), 16, 0, format!("shapes[{shape}].rises_toward"), not_one_of("up", "+x, -x, +z or -z"))
                }
                5 => {
                    map.regions[region] = map.regions[region].replace("\"street\"", "\"water\"").replace("\"grass\"", "\"water\"");
                    ("\"kind\": \"water\"".to_owned(), 8, 0, format!("regions[{region}].kind"), not_one_of("water", "street or grass"))
                }
                6 => {
                    map.shapes[shape] = shape_text(&form, Some("+x"), "box");
                    (format!("\"id\": \"{id}\""), 0, 11, format!("shapes[{shape}]"), unknown("rises_toward"))
                }
                7 => {
                    map.shapes[shape] = shape_text(&form, None, "ramp");
                    let expect = format!("{}|0:0|shapes[{shape}]|MissingKey {{ key: \"rises_toward\" }}", map.path());
                    return Case { path: Some(map.path()), bytes: map.text(), expect };
                }
                _ => {
                    let (more, text) = (rng.below(2) == 0, String::from_utf8(map.text()).unwrap_or_default());
                    let mut lines: Vec<String> = text.lines().map(str::to_owned).collect();
                    let kind = if more { lines.insert(3, "      0,".to_owned()); too_many(4, 3) } else { lines.remove(3); "TooFew { len: 2, min: 3 }".to_owned() };
                    let expect = format!("{}|3:12|bounds.max|{kind}", map.path());
                    return Case { path: Some(map.path()), bytes: text_of(&lines).into_bytes(), expect };
                }
            };
            let bytes = map.text();
            let (line, col) = find_place(&bytes, &token, past);
            Case { path: Some(map.path()), bytes, expect: format!("{}|{}:{col}|{field}|{kind}", map.path(), line + lines) }
        }
        // A map's own rules name the field and no place. Each case breaks one rule on a map that keeps the others,
        // so the rule broken is the first the loader meets.
        "a map's box off its grid, too thin or outside the bounds" => {
            let mut map = Map::valid(rng, 1, true).document;
            let (shape, region, spawn) = (rng.below(map.shapes.len()), rng.below(map.regions.len()), rng.below(map.spawns.len()));
            // An axis, and the same for a region, whose two axes are x and z.
            let (axis, flat) = (rng.below(3), rng.below(2));
            let (Bounds { min, max }, ground) = (map.bounds, [0, 2][flat]);
            let step = |value: f64, millimetres: f64| ((value * 1000.0).round() + millimetres) / 1000.0;
            let off = |value: f64, per_metre: u32| format!("OffGrid {:016x} {per_metre}", value.to_bits());
            let thin = |min: f64, max: f64| format!("TooThin {:016x} {:016x}", min.to_bits(), max.to_bits());
            let outside = |value: f64, bound: f64| format!("OutsideBounds {:016x} {:016x}", value.to_bits(), bound.to_bits());
            let (field, kind) = match rng.below(9) {
                0 => { map.bounds.max[axis] -= 0.005; (format!("bounds.max[{axis}]"), off(map.bounds.max[axis], 100)) }
                1 => { map.shapes[shape].min[axis] += 0.0005; (format!("shapes[{shape}].min[{axis}]"), off(map.shapes[shape].min[axis], 1000)) }
                2 => { map.spawns[spawn].at[axis] += 0.0005; (format!("spawns[{spawn}].at[{axis}]"), off(map.spawns[spawn].at[axis], 1000)) }
                3 => { map.regions[region].max[flat] -= 0.0005; (format!("regions[{region}].max[{flat}]"), off(map.regions[region].max[flat], 1000)) }
                // Nine millimetres thick, and not thick at all.
                4 => {
                    let low = map.shapes[shape].min[axis];
                    map.shapes[shape].max[axis] = step(low, 9.0);
                    (format!("shapes[{shape}].max[{axis}]"), thin(low, step(low, 9.0)))
                }
                5 => {
                    let low = map.regions[region].min[flat];
                    map.regions[region].max[flat] = low;
                    (format!("regions[{region}].max[{flat}]"), thin(low, low))
                }
                // One millimetre past the bounds.
                6 => { map.shapes[shape].max[axis] = step(max[axis], 1.0); (format!("shapes[{shape}].max[{axis}]"), outside(step(max[axis], 1.0), max[axis])) }
                7 => { map.spawns[spawn].at[axis] = step(min[axis], -1.0); (format!("spawns[{spawn}].at[{axis}]"), outside(step(min[axis], -1.0), min[axis])) }
                _ => { map.regions[region].min[flat] = step(min[ground], -1.0); (format!("regions[{region}].min[{flat}]"), outside(step(min[ground], -1.0), min[ground])) }
            };
            let map = Map::of(map);
            Case { path: Some(map.path()), bytes: map.text(), expect: format!("{}|0:0|{field}|{kind}", map.path()) }
        }
        "a map past the aim's reach, with no spawn or an id twice" => {
            let mut map = Map::valid(rng, 2, true).document;
            // One centimetre past each of the reach's six edges: the bound, and the edge in centimetres.
            let edges = [(-327.68, -32_767), (327.68, 32_767), (-0.01, 0), (655.36, 65_535), (-327.68, -32_767), (327.68, 32_767)];
            let twice = |id: &str| format!("DuplicateId {{ id: {} }}", echo(id.as_bytes()));
            let (field, kind) = match rng.below(9) {
                edge @ 0..6 => {
                    let ((bound, reach), axis) = (edges[edge], edge / 2);
                    if edge % 2 == 0 { map.bounds.min[axis] = bound } else { map.bounds.max[axis] = bound };
                    (format!("bounds.{}[{axis}]", ["min", "max"][edge % 2]), format!("BeyondAim {:016x} {reach}", bound.to_bits()))
                }
                6 => { map.spawns.clear(); ("spawns".to_owned(), "TooFew { len: 0, min: 1 }".to_owned()) }
                7 => {
                    let later = 1 + rng.below(map.shapes.len() - 1);
                    map.shapes[later].id = map.shapes[rng.below(later)].id.clone();
                    (format!("shapes[{later}].id"), twice(&map.shapes[later].id))
                }
                _ => {
                    let later = 1 + rng.below(map.regions.len() - 1);
                    map.regions[later].id = map.regions[rng.below(later)].id.clone();
                    (format!("regions[{later}].id"), twice(&map.regions[later].id))
                }
            };
            let map = Map::of(map);
            Case { path: Some(map.path()), bytes: map.text(), expect: format!("{}|0:0|{field}|{kind}", map.path()) }
        }
        // Up to two maps and fourteen movement files, each read as the generator made it, in path order.
        "a valid bundle" => {
            let count = 1 + rng.below(MAX_FILES as usize - 2);
            let files = distinct(rng, count);
            let mut maps: Vec<Map> = (0..rng.below(3)).map(|_| Map::valid(rng, 0, true)).collect();
            maps.sort_by_key(Map::path);
            maps.dedup_by_key(|map| map.path());
            let mut entries: Vec<(String, Vec<u8>)> = maps.iter().map(|map| (map.path(), map.text())).collect();
            entries.extend(entries_of(&files));
            let documents: Vec<&MapDocument> = maps.iter().map(|map| &map.document).collect();
            let reads: Vec<&str> = files.iter().map(|file| file.read.as_str()).collect();
            bundle(&entries, format!("ok {documents:?} [{}]", reads.join(", ")))
        }
        "a path that is no content path" => {
            let paths = [
                format!("weapons/{id}.json"),
                format!("movement/{id}.txt"),
                format!("movement/{id}"),
                format!("/movement/{id}.json"),
                format!("movement//{id}.json"),
                format!("movement/../{id}.json"),
                format!("source/{id}.json"),
                format!("movement/{id}A.json"),
                format!("movement/{id}.json/"),
                format!("{id}.json"),
                format!("movement/-{id}.json"),
                format!("movement/{long}.json"),
                String::new(),
            ];
            let path = paths[rng.below(paths.len())].clone();
            let expect = format!("-|0:0|-|BadPath {{ path: {} }}", echo(path.as_bytes()));
            bundle(&[(path, good.clone())], expect)
        }
        // Entry `k` of a bundle starts on line `3 + 4k`, in column 5.
        "paths out of order" => {
            let mut entries = entries_of(&distinct(rng, 2));
            if rng.below(2) == 0 {
                entries.reverse();
            } else {
                entries[1] = entries[0].clone();
            }
            let second = echo(entries[1].0.as_bytes());
            bundle(&entries, format!("-|7:5|files[1]|PathOrder {{ path: {second} }}"))
        }
        "more files than a bundle holds" => {
            let expect = "-|1:1|files|TooManyFiles { len: 17, cap: 16 }".to_owned();
            bundle(&entries_of(&distinct(rng, 17)), expect)
        }
        // The bundle's last three lines are its format, its version and its closing brace.
        "a bundle with a key too many or too few" | "another bundle format or version" => {
            let files = 1 + rng.below(3);
            let whole = String::from_utf8(bundle_bytes(&entries_of(&distinct(rng, files))));
            let mut lines: Vec<String> = whole.unwrap_or_default().lines().map(str::to_owned).collect();
            let total = lines.len();
            let entry = rng.below(files);
            let (text, value) = VERSIONS[rng.below(5)];
            let expect = match (class.starts_with("another"), rng.below(4)) {
                (true, 0 | 1) => {
                    lines[total - 3] = "  \"format\": \"shamble-content\",".to_owned();
                    let found = "found: Echo(\"shamble-content\"), expected: \"shamble-content-bundle\"";
                    format!("-|0:0|format|WrongFormat {{ {found} }}")
                }
                (true, _) => {
                    lines[total - 2] = format!("  \"version\": {text}");
                    format!("-|{}:14|version|WrongVersion {:016x} 1", total - 1, value.to_bits())
                }
                (false, 0) => {
                    lines.insert(total - 2, "  \"note\": 1,".to_owned());
                    format!("-|{}:3|-|{}", total - 1, unknown("note"))
                }
                (false, 1) => {
                    lines.insert(3 + 4 * entry, "      \"kind\": \"x\",".to_owned());
                    format!("-|{}:7|files[{entry}]|{}", 4 + 4 * entry, unknown("kind"))
                }
                (false, 2) => {
                    lines.remove(4 + 4 * entry);
                    lines[3 + 4 * entry].pop();
                    format!("-|0:0|files[{entry}]|MissingKey {{ key: \"text\" }}")
                }
                (false, _) => {
                    lines.remove(total - 3);
                    "-|0:0|-|MissingKey { key: \"format\" }".to_owned()
                }
            };
            Case { path: None, bytes: text_of(&lines).into_bytes(), expect }
        }
        "a bundle past a byte cap" => match rng.below(4) {
            0 => {
                let length = MAX_BUNDLE_BYTES + 1 + rng.below(64);
                let expect = format!("-|0:0|-|TooLarge {{ bytes: {length}, cap: 8388608 }}");
                Case { path: None, bytes: vec![b' '; length], expect }
            }
            // A file's cap counts its own text, as the bundle's string holds it once read.
            1 => {
                let expect = format!("{}|1:1048577|-|Empty", file.path);
                bundle(&[(file.path.clone(), vec![b' '; 1_048_576])], expect)
            }
            2 => {
                let expect = format!("{}|0:0|-|TooLarge {{ bytes: 1048577, cap: 1048576 }}", file.path);
                bundle(&[(file.path.clone(), vec![b' '; 1_048_577])], expect)
            }
            // A bundle as a file's text: with no file of its own it is read, and is no movement file; with one, its
            // text holds backslashes, which no bundle's string can carry.
            _ if rng.below(2) == 0 => {
                let found = "found: Echo(\"shamble-content-bundle\"), expected: \"shamble-movement\"";
                let expect = format!("{}|3:13|format|WrongFormat {{ {found} }}", file.path);
                bundle(&[(file.path.clone(), bundle_bytes(&[]))], expect)
            }
            _ => {
                let inner = bundle_bytes(&[(file.path.clone(), good.clone())]);
                let before = &inner[..inner.iter().position(|&byte| byte == b'\\').unwrap_or(0)];
                let twice = before.iter().filter(|&&byte| byte == b'"' || byte == b'\n').count();
                let col = 16 + before.len() + twice + 1;
                let expect = format!("-|5:{col}|files[0].text|RefusedEscape {{ byte: 92 }}");
                bundle(&[(file.path.clone(), inner)], expect)
            }
        },
        // A file-level refusal is the same refusal when the file comes in a bundle (a bundle's string carries
        // printable ASCII and line feeds, and no backslash).
        "a file refused, in a bundle" => loop {
            let (class, share) = CLASSES[1 + rng.below(FILE_CLASSES - 1)];
            let inner = if share == 1 {
                case(class, rng)
            } else {
                continue;
            };
            let carried = |&byte: &u8| byte == b'\n' || (0x20..=0x7e).contains(&byte) && byte != b'\\';
            if inner.bytes.iter().all(carried) && !inner.expect.starts_with("ok ") {
                break bundle(&[(inner.path.unwrap_or_default(), inner.bytes)], inner.expect);
            }
        },
        _ => Case { path: None, bytes: Vec::new(), expect: format!("the generator has no class {class:?}") },
    }
}

/// What the loader says of a case, and its message when it refuses.
fn judge(case: &Case) -> (String, String) {
    let read = match &case.path {
        Some(path) => load_file(path, &case.bytes).map(|document| format!("ok {document:?}")),
        None => load(&case.bytes).map(|set| format!("ok {:?} {:?}", set.maps(), set.movements())),
    };
    match read {
        Ok(read) => (read, String::new()),
        Err(error) => (error.record(), error.message()),
    }
}

/// Every class's cases, judged: the verdicts file's text (a first line, then `<class>|<cases>|<digest>` a class; a
/// digest folds each case's bytes, verdict and message), or the first case whose verdict is not its class's.
pub fn verdicts(seed: u64, per_class: u32) -> Result<String, String> {
    let mut text = format!("shamble-content-verdicts 1 {seed} {per_class}\n");
    for (class, (name, share)) in CLASSES.iter().enumerate() {
        let mut rng = Rng::new(seed, class as u64);
        let cases = (per_class / share).max(1);
        let mut digest = 0_u64;
        for index in 0..cases {
            let case = case(name, &mut rng);
            let (got, message) = judge(&case);
            hold(got == case.expect, || {
                let shown = String::from_utf8_lossy(&case.bytes[..case.bytes.len().min(2048)]);
                format!(
                    "{name}, case {index} of seed {seed}: the loader says\n  {got}\nand the class says\n  {}\n\
                     of {} bytes that start:\n{shown}",
                    case.expect,
                    case.bytes.len()
                )
            })?;
            // An input of a megabyte is spaces, or one megabyte text in a bundle: its length says all of it.
            let small = case.bytes.len() <= 262_144;
            let bytes = if small { &case.bytes[..] } else { &[][..] };
            let length = (case.bytes.len() as u64).to_le_bytes();
            let parts = [
                &digest.to_le_bytes()[..],
                &length,
                bytes,
                got.as_bytes(),
                message.as_bytes(),
            ];
            digest = fnv1a_64(&parts.concat());
        }
        let _ = writeln!(text, "{name}|{cases}|{digest:016x}");
    }
    // And the fuzz run's first inputs, so that the two targets are also compared on what only mutation reaches.
    let (inputs, mut digest) = (u64::from(per_class) * 4, 0_u64);
    for index in 0..inputs {
        let (path, bytes) = fuzz_input(seed, index);
        let named = |why| format!("the fuzz run's input {index} of seed {seed}: {why}");
        let stage = fuzz_judge(path.as_deref(), &bytes).map_err(named)?;
        // What is taken goes on into the static world's builder, on this target too (the digest holds the loader's
        // verdict alone; the worlds' own bytes are compared by `world_proofs`).
        crate::world_proofs::fuzz_worlds(path.as_deref(), &bytes).map_err(named)?;
        digest = fnv1a_64(&[&digest.to_le_bytes()[..], &bytes, &[stage as u8]].concat());
    }
    let _ = writeln!(text, "the fuzz run's first inputs|{inputs}|{digest:016x}");
    Ok(text)
}

/// A verdicts file against this build: an empty text when the file's own seed and count give the file's own lines,
/// or the first line that differs (or the first case that is not judged as its class says).
pub fn verdicts_check(expected: &str) -> String {
    let first: Vec<&str> = expected.lines().next().unwrap_or("").split(' ').collect();
    let numbers = match first[..] {
        ["shamble-content-verdicts", "1", seed, count] => seed.parse().ok().zip(count.parse().ok()),
        _ => None,
    };
    let Some((seed, per_class)) = numbers else {
        return format!("not a verdicts file: its first line is {first:?}");
    };
    let made = match verdicts(seed, per_class) {
        Ok(made) => made,
        Err(why) => return why,
    };
    let differs = made
        .lines()
        .zip(expected.lines())
        .find(|(made, expected)| made != expected);
    match differs {
        Some((made, expected)) => {
            format!("this build gives\n  {made}\nand the file has\n  {expected}")
        }
        None if made.len() != expected.len() => {
            "the file has other lines than this build gives".to_owned()
        }
        None => String::new(),
    }
}

/// The inputs that lean on the caps, by name: the node cap, the duplicate-key search at its largest, a bundle of
/// eight files of a megabyte each, a bundle of fifteen texts of just over half a megabyte, and two trees of as many
/// one-member objects as the node cap takes.
pub const LIMIT_CASES: [&str; 7] = [
    "an object of the most keys a document holds",
    "one key 16,383 times",
    "an object of 100,000 keys",
    "a bundle of eight files of a megabyte each",
    "a bundle of fifteen texts of half a megabyte and a byte",
    "a file of 528 chains of thirty objects of one member",
    "a bundle whose own tree and whose first file are such chains",
];

/// 528 chains of thirty objects of one member each around a 0, in one array, on one line: 16,370 nodes in 96,103
/// bytes, six bytes of text for an object.
fn chains_file() -> String {
    let chain = format!("{}0{}", "{\"a\":".repeat(30), "}".repeat(30));
    format!("{{\"a\":[{}]}}", vec![chain; 528].join(","))
}

/// The limit case of that place in [`LIMIT_CASES`], found by its name, with the verdict it must get: what a counting
/// allocator and a clock are held against. Each is made when it is asked for, so that no two heavy inputs are held
/// at once: in WebAssembly the module's memory only grows, and its ceiling is to measure the loader, not this list.
pub fn limit_case(at: usize) -> Option<Case> {
    let mut rng = Rng::new(0, u64::MAX);
    let files = distinct(&mut rng, 8);
    let path = files[0].path.clone();
    let refuse = |bytes: Vec<u8>, verdict: &str| {
        let expect = format!("{path}|{verdict}");
        Case {
            path: Some(path.clone()),
            bytes,
            expect,
        }
    };
    let same = |count: usize| format!("{{{}}}\n", vec!["\"a\": 0"; count].join(", ")).into_bytes();
    // Where the chains' one line first leaves its canonical form: at its second byte, where a line feed belongs.
    let on_one_line = |chains: &[u8]| {
        let (line, col, kind) = form(b"{\n  \"a\": [\n    {\n      \"a\": {\n", chains);
        format!("{line}:{col}|-|{kind}")
    };
    Some(match *LIMIT_CASES.get(at)? {
        // The most keys one object has under the node cap, in key order.
        "an object of the most keys a document holds" => {
            let keys: Vec<String> = (0..16_383).map(|at| format!("  \"k{at:05}\": 0")).collect();
            let bytes = format!("{{\n{}\n}}\n", keys.join(",\n")).into_bytes();
            refuse(bytes, "0:0|-|MissingKey { key: \"format\" }")
        }
        // The same key every time, where the search for a key that is there twice has the most to sort.
        "one key 16,383 times" => refuse(same(16_383), "1:10|-|DuplicateKey { key: Echo(\"a\") }"),
        // 100,000 keys, refused at the first value past the cap.
        "an object of 100,000 keys" => {
            refuse(same(100_000), "1:131071|a|TooManyNodes { cap: 16384 }")
        }
        // A megabyte of one string in each file: read, and refused as a key the kind does not have.
        "a bundle of eight files of a megabyte each" => {
            let heavy = |file: &File| {
                let mut heavy = file.clone();
                let text = format!("\"{}\"", "x".repeat(1_000_000));
                heavy.members.push(("zz".to_owned(), text));
                (file.path.clone(), heavy.text().into_bytes())
            };
            let eight: Vec<(String, Vec<u8>)> = files.iter().map(heavy).collect();
            let line = files[0].members.len() + 2;
            Case {
                path: None,
                bytes: bundle_bytes(&eight),
                expect: format!("{path}|{line}:3|-|UnknownKey {{ key: Echo(\"zz\") }}"),
            }
        }
        // Fifteen texts of half a megabyte and a byte, all spaces: a string that grew by doubling as it was read
        // held twice its size here (security-auditor's bundle; every text is kept at its own size since).
        "a bundle of fifteen texts of half a megabyte and a byte" => {
            let text = |at: usize| (format!("movement/f{at:02}.json"), vec![b' '; 524_289]);
            let fifteen: Vec<(String, Vec<u8>)> = (0..15).map(text).collect();
            Case {
                path: None,
                bytes: bundle_bytes(&fifteen),
                expect: "movement/f00.json|1:524290|-|Empty".to_owned(),
            }
        }
        // As many small objects as the node cap takes: a list that kept the room it grew by held 288 bytes for an
        // object of six (security-auditor's second file; every list is cut to its length since). The whole tree
        // is read before the text is refused for its form.
        "a file of 528 chains of thirty objects of one member" => {
            let bytes = chains_file().into_bytes();
            let verdict = on_one_line(&bytes);
            refuse(bytes, &verdict)
        }
        // Two such trees held at once: the bundle's own (564 chains of 28 under a key no entry has, which is
        // judged after the first file) and its first file's.
        "a bundle whose own tree and whose first file are such chains" => {
            let file = chains_file();
            let laid_out = |depth: usize, indent: usize| {
                let open = |level| format!("{{\n{}\"a\": ", " ".repeat(indent + 2 * level));
                let close = |level| format!("\n{}}}", " ".repeat(indent + 2 * level));
                let (open, close): (String, String) = (
                    (1..=depth).map(open).collect(),
                    (0..depth).rev().map(close).collect(),
                );
                format!("{open}0{close}")
            };
            let entry = |path: &str, text: &str, more: &str| {
                format!(
                    "    {{\n      \"path\": \"{path}\",\n      \"text\": \"{text}\"{more}\n    }}"
                )
            };
            let chains = vec![laid_out(28, 8); 564].join(",\n        ");
            let first = entry("movement/a.json", &file.replace('"', "\\\""), "");
            let second = entry(
                "movement/b.json",
                "{}\\n",
                &format!(",\n      \"zz\": [\n        {chains}\n      ]"),
            );
            let end = "  ],\n  \"format\": \"shamble-content-bundle\",\n  \"version\": 1\n}\n";
            Case {
                path: None,
                bytes: format!("{{\n  \"files\": [\n{first},\n{second}\n{end}").into_bytes(),
                expect: format!("movement/a.json|{}", on_one_line(file.as_bytes())),
            }
        }
        _ => return None,
    })
}

/// The limit cases against this build: an empty text when each gets its verdict, or the first that does not.
pub fn limits_check() -> String {
    let wrong = LIMIT_CASES.iter().enumerate().find_map(|(at, name)| {
        let Some(case) = limit_case(at) else {
            return Some(format!("{name}: the list names a case that is not made"));
        };
        let got = judge(&case).0;
        (got != case.expect).then(|| format!("{name}: the loader says {got}, not {}", case.expect))
    });
    wrong.unwrap_or_default()
}

/// The stages a verdict is settled in, as the fuzz run's histogram names them.
pub const STAGES: [&str; 10] = [
    "taken",
    "the byte caps",
    "the bundle's text",
    "the bundle's form",
    "the bundle's rules",
    "a file's text",
    "a file's form",
    "a file's fields",
    "a file's name against its id",
    "a map's own rules",
];

/// The stage that settled a verdict.
fn stage<T>(result: &Result<T, Error>) -> usize {
    let Err(error) = result else { return 0 };
    #[rustfmt::skip]
    let step = match &error.kind {
        Kind::TooLarge { .. } => return 1,
        Kind::IdNotFileName { .. } => return 8,
        // A stage of their own: only a map that was read whole gets here, so a run without maps settles none.
        Kind::OffGrid { .. } | Kind::TooThin { .. } | Kind::OutsideBounds { .. } | Kind::BeyondAim { .. }
        | Kind::DuplicateId { .. } => return 9,
        Kind::Empty | Kind::ByteOrderMark | Kind::ForbiddenByte { .. } | Kind::Whitespace { .. } | Kind::Comment
        | Kind::TrailingComma | Kind::TopLevelNotObject | Kind::TrailingData | Kind::Unterminated
        | Kind::UnexpectedByte { .. } | Kind::RefusedLiteral { .. } | Kind::RefusedEscape { .. } | Kind::BadKey { .. }
        | Kind::DuplicateKey { .. } | Kind::TooDeep { .. } | Kind::TooManyNodes { .. } | Kind::NumberTooLong { .. }
        | Kind::NumberGrammar { .. } | Kind::NumberNotFinite { .. } => 0,
        Kind::NumberNotCanonical { .. } | Kind::NotCanonical { .. } => 1,
        Kind::UnknownKey { .. } | Kind::MissingKey { .. } | Kind::WrongType { .. } | Kind::NotAWholeNumber { .. }
        | Kind::OutOfRange { .. } | Kind::TooMany { .. } | Kind::TooFew { .. } | Kind::BadId { .. }
        | Kind::NotOneOf { .. } | Kind::WrongFormat { .. } | Kind::WrongVersion { .. } | Kind::BadPath { .. }
        | Kind::PathOrder { .. } | Kind::TooManyFiles { .. } => 2,
    };
    if error.file.is_some() {
        5 + step
    } else {
        2 + step
    }
}

/// Numbers at the edges of what a field, a double and the reader take, and tokens no content file holds.
#[rustfmt::skip]
const EDGE_NUMBERS: [&str; 16] = [
    "0", "-0", "1", "-1", "1.5", "0.1", "100", "1e400", "5e-324", "4294967296", "1e+21", "9007199254740993",
    "1.7976931348623157e+308", "99999999999999999999999999", "0.30000000000000004", "01",
];
#[rustfmt::skip]
const TOKENS: [&[u8]; 16] = [
    b"null", b"true", b"false", b"NaN", b"Infinity", b"//", b"/*", b"\t", b"\r\n", b"\\u0041", b"\\\\", b"\xef\xbb\xbf",
    b"\"", b",", b":", b"{}",
];
const NAMES: [&str; 10] = [
    "id",
    "format",
    "version",
    "walk_speed_m_per_s",
    "files",
    "path",
    "text",
    "zz",
    "__proto__",
    "",
];

/// One change to a text, of the kinds a hand or a tool makes: most leave it close to a content file.
fn mutate(rng: &mut Rng, bytes: &mut Vec<u8>) {
    let at = rng.below(bytes.len() + 1);
    let end = (at + 1 + rng.below(16)).min(bytes.len());
    let rest = &bytes[at..];
    match rng.below(10) {
        0 => {
            if let Some(byte) = bytes.get_mut(at) {
                *byte ^= 1 << rng.below(8);
            }
        }
        // One byte more: any byte, a printable one, or white space (the text stays a document, in another form).
        1 => {
            let byte = match rng.below(4) {
                0 => rng.below(256),
                1 => usize::from(b" \n"[rng.below(2)]),
                _ => 0x20 + rng.below(95),
            };
            bytes.insert(at, byte as u8);
        }
        2 => drop(bytes.drain(at..end)),
        3 => {
            let span = bytes[at..end].to_vec();
            bytes.splice(at..at, span);
        }
        4 => bytes.truncate(at),
        // A nesting pump.
        5 => {
            let pump =
                [b"[".as_slice(), b"{\"a\": ", b"]", b"}"][rng.below(4)].repeat(1 + rng.below(40));
            bytes.splice(at..at, pump);
        }
        // The next number, changed for one at an edge.
        6 | 7 => {
            let starts =
                |pair: &[u8]| pair[0] == b' ' && (pair[1].is_ascii_digit() || pair[1] == b'-');
            let Some(start) = rest.windows(2).position(starts).map(|found| at + found + 1) else {
                return bytes.truncate(at);
            };
            let number = |byte: &u8| byte.is_ascii_digit() || b"+-.eE".contains(byte);
            let length = bytes[start..]
                .iter()
                .take_while(|byte| number(byte))
                .count();
            let edge = EDGE_NUMBERS[rng.below(EDGE_NUMBERS.len())];
            bytes.splice(start..start + length, edge.bytes());
        }
        8 => drop(bytes.splice(at..at, TOKENS[rng.below(TOKENS.len())].iter().copied())),
        // The next string, a key or a value, changed for another name.
        _ => {
            let quote = |byte: &u8| *byte == b'"';
            let Some(start) = rest.iter().position(quote).map(|found| at + found + 1) else {
                return bytes.truncate(at);
            };
            let length = bytes[start..].iter().position(quote).unwrap_or(0);
            bytes.splice(start..start + length, NAMES[rng.below(NAMES.len())].bytes());
        }
    }
}

/// The fuzz run's input number `index` for a seed: its path when it is one file, and its bytes. Nine in ten are a
/// valid bundle or file, changed as content gets broken (its texts, its paths, the order and count of its files); one
/// in ten is random bytes or random text. Each is made from the seed and its own number alone.
pub fn fuzz_input(seed: u64, index: u64) -> (Option<String>, Vec<u8>) {
    let rng = &mut Rng::new(seed, (1 << 32) | index);
    if index % 10 == 9 {
        let (length, printable) = (rng.below(600), rng.below(2) == 0);
        let byte = |_| if printable { 0x20 + rng.below(95) } else { rng.below(256) } as u8;
        return (None, (0..length).map(byte).collect());
    }
    let count = if rng.below(16) == 0 {
        17 + rng.below(4)
    } else {
        1 + rng.below(3)
    };
    let mut entries = entries_of(&distinct(rng, count));
    if rng.below(3) == 0 {
        let map = Map::valid(rng, 0, false);
        entries.insert(0, (map.path(), map.text()));
    }
    let pick = rng.below(entries.len());
    match rng.below(12) {
        0 => entries.reverse(),
        1 => entries.push(entries[pick].clone()),
        2 => {
            entries[pick].0 =
                ["movement/x.txt", "movement/../a.json", "maps/A.json", ""][rng.below(4)].to_owned()
        }
        3 => entries[pick].0 = "movement/another.json".to_owned(),
        _ => {}
    }
    for _ in 0..rng.below(3) {
        mutate(rng, &mut entries[pick].1);
    }
    if index.is_multiple_of(5) {
        let (path, text) = entries.swap_remove(pick);
        return (Some(path), text);
    }
    let mut bundle = bundle_bytes(&entries);
    if rng.below(4) == 0 {
        mutate(rng, &mut bundle);
    }
    (None, bundle)
}

/// Judges one input and holds the loader to what it promises of every input: a refusal is one short printable
/// record and message; what is taken is the canonical writer's own text (by the writer that builds a string, where
/// the loader compared with the one that streams), holds no more files than the cap, and reads the same a second
/// time. The stage that settled the verdict comes back (a place in [`STAGES`]).
pub fn fuzz_judge(path: Option<&str>, bytes: &[u8]) -> Result<usize, String> {
    let read = || match path {
        Some(path) => load_file(path, bytes).map(|document| vec![document.id().to_owned()]),
        None => load(bytes).map(|set| {
            let maps = set.maps().iter().map(|map| map.id.clone());
            maps.chain(set.movements().iter().map(|movement| movement.id.clone()))
                .collect()
        }),
    };
    let result = read();
    let printable = |text: &str| text.bytes().all(|byte| (0x20..=0x7e).contains(&byte));
    match &result {
        Ok(ids) => {
            let top = json::parse(bytes)
                .map_err(|error| format!("taken, but refused alone: {}", error.record()))?;
            hold(canonical(&top).as_bytes() == bytes, || {
                "taken, but not canonical".to_owned()
            })?;
            hold(
                ids.len() <= MAX_FILES as usize && ids.iter().all(|id| json::is_id(id)),
                || format!("taken with more files than the cap or an id that is none: {ids:?}"),
            )?;
            hold(read() == result, || {
                "taken, and read differently a second time".to_owned()
            })?;
        }
        Err(error) => {
            let (record, message) = (error.record(), error.message());
            hold(
                message.len() <= MESSAGE_BYTES && printable(&message) && printable(&record),
                || format!("a refusal that is not short and printable: {record:?}, {message:?}"),
            )?;
            hold((error.line == 0) == (error.col == 0), || {
                format!("half a place: {record}")
            })?;
        }
    }
    Ok(stage(&result))
}
