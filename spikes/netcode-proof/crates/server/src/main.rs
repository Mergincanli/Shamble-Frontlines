//! npt-server's binary: the room process (u6a-brief.md Block 3, S8a; plan E2; U6-Q17 term (2): the binary's settings,
//! lifecycle, panic catch and non-zero exit are U6a's, the WebTransport certificate U6b's).
//!
//! `npt-server --settings <file>`: the settings from one JSON file (at most [`SETTINGS_CAP`] bytes, read through `take`
//! and refused above it before parsing; unknown fields refused; every value range-checked before it sizes anything;
//! the tick rate 30 only, C-U6-16 (b) being moved), then:
//! 1. one join token per slot, 16 bytes each from /dev/urandom (roster.rs: "minting is U6's"), into a new 0600 file
//!    for the run script, one line per slot (the slot, then `nptj_` and the token in base64url), never logged;
//!    U6b (Block 6): the WebTransport certificate made (cert.rs: ECDSA P-256, 13 days, the key in memory only), its
//!    hash file written new, both listeners bound (`wt_bind` UDP through ImpairSocket, `ws_bind` TCP) under one
//!    connection cap of N + 8;
//! 2. the room's log, its header the settings' fields with this machine and this build (the binary's SHA-256), and
//!    U6b's params (the transport, `cc`, the crate, the cap, the certificate's validity, DelayStream's and
//!    ImpairSocket's numbers);
//! 3. the room built and TickLoop running, pinned when the settings name vCPUs; both listeners served on its I/O
//!    runtime; then one `OPEN` line on stdout (both ports accept at once from there);
//! 4. the end: the settings' duration, or stdin closing (no tokio `signal` feature: SEC-U6AD-1), or the tick thread
//!    ending after a panic; each WebSocket connection's `ws_summary` as it ends; then the sessions end, their links'
//!    closings are awaited (each within 2 s of its drop), the last `ws_summary` rows, `ws_split` and one event per
//!    transport, and the log's end (`TickLoop::stop_with`, NR-U6B-1), on the main thread (NR-U6A-7).
//!
//! Exit 0 after a clean end, 2 for bad settings (or a token, log or hash file that already exists, an I/O vCPU on an
//! SMT sibling of the tick vCPU on this machine's map: Q-U6A-5, refused before step 1, or a clock outside 1970 to
//! 9999), 3 after a crash, 1 for any other failure.

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::os::unix::fs::OpenOptionsExt;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use npt_log::{Build, Ended, EventRecord, Header, Machine, Phase, Writer};
use npt_net::{ConnCap, DelayStream, L2Config, Listen, WsServer, WtServer};
use npt_room::CmdTiming;
use npt_server::{LoopSettings, RttSource, TickLoop};

mod cert;
use npt_sim::Scene;
use serde::Deserialize;
use sha2::{Digest, Sha256};

/// The settings file's size limit, bytes.
const SETTINGS_CAP: u64 = 64 * 1024;
/// Exit codes (the module text).
const EXIT_FAILED: u8 = 1;
const EXIT_SETTINGS: u8 = 2;
const EXIT_CRASH: u8 = 3;
/// A text setting's length limit, bytes.
const TEXT_MAX: usize = 256;
/// The longest run the duration may ask for: 24 hours, ms.
const DURATION_MAX_MS: u64 = 24 * 3_600_000;

/// PM-0001-16/double-load (marks.json: "a start that loads its mode file twice over a 1 s delay"; bites builds only):
/// the settings file read twice more, each read behind 1 s, so spawn to Open passes 2 s.
#[cfg(feature = "bites")]
const DOUBLE_LOAD_BITE: &str = "PM-0001-16/double-load";
/// PM-0006-11/no-smt-map (marks.json: "a header without the SMT map"; bites builds only): the header's machine
/// written with no `smt`, so `machine_named` is 0.
#[cfg(feature = "bites")]
const NO_SMT_MAP_BITE: &str = "PM-0006-11/no-smt-map";
/// PM-0001-1/shared-vcpu (bites builds only): planted by the library's TickLoop, which the binary hands the id.
#[cfg(feature = "bites")]
const SHARED_VCPU_BITE: &str = "PM-0001-1/shared-vcpu";

/// The room's settings (the module text). No `Debug`, as the token types have none (SEC-U6AD-3); `deny_unknown_fields`
/// with no `flatten`, `untagged` or `serde_json::Value` field (SEC-U6AD-2).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Settings {
    run: String,
    set: String,
    role: String,
    marks: Vec<String>,
    /// The run's date, `YYYY-MM-DD`, and the build's 40-digit commit.
    date: String,
    commit: String,
    layer: String,
    profile: String,
    seed: u64,
    /// `SS`, `ES`, `CO` or `TC`.
    scene: String,
    /// Slots in the roster: 1 to the scene's slots.
    clients: u32,
    /// OQ-0001-6's option: `A` or `B`.
    cmd_timing: String,
    /// The tick rate: 30 only.
    tick_hz: u32,
    machine_role: String,
    machine_name: String,
    machine_plan: String,
    machine_location: String,
    machine_address: String,
    /// The tick thread's vCPU and the I/O runtime's, when the run pins them (U8's run script sets the slices).
    tick_vcpu: Option<u32>,
    io_vcpus: Vec<u32>,
    /// The I/O runtime's threads: 1 to 64.
    io_threads: u32,
    /// The room log's path and the token file's: each made new, never overwritten.
    log: String,
    token_file: String,
    /// The run's length, ms (0: until stdin closes), and the ticks logged as warm-up.
    duration_ms: u64,
    warmup_ticks: u64,
    /// The CPU model the run's first start read (the run script fills it; empty at the first start).
    first_cpu_model: String,
    /// U6b: what the run's clients use, `wt`, `ws` or `both` (`params.transport`; both listeners always bind).
    transport: String,
    /// The WebTransport (UDP) and WebSocket (TCP) bind addresses (plan.md:909-910: ports 30001 + s and 31001 + s).
    wt_bind: String,
    ws_bind: String,
    /// The WebSocket certificate's chain and key, PEM files (U6c's npt-cert), each read under [`PEM_CAP`]; both empty:
    /// the room's own certificate (runs with no browser: replan.md:69-71).
    ws_cert_chain: String,
    ws_cert_key: String,
    /// The hash file's path (made new: an existing one is refused).
    hash_path: String,
    /// The split's window mark, seconds after OPEN (absent or null: no mark, the process's whole life).
    ws_split_mark_s: Option<u64>,
    /// quinn's congestion controller, `newreno` or `cubic` (absent: `cubic`, quinn's default; E6).
    #[serde(default)]
    cc: Option<String>,
    /// The planted defect (bites builds only: a measured build refuses the field as unknown).
    #[cfg(feature = "bites")]
    #[serde(default)]
    bite: Option<String>,
}

/// What a settings file says, read and checked: the settings and the scene they name, and U6b's transport settings.
struct Checked {
    settings: Settings,
    scene: Scene,
    timing: CmdTiming,
    l2: L2Config,
    wt_addr: SocketAddr,
    ws_addr: SocketAddr,
    cc: String,
}

/// A PEM file's size limit, bytes (the WebSocket certificate's chain and key).
const PEM_CAP: u64 = 64 * 1024;
/// The handshake deadline from the accept stamp, ns: the Join deadline's 2 s (catalogue :93; S5 b).
const DEADLINE_NS: u64 = 2_000_000_000;
/// The end's wait for the links' closings (NR-U6B-1): npt-net's 2 s bound from each drop, plus 1 s for a handshake in
/// flight at the stop (it holds a permit to its own deadline) and the drops themselves (FLUSH, 0.5 s), ns.
const CLOSINGS_WAIT_NS: u64 = 3_000_000_000;
/// The WebSocket gate's tick period from the one tick constant (NR-U6B-2; plan §4.6), ns, rounded up: a truncated one
/// makes 1.5 round trips of a whole number of ticks one tick more (60 Hz at 100 ms: N = 11, not 10).
const GATE_TICK_NS: u64 = 1_000_000_000u64.div_ceil(npt_sim::TICK_HZ as u64);
/// The longest split mark after OPEN, seconds.
const MARK_MAX_S: u64 = 86_400;
/// The WebTransport crate every header names (replan-units.md:84).
const WT_CRATE: &str = "web-transport-quinn 0.13.6";
/// The split pool's rule, recorded in the header (RC-10).
const POOL_RULE: &str = "one measured run per server process: the split's pool (delay.rs's POOL) is never reset; \
     event.ws_split covers the window from the mark (ws_split_mark_after_open_ns after OPEN), else the process's life";

/// Reads the settings file at `path` through `take(SETTINGS_CAP + 1)`, refusing it above the cap before parsing.
fn read_settings(path: &str) -> Result<Checked, String> {
    let mut text = Vec::new();
    File::open(path)
        .and_then(|f| f.take(SETTINGS_CAP + 1).read_to_end(&mut text))
        .map_err(|e| format!("the settings file: {e}"))?;
    if text.len() as u64 > SETTINGS_CAP {
        return Err(format!("the settings file is above {SETTINGS_CAP} bytes"));
    }
    parse_settings(&text)
}

/// Parses and checks one settings text (the module text's rules).
fn parse_settings(text: &[u8]) -> Result<Checked, String> {
    let s: Settings = serde_json::from_slice(text).map_err(|e| format!("the settings: {e}"))?;
    let scene = match s.scene.as_str() {
        "SS" => Scene::stress(),
        "ES" => Scene::estimate(),
        "CO" => Scene::course(),
        "TC" => Scene::targets(),
        other => return Err(format!("scene {other:?} is not SS, ES, CO or TC")),
    };
    let timing = match s.cmd_timing.as_str() {
        "A" => CmdTiming::A,
        "B" => CmdTiming::B,
        other => return Err(format!("cmd_timing {other:?} is not A or B")),
    };
    if s.tick_hz != npt_sim::TICK_HZ {
        return Err(format!(
            "tick_hz {} is not this build's {}",
            s.tick_hz,
            npt_sim::TICK_HZ
        ));
    }
    if s.clients == 0 || s.clients > u32::from(scene.slots()) {
        return Err(format!(
            "clients {} is not 1 to the scene's {} slots",
            s.clients,
            scene.slots()
        ));
    }
    if s.io_threads == 0 || s.io_threads > 64 {
        return Err(format!("io_threads {} is not 1 to 64", s.io_threads));
    }
    if s.tick_vcpu.is_some_and(|c| c >= 1_024)
        || s.io_vcpus.len() > 64
        || s.io_vcpus.iter().any(|&c| c >= 1_024)
        || (s.tick_vcpu.is_some() != !s.io_vcpus.is_empty())
    {
        return Err(
            "tick_vcpu and io_vcpus: both or neither, each vCPU below 1,024, at most 64 I/O vCPUs"
                .to_owned(),
        );
    }
    if s.tick_vcpu.is_some_and(|t| s.io_vcpus.contains(&t)) {
        return Err("the tick vCPU is one of the I/O vCPUs".to_owned());
    }
    if s.duration_ms > DURATION_MAX_MS || s.warmup_ticks > 10_000_000 {
        return Err("duration_ms is at most 24 h and warmup_ticks at most 10,000,000".to_owned());
    }
    let texts = [
        &s.run,
        &s.set,
        &s.role,
        &s.date,
        &s.commit,
        &s.layer,
        &s.profile,
        &s.machine_role,
        &s.machine_name,
        &s.machine_plan,
        &s.machine_location,
        &s.machine_address,
        &s.log,
        &s.token_file,
        &s.first_cpu_model,
        &s.transport,
        &s.wt_bind,
        &s.ws_bind,
        &s.ws_cert_chain,
        &s.ws_cert_key,
        &s.hash_path,
    ];
    if s.marks.len() > 64
        || texts
            .iter()
            .copied()
            .chain(&s.marks)
            .any(|t| t.len() > TEXT_MAX || t.chars().any(char::is_control))
    {
        return Err(format!(
            "a text setting is above {TEXT_MAX} bytes or holds a control character, or above 64 marks"
        ));
    }
    if s.commit.len() != 40
        || !s
            .commit
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("commit is not 40 lowercase hex digits".to_owned());
    }
    if s.log.is_empty() || s.token_file.is_empty() || s.log == s.token_file {
        return Err("log and token_file are two paths".to_owned());
    }
    #[cfg(feature = "bites")]
    if let Some(b) = &s.bite
        && ![DOUBLE_LOAD_BITE, NO_SMT_MAP_BITE, SHARED_VCPU_BITE].contains(&b.as_str())
    {
        return Err(format!("bite {b:?} is not one npt-server's binary plants"));
    }
    let u6b = transport_settings(&s)?;
    Ok(Checked {
        settings: s,
        scene,
        timing,
        l2: u6b.0,
        wt_addr: u6b.1,
        ws_addr: u6b.2,
        cc: u6b.3,
    })
}

/// U6b's settings, range-checked (SEC-U6AD-2's rules): the transport, both bind addresses, the profile's L2
/// configuration, `cc`, the certificate files (both or neither), the hash file's path apart from the others, the mark.
fn transport_settings(s: &Settings) -> Result<(L2Config, SocketAddr, SocketAddr, String), String> {
    if !["wt", "ws", "both"].contains(&s.transport.as_str()) {
        return Err(format!("transport {:?} is not wt, ws or both", s.transport));
    }
    let addr = |name: &str, v: &str| {
        v.parse::<SocketAddr>()
            .map_err(|_| format!("{name} {v:?} is not an address and port"))
    };
    let (wt, ws) = (addr("wt_bind", &s.wt_bind)?, addr("ws_bind", &s.ws_bind)?);
    let l2 = L2Config::named(s.seed, &s.profile)
        .ok_or_else(|| format!("profile {:?} is not one npt-l1 has", s.profile))?;
    let cc = s.cc.clone().unwrap_or_else(|| "cubic".to_owned());
    if !["newreno", "cubic"].contains(&cc.as_str()) {
        return Err(format!("cc {cc:?} is not newreno or cubic"));
    }
    if s.ws_cert_chain.is_empty() != s.ws_cert_key.is_empty() {
        return Err("ws_cert_chain and ws_cert_key: both or neither".to_owned());
    }
    if s.hash_path.is_empty() || [&s.log, &s.token_file].contains(&&s.hash_path) {
        return Err("hash_path is a path apart from log and token_file".to_owned());
    }
    if s.ws_split_mark_s.is_some_and(|m| m == 0 || m > MARK_MAX_S) {
        return Err(format!("ws_split_mark_s is 1 to {MARK_MAX_S}"));
    }
    Ok((l2, wt, ws, cc))
}

/// `n` join tokens, 16 bytes each, from /dev/urandom: a short or failed read is an error, never a fallback
/// (SEC-U6AD-3); `n` was range-checked first. Every token is distinct (128-bit values; checked all the same).
fn mint(n: usize) -> io::Result<Vec<[u8; 16]>> {
    let mut bytes = vec![0u8; n * 16];
    File::open("/dev/urandom")?.read_exact(&mut bytes)?;
    let tokens: Vec<[u8; 16]> = bytes.as_chunks::<16>().0.to_vec();
    for (i, t) in tokens.iter().enumerate() {
        if tokens[..i].contains(t) {
            return Err(io::Error::other("two minted tokens are equal"));
        }
    }
    Ok(tokens)
}

/// `bytes` in base64url without padding (RFC 4648 §5).
fn base64url(bytes: &[u8]) -> String {
    // Each 6-bit value's letter, computed (a 64-letter literal reads as a secret to redact-check's long-token rule).
    let letter = |v: u32| {
        let v = v as u8;
        char::from(match v {
            0..=25 => b'A' + v,
            26..=51 => b'a' + (v - 26),
            52..=61 => b'0' + (v - 52),
            62 => b'-',
            _ => b'_',
        })
    };
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for c in bytes.chunks(3) {
        let n = c
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, &b)| n | (u32::from(b) << (16 - 8 * i)));
        for k in 0..=c.len() {
            out.push(letter((n >> (18 - 6 * k)) & 63));
        }
    }
    out
}

/// Writes the token file: made new with mode 0600 at once (`create_new`, never a chmod after: SEC-U6AD-3); an existing
/// path is refused (`AlreadyExists`).
fn write_tokens(path: &str, tokens: &[[u8; 16]]) -> io::Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    let mut text = String::new();
    for (slot, t) in tokens.iter().enumerate() {
        text.push_str(&format!("{slot} nptj_{}\n", base64url(t)));
    }
    f.write_all(text.as_bytes())?;
    f.sync_all()
}

/// The running binary's SHA-256, by a loop of reads into a fixed buffer (SEC-U6AD-5: the file never read whole).
fn binary_sha256() -> io::Result<String> {
    let mut f = File::open(std::env::current_exe()?)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect())
}

/// The room log's header: the settings' fields, `machine` (this machine with the settings' role, name, plan, location
/// and address, the clock's resolution measured), `build` (the binary's SHA-256), and its params: `cmd_timing`, carry
/// 60's weapon and health from `Scene::weapon()` and `Scene::health()`, and the CPU stand-ins in force (none: U6a
/// burns none, as on L1).
fn header(c: &Checked, binary_sha256: String, machine: Machine) -> Header {
    let s = &c.settings;
    #[allow(
        unused_mut,
        reason = "the no-smt-map bite clears the map in bites builds only"
    )]
    let mut machine = machine;
    #[cfg(feature = "bites")]
    if s.bite.as_deref() == Some(NO_SMT_MAP_BITE) {
        machine.smt.clear();
    }
    let mut features = Vec::new();
    if cfg!(feature = "bites") {
        features.push("bites".to_owned());
    }
    #[cfg(feature = "bites")]
    let bite = s.bite.clone();
    #[cfg(not(feature = "bites"))]
    let bite = None;
    Header {
        schema: npt_log::SCHEMA,
        run: s.run.clone(),
        set: s.set.clone(),
        role: s.role.clone(),
        marks: s.marks.clone(),
        machine,
        build: Build {
            commit: s.commit.clone(),
            binary_sha256,
            features,
            profile: if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            }
            .to_owned(),
        },
        bite,
        layer: s.layer.clone(),
        profile: s.profile.clone(),
        seed: s.seed,
        tick_hz: s.tick_hz,
        clients: s.clients,
        scene: c.scene.name().to_owned(),
        process: "room".to_owned(),
        slot: 0,
        date: s.date.clone(),
        params: params(c),
    }
}

/// This machine with the settings' role, name, plan, location and address, and the clock's resolution measured.
fn local_machine(s: &Settings) -> Machine {
    Machine::read_local(
        &s.machine_role,
        &s.machine_name,
        &s.machine_plan,
        &s.machine_location,
        &s.machine_address,
        TickLoop::clock_resolution_ns(),
    )
}

/// The header's params (the `header` text): `cmd_timing`, carry 60's seven and the stand-ins.
fn params(c: &Checked) -> std::collections::BTreeMap<String, serde_json::Value> {
    let w = c.scene.weapon();
    let mut p = std::collections::BTreeMap::new();
    p.insert(
        "cmd_timing".to_owned(),
        c.settings.cmd_timing.clone().into(),
    );
    p.insert("weapon_interval_us".to_owned(), w.interval_us.into());
    p.insert("weapon_damage_count".to_owned(), w.damage.into());
    p.insert("weapon_reach_cm".to_owned(), w.reach_cm.into());
    p.insert("weapon_magazine_count".to_owned(), w.magazine.into());
    p.insert("weapon_reload_ms".to_owned(), w.reload_ms.into());
    p.insert("weapon_spread_urad".to_owned(), w.spread_urad.into());
    p.insert("soldier_health_count".to_owned(), c.scene.health().into());
    p.insert("stand_ins".to_owned(), serde_json::Value::Array(Vec::new()));
    p.insert("stand_in_burn_ns".to_owned(), 0.into());
    p.insert("stand_in_top_up_ns".to_owned(), 0.into());
    p
}

/// `event.cpu_read`: at the session's start, `changed` 1 when this machine's CPU model differs from the run's first
/// read (the settings' `first_cpu_model`; empty at the first start: 0).
fn cpu_read(first: &str, now: &str) -> EventRecord {
    let changed = !first.is_empty() && first != now;
    EventRecord::new("cpu_read", Phase::Warmup).with("changed", i64::from(changed))
}

/// A failure's exit, with its line on stderr (never a token: no setting or token reaches a message).
fn fail(code: u8, why: &str) -> ExitCode {
    eprintln!("npt-server: {why}");
    ExitCode::from(code)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [flag, path] = args.as_slice() else {
        return fail(EXIT_SETTINGS, "usage: npt-server --settings <file>");
    };
    if flag != "--settings" {
        return fail(EXIT_SETTINGS, "usage: npt-server --settings <file>");
    }
    #[allow(
        unused_mut,
        reason = "the double-load bite reads it again in bites builds only"
    )]
    let mut checked = match read_settings(path) {
        Ok(c) => c,
        Err(e) => return fail(EXIT_SETTINGS, &e),
    };
    #[cfg(feature = "bites")]
    if checked.settings.bite.as_deref() == Some(DOUBLE_LOAD_BITE) {
        for _ in 0..2 {
            std::thread::sleep(Duration::from_secs(1));
            checked = match read_settings(path) {
                Ok(c) => c,
                Err(e) => return fail(EXIT_SETTINGS, &e),
            };
        }
    }
    run(checked)
}

/// Q-U6A-5 (NR-U6A-2): the refusal's line when the settings pin and an I/O vCPU is an SMT sibling of the tick vCPU on
/// `smt` (this machine's map); none otherwise.
fn sibling_refusal(s: &Settings, smt: &[Vec<u32>]) -> Option<String> {
    let t = s.tick_vcpu?;
    TickLoop::io_on_a_tick_sibling(smt, t, &s.io_vcpus).then(|| {
        format!(
            "io_vcpus {:?} hold an SMT sibling of tick_vcpu {t} on this machine",
            s.io_vcpus
        )
    })
}

/// U6b's transports in the room process (u6b-brief.md Block 6): the dev certificate, the hash file, the cap and both
/// listeners, bound before the room's log (the header reads the bound socket) and served on the loop's I/O runtime.
struct Net {
    wt: Arc<WtServer>,
    ws: Arc<WsServer>,
    cap: Arc<ConnCap>,
    /// The split's mark on the room's clock (NR-U6B-6; 0: not taken), for `ws_split_mark_room_ns`.
    mark_room_ns: Arc<std::sync::atomic::AtomicU64>,
}

impl Net {
    /// The certificate (exit 2 for a clock outside 1970 to 9999), the hash file (exit 2 when its path exists), the
    /// cap of N + 8, and both listeners bound (TCP listening; UDP through ImpairSocket, whose threads inherit this
    /// thread's I/O pin). The certificate's key stays in memory.
    fn bind(c: &Checked) -> Result<Net, (u8, String)> {
        let s = &c.settings;
        let today = cert::today().map_err(|e| (EXIT_SETTINGS, e))?;
        cert::validity(today).map_err(|e| (EXIT_SETTINGS, e))?;
        let dev = cert::make(today).map_err(|e| (EXIT_FAILED, e))?;
        if let Err(e) = cert::write_hash(&s.hash_path, &dev.hash) {
            let code = if e.kind() == io::ErrorKind::AlreadyExists {
                EXIT_SETTINGS
            } else {
                EXIT_FAILED
            };
            return Err((code, format!("the hash file: {e}")));
        }
        let cap = ConnCap::new(s.clients as usize);
        let listen = |addr| Listen {
            addr,
            clock: TickLoop::now_ns,
            deadline_ns: DEADLINE_NS,
            cap: Arc::clone(&cap),
            tick_ns: GATE_TICK_NS,
            outside_max: npt_proto::Limits::PING.max(npt_proto::Limits::PONG),
        };
        let chain = [dev.der.clone()];
        let wt = WtServer::bind(listen(c.wt_addr), &c.l2, &chain, &dev.key, &c.cc)
            .map_err(|e| (EXIT_FAILED, format!("the WebTransport listener: {e}")))?;
        let ws = if s.ws_cert_chain.is_empty() {
            WsServer::bind(listen(c.ws_addr), &c.l2, &chain, &dev.key)
        } else {
            let pem = |path: &str| read_capped(path, PEM_CAP);
            let (chain_pem, key_pem) = (pem(&s.ws_cert_chain)?, pem(&s.ws_cert_key)?);
            WsServer::bind_pem(listen(c.ws_addr), &c.l2, &chain_pem, &key_pem)
        }
        .map_err(|e| (EXIT_FAILED, format!("the WebSocket listener: {e}")))?;
        Ok(Net {
            wt: Arc::new(wt),
            ws: Arc::new(ws),
            cap,
            mark_room_ns: Arc::default(),
        })
    }

    /// U6b's header params (Block 5): the transport, `cc`, the crate, the cap, the certificate's validity, the split's
    /// rule and mark, DelayStream's numbers and texts from the one `&L2Config` (`ws_wrapper` 0 on the plain path), and
    /// ImpairSocket's `l2_*` from the bound socket.
    fn params(&self, c: &Checked) -> Vec<(String, serde_json::Value)> {
        let s = &c.settings;
        let mut p: Vec<(String, serde_json::Value)> = vec![
            ("transport".into(), s.transport.clone().into()),
            ("cc".into(), c.cc.clone().into()),
            ("wt_crate".into(), WT_CRATE.into()),
            ("conn_cap_count".into(), self.cap.permits().into()),
            ("cert_validity_ns".into(), cert::VALIDITY_NS.into()),
            ("ws_split_pool_rule".into(), POOL_RULE.into()),
        ];
        if let Some(m) = s.ws_split_mark_s {
            p.push((
                "ws_split_mark_after_open_ns".into(),
                (m * 1_000_000_000).into(),
            ));
        }
        let numbers = DelayStream::header_numbers_from(&c.l2)
            .into_iter()
            .chain(self.wt.socket().header_numbers());
        p.extend(numbers.map(|(k, v)| (k.to_owned(), v.into())));
        let texts = DelayStream::header_texts_from(&c.l2)
            .into_iter()
            .chain(self.wt.socket().header_texts());
        p.extend(texts.map(|(k, v)| (k.to_owned(), v.into())));
        p
    }

    /// Serves both listeners on the loop's I/O runtime, each accepted session handed to the loop with its accept stamp
    /// (a WebTransport session's RTT from its link, a WebSocket one's from the room's pings), and the split's mark
    /// `mark_s` seconds after OPEN (now), taken there. Returns each listener's serving stamp on the room's clock (T-7:
    /// its accept loop started), WebTransport's then WebSocket's.
    fn serve(&self, tl: &TickLoop, mark_s: Option<u64>) -> io::Result<[u64; 2]> {
        let handle = tl.io_handle();
        let _g = handle.enter();
        self.wt.start()?;
        let wt_ns = TickLoop::now_ns();
        self.ws.start()?;
        let serving = [wt_ns, TickLoop::now_ns()];
        let (wt, conn) = (Arc::clone(&self.wt), tl.connector());
        handle.spawn(async move {
            while let Some(a) = wt.accept().await {
                conn.connect(a.link, RttSource::Link, a.accept_ns);
            }
        });
        let (ws, conn) = (Arc::clone(&self.ws), tl.connector());
        handle.spawn(async move {
            while let Some(a) = ws.accept().await {
                conn.connect(a.link, RttSource::Pings, a.accept_ns);
            }
        });
        if let Some(s) = mark_s {
            let at = Arc::clone(&self.mark_room_ns);
            handle.spawn(async move {
                tokio::time::sleep(Duration::from_secs(s)).await;
                let room_ns = TickLoop::now_ns();
                if DelayStream::mark_split() {
                    at.store(room_ns, std::sync::atomic::Ordering::Relaxed);
                }
            });
        }
        Ok(serving)
    }

    /// Each WebSocket connection's `ws_summary` that ended since the last call, to the room's log.
    fn summaries(&self, tl: &TickLoop) {
        for rows in self.ws.take_summaries() {
            tl.event(event("ws_summary", Phase::Measured, &rows));
        }
    }

    /// The run's end (Block 5), from [`TickLoop::stop_with`] once the sessions have ended (NR-U6B-1): every link's
    /// closing awaited (every permit back in the cap; each closing ends within npt-net's 2 s of its link's drop), at
    /// most [`CLOSINGS_WAIT_NS`]; then the connections' last summaries (a cut closing pushes its own too), `ws_split`
    /// once (the wrapper on), and one event per transport.
    fn end(&self, tl: &TickLoop) {
        let until = TickLoop::now_ns() + CLOSINGS_WAIT_NS;
        while self.cap.free() < self.cap.permits() && TickLoop::now_ns() < until {
            std::thread::sleep(Duration::from_millis(1));
        }
        self.summaries(tl);
        if self.ws.wrapper() {
            let mut rows = DelayStream::split_rows();
            let mark = self.mark_room_ns.load(std::sync::atomic::Ordering::Relaxed);
            rows.push(("ws_split_mark_room_ns", mark));
            tl.event(event("ws_split", Phase::Measured, &rows));
        }
        tl.event(event("wt_accept", Phase::After, &self.wt.event_rows()));
        tl.event(event("ws_accept", Phase::After, &self.ws.event_rows()));
    }
}

/// An event of `rows`.
fn event(name: &str, ph: Phase, rows: &[(&'static str, u64)]) -> EventRecord {
    rows.iter()
        .fold(EventRecord::new(name, ph), |e, &(k, v)| e.with(k, v))
}

/// The file at `path` through `take(cap + 1)`, refused above `cap` (exit 2) before it is used.
fn read_capped(path: &str, cap: u64) -> Result<Vec<u8>, (u8, String)> {
    let mut v = Vec::new();
    File::open(path)
        .and_then(|f| f.take(cap + 1).read_to_end(&mut v))
        .map_err(|e| (EXIT_FAILED, format!("a certificate file: {e}")))?;
    if v.len() as u64 > cap {
        return Err((
            EXIT_SETTINGS,
            format!("a certificate file is above {cap} bytes"),
        ));
    }
    Ok(v)
}

/// The start, the run and the end (the module text, steps 1 to 4).
fn run(c: Checked) -> ExitCode {
    let machine = local_machine(&c.settings);
    if let Some(why) = sibling_refusal(&c.settings, &machine.smt) {
        return fail(EXIT_SETTINGS, &why);
    }
    // The listeners' L2 threads, started before the loop, inherit the I/O vCPUs (Q-U6A-5 (a)'s first step).
    if c.settings.tick_vcpu.is_some()
        && let Err(e) = TickLoop::pin_to_io(&c.settings.io_vcpus)
    {
        return fail(EXIT_FAILED, &format!("the I/O pin: {e}"));
    }
    let n = c.settings.clients as usize;
    let tokens = match mint(n) {
        Ok(t) => t,
        Err(e) => return fail(EXIT_FAILED, &format!("minting the tokens: {e}")),
    };
    if let Err(e) = write_tokens(&c.settings.token_file, &tokens) {
        let code = if e.kind() == io::ErrorKind::AlreadyExists {
            EXIT_SETTINGS
        } else {
            EXIT_FAILED
        };
        return fail(code, &format!("the token file: {e}"));
    }
    let net = match Net::bind(&c) {
        Ok(n) => n,
        Err((code, why)) => return fail(code, &why),
    };
    let sha = match binary_sha256() {
        Ok(s) => s,
        Err(e) => return fail(EXIT_FAILED, &format!("the binary's SHA-256: {e}")),
    };
    let smt = machine.smt.clone();
    let mut header = header(&c, sha, machine);
    header.params.extend(net.params(&c));
    let file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&c.settings.log)
    {
        Ok(f) => f,
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            return fail(EXIT_SETTINGS, &format!("the log: {e}"));
        }
        Err(e) => return fail(EXIT_FAILED, &format!("the log: {e}")),
    };
    let out: Box<dyn Write + Send> = Box::new(io::BufWriter::new(file));
    let writer = match Writer::new(out, &header) {
        Ok(w) => w,
        Err(e) => return fail(EXIT_SETTINGS, &format!("the log's header: {e}")),
    };
    let cpu = header.machine.cpu.clone();
    let s = c.settings;
    let loop_settings = LoopSettings {
        scene: c.scene,
        seed: s.seed,
        first_tick: 0,
        tokens,
        cmd_timing: c.timing,
        version: npt_proto::PROTOCOL_VERSION,
        content_hash: npt_proto::CONTENT_HASH,
        mode_hash: npt_proto::MODE_HASH,
        warmup_ticks: s.warmup_ticks,
        io_threads: s.io_threads as usize,
        log: Some(writer),
        tick_vcpu: s.tick_vcpu,
        io_vcpus: s.io_vcpus.clone(),
        smt,
        #[cfg(feature = "bites")]
        bite: s.bite.clone(),
    };
    let tl = match TickLoop::start(loop_settings) {
        Ok(t) => t,
        Err(e) => return fail(EXIT_FAILED, &format!("the room's start: {e}")),
    };
    // OPEN once both listeners accept (advisor-u6a-brief.md condition 3): spawn to Open covers a real start.
    let serving = match net.serve(&tl, s.ws_split_mark_s) {
        Ok(t) => t,
        Err(e) => {
            let _ = tl.stop();
            return fail(EXIT_FAILED, &format!("the listeners: {e}"));
        }
    };
    tl.event(cpu_read(&s.first_cpu_model, &cpu));
    let open_ns = TickLoop::now_ns();
    println!("OPEN");
    // T-7: both listeners' serving stamps and OPEN's, on the room's clock.
    let rows = [
        ("wt_serving_ns", serving[0]),
        ("ws_serving_ns", serving[1]),
        ("open_ns", open_ns),
    ];
    tl.event(event("open", Phase::Warmup, &rows));
    if io::stdout().flush().is_err() {
        return finish(tl, &net);
    }
    wait(&tl, s.duration_ms, || net.summaries(&tl));
    finish(tl, &net)
}

/// Waits for the end: the duration (when set), stdin closing, or the tick thread ending; `each` every 50 ms.
fn wait(tl: &TickLoop, duration_ms: u64, mut each: impl FnMut()) {
    let (tx, rx) = std::sync::mpsc::channel();
    // stdin by std on its own thread (SEC-U6AD-1): read into a fixed buffer and dropped, until it closes.
    let _ = std::thread::Builder::new()
        .name("npt-stdin".into())
        .spawn(move || {
            let mut buf = [0u8; 256];
            let mut stdin = io::stdin().lock();
            while matches!(stdin.read(&mut buf), Ok(n) if n > 0) {}
            let _ = tx.send(());
        });
    let step = Duration::from_millis(50);
    let mut left = if duration_ms == 0 {
        None
    } else {
        Some(duration_ms)
    };
    loop {
        if tl.finished() || rx.recv_timeout(step).is_ok() {
            return;
        }
        each();
        if let Some(l) = left.as_mut() {
            *l = l.saturating_sub(50);
            if *l == 0 {
                return;
            }
        }
    }
}

/// How the loop ended, as the exit code (the module text): 0 after a clean end, 3 after a crash, 1 for a log that
/// could not be finished (TR-U6A-6).
fn exit_code(ended: &io::Result<Ended>) -> u8 {
    match ended {
        Ok(Ended::Clean) => 0,
        Ok(_) => EXIT_CRASH,
        Err(_) => EXIT_FAILED,
    }
}

/// Stops the loop (the sessions first, then `net`'s end, then the log's: NR-U6B-1) and exits with [`exit_code`], one
/// stderr line for a non-zero one.
fn finish(tl: TickLoop, net: &Net) -> ExitCode {
    let ended = tl.stop_with(|tl| net.end(tl));
    match (exit_code(&ended), ended) {
        (0, _) => ExitCode::SUCCESS,
        (code, Err(e)) => fail(code, &format!("the log: {e}")),
        (code, Ok(_)) => fail(code, "the room crashed (a panic on the tick thread)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    /// A good settings text, every field in range, with `edit` applied to it.
    fn settings(edit: impl FnOnce(&mut serde_json::Value)) -> Vec<u8> {
        let mut v = serde_json::json!({
            "run": "C-u6a-1", "set": "C-u6a", "role": "measured", "marks": ["PM-0001-16"],
            "date": "2026-11-22", "commit": "0123456789abcdef0123456789abcdef01234567",
            "layer": "L2", "profile": "P0", "seed": 7, "scene": "SS", "clients": 48, "cmd_timing": "B",
            "tick_hz": 30, "machine_role": "C", "machine_name": "C", "machine_plan": "container",
            "machine_location": "here", "machine_address": "", "tick_vcpu": null, "io_vcpus": [],
            "io_threads": 2, "log": "room.jsonl", "token_file": "tokens", "duration_ms": 0,
            "warmup_ticks": 0, "first_cpu_model": "", "transport": "both",
            "wt_bind": "127.0.0.1:0", "ws_bind": "127.0.0.1:0", "ws_cert_chain": "", "ws_cert_key": "",
            "hash_path": "hash.json", "ws_split_mark_s": null
        });
        edit(&mut v);
        serde_json::to_vec(&v).expect("serializes")
    }

    /// A path in the system's temporary folder, unique to this test process and `name`.
    pub(super) fn temp(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("npt-u6a-{}-{name}", std::process::id()))
    }

    #[test]
    fn s8a_settings_with_every_field_in_range_are_taken() {
        let c = parse_settings(&settings(|_| {})).expect("taken");
        assert_eq!(
            (c.scene.name(), c.timing, c.settings.clients),
            ("SS", CmdTiming::B, 48)
        );
        let c = parse_settings(&settings(|v| {
            v["scene"] = "CO".into();
            v["clients"] = 1.into();
            v["cmd_timing"] = "A".into();
            v["tick_vcpu"] = 3.into();
            v["io_vcpus"] = serde_json::json!([1, 2]);
        }))
        .expect("taken");
        assert_eq!(
            (c.scene.name(), c.timing, c.settings.tick_vcpu),
            ("CO", CmdTiming::A, Some(3))
        );
    }

    /// One refusal case: what it breaks, and the edit to a good settings text.
    type Case = (&'static str, Box<dyn FnOnce(&mut serde_json::Value)>);

    #[test]
    fn s8a_settings_are_refused_for_an_unknown_field_a_value_out_of_range_or_a_file_above_the_cap()
    {
        let cases: Vec<Case> = vec![
            ("unknown field", Box::new(|v| v["extra"] = 1.into())),
            ("tick_hz", Box::new(|v| v["tick_hz"] = 60.into())),
            ("clients", Box::new(|v| v["clients"] = 0.into())),
            ("clients", Box::new(|v| v["clients"] = 49.into())),
            ("scene", Box::new(|v| v["scene"] = "XX".into())),
            ("cmd_timing", Box::new(|v| v["cmd_timing"] = "C".into())),
            ("io_threads", Box::new(|v| v["io_threads"] = 0.into())),
            ("commit", Box::new(|v| v["commit"] = "abc".into())),
            ("control", Box::new(|v| v["run"] = "C\u{7}".into())),
            ("tick_vcpu", Box::new(|v| v["tick_vcpu"] = 3.into())),
            (
                "tick_vcpu",
                Box::new(|v| {
                    v["tick_vcpu"] = 2.into();
                    v["io_vcpus"] = serde_json::json!([2]);
                }),
            ),
            (
                "two paths",
                Box::new(|v| v["token_file"] = "room.jsonl".into()),
            ),
            (
                "duration",
                Box::new(|v| v["duration_ms"] = (DURATION_MAX_MS + 1).into()),
            ),
            ("a seed as text", Box::new(|v| v["seed"] = "7".into())),
            (
                "missing",
                Box::new(|v| {
                    v.as_object_mut().expect("an object").remove("log");
                }),
            ),
        ];
        for (what, edit) in cases {
            assert!(parse_settings(&settings(edit)).is_err(), "{what}");
        }
        #[cfg(not(feature = "bites"))]
        assert!(
            parse_settings(&settings(|v| v["bite"] = "PM-0001-16/double-load".into())).is_err(),
            "a measured build has no bite field"
        );
        let big = temp("big-settings");
        let mut text = settings(|_| {});
        text.resize(SETTINGS_CAP as usize + 1, b' ');
        std::fs::write(&big, &text).expect("written");
        let e = read_settings(big.to_str().expect("utf-8"))
            .err()
            .expect("above the cap");
        assert!(e.contains("above"), "{e}");
        text.truncate(SETTINGS_CAP as usize);
        std::fs::write(&big, &text).expect("written");
        assert!(
            read_settings(big.to_str().expect("utf-8")).is_ok(),
            "at the cap: read"
        );
        std::fs::remove_file(&big).expect("removed");
    }

    #[test]
    fn s8a_the_token_file_is_made_new_with_mode_0600_and_holds_n_distinct_tokens() {
        assert_eq!(base64url(b"foob"), "Zm9vYg", "RFC 4648's vector");
        assert_eq!(base64url(&[0xfb, 0xff]), "-_8", "the URL-safe letters");
        let tokens = mint(48).expect("minted");
        assert_eq!(tokens.len(), 48);
        for (i, t) in tokens.iter().enumerate() {
            assert!(!tokens[..i].contains(t), "token {i} is distinct");
        }
        let path = temp("tokens");
        let p = path.to_str().expect("utf-8");
        write_tokens(p, &tokens).expect("written");
        let mode = std::fs::metadata(&path)
            .expect("there")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o600);
        let text = std::fs::read_to_string(&path).expect("read");
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 48);
        for (slot, line) in lines.iter().enumerate() {
            let (s, t) = line.split_once(' ').expect("slot and token");
            assert_eq!(s, slot.to_string());
            assert!(
                t.starts_with("nptj_") && t.len() == 5 + 22,
                "{slot}: 22 base64url characters"
            );
        }
        let again = write_tokens(p, &tokens).expect_err("an existing path is refused");
        assert_eq!(again.kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(
            std::fs::read_to_string(&path).expect("read"),
            text,
            "and left as it was"
        );
        std::fs::remove_file(&path).expect("removed");
    }

    /// Block 6: U6b's settings taken in range and refused out of it (SEC-U6AD-2's rules): the transport, both
    /// addresses, the profile, `cc`, the certificate files both or neither, the hash path apart, the mark's range; a
    /// certificate file above its cap is a settings refusal (exit 2).
    #[test]
    fn u6b_transport_settings_are_refused_out_of_range() {
        let c = parse_settings(&settings(|v| {
            v["cc"] = "newreno".into();
            v["ws_split_mark_s"] = 120.into();
            v["profile"] = "P2".into();
        }))
        .expect("taken");
        assert_eq!((c.cc.as_str(), c.l2.pass_through), ("newreno", false));
        let c = parse_settings(&settings(|_| {})).expect("taken");
        assert_eq!(
            (c.cc.as_str(), c.l2.pass_through),
            ("cubic", true),
            "P0 goes plain"
        );
        let cases: Vec<Case> = vec![
            ("transport", Box::new(|v| v["transport"] = "udp".into())),
            ("wt_bind", Box::new(|v| v["wt_bind"] = "localhost".into())),
            (
                "ws_bind",
                Box::new(|v| v["ws_bind"] = "192.0.2.1:99999".into()),
            ),
            ("profile", Box::new(|v| v["profile"] = "P9".into())),
            ("cc", Box::new(|v| v["cc"] = "bbr".into())),
            (
                "one certificate file",
                Box::new(|v| v["ws_cert_chain"] = "c.pem".into()),
            ),
            (
                "hash path",
                Box::new(|v| v["hash_path"] = "room.jsonl".into()),
            ),
            ("no hash path", Box::new(|v| v["hash_path"] = "".into())),
            ("mark 0", Box::new(|v| v["ws_split_mark_s"] = 0.into())),
            (
                "mark",
                Box::new(|v| v["ws_split_mark_s"] = (MARK_MAX_S + 1).into()),
            ),
            (
                "a control character",
                Box::new(|v| v["hash_path"] = "h\u{1}".into()),
            ),
            (
                "missing",
                Box::new(|v| {
                    v.as_object_mut().expect("an object").remove("transport");
                }),
            ),
        ];
        for (what, edit) in cases {
            assert!(parse_settings(&settings(edit)).is_err(), "{what}");
        }
        let big = temp("big-pem");
        std::fs::write(&big, vec![b'-'; PEM_CAP as usize + 1]).expect("written");
        let e = read_capped(big.to_str().expect("utf-8"), PEM_CAP).expect_err("above the cap");
        assert_eq!(e.0, EXIT_SETTINGS, "{}", e.1);
        std::fs::remove_file(&big).expect("removed");
    }

    /// TR-U6A-6: the binary's exit by how the loop ended (the module text): clean 0, a crash 3, a log not finished 1.
    #[test]
    fn s8a_the_exit_code_is_0_clean_3_after_a_crash_and_1_for_a_log_not_finished() {
        assert_eq!(exit_code(&Ok(Ended::Clean)), 0);
        assert_eq!(exit_code(&Ok(Ended::Crash)), 3);
        assert_eq!(exit_code(&Err(io::Error::other("the log"))), 1);
    }

    #[test]
    fn s4_cpu_read_changed_is_1_only_when_the_model_differs_from_the_first_starts() {
        let changed = |first: &str, now: &str| cpu_read(first, now).f["changed"].clone();
        assert_eq!(changed("", "AMD EPYC"), 0, "the first start: empty, 0");
        assert_eq!(changed("AMD EPYC", "AMD EPYC"), 0);
        assert_eq!(changed("AMD EPYC", "Intel Xeon"), 1);
        assert_eq!(cpu_read("", "x").ev, "cpu_read");
    }

    /// Carry 60 and the header (Block 4): the machine as the settings name it (this machine's reads, the clock's
    /// resolution above 0), and the scene's weapon and health from `Scene::weapon()` and `Scene::health()` in the params,
    /// for each scene; the stand-ins none (U6a burns none) and `cmd_timing` as set.
    #[test]
    fn s4_the_header_names_the_machine_and_carries_the_scenes_weapon_and_health() {
        for scene in ["SS", "ES", "CO", "TC"] {
            let c = parse_settings(&settings(|v| {
                v["scene"] = scene.into();
                v["clients"] = 1.into();
                v["cmd_timing"] = "A".into();
                v["machine_plan"] = "cx".into();
                v["machine_address"] = "192.0.2.7".into();
            }))
            .expect("taken");
            let h = header(&c, "ab".repeat(32), local_machine(&c.settings));
            let m = &h.machine;
            assert_eq!(
                (
                    m.role.as_str(),
                    m.name.as_str(),
                    m.plan.as_str(),
                    m.location.as_str(),
                    m.address.as_str()
                ),
                ("C", "C", "cx", "here", "192.0.2.7")
            );
            assert!(m.clock_res_ns > 0 && m.vcpus > 0 && !m.cpu.is_empty() && !m.smt.is_empty());
            assert_eq!(h.build.binary_sha256, "ab".repeat(32));
            assert_eq!(
                (h.process.as_str(), h.scene.as_str(), h.clients),
                ("room", scene, 1)
            );
            let w = c.scene.weapon();
            let p = &h.params;
            assert_eq!(p["weapon_interval_us"], w.interval_us, "{scene}");
            assert_eq!(p["weapon_damage_count"], w.damage);
            assert_eq!(p["weapon_reach_cm"], w.reach_cm);
            assert_eq!(p["weapon_magazine_count"], w.magazine);
            assert_eq!(p["weapon_reload_ms"], w.reload_ms);
            assert_eq!(p["weapon_spread_urad"], w.spread_urad);
            assert_eq!(p["soldier_health_count"], c.scene.health());
            assert_eq!(p["stand_ins"], serde_json::json!([]));
            // F4: no CPU stand-in burns at U6a (TR-U6A-8).
            assert_eq!(p["stand_in_burn_ns"], 0);
            assert_eq!(p["stand_in_top_up_ns"], 0);
            assert_eq!(p["cmd_timing"], "A");
            Writer::new(Vec::new(), &h).expect("npt-log takes the header");
        }
    }

    /// Q-U6A-5 (NR-U6A-2), on a fixed 2-thread map: an I/O vCPU on the tick vCPU's SMT sibling is refused (exit 2's
    /// line, `run` returning before the tokens); I/O vCPUs on other cores, or no pinning, are not.
    #[test]
    fn s4_an_io_vcpu_on_the_tick_vcpus_smt_sibling_is_refused() {
        let smt = [vec![0, 4], vec![1, 5], vec![2, 6], vec![3, 7]];
        let pinned = |io: serde_json::Value| {
            parse_settings(&settings(|v| {
                v["tick_vcpu"] = 3.into();
                v["io_vcpus"] = io;
            }))
            .expect("taken")
            .settings
        };
        let why = sibling_refusal(&pinned(serde_json::json!([1, 7])), &smt);
        assert!(
            why.is_some_and(|w| w.contains("SMT sibling of tick_vcpu 3")),
            "7 is 3's sibling"
        );
        assert_eq!(
            sibling_refusal(&pinned(serde_json::json!([1, 2])), &smt),
            None
        );
        let unpinned = parse_settings(&settings(|_| {})).expect("taken").settings;
        assert_eq!(sibling_refusal(&unpinned, &smt), None);
    }
}
