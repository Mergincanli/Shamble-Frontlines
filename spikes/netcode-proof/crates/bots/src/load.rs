//! npt-load and its glue (u6c-brief.md Block 4, its C1 (c); NC-SB-2, NC-SB-7; F-JOIN-ONCE): LG's load, as U8's
//! npt-load-ctl starts it, `npt-load <settings.json>`, mode `run`, `calibrate` or `probe`.
//!
//! - **The glue** ([`Hosted`]): one honest [`Behaviour`] and its endpoint over a `Box<dyn Link + Send>` (WtClient or
//!   WsClient, the room's certificate pinned by `pin_hex`), driven through `Behaviour::host_step` alone (C-U6-18), one
//!   tokio task a client. F-JOIN-ONCE: the endpoint sends its `Join` once, at its first step, and a Connecting link
//!   answers `BufferFull`, so a client steps only once its link reads Open, at once (the 2 s Join deadline runs from
//!   the server's accept), never while Connecting; a link Closed first (a failed handshake) is counted, never stepped.
//!   No rejoin: a client whose link closes stays out.
//! - **The load**: the settings (SEC-U6AD-2's rules), the token file read under a cap (one token a slot, never
//!   logged; not read in probe mode), the log at `log_path` (process `load`, LG's machine and the room's header fields:
//!   NC-SB-7), `READY` once every client holds its `JoinAccepted` (else exit 1 after 30 s), each second
//!   `second.s.inputs_count` (one value a playing client, in slot order) and `second.v.cpu_permille` (the machine's,
//!   from /proc/stat); the end at `duration_s` or when every client's link has closed (the server's close), the footer
//!   written either way, then `DONE` and exit 0. `calibrate` adds one `CALIBRATION` line (lg-80's and V-S's inputs: the
//!   judge's rule decides, never this); `probe` runs npt-net's `probe_peer` to the room's UDP port at 10,000 a second for
//!   the duration and adds one `SUMMARY` line and a `probe_peer` event. Exit 2 for bad settings, tokens, pin or an
//!   existing log; 1 for any other failure. TERM ends it at once (its default action: no signal crate), the log cut at
//!   a line end (each record written through).

use std::fs::{File, OpenOptions};
use std::io::{self, LineWriter, Read, Write};
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering::Relaxed};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use npt_client::ClientEndpoint;
use npt_link::{Link, LinkState};
use npt_log::{Build, Ended, EventRecord, Header, Machine, Phase, SecondRecord, Writer};
use serde::Deserialize;

use crate::Behaviour;

const SEC: u64 = 1_000_000_000;
/// READY's deadline from the start.
const READY_NS: u64 = 30 * SEC;
/// A handshake's bound before the glue counts it failed.
const HANDSHAKE_NS: u64 = 10 * SEC;
/// The settings file's cap, bytes; the token file's (64 slots of about 30 B).
pub const SETTINGS_CAP: u64 = 64 * 1024;
const TOKENS_CAP: u64 = 8 * 1024;
const TEXT_MAX: usize = 256;
/// The controller's lead bound, npt-client's one constant for every honest client (the lab's too: C3).
const MAX_LEAD: u32 = npt_client::HONEST_MAX_LEAD;
/// RS's TLS name on the WebSocket leg (the pin decides; the name is SNI only).
const RS_NAME: &str = "rs.nt.shamblefrontlines.com";
/// The honest behaviours npt-load hosts (C-U6-19's gate line: no throwing, rejoining or restarted client).
const BEHAVIOURS: [&str; 5] = ["brawl", "steady", "presses", "clamp", "never_fires"];
/// The probe's rate each way (ADR 0001:1590).
const PROBE_RATE_PPS: u64 = 10_000;

#[allow(
    clippy::disallowed_types,
    reason = "npt-load's one clock item (bots/clippy.toml asks for U6's reviewed allow): the monotonic clock's zero"
)]
static ORIGIN: OnceLock<std::time::Instant> = OnceLock::new();

/// npt-load's one clock item (bots/clippy.toml: U6's reviewed allow): monotonic ns since its first read. The glue's
/// links stamp on it; every other item takes its time as an argument.
#[allow(
    clippy::disallowed_types,
    clippy::disallowed_methods,
    reason = "npt-load's one clock item (bots/clippy.toml asks for U6's reviewed allow): the hosts' and the load's time"
)]
pub fn now_ns() -> u64 {
    let origin = ORIGIN.get_or_init(std::time::Instant::now);
    u64::try_from(origin.elapsed().as_nanos()).unwrap_or(u64::MAX)
}

/// What one hosted client has done, read by the load each second.
#[derive(Debug)]
pub struct HostState {
    given: AtomicU64,
    /// u32::MAX until `JoinAccepted`.
    slot: AtomicU32,
    /// 0 connecting, 1 open, 2 failed before Open, 3 closed.
    state: AtomicU32,
}

impl HostState {
    /// The commands given so far.
    pub fn given(&self) -> u64 {
        self.given.load(Relaxed)
    }
    /// The slot `JoinAccepted` named.
    pub fn slot(&self) -> Option<u8> {
        u8::try_from(self.slot.load(Relaxed)).ok()
    }
    /// Whether the client is out: its handshake failed or its link closed.
    pub fn done(&self) -> bool {
        self.state.load(Relaxed) >= 2
    }
    /// Whether its handshake failed (the link Closed or still Connecting at its bound).
    pub fn failed(&self) -> bool {
        self.state.load(Relaxed) == 2
    }
}

/// One client hosted over its link (the module text).
pub struct Hosted {
    behaviour: Behaviour,
    endpoint: ClientEndpoint,
    link: Box<dyn Link + Send>,
    state: Arc<HostState>,
}

impl Hosted {
    pub fn new(
        behaviour: Behaviour,
        endpoint: ClientEndpoint,
        link: Box<dyn Link + Send>,
    ) -> Hosted {
        let state = Arc::new(HostState {
            given: AtomicU64::new(0),
            slot: AtomicU32::new(u32::MAX),
            state: AtomicU32::new(0),
        });
        Hosted {
            behaviour,
            endpoint,
            link,
            state,
        }
    }

    pub fn state(&self) -> Arc<HostState> {
        Arc::clone(&self.state)
    }

    /// The host task (F-JOIN-ONCE, the module text) until `end_ns` on `clock`: the link polled (its state moves only
    /// through a call) until Open, then at once and at each due time `host_step` with what arrived, each message with
    /// its `arrival_ns`, and what the step made sent; waits by `tokio::time::sleep` for the time left.
    pub async fn run(mut self, end_ns: u64, clock: fn() -> u64) {
        let mut received = Vec::new();
        let bound = clock().saturating_add(HANDSHAKE_NS).min(end_ns);
        loop {
            let now = clock();
            while let Some(r) = self.link.poll_recv(now) {
                received.push((r.channel, r.bytes, r.arrival_ns));
            }
            match self.link.state() {
                LinkState::Open => break,
                LinkState::Connecting if now < bound => {
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
                _ => return self.state.state.store(2, Relaxed),
            }
        }
        self.state.state.store(1, Relaxed);
        loop {
            let now = clock();
            while let Some(r) = self.link.poll_recv(now) {
                received.push((r.channel, r.bytes, r.arrival_ns));
            }
            if self.link.state() == LinkState::Closed {
                return self.state.state.store(3, Relaxed);
            }
            let (given, out) = self
                .behaviour
                .host_step(&mut self.endpoint, &received, now, &[]);
            received.clear();
            self.state.given.fetch_add(u64::from(given), Relaxed);
            if let Some(s) = self.endpoint.slot() {
                self.state.slot.store(u32::from(s), Relaxed);
            }
            for d in &out.datagrams {
                self.link.send_unreliable(d, now);
            }
            for (c, b) in &out.reliable {
                self.link.send_reliable(*c, b, now);
            }
            if now >= end_ns {
                return;
            }
            let wait = out.next_due_ns.saturating_sub(clock()).min(end_ns - now);
            tokio::time::sleep(Duration::from_nanos(wait)).await;
        }
    }
}

/// LG's machine (C1 (c)).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LgMachine {
    role: String,
    name: String,
    plan: String,
    location: String,
    address: String,
}

/// npt-load's settings (C1 (c): U8's load_settings, wip/u8-build.patch:2180-2186). No `Debug` (the token path is not a
/// secret, but nothing here is printed whole); `deny_unknown_fields` with no `flatten` (SEC-U6AD-2).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoadSettings {
    room: String,
    server: String,
    wt_port: u16,
    ws_port: u16,
    mode: String,
    transport: String,
    clients: u32,
    duration_s: u64,
    token_path: String,
    log_path: String,
    run: String,
    set: String,
    marks: Vec<String>,
    date: String,
    commit: String,
    role: String,
    layer: String,
    profile: String,
    scene: String,
    tick_hz: u32,
    machine: LgMachine,
    behaviour: String,
    seed: u64,
    pin_hex: String,
}

impl LoadSettings {
    /// The settings at `path`, read through `take(SETTINGS_CAP + 1)` and refused above it, then [`LoadSettings::parse`].
    pub fn read(path: &str) -> Result<LoadSettings, String> {
        LoadSettings::parse(&capped(path, SETTINGS_CAP).map_err(|e| format!("the settings: {e}"))?)
    }

    /// Parses and range-checks one settings text (SEC-U6AD-2's rules).
    pub fn parse(text: &[u8]) -> Result<LoadSettings, String> {
        let s: LoadSettings =
            serde_json::from_slice(text).map_err(|e| format!("the settings: {e}"))?;
        let m = &s.machine;
        let texts = [
            &s.room,
            &s.server,
            &s.mode,
            &s.transport,
            &s.token_path,
            &s.log_path,
            &s.run,
            &s.set,
            &s.date,
            &s.commit,
            &s.role,
            &s.layer,
            &s.profile,
            &s.scene,
            &m.role,
            &m.name,
            &m.plan,
            &m.location,
            &m.address,
            &s.behaviour,
            &s.pin_hex,
        ];
        let bad_text = |t: &String| t.len() > TEXT_MAX || t.chars().any(char::is_control);
        let checks = [
            (
                texts.into_iter().chain(&s.marks).any(bad_text) || s.marks.len() > 64,
                "a text above 256 B or with a control character, or above 64 marks",
            ),
            (
                s.room.len() != 2
                    || !s.room.bytes().all(|b| b.is_ascii_digit())
                    || !s.room.parse::<u8>().is_ok_and(|n| n <= 31),
                "room is 00 to 31",
            ),
            (s.server.parse::<Ipv4Addr>().is_err(), "server is RS's IPv4"),
            (s.wt_port == 0 || s.ws_port == 0, "the ports are not 0"),
            (
                !["run", "calibrate", "probe"].contains(&s.mode.as_str()),
                "mode is run, calibrate or probe",
            ),
            (
                !["wt", "ws"].contains(&s.transport.as_str()),
                "transport is wt or ws",
            ),
            (!(1..=64).contains(&s.clients), "clients is 1 to 64"),
            (
                !(1..=86_400).contains(&s.duration_s),
                "duration_s is 1 to 86,400",
            ),
            (
                s.token_path.is_empty() || s.log_path.is_empty() || s.token_path == s.log_path,
                "token_path and log_path are two paths",
            ),
            (!hex(&s.commit, 40), "commit is 40 lowercase hex digits"),
            (!hex(&s.pin_hex, 64), "pin_hex is 64 lowercase hex digits"),
            (s.tick_hz != npt_sim::TICK_HZ, "tick_hz is this build's"),
            (scene(&s.scene).is_none(), "scene is SS, ES, CO or TC"),
            (
                !BEHAVIOURS.contains(&s.behaviour.as_str()),
                "behaviour is brawl, steady, presses, clamp or never_fires",
            ),
        ];
        match checks.iter().find(|c| c.0) {
            Some((_, why)) => Err((*why).to_owned()),
            None => Ok(s),
        }
    }
}

fn hex(t: &str, n: usize) -> bool {
    t.len() == n
        && t.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn scene(name: &str) -> Option<npt_sim::Scene> {
    Some(match name {
        "SS" => npt_sim::Scene::stress(),
        "ES" => npt_sim::Scene::estimate(),
        "CO" => npt_sim::Scene::course(),
        "TC" => npt_sim::Scene::targets(),
        _ => return None,
    })
}

/// The file at `path` through `take(cap + 1)`, refused above `cap`.
fn capped(path: &str, cap: u64) -> io::Result<Vec<u8>> {
    let mut v = Vec::new();
    File::open(path)?.take(cap + 1).read_to_end(&mut v)?;
    if v.len() as u64 > cap {
        return Err(io::Error::other(format!("above {cap} bytes")));
    }
    Ok(v)
}

/// 32 bytes from 64 lowercase hex digits (checked by `parse`).
fn pin(hex: &str) -> [u8; 32] {
    let mut p = [0u8; 32];
    for (i, b) in p.iter_mut().enumerate() {
        *b = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap_or(0);
    }
    p
}

/// RFC 4648 §5 base64url without padding, decoded; `None` for another character or a length that is no whole bytes.
pub(crate) fn base64url(t: &str) -> Option<Vec<u8>> {
    let (mut acc, mut bits, mut out) = (0u32, 0u32, Vec::new());
    for c in t.bytes() {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'-' => 62,
            b'_' => 63,
            _ => return None,
        };
        acc = (acc << 6) | u32::from(v);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    (bits < 6 && acc == 0).then_some(out)
}

/// The token file's text (U6a's form: `<slot> nptj_<22 base64url>` a line, main.rs:351-365): the tokens of slots 0 to
/// `n` − 1. An error never holds a token's text (SEC-U6AD-3).
pub(crate) fn tokens(text: &[u8], n: usize) -> Result<Vec<[u8; 16]>, String> {
    let text = std::str::from_utf8(text).map_err(|_| "the token file is not UTF-8")?;
    let mut out: Vec<Option<[u8; 16]>> = vec![None; n];
    for (i, line) in text.lines().enumerate() {
        let bad = || {
            format!(
                "the token file's line {} is not `<slot> nptj_<22 base64url>`",
                i + 1
            )
        };
        let (slot, tok) = line.split_once(' ').ok_or_else(bad)?;
        let slot: usize = slot.parse().map_err(|_| bad())?;
        let raw = tok
            .strip_prefix("nptj_")
            .filter(|t| t.len() == 22)
            .and_then(base64url)
            .ok_or_else(bad)?;
        if let Some(o) = out.get_mut(slot) {
            *o = Some(raw.try_into().map_err(|_| bad())?);
        }
    }
    out.into_iter()
        .enumerate()
        .map(|(i, t)| t.ok_or(format!("the token file has no slot {i}")))
        .collect()
}

/// The machine's busy and total ticks in a /proc/stat text: its `cpu` line, idle and iowait the idle part.
pub(crate) fn cpu_ticks(stat: &str) -> Option<(u64, u64)> {
    let line = stat
        .lines()
        .find(|l| l.split_whitespace().next() == Some("cpu"))?;
    let v: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .take(8)
        .map(|x| x.parse().ok())
        .collect::<Option<_>>()?;
    (v.len() == 8).then(|| (v.iter().sum::<u64>() - v[3] - v[4], v.iter().sum()))
}

/// The machine's CPU between two `cpu_ticks` reads, permille (`second.v.cpu_permille`).
pub(crate) fn cpu_permille(a: (u64, u64), b: (u64, u64)) -> Option<u64> {
    let total = b.1.checked_sub(a.1).filter(|t| *t > 0)?;
    Some(b.0.saturating_sub(a.0) * 1_000 / total)
}

/// `second.s.inputs_count`: one value a playing client (one holding a slot), in slot order (SERIES.md:12): (slot,
/// commands given this second) in any order.
pub(crate) fn inputs_record(mut by_slot: Vec<(u8, u64)>) -> Vec<i64> {
    by_slot.sort_unstable_by_key(|x| x.0);
    by_slot
        .into_iter()
        .map(|x| i64::try_from(x.1).unwrap_or(i64::MAX))
        .collect()
}

/// `calibrate`'s one line over its measured seconds' (cpu permille, inputs a client): the largest CPU, the seconds at
/// or over 800 permille, and the smallest and largest inputs a client a second (lg-80's and V-S's inputs: the judge's
/// rule decides, never this).
pub(crate) fn calibration(seconds: &[(Option<u64>, Vec<i64>)]) -> String {
    let cpu = seconds.iter().filter_map(|s| s.0);
    let inputs = seconds.iter().flat_map(|s| s.1.iter().copied());
    format!(
        "CALIBRATION cpu_max_permille={} cpu_at_800_count={} inputs_min_count={} inputs_max_count={}",
        cpu.clone().max().unwrap_or(0),
        cpu.filter(|&c| c >= 800).count(),
        inputs.clone().min().unwrap_or(0),
        inputs.max().unwrap_or(0)
    )
}

/// LG's log header (NC-SB-7): the room's role, layer, profile, scene and tick rate beside its run, set, marks, date,
/// commit and seed, all from the settings; LG's machine (`Machine::read_local` with the settings' role and names); process
/// `load`. npt-bots has no hashing crate, so `binary_sha256` is empty (U8 records LG's binary).
pub(crate) fn header(s: &LoadSettings, clock_res_ns: u64) -> Header {
    let m = &s.machine;
    let mut params = std::collections::BTreeMap::new();
    for (k, v) in [
        ("room", &s.room),
        ("mode", &s.mode),
        ("transport", &s.transport),
        ("behaviour", &s.behaviour),
    ] {
        params.insert(k.to_owned(), v.clone().into());
    }
    Header {
        schema: npt_log::SCHEMA,
        run: s.run.clone(),
        set: s.set.clone(),
        role: s.role.clone(),
        marks: s.marks.clone(),
        machine: Machine::read_local(
            &m.role,
            &m.name,
            &m.plan,
            &m.location,
            &m.address,
            clock_res_ns,
        ),
        build: Build {
            commit: s.commit.clone(),
            binary_sha256: String::new(),
            features: Vec::new(),
            profile: if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            }
            .to_owned(),
        },
        bite: None,
        layer: s.layer.clone(),
        profile: s.profile.clone(),
        seed: s.seed,
        tick_hz: s.tick_hz,
        clients: s.clients,
        scene: s.scene.clone(),
        process: "load".to_owned(),
        slot: 0,
        date: s.date.clone(),
        params,
    }
}

/// The clock's smallest step over 1,000 steps (the header's `machine.clock_res_ns`; 1 when none is seen).
fn clock_res() -> u64 {
    let (mut least, mut last, mut seen) = (u64::MAX, now_ns(), 0);
    for _ in 0..10_000_000 {
        let t = now_ns();
        if t != last {
            least = least.min(t - last);
            (last, seen) = (t, seen + 1);
            if seen >= 1_000 {
                break;
            }
        }
    }
    least.clamp(1, 1_000_000)
}

/// The run (the module text); `out` is stdout. Returns the exit code.
pub fn run_load(s: &LoadSettings, out: &mut dyn Write) -> u8 {
    let start = now_ns();
    let end = start + s.duration_s * SEC;
    let n = s.clients as usize;
    let toks = if s.mode == "probe" {
        Vec::new()
    } else {
        match capped(&s.token_path, TOKENS_CAP)
            .map_err(|e| format!("the token file: {e}"))
            .and_then(|t| tokens(&t, n))
        {
            Ok(t) => t,
            Err(e) => return fail(2, &e),
        }
    };
    let file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&s.log_path)
    {
        Ok(f) => f,
        Err(e) => {
            return fail(
                if e.kind() == io::ErrorKind::AlreadyExists {
                    2
                } else {
                    1
                },
                &format!("the log: {e}"),
            );
        }
    };
    let mut w = match Writer::new(LineWriter::new(file), &header(s, clock_res())) {
        Ok(w) => w,
        Err(e) => return fail(2, &format!("the log's header: {e}")),
    };
    let rt = match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => return fail(1, &format!("the runtime: {e}")),
    };
    let server: Ipv4Addr = s.server.parse().unwrap_or(Ipv4Addr::UNSPECIFIED);
    let mut states = Vec::new();
    let mut peer = None;
    {
        let _g = rt.enter();
        if s.mode == "probe" {
            match npt_net::probe_peer(
                SocketAddr::from((server, s.wt_port)),
                PROBE_RATE_PPS,
                s.duration_s * SEC,
            ) {
                Ok(p) => peer = Some(p),
                Err(e) => {
                    // NC-U6C-6 (NC-SB-2): a footer on every end after the header.
                    let _ = w.finish(Ended::Stopped);
                    return fail(1, &format!("the probe peer: {e}"));
                }
            }
        }
        let sc = scene(&s.scene).unwrap_or_else(npt_sim::Scene::stress);
        for (slot, token) in toks.iter().enumerate() {
            let join = npt_proto::Join {
                token: *token,
                bind: bind(s.seed, slot as u8),
                version: npt_proto::PROTOCOL_VERSION,
                content_hash: npt_proto::CONTENT_HASH,
            };
            let link: io::Result<Box<dyn Link + Send>> = if s.transport == "wt" {
                npt_net::WtClient::connect(
                    SocketAddr::from((server, s.wt_port)),
                    pin(&s.pin_hex),
                    now_ns,
                )
                .map(|c| Box::new(c) as Box<dyn Link + Send>)
            } else {
                npt_net::WsClient::connect(
                    SocketAddr::from((server, s.ws_port)),
                    RS_NAME,
                    pin(&s.pin_hex),
                    now_ns,
                )
                .map(|c| Box::new(c) as Box<dyn Link + Send>)
            };
            let link = match link {
                Ok(l) => l,
                Err(e) => {
                    let _ = w.finish(Ended::Stopped);
                    return fail(1, &format!("client {slot}: {e}"));
                }
            };
            let endpoint = ClientEndpoint::new(join, s.tick_hz).with_clock(MAX_LEAD);
            let h = Hosted::new(
                behaviour(&s.behaviour, &sc, slot as u8, s.seed),
                endpoint,
                link,
            );
            states.push(h.state());
            rt.spawn(h.run(end, now_ns));
        }
    }
    let (ended, cal) = match seconds(s, &mut w, out, &states, start, end) {
        Ok(x) => x,
        Err(e) => return fail(1, &format!("the log: {e}")),
    };
    if let Some(p) = peer {
        let counts = p.stop();
        let ev = counts.iter().fold(
            EventRecord::new("probe_peer", Phase::After),
            |e, &(k, v)| e.with(k, v),
        );
        let _ = w.event(&ev);
        let line: Vec<String> = counts.iter().map(|(k, v)| format!("{k}={v}")).collect();
        let _ = writeln!(out, "SUMMARY {}", line.join(" "));
    }
    rt.shutdown_timeout(Duration::from_secs(2));
    if let Err(e) = w.finish(ended) {
        return fail(1, &format!("the log's footer: {e}"));
    }
    if ended != Ended::Clean {
        // NC-U6C-5: how many handshakes failed (F-JOIN-ONCE (2)), in the failing line.
        let failed = states.iter().filter(|h| h.failed()).count();
        return fail(
            1,
            &format!(
                "no READY within 30 s: not every client holds its JoinAccepted \
                 (handshake_failed_count={failed})"
            ),
        );
    }
    if s.mode == "calibrate" {
        let _ = writeln!(out, "{}", calibration(&cal));
    }
    let _ = writeln!(out, "DONE");
    let _ = out.flush();
    0
}

/// The measured seconds' (cpu permille, inputs a client).
type Measured = Vec<(Option<u64>, Vec<i64>)>;

/// The per-second loop: READY once every client holds its slot (no READY in probe mode, which has no client), each
/// second's record, the end at `end` or the server's close. Returns how it ended and the measured seconds.
fn seconds(
    s: &LoadSettings,
    w: &mut Writer<LineWriter<File>>,
    out: &mut dyn Write,
    states: &[Arc<HostState>],
    start: u64,
    end: u64,
) -> io::Result<(Ended, Measured)> {
    let stat = || std::fs::read_to_string("/proc/stat").unwrap_or_default();
    let (mut cpu, mut given) = (cpu_ticks(&stat()), vec![0u64; states.len()]);
    let (mut ready, mut k, mut cal) = (states.is_empty(), 0u64, Vec::new());
    loop {
        std::thread::sleep(Duration::from_millis(10));
        let now = now_ns();
        if !ready && states.iter().all(|h| h.slot().is_some()) {
            writeln!(out, "READY")?;
            out.flush()?;
            ready = true;
        }
        if !ready && now >= start + READY_NS {
            return Ok((Ended::Stopped, cal));
        }
        let over = now >= end || (!states.is_empty() && states.iter().all(|h| h.done()));
        if now < start + (k + 1) * SEC && !over {
            continue;
        }
        let ph = if ready {
            Phase::Measured
        } else {
            Phase::Warmup
        };
        let mut r = SecondRecord::new(k, ph);
        let c = cpu_ticks(&stat());
        let permille = cpu.zip(c).and_then(|(a, b)| cpu_permille(a, b));
        cpu = c;
        if let Some(p) = permille {
            r.v.insert("cpu_permille".into(), i64::try_from(p).unwrap_or(i64::MAX));
        }
        if s.mode != "probe" {
            let by_slot = states.iter().zip(&mut given).filter_map(|(h, g)| {
                let now = h.given();
                let d = now - std::mem::replace(g, now);
                h.slot().map(|s| (s, d))
            });
            let inputs = inputs_record(by_slot.collect());
            if ready {
                cal.push((permille, inputs.clone()));
            }
            r.s.insert("inputs_count".into(), inputs);
        }
        w.second(&r)?;
        k += 1;
        if over {
            return Ok((Ended::Clean, cal));
        }
    }
}

/// A client's bind value: from the run's seed and its slot (the room keeps a slot's first; not a secret).
fn bind(seed: u64, slot: u8) -> [u8; 16] {
    let mut b = [0u8; 16];
    b[..8].copy_from_slice(&seed.to_le_bytes());
    b[8] = slot;
    b[15] = 0x6c;
    b
}

/// The behaviour named in the settings (BEHAVIOURS), honest, of `slot`; the brawl under the controller.
fn behaviour(name: &str, scene: &npt_sim::Scene, slot: u8, seed: u64) -> Behaviour {
    let b = match name {
        "never_fires" => Behaviour::never_fires(scene, slot, seed),
        "brawl" => Behaviour::brawl(scene, slot, seed),
        other => Behaviour::script(scene, slot, seed, other)
            .unwrap_or_else(|| Behaviour::brawl(scene, slot, seed)),
    };
    b.controlled()
}

fn fail(code: u8, why: &str) -> u8 {
    eprintln!("npt-load: {why}");
    code
}

#[cfg(test)]
#[path = "load_tests.rs"]
mod tests;
