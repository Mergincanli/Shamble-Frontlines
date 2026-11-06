//! The WebSocket fallback's three fixes (Mustafa, 2026-11-06), measured on the model: (1) the newest snapshot only,
//! with a small send budget; (2) thin-stream linear timeouts and a lower minimum RTO on the server; (3) the client
//! reconnecting after 300 ms of silence, one attempt every 300 ms in parallel, the session resumed on whichever
//! connection finishes first ("the newest connection for a player id wins", ADR 0001:507). Each configuration runs the
//! same 1,000 B1 phases as S13 (e); the side effects run without a burst at P1 to P4.

use super::*;

const MS: u64 = 1_000_000;
const SEC: u64 = 1_000_000_000;
const TICK: u64 = SEC / 30;
/// A snapshot: 1,024 B of payload, a 4 B WebSocket header and TLS 1.3's 22 B (S0).
const SNAP: u64 = 1_050;
/// A client input: 60 B of payload, a 6 B masked-frame header and TLS's 22 B.
const INPUT: u64 = 88;
/// The handshake's flights on a new connection (TLS 1.3 with a Let's Encrypt ECDSA chain, then the HTTP upgrade):
/// ClientHello; the server's ServerHello to Finished; the client's Finished with its upgrade request; the server's
/// 101; the client's Join (a resume, with a token held in advance); the server's JoinAccepted with the full state.
const CLIENT_HELLO: u64 = 517;
const SERVER_FLIGHT: u64 = 2_900;
const FINISHED_UPGRADE: u64 = 474;
const SWITCHING: u64 = 200;
const JOIN: u64 = 140;
const RESUME: u64 = 4_200;

#[derive(Debug, Clone, Copy)]
struct Opts {
    cfg: TcpConfig,
    /// The client's silence before its first reconnect attempt, and the spacing of the next ones.
    reconnect: Option<u64>,
    /// The room writes a snapshot only while what is outstanding fits about 1.5 round trips of snapshots (at least
    /// 3), read from the socket's RTT: newest only, never throttling a healthy flow.
    adaptive: bool,
    /// The reconnect trigger scales with the client's RTT: max(the gap, 2 x srtt + 100 ms).
    scaled_trigger: bool,
}

/// The adaptive budget: snapshots for 1.5 round trips plus one, at least 3.
fn adaptive_limit(m: &TcpModel) -> u64 {
    let srtt = m.srtt_ns(0).unwrap_or(100 * MS);
    let n = (3 * srtt).div_ceil(2 * TICK) + 1;
    n.max(3) * SNAP
}

/// The silence that starts a reconnect, from the client's own RTT estimate.
fn trigger(o: &Opts, gap: u64, m: &TcpModel) -> u64 {
    if o.scaled_trigger {
        gap.max(2 * m.srtt_ns(1).unwrap_or(100 * MS) + 100 * MS)
    } else {
        gap
    }
}

/// Where a reconnect attempt's script stands.
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

struct Conn {
    m: TcpModel,
    step: Step,
    /// Bytes each end still has to write (handshake flights; written as the budget frees).
    owed: [u64; 2],
    /// The end offset of the first message the server wrote here at or after the link's return.
    fresh_end: Option<u64>,
    delivered_seen: u64,
}

impl Conn {
    fn flush(&mut self, end: usize, t: u64) {
        if self.owed[end] > 0 && self.m.established(end) {
            let n = self.m.write(end, self.owed[end], t);
            self.owed[end] -= n;
        }
    }
}

/// One phase: a B1 burst at `burst_ns`; returns the time from the link's return to the first fresh message the
/// client has in order (on any connection), and the reconnect attempts started.
fn phase(seed: u64, burst_ns: u64, o: Opts) -> (u64, u32) {
    let p = Profile::b1(&[burst_ns]);
    let back = burst_ns + SEC;
    let until = back + 12 * SEC;
    let mut conns = vec![Conn {
        m: TcpModel::with_config(&p, seed, 0, o.cfg),
        step: Step::Live,
        owed: [0, 0],
        fresh_end: None,
        delivered_seen: 0,
    }];
    let (mut tick, mut input) = (TICK, TICK + (seed * 7_919) % TICK);
    let mut pending = false;
    let mut current = 0usize;
    let mut last_data = 0u64;
    let mut next_attempt = 0u64;
    let mut attempts = 0u32;
    loop {
        let mut next = tick.min(input);
        for c in &conns {
            if let Some(e) = c.m.next_event() {
                next = next.min(e);
            }
        }
        if let Some(gap) = o.reconnect
            && current == 0
            && attempts < 8
        {
            let quiet = trigger(&o, gap, &conns[0].m);
            next = next.min((last_data + quiet).max(next_attempt));
        }
        let t = next;
        if t > until {
            break;
        }
        for c in conns.iter_mut() {
            c.m.advance(t);
        }
        // The client: data seen, the scripts' next flights.
        for (i, c) in conns.iter_mut().enumerate() {
            let d = c.m.delivered(1);
            if d > c.delivered_seen {
                c.delivered_seen = d;
                if i == current || c.step == Step::Live {
                    last_data = t;
                }
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
                Step::Flight if c.m.delivered(1) >= SERVER_FLIGHT => {
                    c.owed[1] += FINISHED_UPGRADE;
                    c.step = Step::Upgrade;
                }
                Step::Upgrade if c.m.delivered(0) >= CLIENT_HELLO + FINISHED_UPGRADE => {
                    c.owed[0] += SWITCHING;
                    c.step = Step::Switching;
                }
                Step::Switching if c.m.delivered(1) >= SERVER_FLIGHT + SWITCHING => {
                    c.owed[1] += JOIN;
                    c.step = Step::Joined;
                }
                _ => {}
            }
            c.flush(0, t);
            c.flush(1, t);
        }
        // The server: a Join on a newer connection takes the session over; its first message is the full state.
        for (i, c) in conns.iter_mut().enumerate().skip(1) {
            if c.step == Step::Joined && c.m.delivered(0) >= CLIENT_HELLO + FINISHED_UPGRADE + JOIN
            {
                c.step = Step::Live;
                let before = c.m.ends[0].snd.write_seq + c.owed[0];
                c.owed[0] += RESUME;
                c.flush(0, t);
                if t >= back {
                    c.fresh_end = Some(before + RESUME);
                }
                current = i;
                pending = false;
            }
        }
        if t == tick {
            pending = true;
            tick += TICK;
        }
        let c = &mut conns[current];
        let fits = if o.adaptive {
            c.m.outstanding(0) + SNAP <= adaptive_limit(&c.m)
        } else {
            true
        };
        if pending && c.owed[0] == 0 && c.m.free_budget(0) >= SNAP && fits {
            c.m.write(0, SNAP, t);
            pending = false;
            if t >= back && c.fresh_end.is_none() {
                c.fresh_end = Some(c.m.ends[0].snd.write_seq);
            }
        }
        if t == input {
            c.m.write(1, INPUT, t);
            input += TICK;
        }
        // The reconnect rule: silence for `gap`, then one attempt every `gap`.
        if let Some(gap) = o.reconnect
            && current == 0
            && attempts < 8
            && t >= last_data + trigger(&o, gap, &conns[0].m)
            && t >= next_attempt
        {
            attempts += 1;
            conns.push(Conn {
                m: TcpModel::connect(&p, seed, attempts, o.cfg, t),
                step: Step::Connecting,
                owed: [0, 0],
                fresh_end: None,
                delivered_seen: 0,
            });
            next_attempt = t + gap;
        }
        if conns
            .iter()
            .any(|c| c.fresh_end.is_some_and(|e| c.m.delivered(1) >= e))
        {
            return (t - back, attempts);
        }
    }
    panic!("seed {seed}: no fresh message within 12 s of the return");
}

fn sweep(o: Opts) -> (Vec<u64>, u32) {
    let mut out = Vec::new();
    let mut attempts_max = 0;
    for i in 0..1_000u64 {
        let burst = 5 * SEC + ((i * 618_033_989) % SEC) * 2;
        let (r, a) = phase(1_000 + i, burst, o);
        out.push(r);
        attempts_max = attempts_max.max(a);
    }
    out.sort_unstable();
    (out, attempts_max)
}

fn rank(sorted: &[u64], permille: usize) -> u64 {
    sorted[(permille * sorted.len()).div_ceil(1000).max(1) - 1]
}

/// Three snapshots: the stream stays thin (fewer than 4 segments out).
const SMALL_BUDGET: u64 = 3 * SNAP;
/// Eight snapshots: room for a 30 Hz flow up to P4's round trip, a quarter of what 64 KB holds stale.
const MID_BUDGET: u64 = 8 * SNAP;

fn configs() -> Vec<(&'static str, Opts)> {
    let base = TcpConfig::default();
    let small = TcpConfig {
        write_budget: SMALL_BUDGET,
        ..base
    };
    let thin = TcpConfig {
        thin_linear_timeouts: true,
        ..small
    };
    let thin_100 = TcpConfig {
        rto_min_us: 100_000,
        ..thin
    };
    let thin_50 = TcpConfig {
        rto_min_us: 50_000,
        ..thin
    };
    let mid = TcpConfig {
        write_budget: MID_BUDGET,
        ..base
    };
    let o = |cfg: TcpConfig, reconnect: Option<u64>, adaptive: bool, scaled_trigger: bool| Opts {
        cfg,
        reconnect,
        adaptive,
        scaled_trigger,
    };
    let r = Some(300 * MS);
    vec![
        (
            "defaults (64 KB budget, doubling, rto_min 200 ms)",
            o(base, None, false, false),
        ),
        (
            "fix 1: newest only, 3-snapshot budget",
            o(small, None, false, false),
        ),
        (
            "fixes 1+2: + thin linear timeouts",
            o(thin, None, false, false),
        ),
        (
            "fixes 1+2: + thin linear timeouts, rto_min 100 ms",
            o(thin_100, None, false, false),
        ),
        (
            "fixes 1+2: + thin linear timeouts, rto_min 50 ms",
            o(thin_50, None, false, false),
        ),
        (
            "fixes 1+2+3: rto_min 200 ms, reconnect after 300 ms",
            o(thin, r, false, false),
        ),
        (
            "fixes 1+2+3: rto_min 100 ms, reconnect after 300 ms",
            o(thin_100, r, false, false),
        ),
        (
            "fix 3 alone: defaults, reconnect after 300 ms",
            o(base, r, false, false),
        ),
        (
            "fix 3 with an 8-snapshot budget, reconnect after 300 ms",
            o(mid, r, false, false),
        ),
        (
            "adaptive budget (1.5 round trips), no reconnect",
            o(base, None, true, false),
        ),
        (
            "adaptive budget + thin linear timeouts, no reconnect",
            o(
                TcpConfig {
                    thin_linear_timeouts: true,
                    ..base
                },
                None,
                true,
                false,
            ),
        ),
        (
            "adaptive budget + reconnect after max(300 ms, 2 x RTT + 100 ms)",
            o(base, r, true, true),
        ),
    ]
}

/// The recommended fixes (WS-FIX): the adaptive newest-only budget and the client's reconnect after
/// max(300 ms, 2 x RTT + 100 ms). Mustafa's goal: every recovery within 1.5 s, 99 % well under 1 s. Pinned, so the
/// WS-LIMIT numbers stay tied to the code.
#[test]
fn the_recommended_fixes_meet_the_goal_at_p2() {
    let o = Opts {
        cfg: TcpConfig::default(),
        reconnect: Some(300 * MS),
        adaptive: true,
        scaled_trigger: true,
    };
    let (r, _) = sweep(o);
    let (p50, p99, max) = (
        rank(&r, 500),
        rank(&r, 990),
        r.last().copied().unwrap_or(u64::MAX),
    );
    assert!(max <= 1_500 * MS && p99 < SEC, "p99 {p99} ns, max {max} ns");
    assert_eq!((p50, p99, max), PINNED_RECOMMENDED);
}

/// p50, p99 and maximum, in ns, of the recommended fixes' 1,000 phases.
const PINNED_RECOMMENDED: (u64, u64, u64) = (399_205_920, 610_751_158, 737_585_182);

/// The table for WS-LIMIT (run: cargo test -p npt-net --lib fixes -- --ignored --nocapture).
#[test]
#[ignore]
fn ws_limit_table() {
    for (name, o) in configs() {
        let (r, attempts) = sweep(o);
        let over = |x: u64| r.iter().filter(|&&v| v > x).count();
        println!(
            "{name}: p50 {} ms, p99 {} ms, max {} ms; over 1 s {}, over 1.5 s {}; reconnect attempts per phase, most {}",
            rank(&r, 500) / MS,
            rank(&r, 990) / MS,
            r.last().copied().unwrap_or(0) / MS,
            over(SEC),
            over(1_500 * MS),
            attempts
        );
    }
}

/// Without a burst, 60 s: the snapshots the room could write (fix 1's cost), and the client's silences of 300 ms or
/// more, each of which would start a reconnect (fix 3's false triggers).
fn snapshot_share(
    p: &Profile,
    cfg: TcpConfig,
    seed: u64,
    adaptive: bool,
    scaled: bool,
) -> (u64, u64, u64) {
    let o = Opts {
        cfg,
        reconnect: Some(300 * MS),
        adaptive,
        scaled_trigger: scaled,
    };
    let mut m = TcpModel::with_config(p, seed, 0, cfg);
    let (mut written, mut ticks, mut silences) = (0, 0, 0);
    let (mut last_seen, mut last_data, mut in_silence) = (0u64, TICK, false);
    let mut t = TICK;
    while t <= 60 * SEC {
        m.advance(t);
        let d = m.delivered(1);
        if d > last_seen {
            last_seen = d;
            last_data = t;
            in_silence = false;
        } else if t >= last_data + trigger(&o, 300 * MS, &m) && !in_silence {
            silences += 1;
            in_silence = true;
        }
        ticks += 1;
        if m.free_budget(0) >= SNAP && (!adaptive || m.outstanding(0) + SNAP <= adaptive_limit(&m))
        {
            m.write(0, SNAP, t);
            written += 1;
        }
        m.write(1, INPUT, t + TICK / 2);
        t += TICK;
    }
    (written, ticks, silences)
}

#[test]
#[ignore]
fn side_effects_table() {
    for (label, budget, adaptive, scaled) in [
        ("3-snapshot", SMALL_BUDGET, false, false),
        ("8-snapshot", MID_BUDGET, false, false),
        ("64 KB", WRITE_BUDGET, false, false),
        ("adaptive (scaled trigger)", WRITE_BUDGET, true, true),
    ] {
        let cfg = TcpConfig {
            write_budget: budget,
            thin_linear_timeouts: true,
            ..TcpConfig::default()
        };
        for p in [Profile::p1(), Profile::p2(), Profile::p3(), Profile::p4()] {
            let (w, n, s) = (1..=3).fold((0, 0, 0), |(w, n, s), seed| {
                let (a, b, c) = snapshot_share(&p, cfg, seed, adaptive, scaled);
                (w + a, n + b, s + c)
            });
            println!(
                "{} with the {label} budget: the room writes {w} of {n} snapshots ({} per mille skipped); silences of \
                 300 ms or more (false reconnect triggers) in 3 minutes: {s}",
                p.name,
                (n - w) * 1000 / n
            );
        }
    }
}
