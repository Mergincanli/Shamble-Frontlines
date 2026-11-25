//! The -3 L2 probe's leg runner (u6c-brief.md Block 3, its C1 (b), C2 and C3; SERIES.md:497-501): npt-server's binary
//! in probe mode (settings `mode` `probe`) runs [`run_probe`]; LG's end is npt-net's `probe_peer`, under npt-load.
//!
//! The legs run in the order P0, P1, P2, P3, P4, B1 (any of them, P0 first). Each leg (C-U6-2: one ImpairSocket a leg on
//! the settings' port, rebound for at most 2 s since a dropped socket's threads end on their own):
//! 1. the socket with the leg's profile (P0 the pass-through; the later legs with P0's base round trip) and a fate log
//!    sized for the whole leg, pre-touched (C-U6-3);
//! 2. the poller on the runtime (C-U6-1); the first datagram from the peer's IPv4 (within 30 s, else the run ends
//!    non-zero), then `set_peer` on that address and the down pacer, which so never sends before the peer (C2); every
//!    other source dropped and counted; then, at the leg's zero, the start-edge reads (`leg_edge`, `cpu_stat`: C-U8-2,
//!    C-U8-6), later than the pre-touch, which holds the lock the kernel reader takes;
//! 3. the warm-up, the measured window (the datagrams a direction that put `packets` outside B1's burst, 50 s into the
//!    window, as L1 counts: NC-SB-16) and the tail (the longest one-way delay plus 1 s), inside which the loop calls
//!    `take_second` and reads the steal once a second and nothing else (C-U6-7), and reads the release thread's slack
//!    and policy once, just before the window (the window's start edge); every record stays in memory;
//! 4. the pacer and the poller stopped; with the socket still bound, the end-edge reads, the header's reads
//!    (`header_numbers`, `header_texts`), `run_summary`, the one drain (its hold measured) and the series; the socket
//!    dropped; the leg's log written whole at `<log>-<leg>.jsonl` (NC-SB-1; run `<run>-<leg>`, profile the leg, process
//!    `probe`, layer `L2`: NC-SB-3). The median of P0's round trips in its window's first 10 s is every later leg's
//!    `base_rtt_ns`. Each direction's measured records are L1's count (`leg_series`: the first `packets` fates outside
//!    the burst from the window's start, FP-1), so the tail holds the sends a late edge or a slow clock needs.

use std::fs::OpenOptions;
use std::io::{self, BufWriter};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::Relaxed};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use npt_log::{Ended, EventRecord, Header, Phase, SecondRecord, Writer};
use npt_net::{
    FateEntry, ImpairSocket, L2Config, LegSeries, ProbeDatagram, leg_series, quantile, send_down,
    start_poller,
};

use crate::clock;
use crate::proc_read::{StealReader, cpu_list};

/// The legs, in their one order (leg number = position).
pub const PROBE_LEGS: [&str; 6] = ["P0", "P1", "P2", "P3", "P4", "B1"];
const SEC: u64 = 1_000_000_000;
/// How long a leg waits for its peer's first datagram, and P0 for its first echo (C1 (b), C2).
pub const PEER_WAIT_NS: u64 = 30 * SEC;
/// A leg's rebind of the slot's port, at most (C2).
const REBIND_NS: u64 = 2 * SEC;
/// The base round trip's span: P0's window's first 10 s (ADR 0001:1590).
const BASE_SECS: u64 = 10;
/// The window's start-edge read comes this long before the window.
const EDGE_LEAD_NS: u64 = 10_000_000;

/// The probe's run (from the binary's settings, C1 (b)).
#[derive(Debug, Clone)]
pub struct ProbeSettings {
    /// Every leg's header starts from it (the settings' fields, this machine, this build).
    pub header: Header,
    /// The settings' `wt_bind` (UDP), every leg's port.
    pub bind: SocketAddr,
    /// LG's IPv4: the one peer.
    pub peer: Ipv4Addr,
    /// The legs to run, by number, increasing, P0 (0) first.
    pub legs: Vec<u8>,
    /// Datagrams a direction outside B1's burst in each window.
    pub packets: u64,
    pub warmup_s: u64,
    /// 10,000 at R1; tests run fewer.
    pub rate_pps: u64,
    pub seed: u64,
    /// The settings' `log`: each leg's log is named from it ([`leg_log`]).
    pub log: String,
    pub peer_wait_ns: u64,
}

/// A leg's log path: the leg before the extension (`room-02.jsonl` gives `room-02-P0.jsonl`: NC-SB-1).
pub fn leg_log(log: &str, leg: &str) -> String {
    let name_at = log.rfind('/').map_or(0, |i| i + 1);
    match log[name_at..].rfind('.') {
        Some(dot) => format!("{}-{leg}{}", &log[..name_at + dot], &log[name_at + dot..]),
        None => format!("{log}-{leg}"),
    }
}

/// The window's datagrams a direction: as L1 counts (lab/src/probe.rs:124-132; NC-SB-16), sends at interval / 2 +
/// i × interval from the window's start until `packets` lie outside `burst` (its start and length in the window).
pub(crate) fn window_total(packets: u64, interval_ns: u64, burst: Option<(u64, u64)>) -> u64 {
    let (mut total, mut outside) = (0u64, 0u64);
    while outside < packets {
        let t = interval_ns / 2 + total * interval_ns;
        outside += u64::from(!burst.is_some_and(|(b, len)| t >= b && t - b < len));
        total += 1;
    }
    total
}

/// The socket's own row of a /proc/net/udp or udp6 text, by its local port: its `drops` column (the last), or `None`
/// for no row (a failed read, never 0: NA5).
pub(crate) fn udp_drops(text: &str, port: u16) -> Option<u64> {
    text.lines().skip(1).find_map(|l| {
        let f: Vec<&str> = l.split_whitespace().collect();
        let p = f.get(1)?.rsplit_once(':')?.1;
        (u16::from_str_radix(p, 16).ok()? == port).then(|| f.last()?.parse().ok())?
    })
}

/// A CamelCase SNMP field as snake_case (`InCsumErrors` → `in_csum_errors`).
fn snake(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

/// /proc/net/snmp's two `Udp:` lines (names, then values) as `snmp_udp_<field>_count`, each field the kernel prints.
pub(crate) fn snmp_udp(text: &str) -> Vec<(String, u64)> {
    let mut udp = text.lines().filter(|l| l.starts_with("Udp: "));
    let (Some(names), Some(values)) = (udp.next(), udp.next()) else {
        return Vec::new();
    };
    names
        .split_whitespace()
        .zip(values.split_whitespace())
        .skip(1)
        .filter_map(|(n, v)| Some((format!("snmp_udp_{}_count", snake(n)), v.parse().ok()?)))
        .collect()
}

/// /proc/net/snmp6's `Udp6…` lines as `snmp6_udp_<field>_count`.
pub(crate) fn snmp6_udp(text: &str) -> Vec<(String, u64)> {
    text.lines()
        .filter_map(|l| {
            let (n, v) = l.split_once(char::is_whitespace)?;
            let field = n.strip_prefix("Udp6")?;
            Some((
                format!("snmp6_udp_{}_count", snake(field)),
                v.trim().parse().ok()?,
            ))
        })
        .collect()
}

/// /proc/stat's fields per `cpu` line, in the kernel's clock ticks as read.
const CPU_FIELDS: [&str; 10] = [
    "user",
    "nice",
    "system",
    "idle",
    "iowait",
    "irq",
    "softirq",
    "steal",
    "guest",
    "guest_nice",
];

/// Every `^cpu` line of /proc/stat (C-U8-2) as `cpu_<field>_count` (the summed line) and `cpu<N>_<field>_count`.
pub(crate) fn cpu_stat(text: &str) -> Vec<(String, u64)> {
    let mut out = Vec::new();
    for l in text.lines().filter(|l| l.starts_with("cpu")) {
        let mut f = l.split_whitespace();
        let Some(name) = f.next() else { continue };
        for (field, v) in CPU_FIELDS.iter().zip(f) {
            if let Ok(v) = v.parse() {
                out.push((format!("{name}_{field}_count"), v));
            }
        }
    }
    out
}

fn proc_text(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

/// The leg-edge reads (Q-U8-5; C-U8-2, C-U8-6) as the `leg_edge` and `cpu_stat` events at `ph`, the socket bound: the
/// read borrows the leg's socket (and reads nothing from it), so a read after the socket's drop does not compile (FP-2:
/// P8 is held by the compiler, since the kernel socket outlives the drop a moment and no test can tell).
fn edge_events(_bound: &ImpairSocket, port: u16, ph: Phase) -> [EventRecord; 2] {
    let drops = udp_drops(&proc_text("/proc/net/udp"), port)
        .or_else(|| udp_drops(&proc_text("/proc/net/udp6"), port));
    let mut edge =
        EventRecord::new("leg_edge", ph).with("udp_row_found", i64::from(drops.is_some()));
    if let Some(d) = drops {
        edge = edge.with("udp_drops_count", d);
    }
    let snmp = snmp_udp(&proc_text("/proc/net/snmp"))
        .into_iter()
        .chain(snmp6_udp(&proc_text("/proc/net/snmp6")));
    let edge = snmp.fold(edge, |e, (k, v)| e.with(&k, v));
    let cpu = cpu_stat(&proc_text("/proc/stat"))
        .into_iter()
        .fold(EventRecord::new("cpu_stat", ph), |e, (k, v)| e.with(&k, v));
    [edge, cpu]
}

/// The vCPUs this process may use (`Cpus_allowed_list`), read once (`params.steal_cpus`: C-U6-4).
fn steal_cpus() -> Vec<u32> {
    proc_text("/proc/self/status")
        .lines()
        .find_map(|l| l.strip_prefix("Cpus_allowed_list:"))
        .map(cpu_list)
        .unwrap_or_default()
}

/// One second's reads (C-U6-7): its index, `take_second`'s rows and the steal.
pub(crate) struct Second {
    pub(crate) sec: u64,
    pub(crate) l2: Vec<(&'static str, u64)>,
    pub(crate) steal: Option<u32>,
}

/// What the leg's loop may do (C-U6-7): read the clock, sleep, and per second `take` (the socket's `take_second`) and
/// `steal`; once, `edge` (the window's start-edge read); `give_up` ends it early with its reason.
pub(crate) struct Hooks<'a> {
    pub(crate) now: &'a dyn Fn() -> u64,
    pub(crate) sleep: &'a dyn Fn(u64),
    pub(crate) take: &'a mut dyn FnMut() -> Vec<(&'static str, u64)>,
    pub(crate) steal: &'a mut dyn FnMut() -> Option<u32>,
    pub(crate) edge: &'a mut dyn FnMut(),
    pub(crate) give_up: &'a dyn Fn(u64) -> Option<String>,
}

/// The leg's loop from `zero` to `end` (one clock): `edge` once at `edge_at`, then a [`Second`] at each whole second
/// after `zero` and one at `end`, into `out` (made before with room for them all). Sleeps at most 100 ms at a time.
pub(crate) fn seconds_loop(
    zero: u64,
    edge_at: u64,
    end: u64,
    h: &mut Hooks<'_>,
    out: &mut Vec<Second>,
) -> Result<(), String> {
    let (mut k, mut edged) = (0u64, false);
    loop {
        let t = (h.now)();
        if !edged && t >= edge_at {
            (h.edge)();
            edged = true;
        }
        let close = (zero + (k + 1) * SEC).min(end);
        let target = if edged { close } else { close.min(edge_at) };
        if t < target {
            if let Some(why) = (h.give_up)(t) {
                return Err(why);
            }
            (h.sleep)((target - t).min(100_000_000));
            continue;
        }
        if t >= close {
            out.push(Second {
                sec: k,
                l2: (h.take)(),
                steal: (h.steal)(),
            });
            k += 1;
            if close >= end {
                return Ok(());
            }
        }
    }
}

/// The poller's state (C2, C3): the peer (the first source with the peer's IPv4), one echo record a datagram in release
/// order ([`leg_series`]'s `echoes`), and the counts.
struct Intake {
    ip: Ipv4Addr,
    leg: u8,
    peer: Arc<OnceLock<SocketAddr>>,
    echoes: Arc<AtomicU64>,
    records: Vec<(u64, u64)>,
    other: u64,
    refused: u64,
    dropped: u64,
}

impl Intake {
    fn take(&mut self, b: &[u8], a: SocketAddr) {
        let ours = match a.ip() {
            IpAddr::V4(v) => v == self.ip,
            IpAddr::V6(v) => v.to_ipv4_mapped() == Some(self.ip),
        };
        // SEC-U6C-4: only a probe datagram pins the peer, so another port's junk from LG's IPv4 cannot.
        if ours && self.peer.get().is_none() && ProbeDatagram::decode(b, None).is_some() {
            let _ = self.peer.set(a);
        }
        let rec = if self.peer.get() == Some(&a) {
            match ProbeDatagram::decode(b, Some(self.leg)) {
                Some(d) => {
                    self.echoes.fetch_add(u64::from(d.echo.is_some()), Relaxed);
                    (d.echo.unwrap_or(LegSeries::NO_ECHO), d.hold_ns)
                }
                None => {
                    self.refused += 1;
                    (LegSeries::NO_ECHO, 0)
                }
            }
        } else {
            self.other += 1;
            (LegSeries::OTHER, 0)
        };
        if self.records.len() < self.records.capacity() {
            self.records.push(rec);
        } else {
            self.dropped += 1;
        }
    }
}

/// The leg's configuration: P0 the pass-through, B1 with its bursts, the rest by name; the base round trip set.
fn leg_config(leg: u8, seed: u64, base_rtt_ns: u64, bursts: &[u64]) -> L2Config {
    let mut c = match leg {
        0 => L2Config::clean(seed),
        5 => L2Config::b1(seed, bursts),
        l => L2Config::named(seed, PROBE_LEGS[usize::from(l)])
            .unwrap_or_else(|| L2Config::clean(seed)),
    };
    c.base_rtt_ns = base_rtt_ns;
    c
}

/// Binds the leg's socket, retrying an address in use for at most 2 s (C2).
fn bind(addr: SocketAddr, cfg: &L2Config) -> io::Result<Arc<ImpairSocket>> {
    let end = clock::now_ns() + REBIND_NS;
    loop {
        match ImpairSocket::bind(addr, cfg.clone()) {
            Err(e) if e.kind() == io::ErrorKind::AddrInUse && clock::now_ns() < end => {
                std::thread::sleep(Duration::from_millis(5));
            }
            r => return r,
        }
    }
}

/// A leg's log, whole, written after the leg.
struct LegLog {
    header: Header,
    before: Vec<EventRecord>,
    seconds: Vec<SecondRecord>,
    after: Vec<EventRecord>,
}

impl LegLog {
    /// Writes the log at `path`, made new (an existing path is refused).
    fn write(&self, path: &str) -> io::Result<()> {
        let f = OpenOptions::new().write(true).create_new(true).open(path)?;
        let mut w = Writer::new(BufWriter::new(f), &self.header)?;
        for e in &self.before {
            w.event(e)?;
        }
        for s in &self.seconds {
            w.second(s)?;
        }
        for e in &self.after {
            w.event(e)?;
        }
        w.finish(Ended::Clean)?
            .into_inner()
            .map_err(|e| e.into_error())?
            .sync_all()
    }
}

fn int(x: u64) -> i64 {
    i64::try_from(x).unwrap_or(i64::MAX)
}

/// Runs every leg of `s` (the module text); `open` once P0's socket is bound (the binary's `OPEN`). `stop` (stdin's
/// close) ends the run at once, the finished legs' logs kept. Errors as (exit code, why): 2 for a leg's log that
/// exists, 1 for the rest.
pub fn run_probe(
    s: &ProbeSettings,
    stop: &AtomicBool,
    open: &mut dyn FnMut(),
) -> Result<(), (u8, String)> {
    for &l in &s.legs {
        let p = leg_log(&s.log, PROBE_LEGS[usize::from(l)]);
        if std::path::Path::new(&p).exists() {
            return Err((2, format!("the leg's log {p} exists")));
        }
    }
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("npt-probe")
        .enable_all()
        .build()
        .map_err(|e| (1, format!("the runtime: {e}")))?;
    let cpus = steal_cpus();
    let (mut base, mut open) = (0u64, Some(open));
    for &l in &s.legs {
        let name = PROBE_LEGS[usize::from(l)];
        let mut bound = || {
            if let Some(o) = open.take() {
                o();
            }
        };
        let (log, rtts) = run_leg(s, l, base, stop, &rt, &cpus, &mut bound)
            .map_err(|why| (1, format!("leg {name}: {why}")))?;
        if l == 0 {
            let mut rtts = rtts;
            rtts.sort_unstable();
            if rtts.is_empty() {
                return Err((
                    1,
                    format!("leg P0: no round trip in its window's first {BASE_SECS} s"),
                ));
            }
            base = quantile(&rtts, 1, 2);
        }
        let path = leg_log(&s.log, name);
        log.write(&path)
            .map_err(|e| (1, format!("leg {name}: its log {path}: {e}")))?;
    }
    Ok(())
}

/// The peer's address once the poller has seen its first datagram (at most `peer_wait_ns`), or why not.
fn wait_peer(
    peer: &OnceLock<SocketAddr>,
    stop: &AtomicBool,
    s: &ProbeSettings,
) -> Result<SocketAddr, String> {
    let waited = clock::now_ns();
    loop {
        if let Some(a) = peer.get() {
            return Ok(*a);
        }
        if stop.load(Relaxed) {
            return Err("stdin closed".into());
        }
        if clock::now_ns() - waited > s.peer_wait_ns {
            return Err(format!(
                "no datagram from the peer {} within {} s",
                s.peer,
                s.peer_wait_ns / SEC
            ));
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// One leg (the module text): its log, and its round trips in the window's first 10 s.
fn run_leg(
    s: &ProbeSettings,
    l: u8,
    base: u64,
    stop: &AtomicBool,
    rt: &tokio::runtime::Runtime,
    cpus: &[u32],
    bound: &mut dyn FnMut(),
) -> Result<(LegLog, Vec<u64>), String> {
    let interval = SEC / s.rate_pps.max(1);
    let burst = (l == 5).then_some((50 * SEC, SEC));
    let total = window_total(s.packets, interval, burst);
    let (warm, window) = (s.warmup_s * SEC, total * interval);
    let mut cfg = leg_config(l, s.seed, base, &[]);
    let tail = cfg.profile.rtt_ns / 2 + cfg.profile.half_jitter_ns() + SEC;
    let leg_secs = (warm + window + tail).div_ceil(SEC);
    let records = s.rate_pps * (leg_secs + 2) + 10_000;
    cfg.fate_log_capacity = usize::try_from(2 * records).unwrap_or(usize::MAX);
    let sock = bind(s.bind, &cfg).map_err(|e| format!("bind {}: {e}", s.bind))?;
    bound();
    let touched = sock.pretouch_fate_log();
    // The settings' own port (probe mode refuses port 0), the one LG sends to.
    let port = s.bind.port();
    let poll_stop = Arc::new(tokio::sync::Notify::new());
    let (peer, echoes) = (Arc::new(OnceLock::new()), Arc::new(AtomicU64::new(0)));
    let intake = Intake {
        ip: s.peer,
        leg: l,
        peer: Arc::clone(&peer),
        echoes: Arc::clone(&echoes),
        records: Vec::with_capacity(usize::try_from(records).unwrap_or(usize::MAX)),
        other: 0,
        refused: 0,
        dropped: 0,
    };
    let poller = start_poller(rt.handle(), &sock, &poll_stop, intake, Intake::take);
    let mut bursts = Vec::new();
    let started = wait_peer(&peer, stop, s).and_then(|addr| {
        let at = sock.now_ns() + warm;
        bursts.extend(burst.map(|(b, len)| (at + b, len)));
        let starts: Vec<u64> = bursts.iter().map(|b: &(u64, u64)| b.0).collect();
        sock.set_peer(addr, &leg_config(l, s.seed, base, &starts));
        send_down(&sock, addr, l, interval).map_err(|e| format!("the pacer: {e}"))
    });
    let (pacer, zero) = match started {
        Ok(x) => x,
        Err(why) => {
            poll_stop.notify_one();
            let _ = rt.block_on(poller);
            return Err(why);
        }
    };
    // NPT-EDGE-READ (U6c round 2, item 1 (a)): the start edge at the leg's zero, not just after the pre-touch, which
    // holds the lock the kernel reader takes, so the buffer can still be overflowing then. The drops counter only
    // rises, so a later read can only move such drops from between the edges (C-U8-6) to before them.
    let before = edge_events(&sock, port, Phase::Warmup);
    let pacer_reads = (pacer.slack_ns(), pacer.policy().to_owned());
    // The loop's clock: the socket's, read through the process's own (no socket call: C-U6-7).
    let offset = i128::from(sock.now_ns()) - i128::from(clock::now_ns());
    let now = || u64::try_from(i128::from(clock::now_ns()) + offset).unwrap_or(0);
    let mut readers: Vec<StealReader> = cpus.iter().map(|&c| StealReader::new(c)).collect();
    let stat0 = proc_text("/proc/stat");
    for r in &mut readers {
        let _ = r.read(&stat0);
    }
    let mut start_edge = (None, String::new());
    let mut seconds = Vec::with_capacity(usize::try_from(leg_secs + 1).unwrap_or(0));
    let mut take = || sock.take_second();
    let mut steal = || {
        let t = proc_text("/proc/stat");
        let v: Option<Vec<u32>> = readers.iter_mut().map(|r| r.read(&t)).collect();
        v.filter(|v| !v.is_empty())
            .map(|v| v.iter().sum::<u32>() / v.len() as u32)
    };
    let mut edge = || {
        let n = sock.header_numbers();
        let slack = n.iter().find(|x| x.0 == "l2_timer_slack_ns").map(|x| x.1);
        let policy = sock
            .header_texts()
            .into_iter()
            .find(|x| x.0 == "l2_sched_policy");
        start_edge = (slack, policy.map(|x| x.1).unwrap_or_default());
    };
    let give_up = |t: u64| {
        if stop.load(Relaxed) {
            Some("stdin closed".to_owned())
        } else if l == 0 && t >= zero + PEER_WAIT_NS && echoes.load(Relaxed) == 0 {
            Some(format!("no echo within {} s", PEER_WAIT_NS / SEC))
        } else {
            None
        }
    };
    let sleep = |ns: u64| std::thread::sleep(Duration::from_nanos(ns));
    let mut hooks = Hooks {
        now: &now,
        sleep: &sleep,
        take: &mut take,
        steal: &mut steal,
        edge: &mut edge,
        give_up: &give_up,
    };
    let run = seconds_loop(
        zero,
        zero + warm - warm.min(EDGE_LEAD_NS),
        zero + warm + window + tail,
        &mut hooks,
        &mut seconds,
    );
    let sent = pacer.stop();
    poll_stop.notify_one();
    let (polled, intake) = rt
        .block_on(poller)
        .map_err(|e| format!("the poller: {e}"))?;
    run?;
    let after_edge = edge_events(&sock, port, Phase::After);
    let (numbers, texts, summary) = (
        sock.header_numbers(),
        sock.header_texts(),
        sock.run_summary(),
    );
    let mut fates = Vec::with_capacity(cfg.fate_log_capacity);
    let hold = FateEntry::drain_into(&sock, &mut fates)?;
    // FP-1: L1's count-based records, `packets` a direction outside B1's burst from the window's start.
    let series = leg_series(
        &fates,
        &intake.records,
        zero,
        zero + warm,
        s.packets,
        interval,
        &bursts,
    );
    drop(sock);
    let mut h = s.header.clone();
    h.run = format!("{}-{}", s.header.run, PROBE_LEGS[usize::from(l)]);
    h.profile = PROBE_LEGS[usize::from(l)].to_owned();
    h.process = "probe".to_owned();
    h.layer = "L2".to_owned();
    let p = &mut h.params;
    p.extend(numbers.into_iter().map(|(k, v)| (k.to_owned(), v.into())));
    p.extend(texts.into_iter().map(|(k, v)| (k.to_owned(), v.into())));
    let rel: Vec<u64> = bursts.iter().map(|b| b.0 - zero).collect();
    for (k, v) in [
        ("probe_rate_pps", serde_json::Value::from(s.rate_pps)),
        ("probe_sent_count", total.into()),
        ("probe_packets_count", s.packets.into()),
        ("probe_warmup_ns", warm.into()),
        ("probe_tail_ns", tail.into()),
        ("burst_starts_ns", rel.into()),
        ("steal_cpus", cpus.to_vec().into()),
        (
            "probe_start_timer_slack_ns",
            start_edge.0.unwrap_or(u64::MAX).into(),
        ),
        ("probe_start_sched_policy", start_edge.1.into()),
        (
            "probe_pacer_timer_slack_ns",
            pacer_reads.0.unwrap_or(u64::MAX).into(),
        ),
        ("probe_pacer_sched_policy", pacer_reads.1.into()),
        ("probe_fate_log_touched_count", (touched as u64).into()),
    ] {
        p.insert(k.to_owned(), v);
    }
    let first_window = warm / SEC;
    // Measured to the nominal window's end, and on to the last record's second (FP-1: the records run to it).
    let nominal_end = (warm + window).div_ceil(SEC);
    let measured_end = series
        .seconds
        .last()
        .map_or(nominal_end, |(k, _)| nominal_end.max(k + 1));
    let mut by_sec = series.seconds.into_iter().peekable();
    let mut rtts = Vec::new();
    let records = seconds
        .into_iter()
        .map(|x| {
            let ph = match x.sec {
                k if k < first_window => Phase::Warmup,
                k if k < measured_end => Phase::Measured,
                _ => Phase::After,
            };
            let mut r = SecondRecord::new(x.sec, ph);
            r.v.extend(x.l2.iter().map(|&(k, v)| (k.to_owned(), int(v))));
            if let Some(v) = x.steal {
                r.v.insert("steal_permille".to_owned(), i64::from(v));
            }
            if by_sec.peek().is_some_and(|(k, _)| *k == x.sec) {
                let (k, lists) = by_sec.next().unwrap_or_default();
                if k < first_window + BASE_SECS {
                    rtts.extend(lists[6].iter().map(|&v| v as u64));
                }
                r.s.extend(LegSeries::NAMES.iter().map(|n| (*n).to_owned()).zip(lists));
            }
            r
        })
        .collect();
    let mut summary_ev = EventRecord::new("l2_summary", Phase::After);
    for (k, v) in summary.into_iter().chain(series.summary) {
        summary_ev = summary_ev.with(k, v);
    }
    for (k, v) in [
        ("probe_window_ns", window),
        ("probe_drain_hold_ns", hold),
        ("probe_polled_count", polled.got),
        ("probe_other_source_count", intake.other),
        ("probe_refused_count", intake.refused),
        ("probe_records_dropped_count", intake.dropped),
        ("probe_down_sent_count", sent),
    ] {
        summary_ev = summary_ev.with(k, v);
    }
    let [a, b] = after_edge;
    let log = LegLog {
        header: h,
        before: before.into(),
        seconds: records,
        after: vec![a, b, summary_ev],
    };
    Ok((log, rtts))
}

#[cfg(test)]
#[path = "probe_tests.rs"]
mod tests;
