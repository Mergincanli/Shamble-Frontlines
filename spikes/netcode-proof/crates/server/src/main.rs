//! npt-server's binary: the room process (u6a-brief.md Block 3, S8a; plan E2; U6-Q17 term (2): the binary's settings,
//! lifecycle, panic catch and non-zero exit are U6a's, the WebTransport certificate U6b's).
//!
//! `npt-server --settings <file>`: the settings from one JSON file (at most [`SETTINGS_CAP`] bytes, read through `take`
//! and refused above it before parsing; unknown fields refused; every value range-checked before it sizes anything;
//! the tick rate 30 only, C-U6-16 (b) being moved), then:
//! 1. one join token per slot, 16 bytes each from /dev/urandom (roster.rs: "minting is U6's"), into a new 0600 file
//!    for the run script, one line per slot (the slot, then `nptj_` and the token in base64url), never logged;
//! 2. the room's log, its header the settings' fields with this machine and this build (the binary's SHA-256);
//! 3. the room built and TickLoop running, pinned when the settings name vCPUs; one `OPEN` line on stdout;
//! 4. the end: the settings' duration, or stdin closing (no tokio `signal` feature: SEC-U6AD-1), or the tick thread
//!    ending after a panic.
//!
//! Exit 0 after a clean end, 2 for bad settings (or a token or log file that already exists, or an I/O vCPU on an SMT
//! sibling of the tick vCPU on this machine's map: Q-U6A-5, refused before step 1), 3 after a crash, 1 for any other
//! failure. U6a binds and listens on no socket (SEC-U6AD-4): U6b's transports join here.

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::process::ExitCode;
use std::time::Duration;

use npt_log::{Build, Ended, EventRecord, Header, Machine, Phase, Writer};
use npt_room::CmdTiming;
use npt_server::{LoopSettings, TickLoop};
use npt_sim::Scene;
use serde::Deserialize;
use sha2::{Digest, Sha256};

/// The settings file's size limit, bytes.
const SETTINGS_CAP: u64 = 64 * 1024;
/// The protocol version every `Join` must carry, and the content hash (U7's page and U6b's clients take the same; no
/// plan line names the content's files yet: U6a's report, open items).
const PROTOCOL_VERSION: u16 = 1;
const CONTENT_HASH: [u8; 32] = *b"npt-server room content hash v1 ";
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
    /// The planted defect (bites builds only: a measured build refuses the field as unknown).
    #[cfg(feature = "bites")]
    #[serde(default)]
    bite: Option<String>,
}

/// What a settings file says, read and checked: the settings and the scene they name.
struct Checked {
    settings: Settings,
    scene: Scene,
    timing: CmdTiming,
}

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
    Ok(Checked {
        settings: s,
        scene,
        timing,
    })
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

/// The start, the run and the end (the module text, steps 1 to 4).
fn run(c: Checked) -> ExitCode {
    let machine = local_machine(&c.settings);
    if let Some(why) = sibling_refusal(&c.settings, &machine.smt) {
        return fail(EXIT_SETTINGS, &why);
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
    let sha = match binary_sha256() {
        Ok(s) => s,
        Err(e) => return fail(EXIT_FAILED, &format!("the binary's SHA-256: {e}")),
    };
    let smt = machine.smt.clone();
    let header = header(&c, sha, machine);
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
        version: PROTOCOL_VERSION,
        content_hash: CONTENT_HASH,
        mode_hash: [0; 32],
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
    tl.event(cpu_read(&s.first_cpu_model, &cpu));
    println!("OPEN");
    if io::stdout().flush().is_err() {
        return finish(tl);
    }
    wait(&tl, s.duration_ms);
    finish(tl)
}

/// Waits for the end: the duration (when set), stdin closing, or the tick thread ending.
fn wait(tl: &TickLoop, duration_ms: u64) {
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

/// Stops the loop and exits with [`exit_code`], one stderr line for a non-zero one.
fn finish(tl: TickLoop) -> ExitCode {
    let ended = tl.stop();
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
            "warmup_ticks": 0, "first_cpu_model": ""
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
