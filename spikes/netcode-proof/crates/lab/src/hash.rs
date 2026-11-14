//! npt-lab's `hash` subcommand (u4-build-plan.md §5 U4d2, §4.15; plan §4.5's determinism check): the recordings
//! manifest regenerated and checked, two native legs in two processes, and the comparisons check.mjs's `wasm` step
//! makes with the Node loader's Wasm legs (crates/hash-wasm/loader.mjs). It reads no clock and uses no hash map
//! (crates/lab/clippy.toml bans both).
//!
//! ```text
//! npt-lab hash --manifest <file> --dir <folder> --out <folder> [--only <recording>]
//!              [--bite <id> --arm <tick> [--on <recording>]] [--dump-at <tick>]
//! npt-lab hash compare --ours <lines[,lines...]> --theirs <lines[,lines...]> --manifest <file> --dir <folder>
//!              [--ours-dump <file>] [--theirs-dump <file>] [--digest-field <field>]
//!              [--ours-name <name>] [--theirs-name <name>] [--bite <id> --arm <tick> [--on <recording>]]
//! npt-lab hash vectors --dir <folder> --dumps <folder> --committed <file>
//! npt-lab hash bites
//! ```
//! - The first form writes each scene's header-only recording into `--dir` (`vector-<scene>.rec`: seed 1, first tick
//!   0, no tick), for h2, and says so at once; then regenerates every manifest entry with `Recording::synthetic` there
//!   (`<name>.rec`), failing on any length or SHA-256 that is not the manifest's, and refusing, before anything is
//!   generated, an entry whose generator is not `Recording::GENERATOR` (carry 52); then runs two native legs, this
//!   binary re-run through `std::env::current_exe()` (D-2's form, sim/tests/replay.rs:144-171), each replaying the
//!   recordings (or `--only` one) through `Replayer` in lock step, and compares leg 1 with leg 2 by [`HashCompare`].
//!   Each leg's per-tick lines go to `--out` (`native-1.lines`, `native-2.lines`: `h <recording> <tick> <hash>`). At
//!   the first tick whose hashes differ, leg 2's dump goes to leg 1, which names the first field that differs
//!   (`StateDump::first_difference`). With a bite (bites builds only) the recording `--on` (default: the first
//!   replayed) runs with it armed at `--arm`, and each leg writes its order digest once armed; with `--dump-at`, each
//!   leg's dump at that tick of that recording goes to `--out` (`native-1.dump`, `native-2.dump`).
//! - `compare` compares two lists by [`HashCompare`], each read from its lines files (`--ours` and `--theirs` each
//!   name one file or several, comma-separated, read in order: the loader writes one per recording), each recording's
//!   tick count checked against the manifest. At the first mismatch it replays that recording natively to that tick in
//!   this process (with the bite, if given, armed as the legs arm it: on `--on`'s recording, by default the first the
//!   lists hold) and names the first differing field: against its own dump by `StateDump::first_difference` when
//!   `--ours-dump` is not given, or between the two dump files given, word by word, named by its own dump's
//!   `StateDump::field`. `--digest-field` prints each given dump's words under that field (the hash-map bite's folded
//!   order), or "layouts differ (<n> and <m> words)" for a dump whose length is not this build's own.
//! - `vectors` compares each scene's initial dump and hash on wasm32 (the loader's `vector-<scene>.dump` files in
//!   `--dumps`) with this build's, and CO's with the committed vector file (h2 (ii)).
//! - `bites` prints `World::BITES` in order, numbered from 1 as npt-hash-wasm's `begin` takes them (bites builds only).
//!
//! Exit 0 when everything compared is equal, 1 on a mismatch or a failed run, 2 on a usage error (a bite asked of the
//! measured build among them).

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use npt_lab::{HashCompare, TickHash};
use npt_sim::replay::{Recording, Replayer, Step};
use npt_sim::{FieldDiff, Scene, StateDump};
use serde_json::Value;
use sha2::{Digest, Sha256};

pub(crate) const USAGE: &str = "npt-lab hash --manifest <file> --dir <folder> --out <folder> [--only <recording>] \
[--bite <id> --arm <tick> [--on <recording>]] [--dump-at <tick>]\n       npt-lab hash compare --ours <lines[,lines...]> \
--theirs <lines[,lines...]> --manifest <file> --dir <folder> [--ours-dump <file>] [--theirs-dump <file>] [--digest-field <field>] \
[--ours-name <name>] [--theirs-name <name>] [--bite <id> --arm <tick> [--on <recording>]]\n       npt-lab hash vectors \
--dir <folder> --dumps <folder> --committed <file>\n       npt-lab hash bites";

/// Set in a leg's environment: its number, 1 or 2.
const LEG: &str = "NPT_LAB_HASH_LEG";
/// Words fed to a replayer between steps: the file streams in, never whole before the first step.
const CHUNK_WORDS: usize = 16_384;
/// The header-only recordings' ring span (the manifest's recordings' 2 s).
const VECTOR_SPAN_MS: u32 = 2_000;

/// How a run ended: `Usage` exits 2, `Failed` 1.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Failure {
    Usage(String),
    Failed(String),
}

/// A planted defect armed on one recording (bites builds only).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Arm {
    id: String,
    tick: u32,
    on: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    Legs {
        manifest: String,
        dir: String,
        out: String,
        only: Option<String>,
        arm: Option<Arm>,
        dump_at: Option<u32>,
    },
    Compare {
        ours: String,
        theirs: String,
        manifest: String,
        dir: String,
        ours_dump: Option<String>,
        theirs_dump: Option<String>,
        digest_field: Option<String>,
        names: [String; 2],
        arm: Option<Arm>,
    },
    Vectors {
        dir: String,
        dumps: String,
        committed: String,
    },
    Bites,
}

/// The arguments after `hash`: an optional mode word, then `--name value` pairs, each once.
pub(crate) fn parse(argv: &[String]) -> Result<Mode, String> {
    let (mode, rest) = match argv.first().map(String::as_str) {
        Some(m @ ("compare" | "vectors" | "bites")) => (m, &argv[1..]),
        _ => ("legs", argv),
    };
    let mut map: Vec<(&str, &str)> = Vec::new();
    let mut it = rest.iter();
    while let Some(flag) = it.next() {
        let name = flag
            .strip_prefix("--")
            .ok_or(format!("unexpected argument {flag:?}"))?;
        let value = it.next().ok_or(format!("{flag} needs a value"))?;
        if map.iter().any(|(k, _)| *k == name) {
            return Err(format!("{flag} given twice"));
        }
        map.push((name, value));
    }
    let mut take = |k: &str| {
        let at = map.iter().position(|(n, _)| *n == k)?;
        Some(map.remove(at).1.to_owned())
    };
    let need = |v: Option<String>, k: &str| v.ok_or(format!("--{k} is required"));
    let tick = |v: Option<String>, k: &str| -> Result<Option<u32>, String> {
        v.map(|s| {
            s.parse::<u32>()
                .map_err(|_| format!("--{k} {s:?} is not a tick (0 to 4294967295)"))
        })
        .transpose()
    };
    let parsed = match mode {
        "bites" => Mode::Bites,
        "vectors" => Mode::Vectors {
            dir: need(take("dir"), "dir")?,
            dumps: need(take("dumps"), "dumps")?,
            committed: need(take("committed"), "committed")?,
        },
        _ => {
            let bite = take("bite");
            let arm_tick = tick(take("arm"), "arm")?;
            let on = take("on");
            let arm = match (bite, arm_tick) {
                (Some(id), Some(tick)) => Some(Arm { id, tick, on }),
                (None, None) if on.is_none() => None,
                _ => return Err("--bite and --arm go together (and --on needs them)".to_owned()),
            };
            if mode == "compare" {
                Mode::Compare {
                    ours: need(take("ours"), "ours")?,
                    theirs: need(take("theirs"), "theirs")?,
                    manifest: need(take("manifest"), "manifest")?,
                    dir: need(take("dir"), "dir")?,
                    ours_dump: take("ours-dump"),
                    theirs_dump: take("theirs-dump"),
                    digest_field: take("digest-field"),
                    names: [
                        take("ours-name").unwrap_or_else(|| "ours".to_owned()),
                        take("theirs-name").unwrap_or_else(|| "theirs".to_owned()),
                    ],
                    arm,
                }
            } else {
                Mode::Legs {
                    manifest: need(take("manifest"), "manifest")?,
                    dir: need(take("dir"), "dir")?,
                    out: need(take("out"), "out")?,
                    only: take("only"),
                    arm,
                    dump_at: tick(take("dump-at"), "dump-at")?,
                }
            }
        }
    };
    if let Some((k, _)) = map.first() {
        return Err(format!("unknown option --{k}"));
    }
    let wants_bite = match &parsed {
        Mode::Bites => true,
        Mode::Legs { arm, .. } | Mode::Compare { arm, .. } => arm.is_some(),
        Mode::Vectors { .. } => false,
    };
    if wants_bite && !cfg!(feature = "bites") {
        return Err(
            "a bite asked of a measured build, which holds none (build npt-lab with --features bites)"
                .to_owned(),
        );
    }
    Ok(parsed)
}

/// One recordings manifest entry (u4-build-plan.md §4.15; crates/lab/recordings.json).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Entry {
    pub(crate) name: String,
    pub(crate) scene: u8,
    pub(crate) seed: u64,
    pub(crate) first_tick: u32,
    pub(crate) ticks: u32,
    pub(crate) span_ms: u32,
    pub(crate) generator: u32,
    pub(crate) bytes: u64,
    pub(crate) sha256: String,
}

/// The manifest's entries, in its order.
pub(crate) fn manifest(text: &str) -> Result<Vec<Entry>, String> {
    let v: Value =
        serde_json::from_str(text).map_err(|e| format!("the manifest is not JSON: {e}"))?;
    let list = v
        .get("recordings")
        .and_then(Value::as_array)
        .ok_or("the manifest has no \"recordings\" list")?;
    let mut out = Vec::new();
    for (i, e) in list.iter().enumerate() {
        let name = e
            .get("name")
            .and_then(Value::as_str)
            .ok_or(format!("manifest entry {i} has no name"))?
            .to_owned();
        let num = |k: &str| {
            e.get(k)
                .and_then(Value::as_u64)
                .ok_or(format!("manifest entry {name}: no whole number \"{k}\""))
        };
        let small = |k: &str| {
            num(k).and_then(|n| {
                u32::try_from(n)
                    .map_err(|_| format!("manifest entry {name}: \"{k}\" {n} is above 32 bits"))
            })
        };
        let entry = Entry {
            scene: u8::try_from(num("scene")?)
                .map_err(|_| format!("manifest entry {name}: no such scene"))?,
            seed: num("seed")?,
            first_tick: small("first_tick")?,
            ticks: small("ticks")?,
            span_ms: small("span_ms")?,
            generator: small("generator")?,
            bytes: num("bytes")?,
            sha256: e
                .get("sha256")
                .and_then(Value::as_str)
                .ok_or(format!("manifest entry {name}: no \"sha256\""))?
                .to_owned(),
            name,
        };
        if out.iter().any(|o: &Entry| o.name == entry.name) {
            return Err(format!("manifest entry {} is named twice", entry.name));
        }
        out.push(entry);
    }
    Ok(out)
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// The entry's recording, generated (after its generator is checked: `regenerate_all` checks every entry first).
fn generate(e: &Entry) -> Result<Vec<u8>, String> {
    let scene = Scene::by_id(e.scene).ok_or(format!("{}: no scene {}", e.name, e.scene))?;
    Ok(Recording::synthetic(&scene, e.seed, e.first_tick, e.ticks, e.span_ms).encode())
}

/// Whether `bytes` are the entry's: its length and SHA-256.
pub(crate) fn check(e: &Entry, bytes: &[u8]) -> Result<(), String> {
    let sha = sha256_hex(bytes);
    if bytes.len() as u64 != e.bytes || sha != e.sha256 {
        return Err(format!(
            "{}: regenerated {} bytes with sha256 {sha}; the manifest has {} bytes with sha256 {}",
            e.name,
            bytes.len(),
            e.bytes,
            e.sha256
        ));
    }
    Ok(())
}

/// Every entry regenerated and checked, each handed to `keep` once it holds; an entry whose generator is not
/// `Recording::GENERATOR` is refused before anything is generated (carry 52: only GENERATOR 2's recordings are pinned).
pub(crate) fn regenerate_all(
    entries: &[Entry],
    mut keep: impl FnMut(&Entry, &[u8]) -> Result<(), String>,
) -> Result<Vec<String>, String> {
    for e in entries {
        if e.generator != Recording::GENERATOR {
            return Err(format!(
                "{}: generator {}, not this build's Recording::GENERATOR {} (only its recordings are pinned)",
                e.name,
                e.generator,
                Recording::GENERATOR
            ));
        }
    }
    let mut lines = Vec::new();
    for e in entries {
        let bytes = generate(e)?;
        check(e, &bytes)?;
        keep(e, &bytes)?;
        lines.push(format!(
            "regenerated {}: {} bytes, sha256 {}, as the manifest's",
            e.name,
            bytes.len(),
            e.sha256
        ));
    }
    Ok(lines)
}

/// Bytes as little-endian 32-bit words, the last zero-padded.
fn le_words(bytes: &[u8]) -> Vec<u32> {
    bytes
        .chunks(4)
        .map(|c| {
            let mut w = [0u8; 4];
            w[..c.len()].copy_from_slice(c);
            u32::from_le_bytes(w)
        })
        .collect()
}

/// The scenes h2 reads, by name, and their header-only recordings' file names.
fn vector_scenes() -> [Scene; 4] {
    [
        Scene::stress(),
        Scene::estimate(),
        Scene::course(),
        Scene::targets(),
    ]
}

fn header_only(scene: &Scene) -> Vec<u8> {
    Recording {
        scene: scene.id(),
        seed: 1,
        first_tick: 0,
        span_ms: VECTOR_SPAN_MS,
        generator: 0,
        ticks: Vec::new(),
    }
    .encode()
}

/// A replayer for `bite` (bites builds only; parse refuses one in the measured build).
fn replayer(arm: Option<&Arm>) -> Result<Replayer, Failure> {
    let Some(a) = arm else {
        return Ok(Replayer::new());
    };
    bitten(a)
}

#[cfg(feature = "bites")]
fn bitten(a: &Arm) -> Result<Replayer, Failure> {
    Replayer::with_bite(&a.id, a.tick).ok_or(Failure::Usage(format!(
        "--bite {}: npt-sim's World plants no such bite (it plants {:?})",
        a.id,
        npt_sim::World::BITES
    )))
}

#[cfg(not(feature = "bites"))]
fn bitten(a: &Arm) -> Result<Replayer, Failure> {
    Err(Failure::Usage(format!(
        "--bite {}: a measured build holds no bites",
        a.id
    )))
}

#[cfg(feature = "bites")]
fn order_digest(r: &Replayer) -> Option<u64> {
    r.world().and_then(npt_sim::World::bite_order_digest)
}

#[cfg(not(feature = "bites"))]
fn order_digest(_r: &Replayer) -> Option<u64> {
    None
}

fn read_text(path: &str) -> Result<String, Failure> {
    std::fs::read_to_string(path).map_err(|e| Failure::Failed(format!("{path}: {e}")))
}

fn rec_path(dir: &str, name: &str) -> PathBuf {
    Path::new(dir).join(format!("{name}.rec"))
}

/// The entries a run replays: all, or `--only`'s.
fn selected<'a>(entries: &'a [Entry], only: Option<&str>) -> Result<Vec<&'a Entry>, Failure> {
    match only {
        None => Ok(entries.iter().collect()),
        Some(n) => entries
            .iter()
            .find(|e| e.name == n)
            .map(|e| vec![e])
            .ok_or(Failure::Usage(format!(
                "--only {n}: no such recording in the manifest"
            ))),
    }
}

/// The recording a bite is armed on: `--on`'s, or the first replayed.
fn bitten_name(arm: Option<&Arm>, replayed: &[&Entry]) -> Result<Option<String>, Failure> {
    let Some(a) = arm else {
        return Ok(None);
    };
    match &a.on {
        Some(n) if replayed.iter().any(|e| &e.name == n) => Ok(Some(n.clone())),
        Some(n) => Err(Failure::Usage(format!(
            "--on {n}: not a recording this run replays"
        ))),
        None => Ok(replayed.first().map(|e| e.name.clone())),
    }
}

/// The bite a replay of `e` runs with: `arm`'s when `e` is the recording it is armed on (`on`, bitten_name's), else
/// none. The legs and `compare` arm it alike, one default (the engine review's EA-9).
fn arm_for<'a>(arm: Option<&'a Arm>, on: Option<&str>, e: &Entry) -> Option<&'a Arm> {
    arm.filter(|_| on == Some(e.name.as_str()))
}

/// A line of a leg's (or a Wasm leg's) per-tick list: `h <recording> <tick> <hash>`.
pub(crate) fn hash_line(h: &TickHash) -> String {
    format!("h {} {} {:016x}", h.recording, h.tick, h.hash)
}

pub(crate) fn parse_hash_line(line: &str) -> Option<TickHash> {
    let mut p = line.strip_prefix("h ")?.split(' ');
    let recording = p.next()?.to_owned();
    let tick = p.next()?.parse().ok()?;
    let hash = u64::from_str_radix(p.next()?, 16).ok()?;
    p.next().is_none().then_some(TickHash {
        recording,
        tick,
        hash,
    })
}

/// The entries of one or more lines files, `paths` comma-separated and read in order (the loader writes one file per
/// recording): their `h` lines; any other line, such as the loader's `stepped`, is skipped.
fn read_lines(paths: &str) -> Result<Vec<TickHash>, Failure> {
    let mut out = Vec::new();
    for path in paths.split(',') {
        for (i, l) in read_text(path)?.lines().enumerate() {
            if l.starts_with("h ") {
                out.push(parse_hash_line(l).ok_or(Failure::Failed(format!(
                    "{path}:{}: not a hash line: {l:?}",
                    i + 1
                )))?);
            }
        }
    }
    Ok(out)
}

/// A dump file (the loader's and the legs'): `dump <recording> <tick|end>`, then `hash <16 hex>` (optional), then
/// one word a line as hex.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DumpFile {
    pub(crate) recording: String,
    pub(crate) tick: Option<u32>,
    pub(crate) hash: Option<u64>,
    pub(crate) words: Vec<u32>,
}

pub(crate) fn parse_dump(text: &str, what: &str) -> Result<DumpFile, String> {
    let mut lines = text.lines();
    let head = lines.next().unwrap_or("");
    let mut p = head
        .strip_prefix("dump ")
        .ok_or(format!("{what}: no `dump <recording> <tick>` line"))?
        .split(' ');
    let recording = p.next().unwrap_or("").to_owned();
    let tick = match p.next() {
        Some("end") => None,
        Some(t) => Some(t.parse().map_err(|_| format!("{what}: tick {t:?}"))?),
        None => return Err(format!("{what}: no tick")),
    };
    let mut hash = None;
    let mut words = Vec::new();
    for l in lines {
        if let Some(h) = l.strip_prefix("hash ") {
            hash = Some(u64::from_str_radix(h, 16).map_err(|_| format!("{what}: hash {h:?}"))?);
        } else if !l.is_empty() {
            words.push(u32::from_str_radix(l, 16).map_err(|_| format!("{what}: word {l:?}"))?);
        }
    }
    Ok(DumpFile {
        recording,
        tick,
        hash,
        words,
    })
}

fn dump_text(recording: &str, tick: u32, hash: u64, words: &[u32]) -> String {
    let mut s = format!("dump {recording} {tick}\nhash {hash:016x}\n");
    for w in words {
        s.push_str(&format!("{w:08x}\n"));
    }
    s
}

fn hex_words(words: &[u32]) -> String {
    words
        .iter()
        .map(|w| format!("{w:x}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_hex_words(s: &str) -> Option<Vec<u32>> {
    if s.is_empty() {
        return Some(Vec::new());
    }
    s.split(' ')
        .map(|w| u32::from_str_radix(w, 16).ok())
        .collect()
}

fn diff_text(d: &FieldDiff) -> String {
    format!(
        "field {} (word {}): {:08x} and {:08x}",
        d.field, d.word, d.ours, d.theirs
    )
}

/// The first word two word-only dumps differ at, named by `names` (this build's own dump of the same tick; npt-sim's
/// `StateDump::field`), or `None` when they are equal: the dumps of two other processes arrive as words alone.
fn words_difference(names: &StateDump, ours: &[u32], theirs: &[u32]) -> Option<FieldDiff> {
    let n = ours.len().max(theirs.len());
    (0..n)
        .find(|&i| ours.get(i) != theirs.get(i))
        .map(|i| FieldDiff {
            word: i,
            field: names.field(i).unwrap_or_else(|| "end".to_owned()),
            ours: ours.get(i).copied().unwrap_or(0),
            theirs: theirs.get(i).copied().unwrap_or(0),
        })
}

/// Each recording's tick count in a list, in the order met.
fn counts(list: &[TickHash]) -> Vec<(String, usize)> {
    let mut out: Vec<(String, usize)> = Vec::new();
    for h in list {
        match out.last_mut() {
            Some((n, k)) if *n == h.recording => *k += 1,
            _ => out.push((h.recording.clone(), 1)),
        }
    }
    out
}

fn counts_text(list: &[TickHash]) -> String {
    counts(list)
        .iter()
        .map(|(n, k)| format!("{n} {k}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Whether `list` holds each of `entries`' ticks, in order, exactly the manifest's count each (a short leg fails).
fn count_problems(what: &str, list: &[TickHash], entries: &[&Entry]) -> Vec<String> {
    let want: Vec<(String, usize)> = entries
        .iter()
        .map(|e| (e.name.clone(), e.ticks as usize))
        .collect();
    let got = counts(list);
    if got == want {
        return Vec::new();
    }
    vec![format!(
        "{what} holds {} ticks, not the manifest's {}",
        if got.is_empty() {
            "no".to_owned()
        } else {
            counts_text(list)
        },
        want.iter()
            .map(|(n, k)| format!("{n} {k}"))
            .collect::<Vec<_>>()
            .join(", ")
    )]
}

/// One `HashCompare` as a line: `<a> against <b>: ...`.
fn compare_text(names: &[String; 2], c: &HashCompare, field: Option<&str>) -> String {
    let [a, b] = names;
    match c {
        HashCompare::Equal { entries } => {
            format!("{a} against {b}: {entries} ticks compared, 0 mismatching, equal")
        }
        HashCompare::Empty => format!("{a} against {b}: 0 ticks compared (nothing to compare)"),
        HashCompare::Mismatch {
            index,
            ours,
            theirs,
            compared,
            mismatching,
        } => {
            let side = |h: &Option<TickHash>| match h {
                Some(h) => format!("{} tick {} hash {:016x}", h.recording, h.tick, h.hash),
                None => "its end".to_owned(),
            };
            format!(
                "{a} against {b}: {compared} ticks compared, {mismatching} mismatching; first mismatch at entry \
                 {index}: {} against {}; {}",
                side(ours),
                side(theirs),
                field.unwrap_or("no field (no dump at that tick)")
            )
        }
    }
}

/// A leg of the lock-step comparison: this binary re-run as a child.
struct Leg {
    k: u32,
    child: Child,
    stdin: ChildStdin,
    out: BufReader<ChildStdout>,
    digest: Option<String>,
}

enum LegLine {
    Hash(TickHash),
    End,
}

impl Leg {
    fn spawn(k: u32, argv: &[String]) -> Result<Leg, Failure> {
        let exe = std::env::current_exe()
            .map_err(|e| Failure::Failed(format!("this binary's path: {e}")))?;
        let mut child = Command::new(exe)
            .arg("hash")
            .args(argv)
            .env(LEG, k.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| Failure::Failed(format!("leg {k} did not start: {e}")))?;
        let stdin = child.stdin.take().expect("piped");
        let out = BufReader::new(child.stdout.take().expect("piped"));
        Ok(Leg {
            k,
            child,
            stdin,
            out,
            digest: None,
        })
    }

    fn line(&mut self) -> Result<String, Failure> {
        let mut line = String::new();
        let n = self
            .out
            .read_line(&mut line)
            .map_err(|e| Failure::Failed(format!("leg {}'s output: {e}", self.k)))?;
        if n == 0 {
            return Err(Failure::Failed(format!(
                "leg {} ended without its last line",
                self.k
            )));
        }
        Ok(line.trim_end().to_owned())
    }

    fn next(&mut self) -> Result<LegLine, Failure> {
        loop {
            let l = self.line()?;
            if let Some(d) = l.strip_prefix("o ") {
                self.digest = Some(d.to_owned());
                continue;
            }
            if l == "end" {
                return Ok(LegLine::End);
            }
            return parse_hash_line(&l)
                .map(LegLine::Hash)
                .ok_or(Failure::Failed(format!("leg {} wrote {l:?}", self.k)));
        }
    }

    fn send(&mut self, word: &str) -> Result<(), Failure> {
        writeln!(self.stdin, "{word}")
            .and_then(|()| self.stdin.flush())
            .map_err(|e| Failure::Failed(format!("leg {}'s input: {e}", self.k)))
    }

    fn ask(&mut self, word: &str, prefix: &str) -> Result<String, Failure> {
        self.send(word)?;
        let l = self.line()?;
        l.strip_prefix(prefix)
            .map(str::to_owned)
            .ok_or(Failure::Failed(format!(
                "leg {} answered {word:?} with {l:?}",
                self.k
            )))
    }

    fn finish(mut self) -> Result<(), Failure> {
        drop(self.stdin);
        let status = self
            .child
            .wait()
            .map_err(|e| Failure::Failed(format!("leg {}: {e}", self.k)))?;
        if !status.success() {
            return Err(Failure::Failed(format!("leg {} failed: {status}", self.k)));
        }
        Ok(())
    }
}

fn write_file(path: &Path, text: &str) -> Result<(), Failure> {
    std::fs::write(path, text).map_err(|e| Failure::Failed(format!("{}: {e}", path.display())))
}

fn lines_text(list: &[TickHash]) -> String {
    let mut s = String::new();
    for h in list {
        s.push_str(&hash_line(h));
        s.push('\n');
    }
    s
}

/// The first form: regenerate, check, write the header-only recordings, run the two legs, compare.
fn legs(argv: &[String], mode: &Mode) -> Result<(Vec<String>, bool), Failure> {
    let Mode::Legs {
        manifest: m,
        dir,
        out,
        only,
        arm,
        dump_at,
    } = mode
    else {
        unreachable!("legs runs the first form")
    };
    let entries = manifest(&read_text(m)?).map_err(Failure::Failed)?;
    let replayed = selected(&entries, only.as_deref())?;
    let on = bitten_name(arm.as_ref(), &replayed)?;
    // `--dump-at` names a tick of the bitten recording, or of `--only`'s.
    let dump_on = on.clone().or_else(|| only.clone());
    for d in [dir, out] {
        std::fs::create_dir_all(d).map_err(|e| Failure::Failed(format!("{d}: {e}")))?;
    }
    // The header-only recordings first, said at once, so h2's vectors can run even if a regeneration fails.
    let mut written = Vec::new();
    for s in vector_scenes() {
        let name = format!("vector-{}", s.name());
        std::fs::write(rec_path(dir, &name), header_only(&s))
            .map_err(|e| Failure::Failed(format!("{name}: {e}")))?;
        written.push(name);
    }
    println!(
        "wrote the header-only recordings {} (seed 1, first tick 0, no tick) for h2",
        written.join(", ")
    );
    let mut lines = regenerate_all(&entries, |e, bytes| {
        std::fs::write(rec_path(dir, &e.name), bytes)
            .map_err(|err| format!("{}: {err}", rec_path(dir, &e.name).display()))
    })
    .map_err(Failure::Failed)?;
    let mut a = Leg::spawn(1, argv)?;
    let mut b = Leg::spawn(2, argv)?;
    let (mut la, mut lb) = (Vec::new(), Vec::new());
    let (mut a_done, mut b_done) = (false, false);
    let mut field: Option<String> = None;
    while !(a_done && b_done) {
        let x = if a_done { None } else { Some(a.next()?) };
        let y = if b_done { None } else { Some(b.next()?) };
        let (ha, hb) = match (x, y) {
            (Some(LegLine::Hash(ha)), Some(LegLine::Hash(hb))) => (Some(ha), Some(hb)),
            (Some(LegLine::Hash(ha)), other) => {
                b_done |= matches!(other, None | Some(LegLine::End));
                (Some(ha), None)
            }
            (other, Some(LegLine::Hash(hb))) => {
                a_done |= matches!(other, None | Some(LegLine::End));
                (None, Some(hb))
            }
            _ => {
                a_done = true;
                b_done = true;
                (None, None)
            }
        };
        if let (Some(ha), Some(hb)) = (&ha, &hb)
            && ha != hb
            && field.is_none()
        {
            field = Some(
                if (ha.recording.as_str(), ha.tick) == (hb.recording.as_str(), hb.tick) {
                    let theirs = b.ask("d", "d ")?;
                    let f = a.ask(&format!("c {theirs}"), "f ")?;
                    format!("field {f}")
                } else {
                    "the legs stepped other ticks".to_owned()
                },
            );
        }
        if let (Some(ha), Some(hb), Some(t), Some(r)) = (&ha, &hb, dump_at, &dump_on)
            && ha.tick == *t
            && &ha.recording == r
        {
            for (leg, h, k) in [(&mut a, ha, 1), (&mut b, hb, 2)] {
                let words = parse_hex_words(&leg.ask("d", "d ")?)
                    .ok_or(Failure::Failed(format!("leg {k}'s dump")))?;
                write_file(
                    &Path::new(out).join(format!("native-{k}.dump")),
                    &dump_text(&h.recording, h.tick, h.hash, &words),
                )?;
            }
            lines.push(format!(
                "dump at {r} tick {t}: each leg's written (native-1.dump, native-2.dump)"
            ));
        }
        if let Some(h) = ha {
            la.push(h);
            a.send("n")?;
        }
        if let Some(h) = hb {
            lb.push(h);
            b.send("n")?;
        }
    }
    let digests = [a.digest.clone(), b.digest.clone()];
    a.finish()?;
    b.finish()?;
    write_file(&Path::new(out).join("native-1.lines"), &lines_text(&la))?;
    write_file(&Path::new(out).join("native-2.lines"), &lines_text(&lb))?;
    if let (Some(a), Some(r)) = (arm, &on) {
        for (k, d) in digests.iter().enumerate() {
            lines.push(format!(
                "leg {} order digest {} ({} armed at {r} tick {})",
                k + 1,
                d.as_deref().unwrap_or("none"),
                a.id,
                a.tick
            ));
        }
    }
    let mut problems = count_problems("native leg 1", &la, &replayed);
    problems.extend(count_problems("native leg 2", &lb, &replayed));
    let c = HashCompare::of(&la, &lb);
    let equal = matches!(c, HashCompare::Equal { .. });
    lines.extend(problems.iter().cloned());
    let names = ["native leg 1".to_owned(), "native leg 2".to_owned()];
    lines.push(format!(
        "{} ({})",
        compare_text(&names, &c, field.as_deref()),
        counts_text(&la)
    ));
    Ok((lines, equal && problems.is_empty()))
}

/// A leg: replays each recording in lock step with the parent (the protocol in the module's first form).
fn serve(mode: &Mode) -> Result<(), Failure> {
    let Mode::Legs {
        manifest: m,
        dir,
        only,
        arm,
        ..
    } = mode
    else {
        return Err(Failure::Usage("a leg runs the first form only".to_owned()));
    };
    let entries = manifest(&read_text(m)?).map_err(Failure::Failed)?;
    let replayed = selected(&entries, only.as_deref())?;
    let on = bitten_name(arm.as_ref(), &replayed)?;
    let mut out = std::io::stdout().lock();
    let mut input = std::io::stdin().lock().lines();
    let io = |e: std::io::Error| Failure::Failed(format!("a leg's pipe: {e}"));
    for e in &replayed {
        let path = rec_path(dir, &e.name);
        let bytes = std::fs::read(&path)
            .map_err(|err| Failure::Failed(format!("{}: {err}", path.display())))?;
        let words = le_words(&bytes);
        let mut rp = replayer(arm_for(arm.as_ref(), on.as_deref(), e))?;
        let mut digest_written = false;
        let mut chunks = words.chunks(CHUNK_WORDS);
        loop {
            let tick = match rp.step() {
                Step::Stepped(tick) => tick,
                Step::NeedMore => match chunks.next() {
                    Some(c) => {
                        for w in c {
                            rp.feed(*w);
                        }
                        continue;
                    }
                    None => {
                        return Err(Failure::Failed(format!(
                            "{}: the file ends inside a tick",
                            e.name
                        )));
                    }
                },
                Step::Done => break,
                Step::Failed(why) => return Err(Failure::Failed(format!("{}: {why}", e.name))),
            };
            if !digest_written && let Some(d) = order_digest(&rp) {
                writeln!(out, "o {d:016x}").map_err(io)?;
                digest_written = true;
            }
            let hash = rp.state_hash().expect("a stepped replayer has a world");
            let h = TickHash {
                recording: e.name.clone(),
                tick,
                hash,
            };
            writeln!(out, "{}", hash_line(&h)).map_err(io)?;
            out.flush().map_err(io)?;
            loop {
                let line = match input.next() {
                    Some(l) => l.map_err(io)?,
                    None => return Ok(()),
                };
                let dump = || rp.dump().expect("a stepped replayer has a world");
                let (word, rest) = line.split_once(' ').unwrap_or((line.as_str(), ""));
                match word {
                    "n" => break,
                    "q" => return Ok(()),
                    "d" => writeln!(out, "d {}", hex_words(dump().words())).map_err(io)?,
                    "c" => {
                        let theirs = parse_hex_words(rest)
                            .ok_or(Failure::Failed("the parent's words".to_owned()))?;
                        match dump().first_difference(&theirs) {
                            Some(d) => {
                                writeln!(out, "f {}", diff_text(&d).trim_start_matches("field "))
                            }
                            None => writeln!(out, "f none: the dumps are equal"),
                        }
                        .map_err(io)?;
                    }
                    other => return Err(Failure::Failed(format!("the parent's word {other:?}"))),
                }
                out.flush().map_err(io)?;
            }
        }
    }
    writeln!(out, "end").map_err(io)?;
    out.flush().map_err(io)
}

/// This build's own replay of `entry` to `tick`, its dump then, with `arm`'s bite armed when given (its caller passes
/// arm_for's, so only on the recording the bite is armed on).
fn replay_to(dir: &str, e: &Entry, tick: u32, arm: Option<&Arm>) -> Result<StateDump, Failure> {
    let path = rec_path(dir, &e.name);
    let bytes = std::fs::read(&path)
        .map_err(|err| Failure::Failed(format!("{}: {err}", path.display())))?;
    let mut rp = replayer(arm)?;
    for w in le_words(&bytes) {
        rp.feed(w);
    }
    loop {
        match rp.step() {
            Step::Stepped(t) if t == tick => {
                return Ok(rp.dump().expect("a stepped replayer has a world"));
            }
            Step::Stepped(_) => {}
            other => {
                return Err(Failure::Failed(format!(
                    "{}: this build's replay never reached tick {tick} ({other:?})",
                    e.name
                )));
            }
        }
    }
}

fn read_dump(path: &str) -> Result<DumpFile, Failure> {
    parse_dump(&read_text(path)?, path).map_err(Failure::Failed)
}

/// What `--digest-field` prints for one dump's `words`: the words at `at` (the field's indexes in this build's own
/// dump, `own_len` words long), as one u64 when there are two (the low word first, as npt-sim's hash writes it), or
/// "layouts differ (<n> and <m> words)" when the dump's length is not this build's own: its words at those indexes
/// would be other fields' (the rust review's RR-L3: P6's word-size plant).
fn digest_text(own_len: usize, at: &[usize], words: &[u32]) -> String {
    if words.len() != own_len {
        return format!("layouts differ ({own_len} and {} words)", words.len());
    }
    let picked: Vec<u32> = at.iter().filter_map(|&i| words.get(i).copied()).collect();
    match picked.as_slice() {
        [lo, hi] => format!("{:016x}", u64::from(*hi) << 32 | u64::from(*lo)),
        [] => "none".to_owned(),
        other => hex_words(other),
    }
}

/// `compare`: two lines files, and at a mismatch the field diff.
fn compare(mode: &Mode) -> Result<(Vec<String>, bool), Failure> {
    let Mode::Compare {
        ours,
        theirs,
        manifest: m,
        dir,
        ours_dump,
        theirs_dump,
        digest_field,
        names,
        arm,
    } = mode
    else {
        unreachable!("compare runs the compare form")
    };
    let entries = manifest(&read_text(m)?).map_err(Failure::Failed)?;
    let (lo, lt) = (read_lines(ours)?, read_lines(theirs)?);
    let dumps = [
        ours_dump.as_deref().map(read_dump).transpose()?,
        theirs_dump.as_deref().map(read_dump).transpose()?,
    ];
    // The recordings each list replays, in the manifest's order, each with the manifest's tick count.
    let listed: Vec<&Entry> = entries
        .iter()
        .filter(|e| lo.iter().chain(&lt).any(|h| h.recording == e.name))
        .collect();
    let mut lines = Vec::new();
    let mut problems = count_problems(&names[0], &lo, &listed);
    problems.extend(count_problems(&names[1], &lt, &listed));
    let entry = |name: &str| entries.iter().find(|e| e.name == name);
    // The bite is armed as the legs arm it: on `--on`'s recording, else the first the lists hold (EA-9).
    let on = bitten_name(arm.as_ref(), &listed)?;
    let armed = |e: &Entry| arm_for(arm.as_ref(), on.as_deref(), e);
    if let Some(f) = digest_field
        && let Some(d) = dumps.iter().flatten().next()
        && let (Some(t), Some(e)) = (d.tick, entry(&d.recording))
    {
        let own = replay_to(dir, e, t, armed(e))?;
        let at: Vec<usize> = (0..own.words().len())
            .filter(|&i| own.field(i).as_deref() == Some(f.as_str()))
            .collect();
        for (k, d) in dumps.iter().enumerate() {
            let Some(d) = d else { continue };
            let digest = digest_text(own.words().len(), &at, &d.words);
            lines.push(format!(
                "{} digest {digest} ({f} at words {:?} of its dump at {} tick {t})",
                names[k], at, d.recording
            ));
        }
    }
    let c = HashCompare::of(&lo, &lt);
    let field = match &c {
        HashCompare::Mismatch {
            ours: Some(a),
            theirs: Some(b),
            ..
        } if (a.recording.as_str(), a.tick) == (b.recording.as_str(), b.tick) => {
            let at = |d: &Option<DumpFile>| {
                d.as_ref()
                    .filter(|d| d.recording == a.recording && d.tick == Some(a.tick))
                    .map(|d| d.words.clone())
            };
            match (entry(&a.recording), at(&dumps[0]), at(&dumps[1])) {
                (None, ..) => Some(format!("no field ({} is not in the manifest)", a.recording)),
                (Some(_), _, None) => None,
                (Some(e), None, Some(t)) if ours_dump.is_none() => {
                    let own = replay_to(dir, e, a.tick, armed(e))?;
                    Some(match own.first_difference(&t) {
                        Some(d) => {
                            format!("{} (ours: this build's replay to that tick)", diff_text(&d))
                        }
                        None => "no field: this build's dump there equals theirs".to_owned(),
                    })
                }
                (Some(_), None, Some(_)) => {
                    Some("no field (ours' dump is not at that tick)".to_owned())
                }
                (Some(e), Some(o), Some(t)) => {
                    let own = replay_to(dir, e, a.tick, armed(e))?;
                    Some(match words_difference(&own, &o, &t) {
                        Some(d) => format!(
                            "{} (both dumps given, named by this build's)",
                            diff_text(&d)
                        ),
                        None => "no field: the two dumps given are equal".to_owned(),
                    })
                }
            }
        }
        HashCompare::Mismatch { .. } => Some("the lists step other ticks or one ends".to_owned()),
        _ => None,
    };
    let equal = matches!(c, HashCompare::Equal { .. });
    lines.extend(problems.iter().cloned());
    lines.push(format!(
        "{} ({})",
        compare_text(names, &c, field.as_deref()),
        counts_text(&lo)
    ));
    Ok((lines, equal && problems.is_empty()))
}

/// The committed vector file: its digest and words.
pub(crate) fn committed_vector(text: &str) -> Result<(u64, Vec<u32>), String> {
    let mut digest = None;
    let mut words = Vec::new();
    for l in text.lines().map(str::trim) {
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        if let Some(d) = l.strip_prefix("digest ") {
            digest = Some(u64::from_str_radix(d, 16).map_err(|_| format!("digest {d:?}"))?);
        } else {
            words.push(u32::from_str_radix(l, 16).map_err(|_| format!("word {l:?}"))?);
        }
    }
    Ok((digest.ok_or("no digest line")?, words))
}

/// `vectors`: h2 (ii), every scene's initial dump on wasm32 against this build's, and CO's against the committed file.
fn vectors(mode: &Mode) -> Result<(Vec<String>, bool), Failure> {
    let Mode::Vectors {
        dir,
        dumps,
        committed,
    } = mode
    else {
        unreachable!("vectors runs the vectors form")
    };
    let (digest, words) = committed_vector(&read_text(committed)?)
        .map_err(|e| Failure::Failed(format!("{committed}: {e}")))?;
    let mut lines = Vec::new();
    let mut ok = true;
    let mut sizes = Vec::new();
    for s in vector_scenes() {
        let name = format!("vector-{}", s.name());
        let mut rp = Replayer::new();
        let path = rec_path(dir, &name);
        let bytes = std::fs::read(&path)
            .map_err(|e| Failure::Failed(format!("{}: {e}", path.display())))?;
        for w in le_words(&bytes) {
            rp.feed(w);
        }
        if rp.step() != Step::Done {
            return Err(Failure::Failed(format!(
                "{name}: not a header-only recording"
            )));
        }
        let own = rp.dump().expect("the header builds the world");
        let own_hash = rp.state_hash().expect("the header builds the world");
        let wasm = read_dump(
            &Path::new(dumps)
                .join(format!("{name}.dump"))
                .display()
                .to_string(),
        )?;
        sizes.push(format!("{} {}", s.name(), own.words().len()));
        if let Some(d) = own.first_difference(&wasm.words) {
            ok = false;
            lines.push(format!(
                "h2: {}'s initial dump differs on wasm32: {}",
                s.name(),
                diff_text(&d)
            ));
        }
        if wasm.hash != Some(own_hash) {
            ok = false;
            lines.push(format!(
                "h2: {}'s initial hash differs on wasm32: {:016x} natively, {} on wasm32",
                s.name(),
                own_hash,
                wasm.hash.map_or("none".to_owned(), |h| format!("{h:016x}"))
            ));
        }
        if s.name() == "CO" {
            let n = words.len().max(wasm.words.len());
            if let Some(i) = (0..n).find(|&i| words.get(i) != wasm.words.get(i)) {
                ok = false;
                lines.push(format!(
                    "h2: CO's wasm32 dump differs from the committed vector at word {i} ({}): {} committed, {} on \
                     wasm32",
                    own.field(i).unwrap_or_else(|| "end".to_owned()),
                    words.get(i).map_or("none".to_owned(), |w| format!("{w:08x}")),
                    wasm.words.get(i).map_or("none".to_owned(), |w| format!("{w:08x}"))
                ));
            }
            if wasm.hash != Some(digest) {
                ok = false;
                lines.push(format!(
                    "h2: CO's wasm32 hash is not the committed digest {digest:016x}"
                ));
            }
        }
    }
    lines.push(if ok {
        format!(
            "h2: CO's wasm32 dump is the committed vector ({} words, digest {digest:016x}); every scene's initial dump \
             and hash are equal natively and on wasm32 ({})",
            words.len(),
            sizes.join(", ")
        )
    } else {
        "h2: the vectors differ (the lines above)".to_owned()
    });
    Ok((lines, ok))
}

#[cfg(feature = "bites")]
fn bites() -> Vec<String> {
    npt_sim::World::BITES
        .iter()
        .enumerate()
        .map(|(i, id)| format!("{} {id}", i + 1))
        .collect()
}

#[cfg(not(feature = "bites"))]
fn bites() -> Vec<String> {
    Vec::new()
}

/// Runs `npt-lab hash` with the arguments after `hash`: its lines and whether everything compared was equal.
pub(crate) fn run(argv: &[String]) -> Result<(Vec<String>, bool), Failure> {
    let mode = parse(argv).map_err(Failure::Usage)?;
    if std::env::var_os(LEG).is_some() {
        serve(&mode)?;
        return Ok((Vec::new(), true));
    }
    match &mode {
        Mode::Legs { .. } => legs(argv, &mode),
        Mode::Compare { .. } => compare(&mode),
        Mode::Vectors { .. } => vectors(&mode),
        Mode::Bites => Ok((bites(), true)),
    }
}

#[cfg(test)]
mod tests {
    //! h5, the manifest, and h6, the subcommand's arguments (u4d2-brief.md 2.6).

    use super::*;

    fn argv(s: &str) -> Vec<String> {
        s.split_whitespace().map(str::to_owned).collect()
    }

    /// A short entry, generated in the test: TC, 60 ticks across the wrap (sim/tests/replay.rs:514-515).
    fn short() -> (Entry, Vec<u8>) {
        let bytes = Recording::synthetic(&Scene::targets(), 8, u32::MAX - 20, 60, 2_000).encode();
        let e = Entry {
            name: "TC-s8".to_owned(),
            scene: Scene::targets().id(),
            seed: 8,
            first_tick: u32::MAX - 20,
            ticks: 60,
            span_ms: 2_000,
            generator: Recording::GENERATOR,
            bytes: bytes.len() as u64,
            sha256: sha256_hex(&bytes),
        };
        (e, bytes)
    }

    #[test]
    fn h5_a_regenerated_entry_matches_its_recorded_sha256_and_one_changed_byte_fails_naming_it() {
        let (e, bytes) = short();
        assert_eq!(check(&e, &bytes), Ok(()));
        let mut kept = Vec::new();
        let lines = regenerate_all(std::slice::from_ref(&e), |e, b| {
            kept.push((e.name.clone(), b.to_vec()));
            Ok(())
        })
        .unwrap();
        assert_eq!(kept, [("TC-s8".to_owned(), bytes.clone())]);
        assert_eq!(lines.len(), 1);
        let mut changed = bytes.clone();
        changed[1_000] ^= 1;
        let err = check(&e, &changed).unwrap_err();
        assert!(
            err.starts_with("TC-s8: regenerated 5421 bytes with sha256 "),
            "{err}"
        );
        // The manifest's own digest changed: the regenerated bytes no longer match it, and nothing is kept.
        let mut wrong = e.clone();
        wrong.sha256 = sha256_hex(&changed);
        let mut none = 0;
        let err = regenerate_all(&[wrong], |_, _| {
            none += 1;
            Ok(())
        })
        .unwrap_err();
        assert!(
            err.contains("TC-s8") && err.contains("the manifest has"),
            "{err}"
        );
        assert_eq!(none, 0);
    }

    #[test]
    fn h5_a_generator_other_than_recording_generator_is_refused_before_anything_is_generated() {
        let (good, _) = short();
        let mut old = good.clone();
        old.name = "TC-old".to_owned();
        old.generator = Recording::GENERATOR - 1;
        let mut kept = 0;
        let err = regenerate_all(&[good, old], |_, _| {
            kept += 1;
            Ok(())
        })
        .unwrap_err();
        assert!(
            err.starts_with("TC-old: generator 1, not this build's Recording::GENERATOR 2"),
            "{err}"
        );
        assert_eq!(kept, 0, "the good entry before it was not generated either");
    }

    #[test]
    fn h5_the_committed_manifest_reads_and_names_three_generator_2_recordings() {
        let text = include_str!("../recordings.json");
        let entries = manifest(text).unwrap();
        let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["SS-s1", "ES-s2", "TC-s3"]);
        assert!(
            entries
                .iter()
                .all(|e| e.generator == Recording::GENERATOR && e.ticks == 9_000)
        );
        assert!(manifest("{\"recordings\": [{\"name\": \"x\"}]}").is_err());
        assert!(manifest("[]").is_err());
    }

    #[test]
    fn h6_usage_errors_are_refused() {
        let base = "--manifest m --dir d --out o";
        assert!(parse(&argv(base)).is_ok());
        for bad in [
            "--manifest m --dir d",
            "--manifest m --dir d --out o --colour red",
            "--manifest m --dir d --out o --dir e",
            "--manifest m --dir d --out o --dump-at x",
            "--manifest m --dir d --out o --dump-at -1",
            "--manifest m --dir d --out o --arm 5",
            "--manifest m --dir d --out o --on SS-s1",
            "compare --ours a --manifest m --dir d",
            "vectors --dir d --dumps e",
            "--manifest",
            "stray",
        ] {
            assert!(parse(&argv(bad)).is_err(), "{bad}");
        }
        assert!(parse(&argv("compare --ours a --theirs b --manifest m --dir d")).is_ok());
        assert!(parse(&argv("vectors --dir d --dumps e --committed c")).is_ok());
        assert_eq!(
            parse_hash_line("h SS-s1 4294967295 00000000000000ff"),
            Some(TickHash {
                recording: "SS-s1".to_owned(),
                tick: u32::MAX,
                hash: 255
            })
        );
        assert_eq!(parse_hash_line("h SS-s1 1 ff extra"), None);
    }

    #[cfg(not(feature = "bites"))]
    #[test]
    fn h6_a_bite_is_refused_in_the_measured_build() {
        for bad in [
            "--manifest m --dir d --out o --bite PM-0004-21/hash-map --arm 1234",
            "compare --ours a --theirs b --manifest m --dir d --bite PM-0004-21/hash-map --arm 1",
            "bites",
        ] {
            let err = parse(&argv(bad)).unwrap_err();
            assert!(err.contains("measured build"), "{bad}: {err}");
        }
        assert!(matches!(run(&argv("bites")), Err(Failure::Usage(_))));
    }

    #[cfg(feature = "bites")]
    #[test]
    fn h6_the_bites_build_takes_a_bite_and_lists_world_bites_in_order() {
        let m = parse(&argv(
            "--manifest m --dir d --out o --bite PM-0004-21/hash-map --arm 1234 --on SS-s1",
        ))
        .unwrap();
        assert!(matches!(
            m,
            Mode::Legs {
                arm: Some(Arm { tick: 1_234, .. }),
                ..
            }
        ));
        let listed = bites();
        assert_eq!(listed.len(), npt_sim::World::BITES.len());
        assert_eq!(listed[0], "1 PM-0004-21/hash-map");
        assert_eq!(listed[1], "2 PM-0004-21/std-sin");
        assert!(matches!(
            replayer(Some(&Arm {
                id: "PM-0004-21/nothing".to_owned(),
                tick: 0,
                on: None
            })),
            Err(Failure::Usage(_))
        ));
        // One default for `--on` (EA-9): `compare`, like the legs, arms the bite on the first recording the lists hold
        // (bitten_name), or on `--on`'s, and on no other (arm_for).
        let (first, _) = short();
        let mut second = first.clone();
        second.name = "TC-s9".to_owned();
        let arm = Arm {
            id: "PM-0004-21/hash-map".to_owned(),
            tick: 1_234,
            on: None,
        };
        let on = bitten_name(Some(&arm), &[&first, &second]).unwrap();
        assert_eq!(on.as_deref(), Some("TC-s8"));
        assert_eq!(arm_for(Some(&arm), on.as_deref(), &first), Some(&arm));
        assert_eq!(arm_for(Some(&arm), on.as_deref(), &second), None);
        let named = Arm {
            on: Some("TC-s9".to_owned()),
            ..arm.clone()
        };
        let on = bitten_name(Some(&named), &[&first, &second]).unwrap();
        assert_eq!(arm_for(Some(&named), on.as_deref(), &first), None);
        assert_eq!(arm_for(Some(&named), on.as_deref(), &second), Some(&named));
        assert!(
            bitten_name(
                Some(&Arm {
                    on: Some("XX".to_owned()),
                    ..arm
                }),
                &[&first]
            )
            .is_err()
        );
    }

    #[test]
    fn a_digest_field_of_a_dump_of_another_length_says_the_layouts_differ() {
        // RR-L3: the field's indexes are this build's own dump's, so a dump of another layout (P6's word-size plant
        // made the Wasm dump 2 words longer) would be read at other fields' words; no digest is read from it.
        let own: Vec<u32> = (0..10).collect();
        let at = [6, 7];
        assert_eq!(
            digest_text(own.len(), &at, &own),
            format!("{:016x}", (7u64 << 32) | 6)
        );
        let longer: Vec<u32> = (0..12).collect();
        assert_eq!(
            digest_text(own.len(), &at, &longer),
            "layouts differ (10 and 12 words)"
        );
        assert_eq!(
            digest_text(own.len(), &at, &own[..8]),
            "layouts differ (10 and 8 words)"
        );
        assert_eq!(digest_text(own.len(), &[], &own), "none");
        assert_eq!(digest_text(own.len(), &[2], &own), "2");
    }

    #[test]
    fn a_dump_file_reads_back_and_two_word_dumps_are_named_by_this_builds_dump() {
        let own = npt_sim::World::new(&Scene::course(), 1, 0).dump();
        let text = dump_text("CO", 7, own.hash(), own.words());
        let back = parse_dump(&text, "t").unwrap();
        assert_eq!(
            (back.tick, back.hash, back.words.as_slice()),
            (Some(7), Some(own.hash()), own.words())
        );
        let mut other = own.words().to_vec();
        other[3] ^= 1;
        let d = words_difference(&own, own.words(), &other).unwrap();
        assert_eq!((d.word, d.field.as_str()), (3, "soldier[0].alive"));
        assert_eq!(words_difference(&own, own.words(), own.words()), None);
        assert_eq!(
            own.first_difference(&other).map(|d| d.field),
            Some("soldier[0].alive".to_owned())
        );
    }
}
