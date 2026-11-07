//! The WebSocket fallback's fixes (Mustafa, 2026-11-06), measured on the model as a browser and a tick-driven room
//! can run them (the two skeptics of 2026-11-07: .checkpoints/netcode-proof-test/u3-verify-fixes-model.md and
//! u3-verify-fixes-adr.md):
//! - fix 1: the room writes a snapshot only while what is outstanding (unsent plus unacknowledged, SIOCOUTQ) leaves
//!   room for it within ceil(1.5 x srtt / tick) + 1 snapshots, at least 3 (srtt from TCP_INFO): newest only.
//! - fix 2 (thin-stream linear timeouts, a lower minimum RTO): inert beside fix 1, whose stream is not thin (more than
//!   3 segments out at P2); a lower minimum RTO there only shortens doubled timeouts (the model skeptic's R3).
//! - fix 3: the client reconnects after a silence of max(300 ms, 2 x its own RTT + 100 ms), its RTT measured from the
//!   input echo (a page cannot read TCP's); one attempt at a time per endpoint, as browsers serialize WebSocket
//!   connections to one address and port (RFC 6455 §4.1; Chromium's endpoint lock, freed 10 ms after the 101); an
//!   attempt still connecting after a set time is closed; the session resumes on the newest Join (ADR 0001:507).
//!
//! The room writes snapshots and handles Joins on its 30 Hz tick. A phase ends when the client decodes deltas again
//! (PM-0001-18, ADR 0001:1556): the first snapshot written at or after the link's return, on a new connection the
//! first delta after the full state. Each row runs 6 sets of 1,000 B1 phases (S13 (e)'s timings, shifted per set);
//! the side effects run the same harness without a burst at P1 to P4.

use super::*;
use std::collections::VecDeque;

const MS: u64 = 1_000_000;
const SEC: u64 = 1_000_000_000;
const TICK: u64 = SEC / 30;
/// A snapshot: 1,024 B of payload, a 4 B WebSocket header and TLS 1.3's 22 B (S0).
const SNAP: u64 = 1_050;
/// A client input: 60 B of payload, a 6 B masked-frame header and TLS's 22 B.
const INPUT: u64 = 88;
/// A new connection's flights: the browser's ClientHello (Chromium and Firefox offer an X25519MLKEM768 key share:
/// about 1.8 KB, two segments); the server's ServerHello to Finished (an ML-KEM ciphertext and a Let's Encrypt ECDSA
/// chain: three segments); the client's Finished with its upgrade request; the server's 101; the client's Join (a
/// resume, with a token held in advance: RESUME-CRED); the server's JoinAccepted with the full state.
const CLIENT_HELLO: u64 = 1_800;
const SERVER_FLIGHT: u64 = 3_800;
const FINISHED_UPGRADE: u64 = 474;
const SWITCHING: u64 = 200;
const JOIN: u64 = 140;
const RESUME: u64 = 4_200;
/// Chromium frees a WebSocket endpoint's lock 10 ms after the attempt holding it opens or fails (the model skeptic's
/// read of websocket_endpoint_lock_manager.cc, 2026-11-06).
const UNLOCK_DELAY: u64 = 10 * MS;
/// The phases of a row: sets of 1,000.
const SETS: u64 = 6;
const PHASES: u64 = 1_000;

#[derive(Debug, Clone, Copy)]
struct Opts {
    cfg: TcpConfig,
    /// Fix 1: the adaptive newest-only budget (else every snapshot is written while the write budget has room).
    adaptive: bool,
    reconnect: Option<Reconnect>,
    /// A reconnect's full state counts as decoding again (else the first delta after it, PM-0001-18's wording).
    full_state_counts: bool,
}

/// Fix 3's policy.
#[derive(Debug, Clone, Copy)]
struct Reconnect {
    /// The least silence that starts an attempt: the trigger is max(this, 2 x the client's RTT + 100 ms).
    min_gap: u64,
    /// The room's endpoints (1: one TCP port per room, ADR 0001:429-432); attempts take them in turn.
    endpoints: u32,
    /// The spacing of attempt starts.
    cadence: u64,
    /// An attempt still connecting (before its 101) this long after it started is closed, freeing its endpoint.
    abort_after: Option<u64>,
    /// No browser lock: the first version's parallel attempts, which no browser runs (a reference row).
    unlocked: bool,
    /// The most attempts in one outage.
    max_attempts: u32,
}

/// The adaptive budget: snapshots for 1.5 round trips plus one, at least 3.
fn adaptive_limit(m: &TcpModel) -> u64 {
    let srtt = m.srtt_ns(0).unwrap_or(100 * MS);
    let n = (3 * srtt).div_ceil(2 * TICK) + 1;
    n.max(3) * SNAP
}

/// The silence that starts a reconnect, from the client's own RTT (100 ms before its first sample).
fn trigger(r: &Reconnect, app_rtt: Option<u64>) -> u64 {
    r.min_gap.max(2 * app_rtt.unwrap_or(100 * MS) + 100 * MS)
}

/// Where a connection's script stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Connecting,
    Hello,
    Flight,
    Upgrade,
    Switching,
    Joined,
    Live,
}

impl Step {
    /// Before the 101: the page sees CONNECTING and the browser holds the endpoint's lock.
    fn connecting(self) -> bool {
        !matches!(self, Step::Joined | Step::Live)
    }
}

struct Conn {
    m: TcpModel,
    step: Step,
    endpoint: u32,
    started: u64,
    /// Closed (abandoned while connecting, or left behind by a switch).
    dead: bool,
    /// Bytes each end still has to write (handshake flights and the full state; written as the budget frees).
    owed: [u64; 2],
    /// The full state's end offset, once the server took the session over here.
    resume_end: Option<u64>,
    /// The end offset of the first message here that counts as decoding again.
    fresh_end: Option<u64>,
    delivered_seen: u64,
    /// The client's inputs here (end offset up, write time) and the room's messages (end offset down, the up-stream
    /// bytes the server had at the write): the input echo behind the client's RTT.
    inputs: VecDeque<(u64, u64)>,
    snaps: VecDeque<(u64, u64)>,
}

impl Conn {
    fn new(m: TcpModel, step: Step, endpoint: u32, started: u64) -> Conn {
        Conn {
            m,
            step,
            endpoint,
            started,
            dead: false,
            owed: [0, 0],
            resume_end: None,
            fresh_end: None,
            delivered_seen: 0,
            inputs: VecDeque::new(),
            snaps: VecDeque::new(),
        }
    }

    fn flush(&mut self, end: usize, t: u64) {
        if self.owed[end] > 0 && self.m.established(end) {
            let n = self.m.write(end, self.owed[end], t);
            self.owed[end] -= n;
        }
    }
}

/// What one run did.
#[derive(Debug, Clone, Copy, Default)]
struct Outcome {
    /// From the link's return to decoding again (runs with a burst).
    recovery: Option<u64>,
    /// Reconnect attempts started, sessions taken over, and the snapshots the room wrote of the ticks it had.
    attempts: u32,
    takeovers: u32,
    written: u64,
    ticks: u64,
}

/// One run under `p` to `until`; with `back` (the link's return after a burst), it ends when the client decodes
/// deltas again.
fn run(p: &Profile, seed: u64, back: Option<u64>, until: u64, o: Opts) -> Outcome {
    let mut conns = vec![Conn::new(
        TcpModel::with_config(p, seed, 0, o.cfg),
        Step::Live,
        0,
        0,
    )];
    let (mut tick, mut input) = (TICK, TICK + (seed * 7_919) % TICK);
    let (mut server_cur, mut client_cur) = (0usize, 0usize);
    let mut last_data = 0u64;
    let mut app_rtt: Option<u64> = None;
    let mut out = Outcome::default();
    let mut next_attempt = 0u64;
    let mut unlock_at = [0u64; 2];
    // An attempt waiting for its endpoint's lock: (when its connect starts, endpoint).
    let mut queued: Option<(u64, u32)> = None;
    let after_back = |t: u64| back.is_some_and(|b| t >= b);
    loop {
        let mut next = tick.min(input);
        for c in conns.iter().filter(|c| !c.dead) {
            if let Some(e) = c.m.next_event() {
                next = next.min(e);
            }
        }
        if let Some((at, _)) = queued {
            next = next.min(at);
        } else if let Some(r) = o.reconnect
            && out.attempts < r.max_attempts
        {
            next = next.min((last_data + trigger(&r, app_rtt)).max(next_attempt));
        }
        let t = next;
        if t > until {
            break;
        }
        for c in conns.iter_mut().filter(|c| !c.dead) {
            c.m.advance(t);
        }
        // The client: data on its connection, the input echo, each script's next flight.
        for (i, c) in conns.iter_mut().enumerate().filter(|(_, c)| !c.dead) {
            let d = c.m.delivered(1);
            if d > c.delivered_seen {
                c.delivered_seen = d;
                if i == client_cur {
                    last_data = t;
                }
            }
            let mut echoed = None;
            while let Some(&(end, acked)) = c.snaps.front()
                && d >= end
            {
                c.snaps.pop_front();
                while c.inputs.front().is_some_and(|&(e, _)| e <= acked) {
                    echoed = c.inputs.pop_front();
                }
            }
            if let Some((_, at)) = echoed {
                let s = t - at;
                app_rtt = Some(app_rtt.map_or(s, |r| r - r / 8 + s / 8));
            }
            match c.step {
                Step::Connecting if c.m.established(1) => {
                    c.owed[1] += CLIENT_HELLO;
                    c.step = Step::Hello;
                }
                Step::Hello if c.m.delivered(0) >= CLIENT_HELLO => {
                    c.owed[0] += SERVER_FLIGHT;
                    c.step = Step::Flight;
                }
                Step::Flight if d >= SERVER_FLIGHT => {
                    c.owed[1] += FINISHED_UPGRADE;
                    c.step = Step::Upgrade;
                }
                Step::Upgrade if c.m.delivered(0) >= CLIENT_HELLO + FINISHED_UPGRADE => {
                    c.owed[0] += SWITCHING;
                    c.step = Step::Switching;
                }
                Step::Switching if d >= SERVER_FLIGHT + SWITCHING => {
                    // The 101: the page sees open, sends its Join; the browser frees the endpoint's lock.
                    c.owed[1] += JOIN;
                    c.step = Step::Joined;
                    unlock_at[c.endpoint as usize] = t + UNLOCK_DELAY;
                }
                _ => {}
            }
            c.flush(0, t);
            c.flush(1, t);
        }
        // The client switches once the full state has arrived on a newer connection: it closes the others.
        if let Some(i) = (0..conns.len()).rev().find(|&i| {
            i != client_cur
                && !conns[i].dead
                && conns[i]
                    .resume_end
                    .is_some_and(|e| conns[i].m.delivered(1) >= e)
        }) {
            client_cur = i;
            last_data = t;
            for (j, c) in conns.iter_mut().enumerate() {
                if j != i && !c.dead {
                    if c.step.connecting() {
                        unlock_at[c.endpoint as usize] = t + UNLOCK_DELAY;
                    }
                    c.dead = true;
                }
            }
        }
        // The room, on its tick: the newest Join takes the session over (the older connection is no longer written);
        // its first message is the full state. Then the tick's snapshot, newest only.
        if t == tick {
            out.ticks += 1;
            let mut resumed = false;
            for (i, c) in conns.iter_mut().enumerate() {
                if !c.dead
                    && c.step == Step::Joined
                    && c.m.delivered(0) >= CLIENT_HELLO + FINISHED_UPGRADE + JOIN
                {
                    c.step = Step::Live;
                    let before = c.m.ends[0].snd.write_seq + c.owed[0];
                    c.owed[0] += RESUME;
                    c.flush(0, t);
                    c.resume_end = Some(before + RESUME);
                    c.snaps.push_back((before + RESUME, c.m.delivered(0)));
                    if o.full_state_counts && after_back(t) {
                        c.fresh_end = Some(before + RESUME);
                    }
                    server_cur = i;
                    out.takeovers += 1;
                    resumed = true;
                }
            }
            let c = &mut conns[server_cur];
            let fits = !o.adaptive || c.m.outstanding(0) + SNAP <= adaptive_limit(&c.m);
            if !resumed && c.owed[0] == 0 && c.m.free_budget(0) >= SNAP && fits {
                c.m.write(0, SNAP, t);
                out.written += 1;
                let end = c.m.ends[0].snd.write_seq;
                c.snaps.push_back((end, c.m.delivered(0)));
                if after_back(t) && c.fresh_end.is_none() {
                    c.fresh_end = Some(end);
                }
            }
            tick += TICK;
        }
        if t == input {
            let c = &mut conns[client_cur];
            c.m.write(1, INPUT, t);
            c.inputs.push_back((c.m.ends[1].snd.write_seq, t));
            input += TICK;
        }
        // Fix 3: a queued attempt whose endpoint is free; else, after the silence, the next attempt in turn.
        if let Some((at, k)) = queued
            && t >= at
        {
            conns.push(Conn::new(
                TcpModel::connect(p, seed, out.attempts, o.cfg, t),
                Step::Connecting,
                k,
                t,
            ));
            queued = None;
        } else if let Some(r) = o.reconnect
            && queued.is_none()
            && out.attempts < r.max_attempts
            && t >= last_data + trigger(&r, app_rtt)
            && t >= next_attempt
        {
            let k = out.attempts % r.endpoints;
            let busy = conns
                .iter()
                .position(|c| !c.dead && c.step != Step::Live && c.endpoint == k);
            let start = match busy {
                _ if r.unlocked => Some(t),
                None => Some(t.max(unlock_at[k as usize])),
                Some(b) => {
                    let (connecting, started) = (conns[b].step.connecting(), conns[b].started);
                    match r.abort_after {
                        Some(a) if connecting && t >= started + a => {
                            conns[b].dead = true;
                            unlock_at[k as usize] = t + UNLOCK_DELAY;
                            Some(t + UNLOCK_DELAY)
                        }
                        Some(a) if connecting => {
                            next_attempt = started + a;
                            None
                        }
                        // Past its 101 (its Join in flight) or never abandoned: wait for it.
                        _ => {
                            next_attempt = t + MS;
                            None
                        }
                    }
                }
            };
            if let Some(at) = start {
                out.attempts += 1;
                next_attempt = t + r.cadence;
                if at > t {
                    queued = Some((at, k));
                } else {
                    conns.push(Conn::new(
                        TcpModel::connect(p, seed, out.attempts, o.cfg, t),
                        Step::Connecting,
                        k,
                        t,
                    ));
                }
            }
        }
        if let Some(b) = back
            && conns
                .iter()
                .any(|c| !c.dead && c.fresh_end.is_some_and(|e| c.m.delivered(1) >= e))
        {
            out.recovery = Some(t - b);
            return out;
        }
    }
    if back.is_some() {
        panic!("seed {seed}: no fresh message within the run");
    }
    out
}

/// Set `k`'s phase `i`: its seed and burst start (S13 (e)'s golden-ratio spread over 5 s to 7 s, shifted by 1,000
/// phases per set; set 0 is S13 (e)'s).
fn phase_at(k: u64, i: u64) -> (u64, u64) {
    (
        1_000 * (k + 1) + i,
        5 * SEC + (((i + 1_000 * k) * 618_033_989) % SEC) * 2,
    )
}

/// One B1 phase: the recovery and the attempts.
fn phase(seed: u64, burst_ns: u64, o: Opts) -> Outcome {
    let p = Profile::b1(&[burst_ns]);
    let back = burst_ns + SEC;
    run(&p, seed, Some(back), back + 12 * SEC, o)
}

/// A row's 6 sets, each sorted, with the most attempts in any phase.
fn sweep(o: Opts) -> (Vec<Vec<u64>>, u32) {
    sweep_sets(o, SETS)
}

/// `n` sets of a row, each sorted, with the most attempts in any phase; one thread per set.
fn sweep_sets(o: Opts, n: u64) -> (Vec<Vec<u64>>, u32) {
    let sets: Vec<(Vec<u64>, u32)> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..n)
            .map(|k| {
                s.spawn(move || {
                    let mut r = Vec::with_capacity(PHASES as usize);
                    let mut most = 0;
                    for i in 0..PHASES {
                        let (seed, burst) = phase_at(k, i);
                        let out = phase(seed, burst, o);
                        r.push(out.recovery.unwrap_or(u64::MAX));
                        most = most.max(out.attempts);
                    }
                    r.sort_unstable();
                    (r, most)
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("a set's thread"))
            .collect()
    });
    let most = sets.iter().map(|s| s.1).max().unwrap_or(0);
    (sets.into_iter().map(|s| s.0).collect(), most)
}

fn rank(sorted: &[u64], permille: usize) -> u64 {
    sorted[(permille * sorted.len()).div_ceil(1000).max(1) - 1]
}

/// p50, p99 and maximum over every phase of a row.
fn pooled(sets: &[Vec<u64>]) -> (u64, u64, u64) {
    let mut all: Vec<u64> = sets.iter().flatten().copied().collect();
    all.sort_unstable();
    (
        rank(&all, 500),
        rank(&all, 990),
        all.last().copied().unwrap_or(u64::MAX),
    )
}

/// The browser-runnable reconnect: one endpoint, an attempt every `cadence`, one still connecting after `abort` closed.
fn serial(cadence: u64, abort: Option<u64>) -> Option<Reconnect> {
    Some(Reconnect {
        min_gap: 300 * MS,
        endpoints: 1,
        cadence,
        abort_after: abort,
        unlocked: false,
        max_attempts: 8,
    })
}

fn configs() -> Vec<(&'static str, Opts)> {
    let base = TcpConfig::default();
    let o = |cfg: TcpConfig, adaptive: bool, reconnect: Option<Reconnect>| Opts {
        cfg,
        adaptive,
        reconnect,
        full_state_counts: false,
    };
    let two = Some(Reconnect {
        min_gap: 300 * MS,
        endpoints: 2,
        cadence: 300 * MS,
        abort_after: Some(600 * MS),
        unlocked: false,
        max_attempts: 16,
    });
    let parallel = Some(Reconnect {
        min_gap: 300 * MS,
        endpoints: 1,
        cadence: 300 * MS,
        abort_after: None,
        unlocked: true,
        max_attempts: 8,
    });
    vec![
        ("today: Linux defaults, 64 KB budget", o(base, false, None)),
        ("fix 1: adaptive newest-only budget", o(base, true, None)),
        (
            "fix 1 + fix 3, one endpoint, every 500 ms, abort at 500 ms",
            o(base, true, serial(500 * MS, Some(500 * MS))),
        ),
        (
            "fix 1 + fix 3, one endpoint, every 600 ms, abort at 600 ms",
            o(base, true, serial(600 * MS, Some(600 * MS))),
        ),
        (
            "fix 1 + fix 3, one endpoint, never abort",
            o(base, true, serial(300 * MS, None)),
        ),
        (
            "fix 1 + fix 3, two endpoints, every 300 ms, abort at 600 ms",
            o(base, true, two),
        ),
        (
            "fix 1 + fix 3, one endpoint, abort at 1 s",
            o(base, true, serial(SEC, Some(SEC))),
        ),
        (
            "fix 1 + fix 3, parallel (no browser runs it: reference)",
            o(base, true, parallel),
        ),
    ]
}

/// The table for WS-FIX and WS-LIMIT (run: cargo test --release -p npt-net --lib fixes -- --ignored --nocapture).
#[test]
#[ignore]
fn ws_limit_table() {
    for (name, o) in configs() {
        let (sets, attempts) = sweep(o);
        report(name, &sets, attempts);
    }
    // Sensitivities of the browser-runnable rows: the full state counting as recovery, and a peer without
    // timestamps (Windows' default; MSS 1,460, Karn's rule alone).
    let mut o = configs()[2].1;
    o.full_state_counts = true;
    let (sets, attempts) = sweep(o);
    report("row 3, the full state counting", &sets, attempts);
    for row in [1, 2] {
        let mut o = configs()[row].1;
        o.cfg.timestamps = false;
        let (sets, attempts) = sweep(o);
        report(
            if row == 1 {
                "row 2 without timestamps (a Windows player)"
            } else {
                "row 3 without timestamps (a Windows player)"
            },
            &sets,
            attempts,
        );
    }
}

fn report(name: &str, sets: &[Vec<u64>], attempts: u32) {
    let (p50, p99, max) = pooled(sets);
    let all: Vec<u64> = sets.iter().flatten().copied().collect();
    let over = |x: u64| all.iter().filter(|&&v| v > x).count();
    let per_set: Vec<String> = sets
        .iter()
        .map(|s| {
            format!(
                "{}/{}/{}",
                rank(s, 500) / MS,
                rank(s, 990) / MS,
                s.last().copied().unwrap_or(0) / MS
            )
        })
        .collect();
    println!(
        "{name}: {} phases p50 {} ms, p99 {} ms, p99.9 {} ms, max {} ms; over 0.8 s {}, over 1 s {}, over 1.5 s {}; \
         most attempts in a phase {attempts}; per set p50/p99/max: {}",
        all.len(),
        p50 / MS,
        p99 / MS,
        {
            let mut a = all.clone();
            a.sort_unstable();
            rank(&a, 999) / MS
        },
        max / MS,
        over(800 * MS),
        over(SEC),
        over(1_500 * MS),
        per_set.join(", ")
    );
}

/// Without a burst, 10 minutes per seed: the snapshots the room skips (fix 1's cost), and the reconnect attempts and
/// session takeovers a silence starts (fix 3's false triggers).
#[test]
#[ignore]
fn side_effects_table() {
    let rows = [
        ("64 KB budget", configs()[0].1),
        ("adaptive budget", configs()[1].1),
        ("adaptive + reconnect, never abort", recommended()),
        ("adaptive + reconnect, abort at 1 s", configs()[6].1),
    ];
    for (label, o) in rows {
        for p in [Profile::p1(), Profile::p2(), Profile::p3(), Profile::p4()] {
            let total = (1..=3).fold(Outcome::default(), |acc, seed| {
                let r = run(&p, seed, None, 600 * SEC, o);
                Outcome {
                    recovery: None,
                    attempts: acc.attempts + r.attempts,
                    takeovers: acc.takeovers + r.takeovers,
                    written: acc.written + r.written,
                    ticks: acc.ticks + r.ticks,
                }
            });
            println!(
                "{} with the {label}: the room writes {} of {} snapshots ({} per mille skipped); in 30 minutes, {} \
                 reconnect attempts and {} takeovers",
                p.name,
                total.written,
                total.ticks,
                (total.ticks - total.written) * 1000 / total.ticks.max(1),
                total.attempts,
                total.takeovers
            );
        }
    }
}

/// The leading browser-runnable configuration: fix 1 and fix 3 with one endpoint, one attempt at a time, never
/// abandoned (the browser's own SYN retries carry it). Mustafa's goal (every recovery within 1.5 s, 99 % well under
/// 1 s) over 6,000 phases, pinned so the WS-LIMIT numbers stay tied to the code.
fn recommended() -> Opts {
    configs()[4].1
}

#[test]
fn the_recommended_fixes_meet_the_goal_at_p2() {
    let (sets, _) = sweep(recommended());
    let (p50, p99, max) = pooled(&sets);
    assert!(max <= 1_500 * MS && p99 < SEC, "p99 {p99} ns, max {max} ns");
    assert_eq!((p50, p99, max), PINNED_RECOMMENDED);
}

/// p50, p99 and maximum, in ns, of the recommended configuration's 6,000 phases.
const PINNED_RECOMMENDED: (u64, u64, u64) = (437_197_781, 664_931_012, 1_317_622_421);

/// P(at most `k` of `n` independent client-bursts exceed a bar each exceeds with probability `r`).
fn at_most(n: u64, k: u64, r: f64) -> f64 {
    let mut term = (1.0 - r).powi(n as i32);
    let mut sum = term;
    for i in 1..=k {
        term *= (n - i + 1) as f64 / i as f64 * r / (1.0 - r);
        sum += term;
    }
    sum
}

/// The tail, for setting WS-LIMIT: 60 sets (60,000 phases) of each candidate, and the chance that one proof-test run
/// (PM-0001-18: 48 clients x 10 bursts = 480 client-bursts, taken as independent) passes a two-part bar (at most 4
/// of 480 over the 99 % bar, none over the maximum) if the fallback behaves as the model does.
#[test]
#[ignore]
fn ws_limit_tail() {
    let no_ts = |row: usize| {
        let mut o = configs()[row].1;
        o.cfg.timestamps = false;
        o
    };
    let rows = [
        ("fix 1 alone", configs()[1].1),
        ("fix 1 + fix 3, one endpoint, never abort", configs()[4].1),
        (
            "fix 1 + fix 3, one endpoint, abort at 500 ms",
            configs()[2].1,
        ),
        ("fix 1 + fix 3, two endpoints", configs()[5].1),
        ("fix 1 + fix 3, one endpoint, abort at 1 s", configs()[6].1),
        ("fix 1 alone, no timestamps (Windows)", no_ts(1)),
        (
            "fix 1 + fix 3 never abort, no timestamps (Windows)",
            no_ts(4),
        ),
    ];
    for (name, o) in rows {
        let (sets, attempts) = sweep_sets(o, 60);
        let mut all: Vec<u64> = sets.into_iter().flatten().collect();
        all.sort_unstable();
        let n = all.len() as f64;
        let share = |x: u64| all.iter().filter(|&&v| v > x).count() as f64 / n;
        let counts: Vec<String> = [800, 1_000, 1_200, 1_500, 2_000]
            .iter()
            .map(|&ms| format!("{ms} ms: {}", all.iter().filter(|&&v| v > ms * MS).count()))
            .collect();
        println!(
            "{name}: {} phases, p50 {} ms, p99 {} ms, p99.9 {} ms, p99.99 {} ms, max {} ms; over {}; most attempts {attempts}",
            all.len(),
            rank(&all, 500) / MS,
            rank(&all, 990) / MS,
            rank(&all, 999) / MS,
            all[(all.len() * 9_999).div_ceil(10_000) - 1] / MS,
            all.last().copied().unwrap_or(0) / MS,
            counts.join(", ")
        );
        for (bar99, barmax) in [
            (800, 1_000),
            (800, 1_500),
            (1_000, 1_500),
            (800, 2_000),
            (1_000, 2_000),
        ] {
            let pass = at_most(480, 4, share(bar99 * MS)) * (1.0 - share(barmax * MS)).powi(480);
            println!(
                "  99 % within {bar99} ms and every one within {barmax} ms: a run passes with P = {:.3}",
                pass
            );
        }
    }
}
