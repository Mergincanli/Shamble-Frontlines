//! npt-log: the results log of the netcode proof test (plan E20, §6.1).
//!
//! One file per process and run, in JSON Lines: a header first, then per-tick, per-second and event records in time
//! order, and a footer last, so a file without its footer is known to be cut short. `marks/README.md` §1 is the
//! format's contract with npt-judge. This is not a shipped log: it is written apart from the tracing output, so the
//! canary grep of PM-0006-21 never reads it (security F18).
//!
//! The writer never runs on the tick thread. The tick thread hands fixed-size measurements to a writer thread, and
//! that thread builds these records, whose maps allocate. npt-log reads no clock: times, dates and tick numbers come
//! from the caller.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, BufRead, Write};

/// The format version every header carries. A reader refuses any other.
pub const SCHEMA: u32 = 1;

/// The machine a log was written on (plan §6.1; PM-0001-1, PM-0006-11). Committed results redact `address` for every
/// machine but the run's VMs (plan §10.6).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Machine {
    /// `RS`, `LG`, `MV`, `C`, `ML`, `MP` or `MAC`.
    pub role: String,
    /// The instance: `RS`, `LG1`, `LG2`, ...
    pub name: String,
    pub plan: String,
    pub arch: String,
    pub cpu: String,
    pub vcpus: u32,
    pub mem_mb: u64,
    pub location: String,
    pub kernel: String,
    /// A VM's address on the day; empty on every other machine.
    pub address: String,
    pub threads_per_core: u32,
    /// The SMT sibling lists, as `lscpu` reports them.
    pub smt: Vec<Vec<u32>>,
    pub clock_res_ns: u64,
}

/// The build that wrote the log (plan §4.7: each run's header records the bite, the features and the binary's hash).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Build {
    pub commit: String,
    pub binary_sha256: String,
    pub features: Vec<String>,
    pub profile: String,
}

/// The first record of every log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Header {
    /// Always [`SCHEMA`].
    pub schema: u32,
    /// The run's id. A run is every log whose header has the same `run`.
    pub run: String,
    /// The run set judged together.
    pub set: String,
    /// The run's part in its set: `measured`, `bite`, `clean`, `bracket-first`, `bracket-last`, or a role a mark
    /// names.
    pub role: String,
    /// The marks this run serves.
    pub marks: Vec<String>,
    pub machine: Machine,
    pub build: Build,
    /// The planted defect's id in a bites build; `None` in every measured build.
    pub bite: Option<String>,
    /// `L1`, `L2` or `real`.
    pub layer: String,
    /// `P0` to `P4`, `B1` or `real-path`.
    pub profile: String,
    pub seed: u64,
    pub tick_hz: u32,
    pub clients: u32,
    /// `SS`, `ES`, `TC`, `CO`, ...
    pub scene: String,
    /// `room`, `bots`, `load`, `lab`, `browser`, ...
    pub process: String,
    pub slot: u32,
    /// The run's date, `YYYY-MM-DD`, from the caller.
    pub date: String,
    /// Every other setting a mark reads from the header.
    #[serde(default)]
    pub params: BTreeMap<String, serde_json::Value>,
}

/// The phase a tick or second belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    Warmup,
    Measured,
    After,
}

/// One simulated tick's measurements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TickRecord {
    /// The tick number, never wrapping in the log (the wire's 16-bit counters wrap; this does not).
    pub t: u64,
    pub ph: Phase,
    /// One integer per name.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub v: BTreeMap<String, i64>,
    /// A list of integers per name: one per client, per shot, per pair, ...
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub s: BTreeMap<String, Vec<i64>>,
    /// The tick's state hash, 16 lowercase hex digits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h: Option<String>,
    /// A list of hashes per name, one entry per client slot (index = slot): 16 lowercase hex digits, or `null` where
    /// this log has no hash for that slot at this tick (`marks/README.md` §1). A 64-bit hash does not fit JSON's safe
    /// integers, so it is a string, as `h` is.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub hs: BTreeMap<String, Vec<Option<String>>>,
}

/// True for a hash in the log's form: 16 lowercase hex digits.
fn hash_form(h: &str) -> bool {
    h.len() == 16
        && h.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

impl TickRecord {
    pub fn new(t: u64, ph: Phase) -> Self {
        Self {
            t,
            ph,
            v: BTreeMap::new(),
            s: BTreeMap::new(),
            h: None,
            hs: BTreeMap::new(),
        }
    }

    /// Stores the state hash as 16 lowercase hex digits.
    pub fn set_hash(&mut self, hash: u64) {
        self.h = Some(format!("{hash:016x}"));
    }

    /// The state hash, if the record has one in the right form.
    pub fn hash(&self) -> Option<u64> {
        let h = self.h.as_deref()?;
        if !hash_form(h) {
            return None;
        }
        u64::from_str_radix(h, 16).ok()
    }

    /// Stores one hash per client slot under `name`, each as 16 lowercase hex digits, `None` as `null`.
    pub fn set_hashes(&mut self, name: &str, hashes: &[Option<u64>]) {
        let list = hashes
            .iter()
            .map(|h| h.map(|h| format!("{h:016x}")))
            .collect();
        self.hs.insert(name.to_owned(), list);
    }

    /// The hashes under `name`, one per client slot (`None` for `null`); `None` if the record has no such list or an
    /// entry is not in the right form.
    pub fn hashes(&self, name: &str) -> Option<Vec<Option<u64>>> {
        self.hs
            .get(name)?
            .iter()
            .map(|h| match h.as_deref() {
                None => Some(None),
                Some(h) if hash_form(h) => u64::from_str_radix(h, 16).ok().map(Some),
                Some(_) => None,
            })
            .collect()
    }
}

/// Every `hs` list's entries are `null` or a hash in the log's form.
fn check_hs(r: &TickRecord) -> Result<(), String> {
    for (name, list) in &r.hs {
        if let Some(bad) = list.iter().flatten().find(|h| !hash_form(h)) {
            return Err(format!(
                "tick {}: hs.{name} holds {bad:?}, not 16 lowercase hex digits or null",
                r.t
            ));
        }
    }
    Ok(())
}

/// One second's measurements, per process.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecondRecord {
    /// Seconds since the log's start.
    pub sec: u64,
    pub ph: Phase,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub v: BTreeMap<String, i64>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub s: BTreeMap<String, Vec<i64>>,
}

impl SecondRecord {
    pub fn new(sec: u64, ph: Phase) -> Self {
        Self {
            sec,
            ph,
            v: BTreeMap::new(),
            s: BTreeMap::new(),
        }
    }
}

/// Anything that happens once: a join, a control, an affinity read, a person's verdict, ...
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventRecord {
    pub ev: String,
    /// The phase the event happened in, so the judge can tell which events fall in a measured window.
    pub ph: Phase,
    /// The tick it happened on, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<u64>,
    /// Monotonic nanoseconds since the log's start, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ns: Option<u64>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub f: BTreeMap<String, serde_json::Value>,
}

impl EventRecord {
    pub fn new(ev: &str, ph: Phase) -> Self {
        Self {
            ev: ev.to_owned(),
            ph,
            t: None,
            ns: None,
            f: BTreeMap::new(),
        }
    }

    /// Adds a field (builder style).
    pub fn with(mut self, name: &str, value: impl Into<serde_json::Value>) -> Self {
        self.f.insert(name.to_owned(), value.into());
        self
    }
}

/// How the process ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Ended {
    Clean,
    Crash,
    Stopped,
}

/// The last record of every complete log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Footer {
    /// The count of records before the footer, the header included.
    pub records: u64,
    pub ended: Ended,
}

/// A record as one line of the file, tagged by `k`.
#[derive(Serialize)]
#[serde(tag = "k", rename_all = "snake_case")]
enum Line<'a> {
    Header(&'a Header),
    Tick(&'a TickRecord),
    Second(&'a SecondRecord),
    Event(&'a EventRecord),
    Footer(&'a Footer),
}

/// True for a value name of `marks/README.md` §1: lowercase ASCII letters, digits and underscores, starting with a
/// letter.
fn valid_name(name: &str) -> bool {
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b'a'..=b'z'))
        && bytes.all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}

fn invalid(msg: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, msg)
}

fn check_names<'a>(names: impl Iterator<Item = &'a String>) -> io::Result<()> {
    for name in names {
        if !valid_name(name) {
            return Err(invalid(format!(
                "value name {name:?} is not lowercase letters, digits and underscores"
            )));
        }
    }
    Ok(())
}

fn check_header(h: &Header) -> Result<(), String> {
    if h.schema != SCHEMA {
        return Err(format!(
            "header schema {} (this build reads {SCHEMA})",
            h.schema
        ));
    }
    for (field, value) in [
        ("run", &h.run),
        ("set", &h.set),
        ("role", &h.role),
        ("process", &h.process),
    ] {
        if value.is_empty() {
            return Err(format!("header {field} is empty"));
        }
    }
    let d = h.date.as_bytes();
    let date_ok = d.len() == 10
        && d[4] == b'-'
        && d[7] == b'-'
        && d.iter()
            .enumerate()
            .all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit());
    if !date_ok {
        return Err(format!("header date {:?} is not YYYY-MM-DD", h.date));
    }
    Ok(())
}

/// Writes one log: the header when it is made, then records in time order, then the footer when it is finished.
/// It enforces the line discipline: one record per line, ticks strictly increasing, seconds never going back, valid
/// value names, nothing after the footer.
pub struct Writer<W: Write> {
    out: W,
    records: u64,
    last_tick: Option<u64>,
    last_sec: Option<u64>,
}

impl<W: Write> Writer<W> {
    /// Starts a log by writing its header.
    pub fn new(out: W, header: &Header) -> io::Result<Self> {
        check_header(header).map_err(invalid)?;
        check_names(header.params.keys())?;
        let mut w = Self {
            out,
            records: 0,
            last_tick: None,
            last_sec: None,
        };
        w.line(&Line::Header(header))?;
        Ok(w)
    }

    pub fn tick(&mut self, r: &TickRecord) -> io::Result<()> {
        if self.last_tick.is_some_and(|last| r.t <= last) {
            return Err(invalid(format!(
                "tick {} after tick {}",
                r.t,
                self.last_tick.unwrap_or(0)
            )));
        }
        check_names(r.v.keys().chain(r.s.keys()).chain(r.hs.keys()))?;
        check_hs(r).map_err(invalid)?;
        self.line(&Line::Tick(r))?;
        self.last_tick = Some(r.t);
        Ok(())
    }

    pub fn second(&mut self, r: &SecondRecord) -> io::Result<()> {
        if self.last_sec.is_some_and(|last| r.sec < last) {
            return Err(invalid(format!(
                "second {} after second {}",
                r.sec,
                self.last_sec.unwrap_or(0)
            )));
        }
        check_names(r.v.keys().chain(r.s.keys()))?;
        self.line(&Line::Second(r))?;
        self.last_sec = Some(r.sec);
        Ok(())
    }

    pub fn event(&mut self, r: &EventRecord) -> io::Result<()> {
        if !valid_name(&r.ev) {
            return Err(invalid(format!(
                "event name {:?} is not lowercase letters, digits and underscores",
                r.ev
            )));
        }
        check_names(r.f.keys())?;
        self.line(&Line::Event(r))
    }

    /// Writes the footer, flushes, and gives back the output. Nothing can be written after it.
    pub fn finish(mut self, ended: Ended) -> io::Result<W> {
        let footer = Footer {
            records: self.records,
            ended,
        };
        self.line(&Line::Footer(&footer))?;
        self.out.flush()?;
        Ok(self.out)
    }

    /// The count of records written so far, the header included.
    pub fn records(&self) -> u64 {
        self.records
    }

    fn line(&mut self, line: &Line<'_>) -> io::Result<()> {
        // serde_json escapes control characters inside strings, so a record never spans two lines.
        serde_json::to_writer(&mut self.out, line).map_err(io::Error::other)?;
        self.out.write_all(b"\n")?;
        self.records += 1;
        Ok(())
    }
}

/// One log, as read back.
#[derive(Debug, Clone, PartialEq)]
pub struct Log {
    pub header: Header,
    pub ticks: Vec<TickRecord>,
    pub seconds: Vec<SecondRecord>,
    pub events: Vec<EventRecord>,
    /// `None` when the file was cut short.
    pub footer: Option<Footer>,
}

/// Why a log could not be read, with its 1-based line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadError {
    pub line: u64,
    pub msg: String,
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.msg)
    }
}

impl std::error::Error for ReadError {}

/// Reads one log and checks its discipline: the header first and only once, known record kinds and fields only,
/// ticks strictly increasing, seconds never going back, the footer last with the right record count. A file with no
/// footer reads as cut short (`footer` is `None`); the judge's `complete` rule decides what that means.
pub fn read_log<R: BufRead>(input: R) -> Result<Log, ReadError> {
    let mut header: Option<Header> = None;
    let mut log_ticks = Vec::new();
    let mut log_seconds = Vec::new();
    let mut log_events = Vec::new();
    let mut footer: Option<Footer> = None;
    let mut line_no: u64 = 0;
    let mut last_tick: Option<u64> = None;
    let mut last_sec: Option<u64> = None;
    for line in input.lines() {
        line_no += 1;
        let err = |msg: String| ReadError { line: line_no, msg };
        let text = line.map_err(|e| err(format!("cannot read: {e}")))?;
        if footer.is_some() {
            return Err(err("a record after the footer".to_owned()));
        }
        let mut value: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| err(format!("not one JSON record: {e}")))?;
        let kind = match value.as_object_mut().and_then(|o| o.remove("k")) {
            Some(serde_json::Value::String(k)) => k,
            _ => return Err(err("no record kind \"k\"".to_owned())),
        };
        if header.is_none() && kind != "header" {
            return Err(err(format!("the first record is {kind:?}, not the header")));
        }
        match kind.as_str() {
            "header" => {
                if header.is_some() {
                    return Err(err("a second header".to_owned()));
                }
                let h: Header =
                    serde_json::from_value(value).map_err(|e| err(format!("header: {e}")))?;
                check_header(&h).map_err(err)?;
                header = Some(h);
            }
            "tick" => {
                let r: TickRecord =
                    serde_json::from_value(value).map_err(|e| err(format!("tick: {e}")))?;
                check_hs(&r).map_err(err)?;
                if last_tick.is_some_and(|last| r.t <= last) {
                    return Err(err(format!(
                        "tick {} after tick {}",
                        r.t,
                        last_tick.unwrap_or(0)
                    )));
                }
                last_tick = Some(r.t);
                log_ticks.push(r);
            }
            "second" => {
                let r: SecondRecord =
                    serde_json::from_value(value).map_err(|e| err(format!("second: {e}")))?;
                if last_sec.is_some_and(|last| r.sec < last) {
                    return Err(err(format!(
                        "second {} after second {}",
                        r.sec,
                        last_sec.unwrap_or(0)
                    )));
                }
                last_sec = Some(r.sec);
                log_seconds.push(r);
            }
            "event" => {
                let r: EventRecord =
                    serde_json::from_value(value).map_err(|e| err(format!("event: {e}")))?;
                log_events.push(r);
            }
            "footer" => {
                let f: Footer =
                    serde_json::from_value(value).map_err(|e| err(format!("footer: {e}")))?;
                if f.records != line_no - 1 {
                    return Err(err(format!(
                        "the footer counts {} records; {} came before it",
                        f.records,
                        line_no - 1
                    )));
                }
                footer = Some(f);
            }
            other => return Err(err(format!("unknown record kind {other:?}"))),
        }
    }
    let header = header.ok_or(ReadError {
        line: 0,
        msg: "empty log: no header".to_owned(),
    })?;
    Ok(Log {
        header,
        ticks: log_ticks,
        seconds: log_seconds,
        events: log_events,
        footer,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header() -> Header {
        Header {
            schema: SCHEMA,
            run: "R0-test-1".to_owned(),
            set: "R0-test".to_owned(),
            role: "measured".to_owned(),
            marks: vec!["PM-0001-4".to_owned()],
            machine: Machine {
                role: "C".to_owned(),
                name: "C".to_owned(),
                plan: "container".to_owned(),
                arch: "x86_64".to_owned(),
                cpu: "test".to_owned(),
                vcpus: 4,
                mem_mb: 8192,
                location: "here".to_owned(),
                kernel: "6".to_owned(),
                address: String::new(),
                threads_per_core: 1,
                smt: vec![vec![0], vec![1]],
                clock_res_ns: 1,
            },
            build: Build {
                commit: "0000000".to_owned(),
                binary_sha256: "00".repeat(32),
                features: vec![],
                profile: "release".to_owned(),
            },
            bite: None,
            layer: "L1".to_owned(),
            profile: "P2".to_owned(),
            seed: 7,
            tick_hz: 30,
            clients: 48,
            scene: "SS".to_owned(),
            process: "lab".to_owned(),
            slot: 0,
            date: "2026-11-06".to_owned(),
            params: BTreeMap::from([("sim".to_owned(), serde_json::json!("stress"))]),
        }
    }

    fn sample_log() -> Vec<u8> {
        let mut w = Writer::new(Vec::new(), &header()).expect("header");
        for t in 1..=3 {
            let mut r = TickRecord::new(
                t,
                if t == 1 {
                    Phase::Warmup
                } else {
                    Phase::Measured
                },
            );
            r.v.insert("work_ns".to_owned(), 9_000_000 + t as i64);
            r.s.insert("encode_ns".to_owned(), vec![40_000, 41_000]);
            r.set_hash(0x0123_4567_89ab_cdef ^ t);
            w.tick(&r).expect("tick");
        }
        let mut s = SecondRecord::new(0, Phase::Measured);
        s.v.insert("rss_bytes".to_owned(), 50 << 20);
        w.second(&s).expect("second");
        let e = EventRecord::new("admit", Phase::Measured)
            .with("session", 3)
            .with("in_roster", true)
            .with("note", "two\nlines");
        w.event(&e).expect("event");
        w.finish(Ended::Clean).expect("footer")
    }

    #[test]
    fn round_trip_keeps_every_record() {
        let bytes = sample_log();
        let log = read_log(bytes.as_slice()).expect("reads back");
        assert_eq!(log.header, header());
        assert_eq!(log.ticks.len(), 3);
        assert_eq!(log.ticks[2].v["work_ns"], 9_000_003);
        assert_eq!(log.ticks[0].ph, Phase::Warmup);
        assert_eq!(log.seconds[0].v["rss_bytes"], 50 << 20);
        assert_eq!(log.events[0].f["note"], serde_json::json!("two\nlines"));
        assert_eq!(
            log.footer,
            Some(Footer {
                records: 6,
                ended: Ended::Clean
            })
        );
    }

    #[test]
    fn one_record_per_line_kind_first() {
        let bytes = sample_log();
        let text = String::from_utf8(bytes).expect("utf-8");
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 7, "header, 3 ticks, 1 second, 1 event, footer");
        for line in &lines {
            assert!(line.starts_with("{\"k\":\""), "kind first: {line}");
        }
        assert!(lines[0].starts_with("{\"k\":\"header\""));
        assert!(lines[6].starts_with("{\"k\":\"footer\""));
    }

    #[test]
    fn hash_is_sixteen_hex_digits() {
        let mut r = TickRecord::new(1, Phase::Measured);
        r.set_hash(0xab);
        assert_eq!(r.h.as_deref(), Some("00000000000000ab"));
        assert_eq!(r.hash(), Some(0xab));
        r.h = Some("00000000000000AB".to_owned());
        assert_eq!(r.hash(), None, "uppercase is not the format");
        r.h = Some("ab".to_owned());
        assert_eq!(r.hash(), None, "short is not the format");
    }

    /// `hs` (marks/README.md §1, the advisor's nit c): one entry per client slot, a client with no hash at the tick
    /// written as `null`, and nothing else accepted on either side.
    #[test]
    fn hs_holds_one_hash_or_null_per_slot() {
        let mut r = TickRecord::new(4, Phase::Measured);
        r.set_hashes("decoded_hash", &[Some(0xab), None, Some(0)]);
        assert_eq!(
            r.hs["decoded_hash"],
            vec![
                Some("00000000000000ab".to_owned()),
                None,
                Some("0000000000000000".to_owned())
            ]
        );
        assert_eq!(
            r.hashes("decoded_hash"),
            Some(vec![Some(0xab), None, Some(0)])
        );
        assert_eq!(r.hashes("server_hash"), None, "no such list");
        let mut w = Writer::new(Vec::new(), &header()).expect("header");
        w.tick(&r).expect("tick");
        w.tick(&TickRecord::new(5, Phase::Measured))
            .expect("a tick with no hs");
        let text = String::from_utf8(w.finish(Ended::Clean).expect("footer")).expect("utf-8");
        let lines: Vec<&str> = text.lines().collect();
        assert!(
            lines[1]
                .contains(r#""hs":{"decoded_hash":["00000000000000ab",null,"0000000000000000"]}"#),
            "{}",
            lines[1]
        );
        assert!(!lines[2].contains("hs"), "an empty hs is not written");
        let log = read_log(text.as_bytes()).expect("reads back");
        assert_eq!(log.ticks[0], r);
        assert!(log.ticks[1].hs.is_empty());
    }

    #[test]
    fn hs_refuses_a_bad_entry_or_name_when_writing_and_reading() {
        let mut w = Writer::new(Vec::new(), &header()).expect("header");
        for (t, name, entry) in [
            (1, "decoded_hash", "00000000000000AB"),
            (2, "decoded_hash", "ab"),
            (3, "decoded_hash", "0x000000000000ab"),
        ] {
            let mut r = TickRecord::new(t, Phase::Measured);
            r.hs.insert(name.to_owned(), vec![Some(entry.to_owned())]);
            let e = w.tick(&r).expect_err(entry);
            assert!(e.to_string().contains("16 lowercase hex digits"), "{e}");
            assert_eq!(r.hashes(name), None, "{entry} is not in the form");
        }
        let mut bad_name = TickRecord::new(4, Phase::Measured);
        bad_name.set_hashes("Decoded-Hash", &[None]);
        assert!(w.tick(&bad_name).is_err(), "a bad list name");
        assert_eq!(w.records(), 1, "only the header was written");
        let text = String::from_utf8(sample_log()).expect("utf-8");
        let header_line = text.lines().next().expect("header");
        for entry in [r#""ABCDEF0123456789""#, r#""1""#, "1"] {
            let line = format!(
                "{header_line}\n{{\"k\":\"tick\",\"t\":1,\"ph\":\"measured\",\"hs\":{{\"decoded_hash\":[{entry}]}}}}\n"
            );
            assert!(read_log(line.as_bytes()).is_err(), "{entry} read back");
        }
    }

    #[test]
    fn the_header_comes_first_and_once() {
        let text = String::from_utf8(sample_log()).expect("utf-8");
        let mut lines: Vec<&str> = text.lines().collect();
        let header_line = lines.remove(0);
        let no_header = lines.join("\n");
        let e = read_log(no_header.as_bytes()).expect_err("must fail");
        assert_eq!(e.line, 1);
        assert!(e.msg.contains("not the header"), "{e}");
        let twice = format!("{header_line}\n{header_line}\n");
        let e = read_log(twice.as_bytes()).expect_err("must fail");
        assert_eq!(e.line, 2);
        assert!(e.msg.contains("second header"), "{e}");
        assert!(read_log("".as_bytes()).is_err(), "an empty file is no log");
    }

    #[test]
    fn unknown_kinds_and_fields_are_refused() {
        let text = String::from_utf8(sample_log()).expect("utf-8");
        let header_line = text.lines().next().expect("header");
        let unknown_kind = format!("{header_line}\n{{\"k\":\"frame\",\"t\":1}}\n");
        assert!(
            read_log(unknown_kind.as_bytes())
                .expect_err("kind")
                .msg
                .contains("unknown record kind")
        );
        let unknown_field =
            format!("{header_line}\n{{\"k\":\"tick\",\"t\":1,\"ph\":\"measured\",\"x\":1}}\n");
        assert!(
            read_log(unknown_field.as_bytes())
                .expect_err("field")
                .msg
                .contains("unknown field")
        );
        let float_value = format!(
            "{header_line}\n{{\"k\":\"tick\",\"t\":1,\"ph\":\"measured\",\"v\":{{\"a\":1.5}}}}\n"
        );
        assert!(
            read_log(float_value.as_bytes()).is_err(),
            "values are integers"
        );
    }

    #[test]
    fn the_footer_is_last_and_counts_right() {
        let text = String::from_utf8(sample_log()).expect("utf-8");
        let after = format!("{text}{{\"k\":\"event\",\"ev\":\"late\",\"ph\":\"after\"}}\n");
        assert!(
            read_log(after.as_bytes())
                .expect_err("after")
                .msg
                .contains("after the footer")
        );
        let lines: Vec<&str> = text.lines().collect();
        let mut short = lines[..lines.len() - 1].to_vec();
        short.remove(2);
        short.push("{\"k\":\"footer\",\"records\":6,\"ended\":\"clean\"}");
        let short = short.join("\n");
        assert!(
            read_log(short.as_bytes())
                .expect_err("count")
                .msg
                .contains("came before it")
        );
    }

    #[test]
    fn a_cut_log_reads_with_no_footer() {
        let text = String::from_utf8(sample_log()).expect("utf-8");
        let lines: Vec<&str> = text.lines().collect();
        let cut = lines[..4].join("\n");
        let log = read_log(cut.as_bytes()).expect("a cut log still reads");
        assert_eq!(log.footer, None);
        assert_eq!(log.ticks.len(), 3);
    }

    #[test]
    fn order_and_names_are_enforced_when_writing() {
        let mut w = Writer::new(Vec::new(), &header()).expect("header");
        w.tick(&TickRecord::new(5, Phase::Measured))
            .expect("tick 5");
        assert!(
            w.tick(&TickRecord::new(5, Phase::Measured)).is_err(),
            "tick 5 twice"
        );
        assert!(
            w.tick(&TickRecord::new(4, Phase::Measured)).is_err(),
            "tick 4 after 5"
        );
        let mut bad = TickRecord::new(6, Phase::Measured);
        bad.v.insert("Work-ns".to_owned(), 1);
        assert!(w.tick(&bad).is_err(), "a bad value name");
        w.second(&SecondRecord::new(2, Phase::Measured))
            .expect("second 2");
        w.second(&SecondRecord::new(2, Phase::Measured))
            .expect("second 2 again: several processes share a second");
        assert!(
            w.second(&SecondRecord::new(1, Phase::Measured)).is_err(),
            "second 1 after 2"
        );
        assert!(
            w.event(&EventRecord::new("Bad Name", Phase::Measured))
                .is_err(),
            "a bad event name"
        );
        assert_eq!(
            w.records(),
            4,
            "header, tick 5 and two seconds; the refused records were not written"
        );
        let mut wrong = header();
        wrong.schema = 2;
        assert!(Writer::new(Vec::new(), &wrong).is_err(), "another schema");
        let mut wrong = header();
        wrong.date = "6 Nov 2026".to_owned();
        assert!(
            Writer::new(Vec::new(), &wrong).is_err(),
            "a date not in YYYY-MM-DD"
        );
    }

    #[test]
    fn order_is_enforced_when_reading() {
        let text = String::from_utf8(sample_log()).expect("utf-8");
        let header_line = text.lines().next().expect("header");
        let back = format!(
            "{header_line}\n{{\"k\":\"tick\",\"t\":2,\"ph\":\"measured\"}}\n{{\"k\":\"tick\",\"t\":2,\"ph\":\"measured\"}}\n"
        );
        let e = read_log(back.as_bytes()).expect_err("must fail");
        assert_eq!(e.line, 3);
    }
}
