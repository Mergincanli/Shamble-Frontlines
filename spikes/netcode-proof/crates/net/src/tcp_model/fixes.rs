//! The WebSocket fallback's fixes (Mustafa, 2026-11-06), measured on the model as a browser and a tick-driven room
//! can run them. Harness v3, after the second skeptic round (2026-11-07: .checkpoints/netcode-proof-test/
//! u3-verify2-harness.md H1-H20 and u3-verify2-stats.md B1-B4):
//! - fix 1: the room writes a snapshot only while what is outstanding (unsent plus unacknowledged, SIOCOUTQ) leaves
//!   room for it within ceil(1.5 x srtt / tick) + 1 snapshots, at least 3 (srtt from TCP_INFO): newest only.
//! - fix 2 (thin-stream linear timeouts, a lower minimum RTO): no rows; inert beside fix 1, whose stream is not thin
//!   (more than 3 segments out at P2), and a lower minimum RTO there only shortens doubled timeouts.
//! - fix 3: the client reconnects after a silence of max(300 ms, 2 x its own RTT + 100 ms) between whole messages
//!   (H19), its RTT measured from the input echo (a page cannot read TCP's). One attempt at a time per endpoint: an
//!   attempt blocks the next until the client has switched to it or closed it (H7, what the page sees). At most 8
//!   attempts per outage, counted again from each whole snapshot on the current connection (H8). An attempt still
//!   before its 101 is closed as soon as the current connection delivers a whole snapshot the room wrote at or after
//!   the attempt was created (H6: the old connection recovered); after its 101 it runs to its Join, and the newest Join
//!   takes the session over (ADR 0001:507).
//! - The browser's own rules, Chromium by default (the endpoint lock and the per-renderer throttler) and Firefox as an
//!   option (one admission queue per IP, the fail delay, the open timeout): H1-H4. A reconnect resumes its TLS session
//!   by default (H10).
//!
//! The room writes snapshots and handles Joins on its 30 Hz tick. A burst's recovery ends when the client decodes
//! deltas again (PM-0001-18, ADR 0001:1556): the first snapshot written at or after the link's return, on a new
//! connection the first delta after the full state. A phase is one B1 burst; after its recovery it runs on until no
//! attempt is pending, at most 3 s, so the takeovers that follow a recovery are counted (H6). A multi-burst run is one
//! client through N bursts S apart on one connection lineage, so the browser's counts, the per-outage cap and TCP's
//! state carry from burst to burst. The side effects run the same harness without a burst at P1 to P4.

use super::*;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};

const MS: u64 = 1_000_000;
const SEC: u64 = 1_000_000_000;
const TICK: u64 = SEC / 30;
/// A snapshot: 1,024 B of payload, a 4 B WebSocket header and TLS 1.3's 22 B (S0).
const SNAP: u64 = 1_050;
/// A client input: 60 B of payload, a 6 B masked-frame header and TLS's 22 B.
const INPUT: u64 = 88;
/// After the TLS flights (Tls): the client's Finished with its upgrade request; the server's 101; the client's Join (a
/// resume, with a token held in advance: RESUME-CRED); the server's JoinAccepted with the full state.
const FINISHED_UPGRADE: u64 = 474;
const SWITCHING: u64 = 200;
const JOIN: u64 = 140;
const RESUME: u64 = 4_200;
/// Chromium frees a WebSocket endpoint's lock 10 ms after the attempt holding it opens or ends
/// (websocket_endpoint_lock_manager.cc:142-159, kUnlockDelayInMs; H1).
const UNLOCK_DELAY: u64 = 10 * MS;
/// Chromium's throttler rolls its counts every 2 minutes (services/network/websocket_throttler.cc:124-127; H3).
const THROTTLE_PERIOD: u64 = 120 * SEC;
/// Firefox: a connection not open 20 s after it began fails (network.websocket.timeout.open; H4).
const FF_OPEN_TIMEOUT: u64 = 20 * SEC;
/// Firefox's fail-delay record expires 60 s plus its delay after the last failure; the delay grows to at most 60 s.
const FF_LIFETIME_MS: u64 = 60_000;
const FF_MAX_DELAY_MS: u64 = 60_000;
/// After the recovery a phase runs on until no attempt is pending, at most this long (H6's takeovers).
const FOLLOW: u64 = 3 * SEC;
/// The phases of a set; the sets of the pinned test, of the table's rows, and of the tail's rows.
const PHASES: u64 = 1_000;
const SETS: u64 = 6;
const TABLE_SETS: u64 = 60;
const TAIL_SETS: u64 = 600;
/// Bootstrap replicates behind a pass chance's interval.
const BOOT: usize = 2_000;

#[derive(Debug, Clone, Copy)]
struct Opts {
    cfg: TcpConfig,
    /// Fix 1: the adaptive newest-only budget (else every snapshot is written while the write budget has room).
    adaptive: bool,
    reconnect: Option<Reconnect>,
    /// A reconnect's full state counts as decoding again (else the first delta after it, PM-0001-18's wording).
    full_state_counts: bool,
    browser: Browser,
    tls: Tls,
    client_os: ClientOs,
    /// The adaptive budget's multiplier: 1 is fix 1 as specified; 2 is a bite (a budget twice too large).
    budget_scale: u64,
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
    /// An attempt still before its 101 this long after the page created it is closed when the next is due.
    abort_after: Option<u64>,
    /// The most attempts in one outage (counted again from each whole snapshot on the current connection).
    max_attempts: u32,
}

/// The browser whose rules the page's attempts run under (H1-H4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Browser {
    /// Chromium (the default): the per-renderer throttler delays each new WebSocket; then the endpoint lock (one
    /// connecting attempt per IP and port) until 10 ms after the 101 or the attempt's end. Its 240 s handshake and
    /// 4-minute connect timeouts never bind here.
    Chromium,
    /// Firefox: one connecting attempt per IP (any port); a failed or page-closed attempt still before its 101 leaves
    /// a fail-delay record that holds the next attempt to that endpoint; an attempt not open 20 s after it began fails.
    Firefox,
    /// No browser rules: attempts begin at once and run in parallel (the first version's reference row; no browser
    /// runs it).
    Unbound,
}

/// A reconnect's TLS 1.3 flights (H10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tls {
    /// Session resumption, the default: the browser kept the ticket the server sent on its first connection (rustls
    /// servers send TLS 1.3 tickets by default). A ClientHello with a PSK and an X25519MLKEM768 key share (about
    /// 1.9 KB, two segments); the server's ServerHello (the ML-KEM ciphertext), EncryptedExtensions and Finished, no
    /// certificate (about 1.3 KB, one segment).
    Resumed,
    /// A full handshake: the ClientHello without a PSK (about 1.8 KB); ServerHello to Finished with the ML-KEM
    /// ciphertext and a Let's Encrypt ECDSA chain (about 3.8 KB, three segments).
    Full,
}

impl Tls {
    fn client_hello(self) -> u64 {
        match self {
            Tls::Resumed => 1_900,
            Tls::Full => 1_800,
        }
    }

    fn server_flight(self) -> u64 {
        match self {
            Tls::Resumed => 1_300,
            Tls::Full => 3_800,
        }
    }
}

/// The client's operating system, for a reconnect's SYN timer. The slot for the Windows-like option (H11): once the
/// model build adds the client SYN knob to TcpConfig, `client_cfg`'s Windows arm sets it (doubling from the initial
/// RTO) with the timestamp option off (Windows offers none by default), and the tables gain their Windows rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClientOs {
    /// Linux: the SYN resent after 1 s, linear for 4 tries, then doubling (the model's own handshake).
    Linux,
    /// Windows-like: SYN retries doubling from `initial_rto_ms` (Get-NetTCPSetting's InitialRtoMs: 1,000 or 3,000 as
    /// the skeptic ran them), no timestamps.
    Windows { initial_rto_ms: u64 },
}

/// A connection's settings on the client's operating system: the established one and each reconnect attempt. The
/// model gives both ends one TcpConfig, so a Windows-like client turns the timestamp option off for the connection
/// (Windows offers none by default) and sets the client's SYN timer (the model's syn_rto_us, no linear retries).
fn client_cfg(o: &Opts) -> TcpConfig {
    match o.client_os {
        ClientOs::Linux => o.cfg,
        ClientOs::Windows { initial_rto_ms } => TcpConfig {
            timestamps: false,
            syn_rto_us: initial_rto_ms * 1_000,
            syn_linear_timeouts: 0,
            ..o.cfg
        },
    }
}

/// The adaptive budget: snapshots for 1.5 round trips plus one, at least 3 (100 ms before the first sample), through
/// the one shipped definition the WebSocket server's gate uses too (`ws_gate::snapshot_budget`; u3c-plan.md 1.3 (e)).
fn adaptive_limit(m: &TcpModel) -> u64 {
    crate::ws_gate::snapshot_budget(m.srtt_ns(0), TICK) * SNAP
}

/// The silence that starts a reconnect, from the client's own RTT (100 ms before its first sample).
fn trigger(r: &Reconnect, app_rtt: Option<u64>) -> u64 {
    r.min_gap.max(2 * app_rtt.unwrap_or(100 * MS) + 100 * MS)
}

/// SplitMix64's finaliser (Steele, Lea and Flood, 2014): a seed's hash.
fn splitmix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// A seeded stream apart from the model's fate streams: the browser's random draws, the bootstrap's.
struct SplitMix(u64);

impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        splitmix64(self.0.wrapping_sub(0x9E37_79B9_7F4A_7C15))
    }
}

/// Chromium's per-renderer WebSocket throttler (services/network/websocket_throttler.cc, read 2026-11-07): each new
/// WebSocket waits RandIntInclusive(1000, 5000) x 2^min(p + f / (s + 1), 16) / 65536 ms in int arithmetic, so whole
/// milliseconds and 0 until the exponent reaches 4 (:46-55); p pending, f failed (a tracker destroyed before its
/// handshake completed, :27-33), s succeeded (the 101: websocket.cc:371-374), f and s over the current and the
/// previous 2-minute period (Roll, :70-76). The factory passes IssuePendingConnectionTracker and CalculateDelay as
/// arguments (websocket_factory.cc:99-100); clang evaluates them right to left under the Microsoft ABI, so on Windows
/// the delay is computed first: p leaves out the new attempt, and the first WebSocket finds no throttler (0 ms).
/// Elsewhere p counts it (one more doubling).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Throttler {
    exists: bool,
    pending: u32,
    /// The previous and the current period.
    failed: [u32; 2],
    succeeded: [u32; 2],
    next_roll: Option<u64>,
}

impl Throttler {
    /// The periods rolled up to `t`; a throttler left clean is erased, and with none left the timer stops
    /// (:131-144). IsClean (:62-68) tests the previous successes twice and the previous failures never.
    fn roll_to(&mut self, t: u64) {
        while let Some(r) = self.next_roll
            && t >= r
        {
            self.failed = [self.failed[1], 0];
            self.succeeded = [self.succeeded[1], 0];
            if self.pending == 0 && self.succeeded == [0, 0] && self.failed[1] == 0 {
                *self = Throttler::default();
            } else {
                self.next_roll = Some(r + THROTTLE_PERIOD);
            }
        }
    }

    /// A new WebSocket at `t`: its delay (from `draw` when the throttler exists), then its pending tracker.
    fn issue(&mut self, t: u64, draw: impl FnOnce() -> u64) -> u64 {
        self.roll_to(t);
        let delay_ms = if self.exists {
            let f = u64::from(self.failed[0] + self.failed[1]);
            let s = u64::from(self.succeeded[0] + self.succeeded[1]);
            let e = (u64::from(self.pending) + f / (s + 1)).min(16);
            (1_000 + below(draw(), 4_001)) * (1 << e) / 65_536
        } else {
            0
        };
        self.exists = true;
        if self.next_roll.is_none() {
            self.next_roll = Some(t + THROTTLE_PERIOD);
        }
        self.pending += 1;
        delay_ms * MS
    }

    /// A tracker's end at `t`: its handshake completed (`ok`) or it was destroyed before.
    fn done(&mut self, t: u64, ok: bool) {
        self.roll_to(t);
        self.pending -= 1;
        if ok {
            self.succeeded[1] += 1;
        } else {
            self.failed[1] += 1;
        }
    }
}

/// Firefox's fail-delay record for one endpoint (WebSocketChannel.cpp's FailDelay, :113-160).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FailRecord {
    last: u64,
    next_ms: u64,
}

impl FailRecord {
    fn expired(&self, t: u64) -> bool {
        t >= self.last + (FF_LIFETIME_MS + self.next_ms) * MS
    }
}

/// The browser's state across the page's attempts.
struct Gate {
    browser: Browser,
    /// Chromium: when each endpoint's lock is free again after its holder opened or ended.
    unlock_at: [u64; 2],
    throttler: Throttler,
    /// Firefox: each endpoint's fail-delay record.
    fail: [Option<FailRecord>; 2],
    rng: SplitMix,
}

impl Gate {
    /// The page's first WebSocket opened at 0 (Chromium: one success in the current period, the timer running).
    fn new(browser: Browser, seed: u64) -> Gate {
        let mut throttler = Throttler::default();
        if browser == Browser::Chromium {
            throttler.issue(0, || 0);
            throttler.done(0, true);
        }
        Gate {
            browser,
            unlock_at: [0; 2],
            throttler,
            fail: [None; 2],
            rng: SplitMix(splitmix64(seed ^ 0x6272_6f77_7365_7221)),
        }
    }

    /// The page creates a WebSocket at `t`: how long Chromium's throttler holds it.
    fn create(&mut self, t: u64) -> u64 {
        match self.browser {
            Browser::Chromium => {
                let rng = &mut self.rng;
                self.throttler.issue(t, || rng.next())
            }
            Browser::Firefox | Browser::Unbound => 0,
        }
    }

    /// An attempt on endpoint `k` saw its 101 at `t`.
    fn opened(&mut self, t: u64, k: u32) {
        match self.browser {
            Browser::Chromium => {
                self.unlock_at[k as usize] = t + UNLOCK_DELAY;
                self.throttler.done(t, true);
            }
            // OnConnected removes the endpoint's record and every expired one (:376-379, Remove :268-281).
            Browser::Firefox => {
                for (e, r) in self.fail.iter_mut().enumerate() {
                    if e == k as usize || r.is_some_and(|r| r.expired(t)) {
                        *r = None;
                    }
                }
            }
            Browser::Unbound => {}
        }
    }

    /// An attempt on endpoint `k` ended before its 101 at `t`: closed by the page (`failure` false; Firefox stops it
    /// with NS_ERROR_NOT_CONNECTED) or failed. `began`: it had started its TCP connect (held Chromium's lock).
    fn stopped(&mut self, t: u64, k: u32, began: bool, failure: bool) {
        match self.browser {
            Browser::Chromium => {
                if began {
                    self.unlock_at[k as usize] = t + UNLOCK_DELAY;
                }
                self.throttler.done(t, false);
            }
            // OnStopSession (:389-421): a known record (found even when expired) grows x1.5 on a failure and stays as
            // it is on a page close; with none, either adds one of 200-400 ms.
            Browser::Firefox => match &mut self.fail[k as usize] {
                Some(r) => {
                    if failure {
                        r.last = t;
                        r.next_ms = (r.next_ms * 3 / 2).min(FF_MAX_DELAY_MS);
                    }
                }
                None => {
                    let next_ms = 200 + below(self.rng.next(), 200);
                    self.fail[k as usize] = Some(FailRecord { last: t, next_ms });
                }
            },
            Browser::Unbound => {}
        }
    }

    /// Firefox's DelayOrBegin (:232-266) for endpoint `k` at `t`: how long its record still holds the attempt (whole
    /// milliseconds since the last failure); an expired record that no longer holds it is removed.
    fn fail_delay(&mut self, t: u64, k: u32) -> u64 {
        let Some(r) = self.fail[k as usize] else {
            return 0;
        };
        let since_ms = (t - r.last) / MS;
        if since_ms < r.next_ms {
            return (r.next_ms - since_ms) * MS;
        }
        if r.expired(t) {
            self.fail[k as usize] = None;
        }
        0
    }
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
    /// Before the 101: the page sees CONNECTING and the browser holds the endpoint's lock or admission slot.
    fn connecting(self) -> bool {
        !matches!(self, Step::Joined | Step::Live)
    }
}

/// A message the room wrote: its end offset down, the up-stream bytes the room had then (the input echo), and when.
#[derive(Debug, Clone, Copy)]
struct Snap {
    end: u64,
    acked: u64,
    written: u64,
}

struct Conn {
    m: TcpModel,
    step: Step,
    endpoint: u32,
    /// When the page created the WebSocket, and when its TCP connect began.
    created: u64,
    began: u64,
    /// Closed (by the page, or failed).
    dead: bool,
    /// Bytes each end still has to write (handshake flights and the full state; written as the budget frees).
    owed: [u64; 2],
    /// The full state's end offset and write time, once the room took the session over here.
    resume: Option<(u64, u64)>,
    /// The end offset of the first message here that counts as decoding again.
    fresh_end: Option<u64>,
    /// The client's inputs here (end offset up, write time) and the room's messages: the input echo behind the
    /// client's RTT, and the whole messages the page sees.
    inputs: VecDeque<(u64, u64)>,
    snaps: VecDeque<Snap>,
}

impl Conn {
    fn new(m: TcpModel, step: Step, endpoint: u32, created: u64) -> Conn {
        Conn {
            m,
            step,
            endpoint,
            created,
            began: created,
            dead: false,
            owed: [0, 0],
            resume: None,
            fresh_end: None,
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

/// A WebSocket the page created that has not begun its TCP connect: held by Chromium's throttler or endpoint lock,
/// or by Firefox's admission queue or fail delay.
#[derive(Debug, Clone, Copy)]
struct Waiting {
    created: u64,
    endpoint: u32,
    /// Chromium: the throttler's delay ends; Firefox: the fail delay ends (once `delayed`).
    not_before: u64,
    delayed: bool,
}

/// What one burst did: from its link's return to decoding again, and the attempts of its window (its start to the
/// next burst's).
#[derive(Debug, Clone, Copy, Default)]
struct BurstOut {
    recovery: Option<u64>,
    attempts: u32,
    /// Sessions taken over before and after the recovery.
    takeovers: u32,
    takeovers_after: u32,
    /// Attempts the page closed because the old connection recovered (H6).
    cancels: u32,
    /// The longest an attempt waited between its creation and its TCP connect, and the longest throttler delay.
    wait_max: u64,
    throttle_max: u64,
    /// The per-outage cap was reached.
    capped: bool,
}

/// What one run did.
#[derive(Debug, Clone, Default)]
struct Outcome {
    bursts: Vec<BurstOut>,
    attempts: u32,
    takeovers: u32,
    cancels: u32,
    aborts: u32,
    timeouts: u32,
    written: u64,
    ticks: u64,
    /// An attempt was still pending when the run stopped, 3 s after the last recovery.
    pending_at_end: bool,
}

/// Which outstanding attempt blocks an endpoint.
#[derive(Debug, Clone, Copy)]
enum Which {
    Waiting(usize),
    Conn(usize),
}

/// One client and its room.
struct Sim<'a> {
    p: &'a Profile,
    seed: u64,
    o: Opts,
    conns: Vec<Conn>,
    waiting: Vec<Waiting>,
    gate: Gate,
    client_cur: usize,
    server_cur: usize,
    /// The last whole message on the current connection, and the newest write time among them.
    last_data: u64,
    newest_whole: u64,
    app_rtt: Option<u64>,
    next_attempt: u64,
    /// Attempts in this outage, in the run, and connections begun (each its own fate stream).
    outage: u32,
    made: u32,
    conn_no: u32,
    /// The bursts' starts and returns; the latest burst started, and the burst whose recovery is awaited.
    starts: Vec<u64>,
    backs: Vec<u64>,
    win: Option<usize>,
    meas: usize,
    out: Outcome,
}

impl<'a> Sim<'a> {
    fn new(p: &'a Profile, seed: u64, o: Opts) -> Sim<'a> {
        let starts: Vec<u64> = p.bursts_ns.iter().map(|b| b.0).collect();
        let backs: Vec<u64> = p.bursts_ns.iter().map(|b| b.0 + b.1).collect();
        Sim {
            p,
            seed,
            o,
            conns: vec![Conn::new(
                TcpModel::with_config(p, seed, 0, client_cfg(&o)),
                Step::Live,
                0,
                0,
            )],
            waiting: Vec::new(),
            gate: Gate::new(o.browser, seed),
            client_cur: 0,
            server_cur: 0,
            last_data: 0,
            newest_whole: 0,
            app_rtt: None,
            next_attempt: 0,
            outage: 0,
            made: 0,
            conn_no: 0,
            out: Outcome {
                bursts: vec![BurstOut::default(); starts.len()],
                ..Outcome::default()
            },
            starts,
            backs,
            win: None,
            meas: 0,
        }
    }

    /// This burst window's record (none before the first burst).
    fn slot(&mut self) -> Option<&mut BurstOut> {
        let w = self.win?;
        Some(&mut self.out.bursts[w])
    }

    /// Whether the current window's burst has recovered.
    fn after_recovery(&self) -> bool {
        self.win.is_some_and(|w| self.meas > w)
    }

    /// An attempt still before its 101 on endpoint `k` (any endpoint: `None`).
    fn connecting_on(&self, k: Option<u32>) -> bool {
        self.conns.iter().enumerate().any(|(i, c)| {
            i != self.client_cur
                && !c.dead
                && c.step.connecting()
                && k.is_none_or(|k| c.endpoint == k)
        })
    }

    /// Anything the page has not switched to or closed.
    fn pending(&self) -> bool {
        !self.waiting.is_empty()
            || self
                .conns
                .iter()
                .enumerate()
                .any(|(i, c)| i != self.client_cur && !c.dead)
    }

    /// The outstanding attempt on endpoint `k` as the page sees it (H7): its creation, whether it is before its 101,
    /// and which it is.
    fn blocker(&self, k: u32) -> Option<(u64, bool, Which)> {
        if let Some(i) = self.waiting.iter().position(|w| w.endpoint == k) {
            return Some((self.waiting[i].created, true, Which::Waiting(i)));
        }
        self.conns
            .iter()
            .enumerate()
            .find(|(i, c)| *i != self.client_cur && !c.dead && c.endpoint == k)
            .map(|(i, c)| (c.created, c.step.connecting(), Which::Conn(i)))
    }

    /// The page closes connection `j`; one still before its 101 ends as an attempt (`failure`: it failed instead).
    fn close_conn(&mut self, j: usize, t: u64, failure: bool) {
        let c = &mut self.conns[j];
        let connecting = c.step.connecting();
        let k = c.endpoint;
        c.dead = true;
        if connecting {
            self.gate.stopped(t, k, true, failure);
        }
    }

    /// The client: whole messages on each connection, the input echo, each script's next flight.
    fn client(&mut self, t: u64) {
        let (ch, sf) = (self.o.tls.client_hello(), self.o.tls.server_flight());
        for i in 0..self.conns.len() {
            let c = &mut self.conns[i];
            if c.dead {
                continue;
            }
            let d = c.m.delivered(1);
            let (mut echoed, mut whole) = (None, None);
            while let Some(&s) = c.snaps.front()
                && d >= s.end
            {
                c.snaps.pop_front();
                whole = Some(s.written);
                while c.inputs.front().is_some_and(|&(e, _)| e <= s.acked) {
                    echoed = c.inputs.pop_front();
                }
            }
            let mut opened = false;
            match c.step {
                Step::Connecting if c.m.established(1) => {
                    c.owed[1] += ch;
                    c.step = Step::Hello;
                }
                Step::Hello if c.m.delivered(0) >= ch => {
                    c.owed[0] += sf;
                    c.step = Step::Flight;
                }
                Step::Flight if d >= sf => {
                    c.owed[1] += FINISHED_UPGRADE;
                    c.step = Step::Upgrade;
                }
                Step::Upgrade if c.m.delivered(0) >= ch + FINISHED_UPGRADE => {
                    c.owed[0] += SWITCHING;
                    c.step = Step::Switching;
                }
                Step::Switching if d >= sf + SWITCHING => {
                    // The 101: the page sees open and sends its Join; the browser frees the endpoint.
                    c.owed[1] += JOIN;
                    c.step = Step::Joined;
                    opened = true;
                }
                _ => {}
            }
            c.flush(0, t);
            c.flush(1, t);
            let k = c.endpoint;
            if let Some((_, at)) = echoed {
                let s = t - at;
                self.app_rtt = Some(self.app_rtt.map_or(s, |r| r - r / 8 + s / 8));
            }
            if i == self.client_cur
                && let Some(w) = whole
            {
                self.last_data = t;
                self.outage = 0;
                self.newest_whole = self.newest_whole.max(w);
            }
            if opened {
                self.gate.opened(t, k);
            }
        }
    }

    /// The client switches once the full state has arrived on another connection: it closes every other.
    fn switch(&mut self, t: u64) {
        let cur = self.client_cur;
        let Some(i) = (0..self.conns.len()).rev().find(|&i| {
            let c = &self.conns[i];
            i != cur && !c.dead && c.resume.is_some_and(|(e, _)| c.m.delivered(1) >= e)
        }) else {
            return;
        };
        self.client_cur = i;
        self.last_data = t;
        self.outage = 0;
        self.newest_whole = self.conns[i].resume.map_or(0, |r| r.1);
        for j in 0..self.conns.len() {
            if j != i && !self.conns[j].dead {
                self.close_conn(j, t, false);
            }
        }
        for w in std::mem::take(&mut self.waiting) {
            self.gate.stopped(t, w.endpoint, false, false);
        }
    }

    /// H6: the page closes each attempt still before its 101 once the current connection has delivered a whole
    /// snapshot the room wrote at or after the attempt's creation.
    fn cancel(&mut self, t: u64) {
        let nw = self.newest_whole;
        let mut n = 0;
        let mut kept = Vec::new();
        for w in std::mem::take(&mut self.waiting) {
            if w.created <= nw {
                self.gate.stopped(t, w.endpoint, false, false);
                n += 1;
            } else {
                kept.push(w);
            }
        }
        self.waiting = kept;
        for j in 0..self.conns.len() {
            let c = &self.conns[j];
            if j != self.client_cur && !c.dead && c.step.connecting() && c.created <= nw {
                self.close_conn(j, t, false);
                n += 1;
            }
        }
        if n > 0 {
            self.out.cancels += n;
            if let Some(b) = self.slot() {
                b.cancels += n;
            }
        }
    }

    /// The room's tick: the newest Join takes the session over (the older connection is no longer written); its first
    /// message is the full state. Then the tick's snapshot, newest only.
    fn room(&mut self, t: u64) {
        self.out.ticks += 1;
        let joined = self.o.tls.client_hello() + FINISHED_UPGRADE + JOIN;
        let after_back = self.meas < self.backs.len() && t >= self.backs[self.meas];
        let mut takeovers = 0;
        for i in 0..self.conns.len() {
            let c = &mut self.conns[i];
            if !c.dead && c.step == Step::Joined && c.m.delivered(0) >= joined {
                c.step = Step::Live;
                let before = c.m.ends[0].snd.write_seq + c.owed[0];
                c.owed[0] += RESUME;
                c.flush(0, t);
                c.resume = Some((before + RESUME, t));
                c.snaps.push_back(Snap {
                    end: before + RESUME,
                    acked: c.m.delivered(0),
                    written: t,
                });
                if self.o.full_state_counts && after_back {
                    c.fresh_end = Some(before + RESUME);
                }
                self.server_cur = i;
                takeovers += 1;
            }
        }
        if takeovers > 0 {
            self.out.takeovers += takeovers;
            let after = self.after_recovery();
            if let Some(b) = self.slot() {
                if after {
                    b.takeovers_after += takeovers;
                } else {
                    b.takeovers += takeovers;
                }
            }
        }
        let (adaptive, scale) = (self.o.adaptive, self.o.budget_scale);
        let c = &mut self.conns[self.server_cur];
        // Fix 1's gate is the shipped one (W22's three conditions in ws_gate::may_write, N from snapshot_budget), so the
        // harness and the WebSocket server cannot drift (the critic's W7); without the adaptive limit the budget
        // condition is off (an unbounded N). The room's own rule stays here: no snapshot on a tick with a takeover.
        let n = if adaptive {
            scale * crate::ws_gate::snapshot_budget(c.m.srtt_ns(0), TICK)
        } else {
            u64::MAX
        };
        if takeovers == 0
            && crate::ws_gate::may_write(c.m.outstanding(0), c.owed[0], c.m.free_budget(0), SNAP, n)
        {
            c.m.write(0, SNAP, t);
            self.out.written += 1;
            let end = c.m.ends[0].snd.write_seq;
            c.snaps.push_back(Snap {
                end,
                acked: c.m.delivered(0),
                written: t,
            });
            if after_back && c.fresh_end.is_none() {
                c.fresh_end = Some(end);
            }
        }
    }

    /// When the next attempt may start, if one can.
    fn attempt_due(&self) -> Option<u64> {
        let r = self.o.reconnect?;
        if self.outage >= r.max_attempts {
            return None;
        }
        let due = (self.last_data + trigger(&r, self.app_rtt)).max(self.next_attempt);
        if self.o.browser == Browser::Unbound {
            return Some(due);
        }
        match self.blocker(self.made % r.endpoints) {
            None => Some(due),
            Some((created, true, _)) => r.abort_after.map(|a| due.max(created + a)),
            // Past its 101: it runs to its Join.
            Some(_) => None,
        }
    }

    /// Fix 3: after the silence, the next attempt in turn (closing a blocker past its abort time first).
    fn attempt(&mut self, t: u64) {
        if self.attempt_due().is_none_or(|due| t < due) {
            return;
        }
        let Some(r) = self.o.reconnect else {
            return;
        };
        let k = self.made % r.endpoints;
        if self.o.browser != Browser::Unbound
            && let Some((_, _, which)) = self.blocker(k)
        {
            match which {
                Which::Waiting(i) => {
                    let w = self.waiting.remove(i);
                    self.gate.stopped(t, w.endpoint, false, false);
                }
                Which::Conn(j) => self.close_conn(j, t, false),
            }
            self.out.aborts += 1;
        }
        let delay = self.gate.create(t);
        self.waiting.push(Waiting {
            created: t,
            endpoint: k,
            not_before: t + delay,
            delayed: false,
        });
        self.made += 1;
        self.outage += 1;
        self.out.attempts += 1;
        self.next_attempt = t + r.cadence;
        let capped = self.outage >= r.max_attempts;
        if let Some(b) = self.slot() {
            b.attempts += 1;
            b.throttle_max = b.throttle_max.max(delay);
            b.capped |= capped;
        }
    }

    /// Waiting attempts whose browser lets them connect now begin their TCP connect.
    fn pump(&mut self, t: u64) {
        let mut i = 0;
        while i < self.waiting.len() {
            let w = self.waiting[i];
            let k = w.endpoint;
            let go = match self.o.browser {
                Browser::Unbound => true,
                Browser::Chromium => {
                    self.waiting[..i].iter().all(|v| v.endpoint != k)
                        && t >= w.not_before
                        && t >= self.gate.unlock_at[k as usize]
                        && !self.connecting_on(Some(k))
                }
                Browser::Firefox => {
                    if i > 0 || self.connecting_on(None) {
                        false
                    } else if w.delayed {
                        t >= w.not_before
                    } else {
                        let d = self.gate.fail_delay(t, k);
                        if d > 0 {
                            self.waiting[i].delayed = true;
                            self.waiting[i].not_before = t + d;
                        }
                        d == 0
                    }
                }
            };
            if go {
                self.waiting.remove(i);
                self.conn_no += 1;
                let m = TcpModel::connect(self.p, self.seed, self.conn_no, client_cfg(&self.o), t);
                let mut c = Conn::new(m, Step::Connecting, k, w.created);
                c.began = t;
                self.conns.push(c);
                if let Some(b) = self.slot() {
                    b.wait_max = b.wait_max.max(t - w.created);
                }
            } else {
                i += 1;
            }
        }
    }

    /// When a waiting attempt may begin, if no event of another kind comes first.
    fn waiting_due(&self) -> Option<u64> {
        match self.o.browser {
            Browser::Unbound => None,
            Browser::Chromium => self
                .waiting
                .iter()
                .enumerate()
                .filter(|&(i, w)| {
                    self.waiting[..i].iter().all(|v| v.endpoint != w.endpoint)
                        && !self.connecting_on(Some(w.endpoint))
                })
                .map(|(_, w)| w.not_before.max(self.gate.unlock_at[w.endpoint as usize]))
                .min(),
            Browser::Firefox => self
                .waiting
                .first()
                .filter(|w| w.delayed)
                .map(|w| w.not_before),
        }
    }

    /// Runs to `until`; with bursts, it stops once every burst has recovered and nothing is pending (at most 3 s
    /// after the last recovery).
    fn run(mut self, until: u64) -> Outcome {
        let n = self.starts.len();
        let (mut tick, mut input) = (TICK, TICK + splitmix64(self.seed) % TICK);
        let mut follow_until: Option<u64> = None;
        let (mut last_t, mut same_t) = (0u64, 0u32);
        loop {
            let mut next = tick.min(input);
            let ff = self.o.browser == Browser::Firefox;
            for (i, c) in self.conns.iter().enumerate() {
                if c.dead {
                    continue;
                }
                if let Some(e) = c.m.next_event() {
                    next = next.min(e);
                }
                if ff && i != self.client_cur && c.step.connecting() {
                    next = next.min(c.began + FF_OPEN_TIMEOUT);
                }
            }
            for due in [
                self.waiting_due(),
                self.attempt_due(),
                follow_until,
                self.starts.get(self.win.map_or(0, |w| w + 1)).copied(),
            ]
            .into_iter()
            .flatten()
            {
                next = next.min(due);
            }
            let t = next.max(last_t);
            same_t = if t == last_t { same_t + 1 } else { 0 };
            assert!(
                same_t < 10_000,
                "seed {}: the harness is stuck at {t} ns",
                self.seed
            );
            last_t = t;
            if t > until {
                break;
            }
            for c in self.conns.iter_mut().filter(|c| !c.dead) {
                c.m.advance(t);
            }
            // A burst that starts before the last one recovered leaves that one without a recovery.
            while let Some(&s) = self.starts.get(self.win.map_or(0, |w| w + 1))
                && t >= s
            {
                self.win = Some(self.win.map_or(0, |w| w + 1));
            }
            if let Some(w) = self.win
                && self.meas < w
            {
                self.meas = w;
                for c in &mut self.conns {
                    c.fresh_end = None;
                }
            }
            self.client(t);
            self.switch(t);
            self.cancel(t);
            if t == tick {
                self.room(t);
                tick += TICK;
            }
            if t == input {
                let c = &mut self.conns[self.client_cur];
                c.m.write(1, INPUT, t);
                c.inputs.push_back((c.m.ends[1].snd.write_seq, t));
                input += TICK;
            }
            if ff {
                for j in 0..self.conns.len() {
                    let c = &self.conns[j];
                    if j != self.client_cur
                        && !c.dead
                        && c.step.connecting()
                        && t >= c.began + FF_OPEN_TIMEOUT
                    {
                        self.close_conn(j, t, true);
                        self.out.timeouts += 1;
                    }
                }
            }
            self.attempt(t);
            self.pump(t);
            if self.meas < n
                && self
                    .conns
                    .iter()
                    .any(|c| !c.dead && c.fresh_end.is_some_and(|e| c.m.delivered(1) >= e))
            {
                self.out.bursts[self.meas].recovery = Some(t - self.backs[self.meas]);
                self.meas += 1;
                for c in &mut self.conns {
                    c.fresh_end = None;
                }
                if self.meas == n {
                    follow_until = Some(t + FOLLOW);
                }
            }
            if n > 0 && self.meas == n {
                let pending = self.pending();
                if !pending || follow_until.is_some_and(|f| t >= f) {
                    self.out.pending_at_end = pending;
                    break;
                }
            }
        }
        self.out
    }
}

/// One run of `p` to `until` (its bursts are B1's, each measured).
fn run(p: &Profile, seed: u64, until: u64, o: Opts) -> Outcome {
    Sim::new(p, seed, o).run(until)
}

/// Set `k`'s phase `i`: its seed and burst start (S13 (e)'s golden-ratio spread over 5 s to 7 s, shifted by 1,000
/// phases per set; set 0 is S13 (e)'s).
fn phase_at(k: u64, i: u64) -> (u64, u64) {
    (
        1_000 * (k + 1) + i,
        5 * SEC + (((i + 1_000 * k) * 618_033_989) % SEC) * 2,
    )
}

/// How long after the link's return a phase may run before its recovery counts as censored at this value: long
/// enough for four lost resends in a row on a one-segment window (RTOs of about 1.2, 2.4, 4.8 and 9.6 s: fix 1 alone,
/// seed 35518 on model v3), which the first 12 s window cut off.
const PHASE_WINDOW: u64 = 60 * SEC;

/// Phases that had not recovered within PHASE_WINDOW (counted at PHASE_WINDOW, over every bar).
static CENSORED: AtomicU64 = AtomicU64::new(0);

/// One B1 phase: one burst, its recovery (censored at PHASE_WINDOW), and what follows it.
fn phase(seed: u64, burst_ns: u64, o: Opts) -> (BurstOut, bool) {
    let p = Profile::b1(&[burst_ns]);
    let out = run(&p, seed, burst_ns + SEC + PHASE_WINDOW, o);
    let mut b = out.bursts[0];
    if b.recovery.is_none() {
        CENSORED.fetch_add(1, Ordering::Relaxed);
        eprintln!("seed {seed}: no fresh message within 60 s of the return: counted as 60 s");
        b.recovery = Some(PHASE_WINDOW);
    }
    (b, out.pending_at_end)
}

/// `jobs` jobs on every core, in order.
fn pool<T: Send>(jobs: u64, f: impl Fn(u64) -> T + Sync) -> Vec<T> {
    let workers = std::thread::available_parallelism()
        .map_or(4, |n| n.get())
        .min(usize::try_from(jobs).unwrap_or(1))
        .max(1);
    let next = AtomicU64::new(0);
    let (f, next) = (&f, &next);
    let mut done: Vec<(u64, T)> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..workers)
            .map(|_| {
                s.spawn(move || {
                    let mut mine = Vec::new();
                    loop {
                        let j = next.fetch_add(1, Ordering::Relaxed);
                        if j >= jobs {
                            return mine;
                        }
                        mine.push((j, f(j)));
                    }
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("a worker"))
            .collect()
    });
    done.sort_by_key(|d| d.0);
    done.into_iter().map(|d| d.1).collect()
}

/// A set of 1,000 phases: the recoveries (sorted) and the attempts' side counts.
#[derive(Debug, Clone, Default)]
struct SetOut {
    rec: Vec<u64>,
    attempts: u64,
    attempts_max: u32,
    takeovers: u64,
    takeovers_after: u64,
    cancels: u64,
    pending: u64,
    capped: u64,
    wait_max: u64,
}

/// `n` sets of a row, in order.
fn sweep_sets(o: Opts, n: u64) -> Vec<SetOut> {
    pool(n, |k| {
        let mut s = SetOut::default();
        for i in 0..PHASES {
            let (seed, burst) = phase_at(k, i);
            let (b, pending) = phase(seed, burst, o);
            s.rec.push(b.recovery.unwrap_or(u64::MAX));
            s.attempts += u64::from(b.attempts);
            s.attempts_max = s.attempts_max.max(b.attempts);
            s.takeovers += u64::from(b.takeovers);
            s.takeovers_after += u64::from(b.takeovers_after);
            s.cancels += u64::from(b.cancels);
            s.pending += u64::from(pending);
            s.capped += u64::from(b.capped);
            s.wait_max = s.wait_max.max(b.wait_max);
        }
        s.rec.sort_unstable();
        s
    })
}

fn rank(sorted: &[u64], permille: usize) -> u64 {
    sorted[(permille * sorted.len()).div_ceil(1000).max(1) - 1]
}

/// Recoveries over `x`.
fn over(sorted: &[u64], x: u64) -> u64 {
    (sorted.len() - sorted.partition_point(|&v| v <= x)) as u64
}

fn pooled(sets: &[SetOut]) -> Vec<u64> {
    let mut all: Vec<u64> = sets.iter().flat_map(|s| s.rec.iter().copied()).collect();
    all.sort_unstable();
    all
}

/// The order statistics and the counts over the bars, in ms.
fn summary(all: &[u64]) -> String {
    let n = all.len();
    let counts: Vec<String> = [800, 1_000, 1_200, 1_500, 2_000]
        .iter()
        .map(|&ms| format!("{ms} ms {}", over(all, ms * MS)))
        .collect();
    format!(
        "{n} p50 {} ms, p99 {} ms, p99.9 {} ms, p99.99 {} ms, max {} ms; over {}",
        rank(all, 500) / MS,
        rank(all, 990) / MS,
        rank(all, 999) / MS,
        all[(n * 9_999).div_ceil(10_000).max(1) - 1] / MS,
        all.last().copied().unwrap_or(0) / MS,
        counts.join(", ")
    )
}

/// The attempts' side counts of a row, per phase.
fn side(sets: &[SetOut]) -> String {
    let n = (sets.len() as u64 * PHASES) as f64;
    let sum = |f: fn(&SetOut) -> u64| sets.iter().map(f).sum::<u64>();
    format!(
        "attempts {:.3} a phase (most {}), takeovers {:.4} before the recovery and {:.4} after it, cancelled \
         (old connection recovered) {:.3}, still pending 3 s after the recovery {}, capped {}, longest wait before a \
         connect {} ms",
        sum(|s| s.attempts) as f64 / n,
        sets.iter().map(|s| s.attempts_max).max().unwrap_or(0),
        sum(|s| s.takeovers) as f64 / n,
        sum(|s| s.takeovers_after) as f64 / n,
        sum(|s| s.cancels) as f64 / n,
        sum(|s| s.pending),
        sum(|s| s.capped),
        sets.iter().map(|s| s.wait_max).max().unwrap_or(0) / MS
    )
}

/// The browser-runnable reconnect: one endpoint, an attempt every `cadence`, one still connecting `abort` after its
/// creation closed.
fn serial(cadence: u64, abort: Option<u64>) -> Option<Reconnect> {
    Some(Reconnect {
        min_gap: 300 * MS,
        endpoints: 1,
        cadence,
        abort_after: abort,
        max_attempts: 8,
    })
}

fn opts(cfg: TcpConfig, adaptive: bool, reconnect: Option<Reconnect>) -> Opts {
    Opts {
        cfg,
        adaptive,
        reconnect,
        full_state_counts: false,
        browser: Browser::Chromium,
        tls: Tls::Resumed,
        client_os: ClientOs::Linux,
        budget_scale: 1,
    }
}

fn configs() -> Vec<(&'static str, Opts)> {
    let base = TcpConfig::default();
    let two = Some(Reconnect {
        min_gap: 300 * MS,
        endpoints: 2,
        cadence: 300 * MS,
        abort_after: Some(600 * MS),
        max_attempts: 16,
    });
    let mut parallel = opts(base, true, serial(300 * MS, None));
    parallel.browser = Browser::Unbound;
    vec![
        (
            "today: Linux defaults, 64 KB budget",
            opts(base, false, None),
        ),
        (
            "fix 1 alone: adaptive budget (the reconnect broken)",
            opts(base, true, None),
        ),
        (
            "fix 3 alone: 64 KB budget + reconnect (the adaptive budget broken)",
            opts(base, false, serial(300 * MS, None)),
        ),
        (
            "fix 1 + fix 3: one endpoint, never abandoned (recommended)",
            opts(base, true, serial(300 * MS, None)),
        ),
        (
            "fix 1 + fix 3: one endpoint, every 500 ms, abort at 500 ms",
            opts(base, true, serial(500 * MS, Some(500 * MS))),
        ),
        (
            "fix 1 + fix 3: one endpoint, abort at 1 s",
            opts(base, true, serial(SEC, Some(SEC))),
        ),
        (
            "fix 1 + fix 3: two endpoints, every 300 ms, abort at 600 ms",
            opts(base, true, two),
        ),
        (
            "fix 1 + fix 3: parallel attempts (reference only: no browser runs it)",
            parallel,
        ),
    ]
}

/// The recommended configuration: fix 1 and fix 3 with one endpoint, one attempt at a time, never abandoned by a
/// timer (the browser's own SYN retries carry it), closed when the old connection recovers; Chromium; TLS resumed.
fn recommended() -> Opts {
    configs()[3].1
}

/// The four rows the WS-LIMIT bars must tell apart (u3-verify2-stats.md S10).
fn bite_rows() -> [(&'static str, Opts); 4] {
    let c = configs();
    [
        ("today", c[0].1),
        ("fix 1 alone", c[1].1),
        ("fix 3 alone", c[2].1),
        ("recommended", c[3].1),
    ]
}

/// The table for WS-FIX and WS-LIMIT, 60,000 phases a row (run: cargo test --release -p npt-net --lib fixes --
/// --ignored --nocapture --test-threads 1).
#[test]
#[ignore]
fn ws_limit_table() {
    let mut rows: Vec<(String, Opts)> = configs()
        .into_iter()
        .map(|(n, o)| (n.to_string(), o))
        .collect();
    let rec = recommended();
    let with = |label: &str, f: &dyn Fn(&mut Opts)| {
        let mut o = rec;
        f(&mut o);
        (format!("recommended, {label}"), o)
    };
    rows.push(with("in Firefox", &|o| o.browser = Browser::Firefox));
    rows.push(with("full TLS handshakes (no resumption)", &|o| {
        o.tls = Tls::Full;
    }));
    rows.push(with("without timestamps (a Windows peer)", &|o| {
        o.cfg.timestamps = false;
    }));
    rows.push(with("the full state counting as recovery", &|o| {
        o.full_state_counts = true;
    }));
    for (row, label) in [(4, "abort at 500 ms"), (5, "abort at 1 s")] {
        let mut o = configs()[row].1;
        o.browser = Browser::Firefox;
        rows.push((format!("fix 1 + fix 3: {label}, in Firefox"), o));
    }
    // A Windows-like client (H11): no timestamps, SYN retries doubling from 1 s or 3 s (Get-NetTCPSetting's
    // InitialRtoMs settles which), for the never-abandoned and the abort-at-500 ms policies.
    for initial_rto_ms in [1_000, 3_000] {
        for (row, label) in [(3, "never abandoned"), (4, "abort at 500 ms")] {
            let mut o = configs()[row].1;
            o.client_os = ClientOs::Windows { initial_rto_ms };
            rows.push((
                format!("fix 1 + fix 3: {label}, a Windows-like client, first SYN timeout {initial_rto_ms} ms"),
                o,
            ));
        }
    }
    for (name, o) in rows {
        let sets = sweep_sets(o, TABLE_SETS);
        println!("{name}: {}", summary(&pooled(&sets)));
        println!("  {}", side(&sets));
    }
    println!(
        "phases censored at 60 s after the return (counted over every bar): {}",
        CENSORED.load(Ordering::Relaxed)
    );
}

/// Without a burst, 10 minutes per seed: the snapshots the room skips (fix 1's cost), and the reconnect attempts, the
/// attempts closed because data came back (H6), and the session takeovers a silence starts (fix 3's false triggers).
#[test]
#[ignore]
fn side_effects_table() {
    let mut firefox = recommended();
    firefox.browser = Browser::Firefox;
    let rows = [
        ("64 KB budget", configs()[0].1),
        ("adaptive budget", configs()[1].1),
        ("adaptive + reconnect, never abandoned", recommended()),
        ("adaptive + reconnect, never abandoned, Firefox", firefox),
        ("adaptive + reconnect, abort at 1 s", configs()[5].1),
    ];
    for (label, o) in rows {
        for p in [Profile::p1(), Profile::p2(), Profile::p3(), Profile::p4()] {
            let runs = pool(3, |seed| run(&p, seed + 1, 600 * SEC, o));
            let sum = |f: fn(&Outcome) -> u64| runs.iter().map(f).sum::<u64>();
            let (written, ticks) = (sum(|r| r.written), sum(|r| r.ticks));
            println!(
                "{} with the {label}: the room writes {written} of {ticks} snapshots ({} per mille skipped); in 30 \
                 minutes, {} reconnect attempts, {} closed as data came back, {} closed by the abort timer, {} timed \
                 out, {} takeovers",
                p.name,
                (ticks - written) * 1000 / ticks.max(1),
                sum(|r| u64::from(r.attempts)),
                sum(|r| u64::from(r.cancels)),
                sum(|r| u64::from(r.aborts)),
                sum(|r| u64::from(r.timeouts)),
                sum(|r| u64::from(r.takeovers))
            );
        }
    }
}

/// A multi-burst run: one client, `n` bursts `spacing` apart on one connection lineage (its first burst at 5 s to
/// 7 s, golden-ratio spread over the runs).
fn multi(o: Opts, runs: u64, n: u64, spacing: u64) -> Vec<Outcome> {
    pool(runs, |r| {
        let b0 = 5 * SEC + ((r * 618_033_989) % SEC) * 2;
        let starts: Vec<u64> = (0..n).map(|j| b0 + j * spacing).collect();
        let p = Profile::b1(&starts);
        run(&p, 9_000_000 + r, b0 + (n - 1) * spacing + 13 * SEC, o)
    })
}

/// The multi-burst runs' recoveries next to the single-burst phases', with what carries between bursts (Chromium's
/// throttler, Firefox's fail delay, the per-outage cap, TCP's state). Bursts 20 s apart, 10 and 30 per client.
#[test]
#[ignore]
fn multi_burst_table() {
    multi_burst_rows(20 * SEC, &[(10, 2_000), (30, 1_000)]);
}

fn multi_burst_rows(spacing: u64, shapes: &[(u64, u64)]) {
    let rec = recommended();
    let variant = |browser: Browser, abort: Option<u64>| {
        let mut o = rec;
        o.browser = browser;
        if let Some(a) = abort {
            o.reconnect = serial(a, Some(a));
        }
        o
    };
    let rows = [
        ("fix 1 alone", configs()[1].1),
        ("recommended (Chromium)", rec),
        ("recommended in Firefox", variant(Browser::Firefox, None)),
        (
            "abort at 500 ms (Chromium)",
            variant(Browser::Chromium, Some(500 * MS)),
        ),
        (
            "abort at 500 ms in Firefox",
            variant(Browser::Firefox, Some(500 * MS)),
        ),
    ];
    for (name, o) in rows {
        let single = pooled(&sweep_sets(o, TABLE_SETS));
        println!("{name}: single bursts {}", summary(&single));
        for &(n, runs) in shapes {
            let outs = multi(o, runs, n, spacing);
            let bursts: Vec<&BurstOut> = outs.iter().flat_map(|r| r.bursts.iter()).collect();
            let mut all: Vec<u64> = bursts
                .iter()
                .map(|b| b.recovery.unwrap_or(u64::MAX))
                .collect();
            all.sort_unstable();
            let per = bursts.len() as f64;
            let sum = |f: fn(&BurstOut) -> u64| bursts.iter().map(|b| f(b)).sum::<u64>();
            println!(
                "  {n} bursts {} s apart x {runs} clients: {}",
                spacing / SEC,
                summary(&all)
            );
            println!(
                "    per burst: attempts {:.3}, takeovers {:.4} before the recovery and {:.4} after it, cancelled \
                 {:.3}; capped {}; throttled >= 1 ms {} (longest {} ms); longest wait before a connect {} ms; no \
                 recovery {}",
                sum(|b| u64::from(b.attempts)) as f64 / per,
                sum(|b| u64::from(b.takeovers)) as f64 / per,
                sum(|b| u64::from(b.takeovers_after)) as f64 / per,
                sum(|b| u64::from(b.cancels)) as f64 / per,
                sum(|b| u64::from(b.capped)),
                sum(|b| u64::from(b.throttle_max >= MS)),
                bursts.iter().map(|b| b.throttle_max).max().unwrap_or(0) / MS,
                bursts.iter().map(|b| b.wait_max).max().unwrap_or(0) / MS,
                sum(|b| u64::from(b.recovery.is_none()))
            );
            let marks: Vec<String> = [1, 2, 3, 5, 7, 10, 15, 20, 25, 30]
                .into_iter()
                .filter(|&j| j <= n)
                .map(|j| {
                    let mut v: Vec<u64> = outs
                        .iter()
                        .map(|r| r.bursts[j as usize - 1].recovery.unwrap_or(u64::MAX))
                        .collect();
                    v.sort_unstable();
                    let waits = outs
                        .iter()
                        .map(|r| r.bursts[j as usize - 1].throttle_max)
                        .max()
                        .unwrap_or(0);
                    format!(
                        "#{j} {}/{}/{} ms, {} over 1 s, throttle max {} ms",
                        rank(&v, 500) / MS,
                        rank(&v, 990) / MS,
                        v.last().copied().unwrap_or(0) / MS,
                        over(&v, SEC),
                        waits / MS
                    )
                })
                .collect();
            println!("    by burst (p50/p99/max): {}", marks.join("; "));
            // No pass lines here: the round-2 three-part bar these printed is gone (the note after `two_part`), and the
            // WebSocket bar's chance on whole runs is the run-structured bound's (advisor-ws-docs-r2.md:383).
        }
    }
}

/// A WS-LIMIT bar for a run of n client-bursts: the median within `median` (when set), at most `k` over `y`, none
/// over `z` (PM-0001-18's nearest-rank percentiles: the judge's p50 is the ceil(n / 2)-th value).
#[derive(Debug, Clone, Copy)]
struct Bar {
    median: Option<u64>,
    y: u64,
    k: usize,
    z: u64,
}

impl Bar {
    fn need(&self, n: usize) -> usize {
        if self.median.is_some() {
            n.div_ceil(2)
        } else {
            0
        }
    }

    /// Recoveries at or below X, X to Y, Y to Z and above Z (X = Y without a median part).
    fn counts(&self, sorted: &[u64]) -> [u64; 4] {
        let at = |x: u64| sorted.partition_point(|&v| v <= x) as u64;
        let x = self.median.unwrap_or(self.y).min(self.y);
        let (a, b, c) = (at(x), at(self.y), at(self.z));
        [a, b - a, c - b, sorted.len() as u64 - c]
    }

    fn describe(&self, n: usize) -> String {
        let s = |v: u64| format!("{:.2} s", v as f64 / SEC as f64);
        let head = self
            .median
            .map_or(String::new(), |m| format!("half within {}, ", s(m)));
        format!(
            "{head}at most {} of {n} over {}, none over {}",
            self.k,
            s(self.y),
            s(self.z)
        )
    }
}

/// "99 % within y, every one within z" at n client-bursts: at most n - ceil(0.99 n) over y.
fn two_part(n: usize, y_ms: u64, z_ms: u64) -> Bar {
    Bar {
        median: None,
        y: y_ms * MS,
        k: n - (99 * n).div_ceil(100),
        z: z_ms * MS,
    }
}

// Removed at U3c (u3c-plan.md 1.11, C15; the stats skeptic's F11 and B3, u3-verify3-stats.md:133 and :219):
// `three_part`, round 2's bar (half within 0.6 s, at most 1 or 3 over 1.2 s, none over 2.0 s). Harness v3's
// recommended row fails it with certainty (p50 654 ms > 600 ms), so the lineage table's and the tail's three-part lines
// printed 0.000 for a correct build and evaluated nothing that was chosen. The WebSocket bar now comes from
// `ws_bar_fix1`'s model bound (fix 1 alone, Mustafa's WS-SCOPE 2026-11-07) and, on whole runs, from the run-structured
// bound (advisor-ws-docs-r2.md:383), so no fixed bar is kept here to go stale again.

fn rates(c: [u64; 4]) -> [f64; 4] {
    let n = c.iter().sum::<u64>() as f64;
    c.map(|x| x as f64 / n)
}

/// The chance a run of `n` independent client-bursts passes `bar`, exact (the dynamic program).
fn pass(n: usize, bar: Bar, q: [f64; 4]) -> f64 {
    joint_dp(n, bar.need(n), bar.k, q)
}

/// P(at least `need` of `n` independent client-bursts at or below X, at most `k` over Y, none over Z), with one
/// client-burst's class chances `q`: at or below X, X to Y, Y to Z, above Z. A dynamic program over the client-bursts;
/// its state is the count at or below X (capped at `need`) and the count over Y; a client-burst over Z ends the run.
fn joint_dp(n: usize, need: usize, k: usize, q: [f64; 4]) -> f64 {
    let w = k + 1;
    let mut cur = vec![0.0f64; (need + 1) * w];
    let mut nxt = cur.clone();
    cur[0] = 1.0;
    for _ in 0..n {
        nxt.fill(0.0);
        for a in 0..=need {
            for j in 0..w {
                let p = cur[a * w + j];
                if p == 0.0 {
                    continue;
                }
                nxt[(a + 1).min(need) * w + j] += p * q[0];
                nxt[a * w + j] += p * q[1];
                if j < k {
                    nxt[a * w + j + 1] += p * q[2];
                }
            }
        }
        std::mem::swap(&mut cur, &mut nxt);
    }
    cur[need * w..].iter().sum()
}

/// The same chance in closed form: the sum over j <= k client-bursts in (Y, Z] of C(n, j) q2^j (q0 + q1)^(n - j)
/// times P(Binomial(n - j, q0 / (q0 + q1)) >= need), from `lf` (log-factorials up to n). Fast enough for the
/// bootstrap and the bar search; a test holds it to the dynamic program.
fn joint_closed(n: usize, need: usize, k: usize, q: [f64; 4], lf: &[f64]) -> f64 {
    let rest = q[0] + q[1];
    (0..=k.min(n))
        .map(|j| {
            let outer = (lf[n] - lf[j] - lf[n - j] + xlny(j, q[2]) + xlny(n - j, rest)).exp();
            if outer == 0.0 || need == 0 {
                outer
            } else {
                outer * bin_tail(n - j, q[0] / rest, need, lf)
            }
        })
        .sum()
}

/// P(Binomial(m, p) >= need).
fn bin_tail(m: usize, p: f64, need: usize, lf: &[f64]) -> f64 {
    (need..=m)
        .map(|a| (lf[m] - lf[a] - lf[m - a] + xlny(a, p) + xlny(m - a, 1.0 - p)).exp())
        .sum()
}

/// x ln y, 0 when x is 0 (so 0^0 = 1).
fn xlny(x: usize, y: f64) -> f64 {
    if x == 0 { 0.0 } else { x as f64 * y.ln() }
}

fn log_factorials(n: usize) -> Vec<f64> {
    let mut v = vec![0.0; n + 1];
    for i in 1..=n {
        v[i] = v[i - 1] + (i as f64).ln();
    }
    v
}

/// Wilson's 95 % interval for `x` of `n`.
fn wilson(x: u64, n: u64) -> (f64, f64) {
    let (z, nf) = (1.959_963_984_540_054, n as f64);
    let p = x as f64 / nf;
    let d = 1.0 + z * z / nf;
    let c = (p + z * z / (2.0 * nf)) / d;
    let h = z / d * (p * (1.0 - p) / nf + z * z / (4.0 * nf * nf)).sqrt();
    ((c - h).max(0.0), (c + h).min(1.0))
}

/// A pass chance's 95 % intervals: a percentile bootstrap over the sets (blocks of 1,000 phases), and the bounds
/// from the class rates' Wilson intervals (each at its worst and its best end at once: conservative).
fn intervals(
    n: usize,
    bar: Bar,
    sets: &[SetOut],
    lf: &[f64],
    seed: u64,
) -> ((f64, f64), (f64, f64)) {
    let (need, k) = (bar.need(n), bar.k);
    let blocks: Vec<[u64; 4]> = sets.iter().map(|s| bar.counts(&s.rec)).collect();
    let mut rng = SplitMix(splitmix64(seed));
    let mut boot: Vec<f64> = (0..BOOT)
        .map(|_| {
            let mut c = [0u64; 4];
            for _ in 0..blocks.len() {
                let b = blocks[below(rng.next(), blocks.len() as u64) as usize];
                for (x, y) in c.iter_mut().zip(b) {
                    *x += y;
                }
            }
            joint_closed(n, need, k, rates(c), lf)
        })
        .collect();
    boot.sort_by(f64::total_cmp);
    let total = blocks.iter().fold([0u64; 4], |mut a, b| {
        for (x, y) in a.iter_mut().zip(b) {
            *x += y;
        }
        a
    });
    let nn: u64 = total.iter().sum();
    let (w0, w2, w3) = (
        wilson(total[0], nn),
        wilson(total[2], nn),
        wilson(total[3], nn),
    );
    // X to Y takes up what the other classes leave; without a median part, at or below Y does.
    let at = |q0: f64, q2: f64, q3: f64| {
        let q0 = if need == 0 {
            1.0 - q2 - q3
        } else {
            q0.min(1.0 - q2 - q3)
        };
        let q1 = (1.0 - q0 - q2 - q3).max(0.0);
        joint_closed(n, need, k, [q0, q1, q2, q3], lf)
    };
    (
        (boot[BOOT * 25 / 1_000], boot[BOOT * 975 / 1_000 - 1]),
        (at(w0.0, w2.1, w3.1), at(w0.1, w2.0, w3.0)),
    )
}

/// The tail, for setting WS-LIMIT: 600 sets (600,000 phases) of each bite row; every rate with its Wilson interval;
/// the exact chance that a run of 480 or 1,440 independent client-bursts (PM-0001-18: 48 clients x 10 or 30 bursts)
/// passes each bar, with its intervals; and a search for the bar a correct build passes with at least 0.95 while the
/// broken builds pass least.
#[test]
#[ignore]
fn ws_limit_tail() {
    let rows: Vec<(&str, Vec<SetOut>)> = bite_rows()
        .into_iter()
        .map(|(name, o)| (name, sweep_sets(o, TAIL_SETS)))
        .collect();
    let lf = log_factorials(1_440);
    for (name, sets) in &rows {
        let all = pooled(sets);
        let nn = all.len() as u64;
        println!("{name}: {}", summary(&all));
        let rated: Vec<String> = [600, 800, 1_000, 1_200, 1_500, 2_000, 2_500, 3_000]
            .iter()
            .map(|&ms| {
                let x = over(&all, ms * MS);
                let (lo, hi) = wilson(x, nn);
                format!(
                    "{ms} ms {x} ({:.3} [{:.3}-{:.3}])",
                    x as f64 / nn as f64 * 1e3,
                    lo * 1e3,
                    hi * 1e3
                )
            })
            .collect();
        println!(
            "  over, per mille with 95 % intervals: {}",
            rated.join(", ")
        );
        println!("  {}", side(sets));
    }
    let bars: Vec<(usize, Bar)> = [480, 1_440]
        .into_iter()
        .flat_map(|n| {
            [(800, 1_000), (1_000, 1_500), (1_000, 2_000)]
                .into_iter()
                .map(move |(y, z)| (n, two_part(n, y, z)))
        })
        .collect();
    for (b, &(n, bar)) in bars.iter().enumerate() {
        let cells: Vec<String> = rows
            .iter()
            .enumerate()
            .map(|(r, (name, sets))| {
                let p = pass(n, bar, rates(bar.counts(&pooled(sets))));
                let ((blo, bhi), (rlo, rhi)) = intervals(n, bar, sets, &lf, (b * 16 + r) as u64);
                format!("{name} {p:.3} [boot {blo:.3}-{bhi:.3}; rates {rlo:.3}-{rhi:.3}]")
            })
            .collect();
        println!("{}: {}", bar.describe(n), cells.join(", "));
    }
    let alls: Vec<Vec<u64>> = rows.iter().map(|(_, s)| pooled(s)).collect();
    for n in [480usize, 1_440] {
        // (the broken builds' highest pass chance in thousandths, the bar, the four pass chances)
        let mut found: Vec<(i64, Bar, [f64; 4])> = Vec::new();
        let ks = if n <= 480 { 0..=6 } else { 0..=16 };
        for median in [None, Some(500), Some(550), Some(600), Some(650), Some(700)] {
            for y in [800u64, 900, 1_000, 1_100, 1_200, 1_300, 1_400, 1_500] {
                for z in [1_500u64, 1_600, 1_800, 2_000, 2_200, 2_500, 3_000] {
                    if z <= y {
                        continue;
                    }
                    for k in ks.clone() {
                        let bar = Bar {
                            median: median.map(|m| m * MS),
                            y: y * MS,
                            k,
                            z: z * MS,
                        };
                        let p = [0, 1, 2, 3].map(|r| {
                            joint_closed(n, bar.need(n), k, rates(bar.counts(&alls[r])), &lf)
                        });
                        if p[3] >= 0.95 {
                            let worst = p[..3].iter().copied().fold(0.0, f64::max);
                            found.push(((worst * 1e3).round() as i64, bar, p));
                        }
                    }
                }
            }
        }
        // The broken builds' best chance first; then the correct build's highest; then the loosest median part.
        let milli = |x: f64| (x * 1e3).round() as i64;
        found.sort_by(|a, b| {
            a.0.cmp(&b.0)
                .then(milli(b.2[3]).cmp(&milli(a.2[3])))
                .then(b.1.median.cmp(&a.1.median))
        });
        println!(
            "search at {n} client-bursts ({} bars a correct build passes with at least 0.95 at the point), the five \
             whose broken builds pass least among those whose bootstrap interval for the correct build stays at or \
             above 0.95 (a looser median part that changes no chance to 3 decimals is shown alone):",
            found.len()
        );
        let mut shown: Vec<(u64, usize, u64, [i64; 4])> = Vec::new();
        for (i, (_, bar, p)) in found.iter().enumerate() {
            let key = (bar.y, bar.k, bar.z, p.map(|x| (x * 1e3).round() as i64));
            if shown.contains(&key) {
                continue;
            }
            let seed = |r: usize| (1_000 + i * 16 + r) as u64;
            let correct = intervals(n, *bar, &rows[3].1, &lf, seed(3));
            if correct.0.0 < 0.95 {
                continue;
            }
            let iv: Vec<((f64, f64), (f64, f64))> = rows[..3]
                .iter()
                .enumerate()
                .map(|(r, (_, sets))| intervals(n, *bar, sets, &lf, seed(r)))
                .chain(std::iter::once(correct))
                .collect();
            shown.push(key);
            let cells: Vec<String> = rows
                .iter()
                .enumerate()
                .map(|(r, (name, _))| {
                    let ((blo, bhi), (rlo, rhi)) = iv[r];
                    format!(
                        "{name} {:.3} [boot {blo:.3}-{bhi:.3}; rates {rlo:.3}-{rhi:.3}]",
                        p[r]
                    )
                })
                .collect();
            println!("  {}: {}", bar.describe(n), cells.join(", "));
            if shown.len() == 5 {
                break;
            }
        }
    }
    println!(
        "phases censored at 60 s after the return (counted over every bar): {}",
        CENSORED.load(Ordering::Relaxed)
    );
}

/// FNV-1a over the values' little-endian bytes.
fn fnv(values: &[u64]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for v in values {
        for b in v.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
    }
    h
}

/// The recommended configuration's 6,000 phases (sets 0 to 5), pinned so the WS-LIMIT numbers stay tied to the code:
/// the hash of the sorted recoveries, the counts over 0.8, 1.0, 1.2 and 1.5 s, and the attempts' side counts. A pin,
/// not a goal: a 6,000-phase block's maximum is noise (under harness v2 only 71 of 100 blocks kept every recovery
/// within 1.5 s: u3-verify2-stats.md S14), so no goal is asserted here; WS-LIMIT's bars come from ws_limit_tail.
#[test]
fn the_recommended_fixes_are_pinned_at_b1() {
    let sets = sweep_sets(recommended(), SETS);
    let all = pooled(&sets);
    let counts = [800, 1_000, 1_200, 1_500].map(|ms| over(&all, ms * MS));
    let sum = |f: fn(&SetOut) -> u64| sets.iter().map(f).sum::<u64>();
    let attempts = [
        sum(|s| s.attempts),
        sum(|s| s.cancels),
        sum(|s| s.takeovers),
        sum(|s| s.takeovers_after),
    ];
    assert_eq!(
        (fnv(&all), counts, attempts),
        PINNED_RECOMMENDED,
        "{}",
        summary(&all)
    );
}

/// The hash of the sorted recoveries; the counts over 0.8, 1.0, 1.2 and 1.5 s; attempts, attempts closed as the old
/// connection recovered, takeovers before and after the recovery.
const PINNED_RECOMMENDED: (u64, [u64; 4], [u64; 4]) = (
    0x52e7_b772_29d8_5ebb,
    [467, 77, 32, 10],
    [5_998, 3_999, 946, 1_056],
);

/// The proof test's own row, fix 1 alone (Mustafa's WS-SCOPE, 2026-11-07; one of the WS-LIMIT bite rows), pinned the
/// same way over sets 0 to 5 (the stats skeptic's B5, u3-verify3-stats.md:225: the pin covered the recommended row
/// only, so a regression in a bite row passed CI; u3c-plan.md 1.11, C15). Fix 1 alone never reconnects, so its side
/// counts are 0. A pin, not a goal.
#[test]
fn the_fix_1_row_is_pinned_at_b1() {
    let sets = sweep_sets(configs()[1].1, SETS);
    let all = pooled(&sets);
    let counts = [800, 1_000, 1_200, 1_500].map(|ms| over(&all, ms * MS));
    let sum = |f: fn(&SetOut) -> u64| sets.iter().map(f).sum::<u64>();
    let attempts = [
        sum(|s| s.attempts),
        sum(|s| s.cancels),
        sum(|s| s.takeovers),
        sum(|s| s.takeovers_after),
    ];
    assert_eq!(
        (fnv(&all), counts, attempts),
        PINNED_FIX1,
        "{}",
        summary(&all)
    );
}

/// Fix 1 alone: the hash of the sorted recoveries; the counts over 0.8, 1.0, 1.2 and 1.5 s; the side counts. Taken at
/// U3c (6,000 phases: p50 653 ms, p99 2,172 ms, max 5,160 ms; consistent with ws_bar_fix1's 600,000: 654 / 2,164).
const PINNED_FIX1: (u64, [u64; 4], [u64; 4]) =
    (0xb6f3_4ac0_188a_2727, [675, 523, 429, 228], [0, 0, 0, 0]);

/// Mechanism test (e) (u3c-plan.md 1.4, C9): the harness's adaptive limit is the shipped `ws_gate::snapshot_budget`
/// (one definition: `adaptive_limit` calls it), and that gives the same N as fixes.rs's rule as it stood before U3c
/// (ceil(3 x srtt / (2 x TICK)) + 1, at least 3, 100 ms before a sample; written out here as the reference) over every
/// 7th µs of srtt to 3 s, with no sample, and at the srtt of real models settled at P0 to P4.
#[test]
fn e_the_adaptive_limit_is_the_shipped_snapshot_budget() {
    use crate::ws_gate::snapshot_budget;
    let before = |srtt: Option<u64>| {
        let s = srtt.unwrap_or(100 * MS);
        ((3 * s).div_ceil(2 * TICK) + 1).max(3)
    };
    for us in (0..=3_000_000u64).step_by(7) {
        let s = Some(us * 1_000);
        assert_eq!(snapshot_budget(s, TICK), before(s), "{us} µs");
    }
    assert_eq!(snapshot_budget(None, TICK), before(None));
    for p in [
        Profile::p0(),
        Profile::p1(),
        Profile::p2(),
        Profile::p3(),
        Profile::p4(),
    ] {
        let mut m = TcpModel::new(&p, 7, 0);
        assert_eq!(adaptive_limit(&m), 6 * SNAP, "no sample yet");
        for k in 1..=90 {
            m.advance(k * TICK);
            m.write(0, SNAP, k * TICK);
        }
        m.advance(4 * SEC);
        let srtt = m.srtt_ns(0);
        assert!(srtt.is_some(), "{}: a sample", p.name);
        assert_eq!(adaptive_limit(&m), before(srtt) * SNAP, "{}", p.name);
    }
}

#[test]
fn the_joint_chance_matches_brute_force_and_its_closed_form() {
    let lf = log_factorials(1_440);
    let cases: [(usize, usize, usize, [f64; 4]); 6] = [
        (7, 4, 1, [0.5, 0.3, 0.15, 0.05]),
        (7, 0, 2, [0.6, 0.0, 0.3, 0.1]),
        (6, 3, 0, [0.0, 0.9, 0.1, 0.0]),
        (480, 240, 1, [0.97, 0.025, 0.004, 0.001]),
        (1_440, 720, 3, [0.975, 0.0235, 5e-4, 6e-5]),
        (1_440, 0, 14, [0.99, 0.0, 0.0099, 1e-4]),
    ];
    for (n, need, k, q) in cases {
        let dp = joint_dp(n, need, k, q);
        let closed = joint_closed(n, need, k, q, &lf);
        assert!(
            (dp - closed).abs() < 1e-12,
            "n {n}: dp {dp}, closed {closed}"
        );
        if n <= 7 {
            // Every sequence of classes: 4^n.
            let mut brute = 0.0;
            for code in 0..4usize.pow(n as u32) {
                let (mut c, mut p, mut x) = ([0usize; 4], 1.0, code);
                for _ in 0..n {
                    c[x % 4] += 1;
                    p *= q[x % 4];
                    x /= 4;
                }
                if c[0] >= need && c[2] + c[3] <= k && c[3] == 0 {
                    brute += p;
                }
            }
            assert!((dp - brute).abs() < 1e-12, "n {n}: dp {dp}, brute {brute}");
        }
    }
    // The skeptic's exact joint for the old recommended row at (1.0 s, 1.5 s), 60,000 phases: 63 over 1 s, 2 over
    // 1.5 s, 0.984 (u3-verify2-stats.md S5).
    let q = rates([59_937, 0, 61, 2]);
    assert!((pass(480, two_part(480, 1_000, 1_500), q) - 0.984).abs() < 5e-4);
    // Wilson's interval for 5 of 100 (0.0215-0.1118) and 0 of 100 (0-0.0370).
    let (lo, hi) = wilson(5, 100);
    assert!(
        (lo - 0.0215).abs() < 1e-4 && (hi - 0.1118).abs() < 1e-4,
        "{lo} {hi}"
    );
    let (lo, hi) = wilson(0, 100);
    assert!(lo < 1e-12 && (hi - 0.0370).abs() < 1e-4, "{lo} {hi}");
}

#[test]
fn chromiums_throttler_and_firefoxs_fail_delay_follow_their_sources() {
    // Chromium: the page's first WebSocket finds no throttler (0 ms) and starts its 2-minute timer.
    let mut th = Throttler::default();
    assert_eq!(
        th.issue(0, || unreachable!("no draw without a throttler")),
        0
    );
    th.done(0, true);
    // 8 failures, 1 success: 2^(0 + 8 / 2) = 16, so 1,000-5,000 x 16 / 65,536 ms: 0 or 1 ms.
    for _ in 0..8 {
        th.issue(SEC, || 0);
        th.done(SEC, false);
    }
    assert_eq!(th.issue(2 * SEC, || 0), 0, "1,000 x 16 / 65,536 = 0 ms");
    th.done(2 * SEC, false);
    assert_eq!(
        th.issue(2 * SEC, || u64::MAX),
        MS,
        "5,000 x 16 / 65,536 = 1 ms"
    );
    th.done(2 * SEC, false);
    // After one roll the counts are the previous period's: 10 failed, 1 succeeded; with one pending, 2^(1 + 5).
    th.issue(THROTTLE_PERIOD, || 0);
    assert_eq!(th.failed, [10, 0]);
    assert_eq!(
        th.issue(THROTTLE_PERIOD, || u64::MAX),
        5_000 * 64 / 65_536 * MS
    );
    th.done(THROTTLE_PERIOD, false);
    th.done(THROTTLE_PERIOD, false);
    // Two rolls later without a success the throttler is erased (IsClean ignores the previous failures): 0 ms again.
    th.roll_to(3 * THROTTLE_PERIOD);
    assert_eq!(th, Throttler::default());
    // 16 failures and no success: the exponent caps at 16, so 1-5 s.
    let mut th = Throttler::default();
    th.issue(0, || 0);
    for _ in 0..16 {
        th.issue(0, || 0);
        th.done(0, false);
    }
    th.done(0, false);
    assert_eq!(th.issue(0, || 0), 1_000 * MS);

    // Firefox: a page close adds a 200-400 ms record and never grows it; a failure grows it x1.5; an open clears it.
    let mut g = Gate::new(Browser::Firefox, 7);
    g.stopped(SEC, 0, true, false);
    let r = g.fail[0].expect("a record");
    assert!((200..400).contains(&r.next_ms) && r.last == SEC);
    assert_eq!(g.fail_delay(SEC + 100 * MS, 0), (r.next_ms - 100) * MS);
    g.stopped(2 * SEC, 0, true, false);
    assert_eq!(
        g.fail[0],
        Some(r),
        "a page close leaves a known record as it is"
    );
    g.stopped(2 * SEC, 0, true, true);
    assert_eq!(
        g.fail[0],
        Some(FailRecord {
            last: 2 * SEC,
            next_ms: r.next_ms * 3 / 2
        })
    );
    assert_eq!(g.fail_delay(2 * SEC + (r.next_ms * 3 / 2) * MS, 0), 0);
    assert!(g.fail[0].is_some(), "not expired: kept");
    assert_eq!(g.fail_delay(2 * SEC + 61 * SEC, 0), 0);
    assert!(
        g.fail[0].is_none(),
        "expired 60 s + its delay after the last failure"
    );
    g.stopped(70 * SEC, 0, true, false);
    g.opened(71 * SEC, 0);
    assert!(g.fail[0].is_none(), "an open clears it");
}

/// P(Binomial(n, p) <= k), from log-factorials up to n.
fn binom_le(n: usize, k: usize, p: f64, lf: &[f64]) -> f64 {
    if p <= 0.0 {
        return 1.0;
    }
    if p >= 1.0 {
        return if k >= n { 1.0 } else { 0.0 };
    }
    let (lp, lq) = (p.ln(), (1.0 - p).ln());
    (0..=k.min(n))
        .map(|j| (lf[n] - lf[j] - lf[n - j] + j as f64 * lp + (n - j) as f64 * lq).exp())
        .sum::<f64>()
        .min(1.0)
}

/// Fix 1 alone, as the proof test measures it (WS-SCOPE "no", Mustafa 2026-11-07): the bound he set for PM-0001-18's
/// WebSocket bar ("never looser than the model predicts for a correct build with fix 1"). Over 600,000 phases each:
/// the correct build (fix 1 as specified) and two broken ones, today (the 64 KB budget, no fix 1) and a budget twice too
/// large. The bound splits a 5 % false-fail allowance: the median part fails a correct run at most 1 % of the time (the
/// smallest X on a 10 ms grid with P(at least ceil(n / 2) at or below X) >= 0.99), the count part 2 % (for Y of 2.0,
/// 2.5 and 3.0 s, the smallest k with P(at most k over Y) >= 0.98), the max part 2 % (the smallest Z on a 100 ms grid
/// with P(none over Z) >= 0.98). Then each bar's exact joint chance for the three builds, the correct build's bootstrap
/// interval, and its chance if every recovery were 5 % or 10 % slower or 20 or 40 ms later (model error: why the
/// numbers wait for the practice run).
#[test]
#[ignore]
fn ws_bar_fix1() {
    let mut double = configs()[1].1;
    double.budget_scale = 2;
    let rows: Vec<(&str, Vec<SetOut>)> = [
        ("fix 1 (correct)", configs()[1].1),
        ("today: 64 KB, no fix 1", configs()[0].1),
        ("fix 1 with a budget twice too large", double),
    ]
    .into_iter()
    .map(|(name, o)| (name, sweep_sets(o, TAIL_SETS)))
    .collect();
    let lf = log_factorials(1_440);
    let alls: Vec<Vec<u64>> = rows.iter().map(|(_, s)| pooled(s)).collect();
    for ((name, _), all) in rows.iter().zip(&alls) {
        println!("{name}: {}", summary(all));
    }
    let correct = &alls[0];
    let nn = correct.len() as f64;
    let share_le = |x: u64| correct.partition_point(|&v| v <= x) as f64 / nn;
    let moved = |f: &dyn Fn(u64) -> u64| -> Vec<u64> {
        let mut v: Vec<u64> = correct.iter().map(|&x| f(x)).collect();
        v.sort_unstable();
        v
    };
    let errors: [(&str, Vec<u64>); 4] = [
        ("x1.05", moved(&|x| x + x / 20)),
        ("x1.10", moved(&|x| x + x / 10)),
        ("+20 ms", moved(&|x| x + 20 * MS)),
        ("+40 ms", moved(&|x| x + 40 * MS)),
    ];
    for n in [480usize, 1_440] {
        let need = n.div_ceil(2);
        let x = (0..400u64)
            .map(|i| 400 * MS + i * 10 * MS)
            .find(|&x| 1.0 - binom_le(n, need - 1, share_le(x), &lf) >= 0.99)
            .expect("a median bound");
        let z = (0..600u64)
            .map(|i| SEC + i * 100 * MS)
            .find(|&z| share_le(z).powi(n as i32) >= 0.98)
            .expect("a max bound");
        for y in [2_000 * MS, 2_500 * MS, 3_000 * MS] {
            if y >= z {
                continue;
            }
            let r = 1.0 - share_le(y);
            let k = (0..=n)
                .find(|&k| binom_le(n, k, r, &lf) >= 0.98)
                .expect("a count bound");
            let bar = Bar {
                median: Some(x),
                y,
                k,
                z,
            };
            let cells: Vec<String> = rows
                .iter()
                .zip(&alls)
                .map(|((name, _), all)| {
                    format!("{name} {:.4}", pass(n, bar, rates(bar.counts(all))))
                })
                .collect();
            let ((blo, bhi), _) = intervals(n, bar, &rows[0].1, &lf, n as u64);
            let err: Vec<String> = errors
                .iter()
                .map(|(label, v)| format!("{label} {:.3}", pass(n, bar, rates(bar.counts(v)))))
                .collect();
            println!(
                "model bound at {n} client-bursts: {}: {} (correct build's bootstrap {blo:.4}-{bhi:.4}); the correct \
                 build under model error: {}",
                bar.describe(n),
                cells.join(", "),
                err.join(", ")
            );
        }
    }
    println!(
        "phases censored at 60 s after the return (counted over every bar): {}",
        CENSORED.load(Ordering::Relaxed)
    );
}
