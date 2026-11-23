//! S1's tests (u6a-brief.md section 3): the tick thread's core on an injected clock (no threads: `Core::wake` and
//! `SessionIo::poll` driven in turn, each Loopback end's times the test's), and the threaded loop on the real clock
//! (each at most 5 s, no timing bar: the container's numbers are printed, never judged). U6a's tests hold no throwing
//! or rejoining hostile client and no client restarted mid-run; the one replacement test checks routing only, no press
//! or shot count (C-U6-19's gate line).

use std::collections::BTreeMap;

use super::*;
use npt_client::ClientEndpoint;
use npt_link::{Loopback, Recv, SendResult};
use npt_log::{Build, Header, Machine, read_log};
use npt_proto::{Direction, Join, Leave, Resync, decode, encode};
use npt_sim::StandIns;

const VERSION: u16 = 1;
const CONTENT: [u8; 32] = [7; 32];
const MS: u64 = 1_000_000;

fn token(slot: u8) -> [u8; 16] {
    let mut t = [0x5a; 16];
    t[0] = slot;
    t
}

fn join(slot: u8) -> Join {
    Join {
        token: token(slot),
        bind: [slot ^ 0xa5; 16],
        version: VERSION,
        content_hash: CONTENT,
    }
}

fn bytes(m: &Message) -> Vec<u8> {
    let mut out = Vec::new();
    encode(m, &mut out).expect("encodes");
    out
}

/// The tick period at the build's rate, ns (tick 1's due time).
fn period() -> u64 {
    due(1)
}

/// When tick `k` of the loop falls due: ⌊k × 10^9 / tick_hz⌋ ns (U4b's schedule).
fn due(k: u64) -> u64 {
    k * 1_000_000_000 / u64::from(npt_sim::TICK_HZ)
}

pub(super) fn settings(scene: Scene, n: u8) -> LoopSettings {
    LoopSettings {
        scene: scene.with_stand_ins(StandIns::OFF),
        seed: 3,
        first_tick: 300,
        tokens: (0..n).map(token).collect(),
        cmd_timing: CmdTiming::B,
        version: VERSION,
        content_hash: CONTENT,
        mode_hash: [0; 32],
        warmup_ticks: 0,
        io_threads: 2,
        log: None,
        tick_vcpu: None,
        io_vcpus: vec![],
        smt: vec![],
        #[cfg(feature = "bites")]
        bite: None,
    }
}

/// A test log's header (container machine, made-up build).
pub(super) fn header() -> Header {
    Header {
        schema: npt_log::SCHEMA,
        run: "C-u6a-test".to_owned(),
        set: "C-u6a".to_owned(),
        role: "measured".to_owned(),
        marks: vec!["PM-0001-4".to_owned()],
        machine: Machine {
            role: "C".to_owned(),
            name: "C".to_owned(),
            plan: "container".to_owned(),
            arch: "x86_64".to_owned(),
            cpu: "test".to_owned(),
            vcpus: 4,
            mem_mb: 1,
            location: "here".to_owned(),
            kernel: "6".to_owned(),
            address: String::new(),
            threads_per_core: 1,
            smt: vec![vec![0]],
            clock_res_ns: 1,
        },
        build: Build {
            commit: "0".repeat(40),
            binary_sha256: "00".repeat(32),
            features: vec![],
            profile: "debug".to_owned(),
        },
        bite: None,
        layer: "L2".to_owned(),
        profile: "P0".to_owned(),
        seed: 3,
        tick_hz: npt_sim::TICK_HZ,
        clients: 2,
        scene: "SS".to_owned(),
        process: "room".to_owned(),
        slot: 0,
        date: "2026-11-22".to_owned(),
        params: BTreeMap::new(),
    }
}

/// An in-memory log file the test reads back.
#[derive(Clone, Default)]
pub(super) struct Sink(pub(super) Arc<Mutex<Vec<u8>>>);

impl Write for Sink {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(super) fn log_to(sink: &Sink) -> Writer<Box<dyn Write + Send>> {
    let out: Box<dyn Write + Send> = Box::new(sink.clone());
    Writer::new(out, &header()).expect("the header writes")
}

/// The tick thread's core and its queues, with no thread: the injected-clock tests drive it.
struct Bench {
    core: Core,
    to_tick: ToTick,
    shared: Arc<Shared>,
    rows: Receiver<Row>,
}

fn bench(scene: Scene, n: u8) -> Bench {
    let mut s = settings(scene, n);
    let room = build_room(&mut s);
    let (to_tick, inbound) = to_tick();
    let shared = Arc::new(Shared::default());
    let (tx, rows) = sync_channel(ROWS_CAP);
    Bench {
        core: Core::new(
            room,
            inbound,
            Arc::clone(&shared),
            Some(tx),
            (0, usize::from(n)),
        ),
        to_tick,
        shared,
        rows,
    }
}

impl Bench {
    fn session(&self, id: u32, link: Box<dyn Link + Send>, now: u64) -> SessionIo {
        self.session_with(id, link, now, RttSource::Link, OUTBOUND_CAP)
    }

    fn session_with(
        &self,
        id: u32,
        link: Box<dyn Link + Send>,
        now: u64,
        rtt: RttSource,
        out_cap: usize,
    ) -> SessionIo {
        SessionIo::open(
            &self.to_tick,
            &self.shared,
            id,
            link,
            rtt,
            (VERSION, CONTENT),
            now,
            out_cap,
        )
        .expect("the queue takes the opening")
    }

    /// One wake at `now`, every edge read as `now`.
    fn wake(&mut self, now: u64) -> u64 {
        self.core.wake(now, &mut || now)
    }

    fn ticks(&self) -> Vec<TickRow> {
        self.rows
            .try_iter()
            .filter_map(|r| match r {
                Row::Tick(t) => Some(t),
                _ => None,
            })
            .collect()
    }
}

/// Every message waiting at a client's end by `now`, decoded room → client.
fn received(end: &mut dyn Link, now: u64) -> Vec<(Channel, Message)> {
    let mut out = Vec::new();
    while let Some(r) = end.poll_recv(now) {
        let m =
            decode(Direction::RoomToClient, r.channel, &r.bytes).expect("the room's bytes decode");
        out.push((r.channel, m));
    }
    out
}

/// The stream messages (not the datagrams: the decoder's pings) of `got`, each as (its stream, its type's name).
fn streams(got: &[(Channel, Message)]) -> Vec<(Channel, &'static str)> {
    got.iter()
        .filter(|(c, _)| *c != Channel::Datagram)
        .map(|(c, m)| (*c, m.name()))
        .collect()
}

/// A session over a Loopback pair, its `Join` for `slot` sent at `now`.
fn joining(b: &Bench, id: u32, slot: u8, now: u64) -> (SessionIo, Loopback) {
    let (server, mut client) = Loopback::pair(1_200);
    let io = b.session(id, Box::new(server), now);
    client.send_reliable(Channel::Control, &bytes(&Message::Join(join(slot))), now);
    (io, client)
}

#[test]
fn s1_the_core_runs_each_tick_when_it_falls_due_and_times_its_start() {
    let mut b = bench(Scene::stress(), 2);
    let first = b.core.room.last_tick().wrapping_add(1);
    let next = b.wake(0);
    assert_eq!(
        b.core.room.last_tick(),
        first,
        "tick 0 falls due at the loop's zero"
    );
    assert_eq!(next, period(), "tick 1 at ⌊10^9 / tick_hz⌋");
    assert_eq!(b.wake(next - 1), next, "a wake before it runs nothing");
    assert_eq!(b.core.room.last_tick(), first);
    let late = 5 * MS;
    b.wake(next + late);
    for k in 2..32 {
        assert_eq!(
            b.wake(due(k)),
            due(k + 1),
            "tick {k} runs; tick {} is next",
            k + 1
        );
    }
    let rows = b.ticks();
    assert_eq!(rows.len(), 32);
    assert_eq!(b.core.room.last_tick(), first.wrapping_add(31));
    for (k, r) in rows.iter().enumerate() {
        assert_eq!(
            r.t,
            u64::from(first) + k as u64,
            "the log's tick is the room's"
        );
        assert_eq!(r.timers.wake_ticks(), 1);
        assert!(!r.timers.overrun() && r.timers.dropped_ns() == 0);
        let jitter = if k == 1 { late } else { 0 };
        assert_eq!(r.timers.start_jitter_ns(), jitter, "tick {k}");
    }
    let c = b.core.schedule.counters();
    assert_eq!((c.overruns, c.drops), (0, 0));
}

#[test]
fn s1_a_stall_runs_at_most_the_catch_up_cap_and_counts_the_dropped_time() {
    let mut b = bench(Scene::stress(), 2);
    let first = b.core.room.last_tick().wrapping_add(1);
    b.wake(0);
    let stall = 1_000_000_000;
    let next = b.wake(stall);
    assert!(next > stall, "after any wake the next tick is due after it");
    let cap = npt_sim::LOOP_CATCHUP;
    let rows = b.ticks();
    let wake: Vec<&TickRow> = rows.iter().skip(1).collect();
    assert_eq!(
        wake.len(),
        cap as usize,
        "the wake ran the catch-up cap, no more"
    );
    assert_eq!(b.core.room.last_tick(), first.wrapping_add(cap));
    let c = b.core.schedule.counters();
    assert!(c.dropped_ticks > 0 && c.dropped_ns > 0, "{c:?}");
    for (j, r) in wake.iter().enumerate() {
        assert_eq!(
            r.timers.wake_ticks(),
            cap,
            "every tick of the wake says {cap}"
        );
        if j == 0 {
            assert_eq!(
                r.timers.dropped_ns(),
                c.dropped_ns,
                "the wake's dropped time on its first tick"
            );
            assert!(r.timers.overrun());
        } else {
            assert_eq!(r.timers.dropped_ns(), 0);
            assert!(!r.timers.overrun());
        }
    }
    let dropped: u64 = rows.iter().map(|r| r.timers.dropped_ns()).sum();
    let overruns = rows.iter().filter(|r| r.timers.overrun()).count() as u64;
    assert_eq!(
        (dropped, overruns),
        (c.dropped_ns, c.overruns),
        "each series sums to the loop's counter"
    );
    println!(
        "catch-up: {} ticks in the stall's wake, {} dropped ({} ns), next due at {next} ns",
        wake.len(),
        c.dropped_ticks,
        c.dropped_ns
    );
}

#[test]
fn s1_the_bounded_channels_count_a_flood_and_never_block_the_tick() {
    // The course scene's one slot (cheap ticks); its queue from the tick thread holds 4 items (TickLoop's hold
    // OUTBOUND_CAP), so the door's changes fill it soon once its I/O task stops.
    let mut b = bench(Scene::course(), 1);
    let (server, mut client) = Loopback::pair(1_200);
    let mut io = b.session_with(0, Box::new(server), 0, RttSource::Link, 4);
    client.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
    assert!(io.poll(0));
    b.wake(0);
    assert!(io.poll(1));
    let p = period();
    b.wake(p);
    // The flood: 10,000 `Resync`s on the control stream, all decoded in one poll on the I/O side.
    let resync = bytes(&Message::Resync(Resync));
    for _ in 0..10_000 {
        client.send_reliable(Channel::Control, &resync, p + 1);
    }
    assert!(io.poll(p + 1), "the poll returns: an offer never waits");
    let full = b.shared.inbound_full.load(Ordering::Relaxed);
    assert_eq!(
        full,
        10_000 - INBOUND_CAP as u64 + 1,
        "the queue took {INBOUND_CAP}; the rest and the poll's report were counted and dropped"
    );
    b.wake(2 * p);
    let k = b.core.room.client(0).expect("joined").counters().clone();
    assert_eq!(
        k.resyncs + k.resyncs_dropped + k.resyncs_limited,
        INBOUND_CAP as u64,
        "the room took what the queue held, no more"
    );
    // The other way (the stress scene, where every tick changes): a real client acknowledges its full state, so the
    // room sends it a snapshot each tick; then its session's I/O task stops, its 4-item queue fills, and every due
    // tick still runs.
    let mut b = bench(Scene::stress(), 2);
    let (server, mut link) = Loopback::pair(1_200);
    let mut io = b.session_with(0, Box::new(server), 0, RttSource::Link, 4);
    let mut e = ClientEndpoint::new(join(0), npt_sim::TICK_HZ);
    for k in 0..8 {
        step_client(&mut e, &mut link, due(k));
        assert!(io.poll(due(k) + 1));
        b.wake(due(k) + 2);
        assert!(io.poll(due(k) + 3));
    }
    assert!(
        b.core
            .room
            .client(0)
            .is_some_and(|c| c.counters().snapshots > 0),
        "snapshots flow"
    );
    assert_eq!(b.shared.out_full.load(Ordering::Relaxed), 0);
    for k in 8..40 {
        b.wake(due(k));
    }
    assert_eq!(b.core.run, 40, "every due tick ran");
    let out_full = b.shared.out_full.load(Ordering::Relaxed);
    assert!(out_full > 0, "the full session queue was counted");
    println!(
        "flood: {full} dropped of 10,000 into the tick thread's queue; {out_full} dropped into the stalled session's"
    );
}

/// One step of a client at `now`: what arrived goes to its endpoint, then what it sends goes out.
fn step_client(e: &mut ClientEndpoint, link: &mut Loopback, now: u64) {
    let mut got = Vec::new();
    while let Some(r) = link.poll_recv(now) {
        got.push((r.channel, r.bytes, r.arrival_ns));
    }
    e.input(None, &got, now);
    let out = e.step();
    for d in &out.datagrams {
        link.send_unreliable(d, now);
    }
    for (ch, b) in &out.reliable {
        link.send_reliable(*ch, b, now);
    }
}

/// A test link that records which threads polled it: `Decoder::pump` polls the link it decodes, so these are the
/// threads that decoded.
struct Where {
    inner: Loopback,
    threads: Arc<Mutex<Vec<String>>>,
}

impl Link for Where {
    fn send_unreliable(&mut self, bytes: &[u8], now_ns: u64) -> SendResult {
        self.inner.send_unreliable(bytes, now_ns)
    }
    fn send_reliable(&mut self, channel: Channel, bytes: &[u8], now_ns: u64) -> SendResult {
        self.inner.send_reliable(channel, bytes, now_ns)
    }
    fn poll_recv(&mut self, now_ns: u64) -> Option<Recv> {
        let name = std::thread::current()
            .name()
            .unwrap_or("unnamed")
            .to_owned();
        let mut t = self.threads.lock().unwrap_or_else(PoisonError::into_inner);
        if !t.contains(&name) {
            t.push(name);
        }
        drop(t);
        self.inner.poll_recv(now_ns)
    }
    fn state(&self) -> LinkState {
        self.inner.state()
    }
    fn rtt_ns(&self) -> Option<u64> {
        self.inner.rtt_ns()
    }
    fn max_datagram_size(&self) -> usize {
        self.inner.max_datagram_size()
    }
    fn unsent_unreliable(&self) -> usize {
        self.inner.unsent_unreliable()
    }
}

/// Waits (real clock, at most 3 s) until `client` has received a message `want` holds; returns all it got.
fn wait_for(client: &mut dyn Link, want: impl Fn(&Message) -> bool) -> Vec<(Channel, Message)> {
    let start = clock::now_ns();
    let mut got = Vec::new();
    while clock::now_ns() - start < 3_000 * MS {
        got.extend(received(client, clock::now_ns()));
        if got.iter().any(|(_, m)| want(m)) {
            return got;
        }
        std::thread::sleep(Duration::from_millis(2));
    }
    panic!("not received within 3 s; got {got:?}");
}

#[test]
fn s1_every_message_is_decoded_on_the_io_runtime_never_on_the_tick_thread() {
    let tl = TickLoop::start(settings(Scene::stress(), 2)).expect("starts");
    let (server, mut client) = Loopback::pair(1_200);
    let threads = Arc::new(Mutex::new(Vec::new()));
    let link = Where {
        inner: server,
        threads: Arc::clone(&threads),
    };
    assert!(tl.connect(Box::new(link), RttSource::Link));
    client.send_reliable(
        Channel::Control,
        &bytes(&Message::Join(join(0))),
        clock::now_ns(),
    );
    let got = wait_for(&mut client, |m| matches!(m, Message::FullState(_)));
    assert!(
        matches!(got[0], (Channel::Control, Message::JoinAccepted(_))),
        "JoinAccepted first, before any ping: {:?}",
        streams(&got)
    );
    assert_eq!(tl.stop().expect("the loop ends"), Ended::Clean);
    let threads = threads
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    println!("the link was polled on: {threads:?}");
    assert!(!threads.is_empty());
    assert!(threads.iter().all(|t| t == "npt-io"), "{threads:?}");
    assert!(!threads.iter().any(|t| t == TICK_THREAD));
    let end = received(&mut client, u64::MAX);
    assert!(
        end.iter().any(|(c, m)| *c == Channel::Control
            && matches!(
                m,
                Message::RoomEnded(RoomEnded {
                    reason: EndReason::Ended,
                    ..
                })
            )),
        "the stop sends RoomEnded: {end:?}"
    );
}

/// A test link whose smoothed RTT the test sets (a `wt` session's `Link::rtt_ns`).
struct FixedRtt {
    inner: Loopback,
    rtt: Arc<AtomicU64>,
}

impl Link for FixedRtt {
    fn send_unreliable(&mut self, bytes: &[u8], now_ns: u64) -> SendResult {
        self.inner.send_unreliable(bytes, now_ns)
    }
    fn send_reliable(&mut self, channel: Channel, bytes: &[u8], now_ns: u64) -> SendResult {
        self.inner.send_reliable(channel, bytes, now_ns)
    }
    fn poll_recv(&mut self, now_ns: u64) -> Option<Recv> {
        self.inner.poll_recv(now_ns)
    }
    fn state(&self) -> LinkState {
        self.inner.state()
    }
    fn rtt_ns(&self) -> Option<u64> {
        Some(self.rtt.load(Ordering::Relaxed))
    }
    fn max_datagram_size(&self) -> usize {
        self.inner.max_datagram_size()
    }
    fn unsent_unreliable(&self) -> usize {
        self.inner.unsent_unreliable()
    }
}

/// Register 96: the tick thread takes each session's `Decoder::rtt_ns` before every tick, as its session's latest
/// report gives it (a `wt` session's link estimate; a `ws` session's ping estimate, `None` before its first sample).
#[test]
fn s1_the_tick_thread_takes_each_sessions_decoder_rtt_before_every_tick() {
    let mut b = bench(Scene::stress(), 2);
    let rtt = Arc::new(AtomicU64::new(42 * MS));
    let (server, mut client) = Loopback::pair(1_200);
    let link = FixedRtt {
        inner: server,
        rtt: Arc::clone(&rtt),
    };
    let mut io = b.session(0, Box::new(link), 0);
    client.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
    let (server1, mut client1) = Loopback::pair(1_200);
    let mut pings = b.session_with(1, Box::new(server1), 0, RttSource::Pings, OUTBOUND_CAP);
    client1.send_reliable(Channel::Control, &bytes(&Message::Join(join(1))), 0);
    assert!(io.poll(0) && pings.poll(0));
    b.wake(0);
    assert_eq!(
        b.core.rtt_ns[0],
        Some(42 * MS),
        "slot 0: its link's estimate"
    );
    assert_eq!(b.core.rtt_ns[1], None, "slot 1 (ws): no ping sample yet");
    rtt.store(77 * MS, Ordering::Relaxed);
    assert!(io.poll(1) && pings.poll(1));
    b.wake(period());
    assert_eq!(
        b.core.rtt_ns[0],
        Some(77 * MS),
        "the next tick takes the next report's"
    );
}

/// C-U6-16 (c): each of `take_control`'s messages goes out on the stream its own framing names (npt-proto's
/// `channel_of`), at any length: a hand-built full state above 262,144 B (the room's own stay within
/// `Limits::FULL_STATE`: finding F5) is the full-state stream, where a copy of the lab's `stream_of` reads a control
/// type in its length's third byte; the room's `JoinAccepted` and `FullState` reach the client on their streams.
#[test]
fn s1_take_controls_messages_go_out_on_the_stream_their_framing_names_at_any_length() {
    let body = 262_144 + 4;
    let mut big = (body as u32).to_le_bytes().to_vec();
    big.push(3);
    big.resize(4 + body, 0);
    assert_eq!(reliable_channel(&big), Channel::FullState);
    let kick = bytes(&Message::Kick(Kick::Replaced));
    assert_eq!(reliable_channel(&kick), Channel::Control);
    let mut b = bench(Scene::stress(), 2);
    let (mut io, mut client) = joining(&b, 0, 0, 0);
    assert!(io.poll(0));
    b.wake(0);
    assert!(io.poll(1));
    let got = received(&mut client, 1);
    assert_eq!(
        streams(&got),
        [
            (Channel::Control, "JoinAccepted"),
            (Channel::FullState, "FullState")
        ]
    );
}

/// C-U6-14's first clause: a `Join` that replaces a live session's sends the old one `Kick(replaced)` and closes it
/// before anything else is routed (its `Leave`, queued after the new `Join`, never reaches the room, which would free
/// the new session's slot), and a session that sends `Leave` is closed and its slot freed.
#[test]
fn s1_a_replaced_session_is_kicked_and_closed_before_anything_else_is_routed_and_a_left_one_is_closed()
 {
    let mut b = bench(Scene::stress(), 2);
    let (mut a, mut a_client) = joining(&b, 0, 0, 0);
    assert!(a.poll(0));
    b.wake(0);
    assert!(a.poll(1));
    let joined = [
        (Channel::Control, "JoinAccepted"),
        (Channel::FullState, "FullState"),
    ];
    assert_eq!(streams(&received(&mut a_client, 1)), joined);
    let p = period();
    // B's `Join` for the same slot reaches the tick thread's queue first, then A's `Leave`.
    let (mut b_io, mut b_client) = joining(&b, 1, 0, p - 2);
    assert!(b_io.poll(p - 2));
    a_client.send_reliable(
        Channel::Control,
        &bytes(&Message::Leave(Leave::Quit)),
        p - 1,
    );
    assert!(a.poll(p - 1));
    b.wake(p);
    assert!(
        b.core.room.client(0).is_some(),
        "A's Leave was not routed: slot 0 is B's"
    );
    assert!(!a.poll(p + 1), "A's session is over");
    drop(a);
    let a_got = received(&mut a_client, p + 1);
    assert!(
        matches!(
            a_got.last(),
            Some((Channel::Control, Message::Kick(Kick::Replaced)))
        ),
        "A's last message is Kick(replaced): {a_got:?}"
    );
    assert_eq!(a_client.state(), LinkState::Closed, "then its link closed");
    assert!(b_io.poll(p + 1));
    assert_eq!(streams(&received(&mut b_client, p + 1)), joined);
    // B leaves: its slot is freed and its session closed.
    b_client.send_reliable(
        Channel::Control,
        &bytes(&Message::Leave(Leave::Quit)),
        p + 2,
    );
    assert!(b_io.poll(p + 2));
    b.wake(2 * p);
    assert!(b.core.room.client(0).is_none(), "the Leave freed slot 0");
    assert!(!b_io.poll(2 * p + 1), "B's session is over");
    drop(b_io);
    assert_eq!(b_client.state(), LinkState::Closed);
}

#[test]
fn s1_a_tick_thread_panic_sends_room_ended_crash_and_writes_the_crash_record_and_footer() {
    let sink = Sink::default();
    let mut s = settings(Scene::stress(), 2);
    s.log = Some(log_to(&sink));
    let tl = TickLoop::start_with(s, |c| c.panic_at = Some(8)).expect("starts");
    let (server, mut client) = Loopback::pair(1_200);
    assert!(tl.connect(Box::new(server), RttSource::Link));
    client.send_reliable(
        Channel::Control,
        &bytes(&Message::Join(join(0))),
        clock::now_ns(),
    );
    let got = wait_for(&mut client, |m| matches!(m, Message::RoomEnded(_)));
    assert!(
        got.iter().any(|(c, m)| *c == Channel::Control
            && matches!(
                m,
                Message::RoomEnded(RoomEnded {
                    reason: EndReason::Crash,
                    ..
                })
            )),
        "{got:?}"
    );
    assert_eq!(
        tl.stop().expect("the log ends"),
        Ended::Crash,
        "the caller's non-zero end"
    );
    let text = sink
        .0
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    let log = read_log(text.as_slice()).expect("the log reads back");
    assert_eq!(log.ticks.len(), 8, "ticks 0 to 7 logged; tick 8 panicked");
    let panic = log
        .events
        .iter()
        .find(|e| e.ev == "tick_panic")
        .expect("the crash record");
    assert!(
        panic
            .f
            .get("server_ns")
            .and_then(serde_json::Value::as_u64)
            .is_some()
    );
    assert_eq!(
        log.footer.map(|f| f.ended),
        Some(Ended::Crash),
        "the crash footer"
    );
}

#[test]
fn s1_phase_timers_give_the_loops_five_values_exactly() {
    let mut t = PhaseTimers::start(1_000, 1_250, 3, true, 400, true);
    t.inputs_done(1_300);
    t.room_done(1_900);
    t.end(2_050);
    assert_eq!(
        (
            t.work_ns(),
            t.start_jitter_ns(),
            t.dropped_ns(),
            t.overrun(),
            t.wake_ticks()
        ),
        (800, 250, 400, true, 3)
    );
    assert_eq!(t.phases_ns(), [50, 600, 150]);
    let later = PhaseTimers::start(1_000, 2_100, 3, false, 400, true);
    assert_eq!(
        (later.dropped_ns(), later.overrun()),
        (0, false),
        "the wake's values on its first tick only"
    );
    // Through the core and the writer's record: the loop's clock read at each edge, in order.
    let mut b = bench(Scene::stress(), 2);
    let reads = [100_u64, 110, 150, 170];
    let mut i = 0;
    b.core.wake(0, &mut || {
        let r = reads[i.min(3)];
        i += 1;
        r
    });
    assert_eq!(i, 4, "four reads: the start and three edges");
    let rows = b.ticks();
    let rec = tick_record(&rows[0], 2);
    let five = [
        "dropped_ns",
        "overrun",
        "start_jitter_ns",
        "wake_ticks_count",
        "work_ns",
    ];
    let v: Vec<(&str, i64)> = five.iter().map(|k| (*k, rec.v[*k])).collect();
    assert_eq!(
        v,
        [
            ("dropped_ns", 0),
            ("overrun", 0),
            ("start_jitter_ns", 100),
            ("wake_ticks_count", 1),
            ("work_ns", 70)
        ]
    );
}

/// The tick vCPU and the I/O vCPUs for a pinned test run: the last vCPU this thread may use and the others; `None`
/// with fewer than 2.
pub(super) fn vcpus() -> Option<(u32, Vec<u32>)> {
    let cpus = crate::proc_read::allowed(crate::proc_read::thread_tid()?)?;
    let (&tick, io) = cpus.split_last()?;
    (!io.is_empty()).then(|| (tick, io.to_vec()))
}

/// A pinned room (Block 4; Q-U6A-5 (a)): the tick thread's pin and the I/O threads' inherited one read back, the
/// affinity read at the start and at the end (`alone` and `sibling_free`, printed: in a container every process may
/// run on every vCPU, so `alone` is a reading here, never judged), and a second record each second with the tick
/// vCPU's steal, the RSS, the process's and the I/O runtime's CPU and each slot's session queue.
#[test]
fn s4_a_pinned_room_logs_its_affinity_at_start_and_end_and_its_per_second_reads() {
    let Some((tick, io)) = vcpus() else {
        eprintln!(
            "SKIPPED tick_loop::tests::s4_a_pinned_room_logs_its_affinity_at_start_and_end_and_its_per_second_reads: \
             fewer than 2 vCPUs for this thread"
        );
        return;
    };
    let sink = Sink::default();
    let mut s = settings(Scene::stress(), 2);
    s.log = Some(log_to(&sink));
    s.tick_vcpu = Some(tick);
    s.io_vcpus = io.clone();
    s.smt = Machine::read_local("C", "C", "", "", "", 1).smt;
    let tl = TickLoop::start(s).expect("starts pinned");
    let tick_tid = tl.shared.tick_tid.load(Ordering::Acquire);
    assert_eq!(
        crate::proc_read::allowed(tick_tid),
        Some(vec![tick]),
        "the tick thread's pin"
    );
    let io_tids = tl
        .shared
        .io_tids
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    assert_eq!(io_tids.len(), 2, "both I/O threads seen");
    for t in &io_tids {
        assert_eq!(
            crate::proc_read::allowed(*t).as_ref(),
            Some(&io),
            "I/O thread {t} inherited the I/O vCPUs"
        );
    }
    // On the loop's own progress (TR-U6A-4 (b)): until it has run 40 ticks (past a second's edge), at most 4 s.
    let start = clock::now_ns();
    while tl.ticks() < 40 && clock::now_ns() - start < 4_000 * MS {
        std::thread::sleep(Duration::from_millis(5));
    }
    assert!(tl.ticks() >= 40, "40 ticks within 4 s");
    assert_eq!(tl.stop().expect("the log ends"), Ended::Clean);
    let text = sink
        .0
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    let log = read_log(text.as_slice()).expect("the log reads back");
    let affinity: Vec<&EventRecord> = log.events.iter().filter(|e| e.ev == "affinity").collect();
    let whens: Vec<&str> = affinity
        .iter()
        .filter_map(|e| e.f["when"].as_str())
        .collect();
    assert_eq!(whens, ["start", "end"]);
    for e in &affinity {
        for k in ["alone", "sibling_free"] {
            assert!(matches!(e.f[k].as_i64(), Some(0 | 1)), "{k}: {e:?}");
        }
    }
    assert!(
        log.seconds.len() >= 2,
        "{} second records",
        log.seconds.len()
    );
    // The last record is the close's partial second: its steal read may come within one jiffy of the one before (no
    // time passed: no share, its key left out; FP-13), so only its other keys are required.
    let last = log.seconds.len() - 1;
    for (i, r) in log.seconds.iter().enumerate() {
        for k in [
            "steal_permille",
            "rss_bytes",
            "proc_cpu_permille",
            "io_cpu_permille",
        ] {
            let exempt = i == last && k == "steal_permille";
            assert!(exempt || r.v.contains_key(k), "second {}: {k}", r.sec);
        }
        assert!(r.v["rss_bytes"] > 0);
        assert_eq!(r.s["out_queue_count"].len(), 2, "one per slot");
    }
    println!(
        "pinned (container readings): tick vCPU {tick}, I/O vCPUs {io:?}; affinity {:?}; seconds {:?}",
        affinity
            .iter()
            .map(|e| (&e.f["when"], &e.f["alone"], &e.f["sibling_free"]))
            .collect::<Vec<_>>(),
        log.seconds
            .iter()
            .map(|r| (r.sec, r.v.clone()))
            .collect::<Vec<_>>()
    );
}

/// NR-U6A-4 (TR-U6A-9's steal part): a per-second read that fails leaves its key out of the record (a gap, never a
/// best value) and sets its failed flag (its one stderr line); a good read puts its keys in.
#[test]
fn s4_a_failed_per_second_read_leaves_its_key_out() {
    let mut s = Seconds::new(1, Some(3), Arc::new(Shared::default()));
    assert_eq!(s.baseline("cpu3 100 0 10 1000 0 0 0 0 0 0\n"), Ok(None));
    for bad in ["", "cpu0 100 0 10 1900 0 0 0 100 0 0\n"] {
        let steal = s.baseline(bad);
        let mut v = BTreeMap::new();
        s.values(&mut v, steal, None, Some((5, 1)), 10);
        assert!(
            !v.contains_key("steal_permille") && !v.contains_key("rss_bytes"),
            "{bad:?}: {v:?}"
        );
        assert_eq!((v["proc_cpu_permille"], v["io_cpu_permille"]), (500, 100));
        assert_eq!(s.failed, [true, true, false], "{bad:?}");
    }
    let steal = s.baseline("cpu3 100 0 10 1900 0 0 0 100 0 0\n");
    let mut v = BTreeMap::new();
    s.values(&mut v, steal, Some(4_096), None, 10);
    assert_eq!(
        (v["steal_permille"], v["rss_bytes"]),
        (100, 4_096),
        "100 of 1,000 ticks"
    );
    assert!(
        !v.contains_key("proc_cpu_permille") && s.failed[2],
        "the CPU times not read"
    );
}

/// FP-13 (TR2-U6A-1, NR2-U6A-3): a steal read with no time since the last (two equal /proc/stat texts) has no share:
/// its key is left out, with no failure flag (so no stderr line).
#[test]
fn s4_a_steal_read_with_no_time_passed_leaves_its_key_out_with_no_failure() {
    let mut s = Seconds::new(1, Some(3), Arc::new(Shared::default()));
    let stat = "cpu3 100 0 10 1000 0 0 0 0 0 0\n";
    assert_eq!(s.baseline(stat), Ok(None), "the baseline");
    let steal = s.baseline(stat);
    let mut v = BTreeMap::new();
    s.values(&mut v, steal, Some(4_096), Some((5, 1)), 10);
    assert!(!v.contains_key("steal_permille"), "{v:?}");
    assert_eq!(s.failed, [false; 3], "no read failed");
}

/// The shared-vcpu bite (bites builds only).
#[cfg(feature = "bites")]
mod bites {
    use super::*;

    /// PM-0001-1/shared-vcpu: a second thread pinned to the tick vCPU before the start's read. The read counts it (its
    /// pin reads back as exactly the tick vCPU, and it is among the tasks against `alone`), and the log's affinity
    /// events fail -1's check "tick thread alone on its vCPU, at start and end" (min == 1).
    #[test]
    fn pm_0001_1_shared_vcpu_fails_the_alone_check() {
        let Some((tick, io)) = vcpus() else {
            eprintln!(
                "SKIPPED tick_loop::tests::bites::pm_0001_1_shared_vcpu_fails_the_alone_check: fewer than 2 vCPUs"
            );
            return;
        };
        let sink = Sink::default();
        let mut s = settings(Scene::stress(), 2);
        s.log = Some(log_to(&sink));
        s.tick_vcpu = Some(tick);
        s.io_vcpus = io;
        s.bite = Some("PM-0001-1/shared-vcpu".to_owned());
        let tl = TickLoop::start(s).expect("starts pinned");
        let bite = tl.shared.bite_tid.load(Ordering::Acquire);
        let tick_tid = tl.shared.tick_tid.load(Ordering::Acquire);
        assert!(bite != 0 && bite != tick_tid);
        assert_eq!(
            crate::proc_read::allowed(bite),
            Some(vec![tick]),
            "the second thread is on the tick vCPU"
        );
        let against =
            crate::proc_read::not_alone(&crate::proc_read::tasks("/proc"), tick_tid, tick);
        assert!(against.contains(&bite), "the read counts it: {against:?}");
        assert_eq!(tl.stop().expect("the log ends"), Ended::Clean);
        let text = sink
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone();
        let log = read_log(text.as_slice()).expect("reads back");
        let alone: Vec<i64> = log
            .events
            .iter()
            .filter(|e| e.ev == "affinity")
            .filter_map(|e| e.f["alone"].as_i64())
            .collect();
        assert_eq!(alone.len(), 2, "both reads present");
        assert_eq!(
            alone.iter().min(),
            Some(&0),
            "PM-0001-1's check 'tick thread alone on its vCPU, at start and end' fails: alone 0"
        );
    }
}

/// The sim's own events of the tick it just ran, from its canonical dump (npt-lab's reading, logs.rs:98-121): its
/// shots, and its hits and kills.
fn dump_events(world: &World) -> (u32, u32) {
    let dump = world.dump();
    let words = dump.words();
    let a = (0..words.len())
        .find(|&i| dump.field(i).as_deref() == Some("events[0].count"))
        .expect("the dump's events count");
    let n = words[a] as usize;
    let kind = |i: usize| words[a + 1 + 4 * i];
    let shots = (0..n).filter(|&i| kind(i) == 1).count() as u32;
    let reliable = (0..n).filter(|&i| kind(i) == 2 || kind(i) == 3).count() as u32;
    (shots, reliable)
}

/// What a test's independent count keeps from the tick before (npt-lab's `RoomTrack`, by maps).
#[derive(Default)]
struct Mirror {
    doors: BTreeMap<u16, i32>,
    projectiles: BTreeMap<u16, (u16, [i32; npt_sim::EntityRecord::FIELDS])>,
}

/// The -2 counts of the tick the room just ran, counted again the lab's way (maps, the dump's events).
fn counted(room: &Room<Tallied>, m: &mut Mirror) -> SceneRow {
    use npt_sim::EntityKind as K;
    let world = &room.sim().world;
    let records = world.entity_records();
    let scene = world.scene();
    let t = room.last_tick();
    let count = |k: K| records.iter().filter(|r| r.kind == k).count() as u32;
    let mut teams: BTreeMap<u8, u32> = BTreeMap::new();
    let mut parts: BTreeMap<u16, u16> = BTreeMap::new();
    let mut effects: BTreeMap<u16, u32> = BTreeMap::new();
    let (mut portal, mut spawns, mut changes, mut stand_spawns) = (0, 0, 0, 0);
    let mut live = BTreeMap::new();
    for r in records {
        match r.kind {
            K::Soldier => {
                *teams
                    .entry(scene.team(r.index as u8).expect("a slot"))
                    .or_default() += 1
            }
            K::VehiclePart => *parts.entry(r.grid[5] as u16).or_default() += 1,
            K::Effect if (r.grid[1] as u32).wrapping_sub(t) as i32 > 0 => {
                *effects
                    .entry(r.index / u16::from(scene.effect_rows().max(1)))
                    .or_default() += 1;
            }
            K::Door => {
                if m.doors
                    .insert(r.index, r.grid[0])
                    .is_some_and(|o| o != r.grid[0])
                {
                    portal += 1;
                }
            }
            K::Projectile => {
                let stand_in = r.grid[7] == 0;
                match m.projectiles.get(&r.index) {
                    Some((g, grid)) if *g == r.generation => {
                        changes += u32::from(stand_in && *grid != r.grid)
                    }
                    _ => {
                        spawns += 1;
                        stand_spawns += u32::from(stand_in);
                    }
                }
                live.insert(r.index, (r.generation, r.grid));
            }
            _ => {}
        }
    }
    m.projectiles = live;
    let (shots, reliable) = dump_events(world);
    let snapshot_clients = (0..scene.slots())
        .filter_map(|s| room.client(s))
        .filter(|c| !c.kicked() && c.counters().snapshots > 0)
        .count() as u32;
    // The lab's rule (npt-lab mirror.rs:324-346, TR-U6A-2): every other soldier, and ES's vehicles, in the client's
    // set; the room's `ClientSlot::relevant` stands for the client's frame here. No own-soldier condition.
    let with_vehicles = scene.name() == "ES";
    let mut everyone = 0u64;
    for s in (0..scene.slots()).filter(|&s| usize::from(s) < LOGGED_SLOTS) {
        let Some(c) = room.client(s).filter(|c| !c.kicked()) else {
            continue;
        };
        let seen = |k: K, skip: Option<u16>| {
            records
                .iter()
                .enumerate()
                .filter(|(_, r)| r.kind == k && Some(r.index) != skip)
                .all(|(i, _)| c.relevant().contains(&i))
        };
        if seen(K::Soldier, Some(u16::from(s))) && (!with_vehicles || seen(K::Vehicle, None)) {
            everyone |= 1 << s;
        }
    }
    SceneRow {
        soldiers: count(K::Soldier),
        smallest_team: teams.values().copied().min().unwrap_or(0),
        snapshot_clients,
        big_vehicles: count(K::Vehicle),
        very_large_units: parts.values().filter(|&&p| p >= 20).count() as u32,
        projectiles: count(K::Projectile),
        effects_max: effects.values().copied().max().unwrap_or(0),
        per_second: [shots, reliable, portal, spawns, changes, stand_spawns],
        everyone,
    }
}

/// Two clients on `b`, each a real endpoint commanded by npt-sim's synthetic recording, stepped on the injected clock
/// with their sessions; `after` sees the bench after each tick.
fn drive(b: &mut Bench, ticks: u64, mut after: impl FnMut(&mut Bench)) {
    let scene = b.core.room.sim().world.scene().clone();
    let first = b.core.room.last_tick().wrapping_add(1);
    let rec = npt_sim::replay::Recording::synthetic(&scene, 3, first, 200, 2_000);
    let mut script: BTreeMap<(u8, u32), npt_sim::Command> = BTreeMap::new();
    for t in &rec.ticks {
        for (c, _) in &t.commands {
            script.entry((c.slot, c.tick)).or_insert(*c);
        }
    }
    let mut clients: Vec<(SessionIo, Loopback, ClientEndpoint)> = (0..2u8)
        .map(|slot| {
            let (server, link) = Loopback::pair(1_200);
            let io = b.session(u32::from(slot), Box::new(server), 0);
            (io, link, ClientEndpoint::new(join(slot), npt_sim::TICK_HZ))
        })
        .collect();
    for k in 0..ticks {
        let now = due(k);
        for (io, link, e) in &mut clients {
            let mut got = Vec::new();
            while let Some(r) = link.poll_recv(now) {
                got.push((r.channel, r.bytes, r.arrival_ns));
            }
            e.input(None, &got, now);
            let stamp = e.newest_tick().map(|n| n.wrapping_add(1));
            if let (Some(slot), Some(t)) = (e.slot(), stamp)
                && let Some(c) = script.get(&(slot, t))
            {
                e.input(Some(*c), &[], now);
            }
            let out = e.step();
            for d in &out.datagrams {
                link.send_unreliable(d, now);
            }
            for (ch, bytes) in &out.reliable {
                link.send_reliable(*ch, bytes, now);
            }
            assert!(io.poll(now + 1));
        }
        b.wake(now + 2);
        for (io, _, _) in &mut clients {
            assert!(io.poll(now + 3));
        }
        after(b);
    }
}

/// Q-U6A-2 (a): the room log's PM-0001-2 counts on a fixed SS run (two clients firing by the synthetic recording) are,
/// tick by tick, the sim's own: the records counted again by maps, the shots, hits and kills from the sim's dump, and
/// `everyone_in_view` by the lab's rule (TR-U6A-2): the whole row compared.
#[test]
fn s5_the_room_logs_minus_2_counts_equal_the_sims_own_tick_by_tick() {
    let mut b = bench(Scene::stress(), 2);
    let mut mirror = Mirror::default();
    let mut totals = [0u32; 6];
    let mut seen = 0;
    drive(&mut b, 90, |b| {
        let rows = b.ticks();
        let row = rows.last().expect("a tick ran");
        let want = counted(&b.core.room, &mut mirror);
        assert_eq!(row.scene, want, "tick {}", row.t);
        seen += u32::from(want.everyone != 0);
        for (t, x) in totals.iter_mut().zip(row.scene.per_second) {
            *t += x;
        }
    });
    println!(
        "-2 over 90 ticks: shots, reliable, portal changes, spawns, stand-in changes, stand-in spawns {totals:?}; \
         ticks with an everyone_in_view bit {seen}"
    );
    assert!(
        seen > 0,
        "a client saw everyone, so the flag was compared set"
    );
    assert!(
        totals[0] > 0,
        "shots were fired, so the shots' count was tried"
    );
    assert!(
        totals[2] > 0 && totals[3] > 0,
        "doors changed and projectiles spawned"
    );
}

/// `tick.s.everyone_in_view` from `ClientSlot::relevant`: 1 for a joined client whose set holds every other soldier,
/// 0 once one soldier is moved past the interest radius, and 0 for a slot with no client.
#[test]
fn s5_everyone_in_view_is_1_then_0_with_one_soldier_moved_past_the_radius() {
    let mut b = bench(Scene::stress(), 2);
    let mut flags = Vec::new();
    drive(&mut b, 12, |b| {
        let row = *b.ticks().last().expect("a tick ran");
        flags.push(row.scene.everyone);
        if b.core.run == 10 {
            // Slot 5's soldier, 75 m from the cluster (the radius is 64 m), from the next tick on.
            let far = npt_sim::Placement {
                slot: 5,
                position: [8_500, 0, 0],
            };
            b.core.room.script(&[], &[far], &[]);
        }
    });
    println!("everyone_in_view bits by tick: {flags:?}");
    assert_eq!(flags[9], 0b11, "both joined clients see everyone");
    assert_eq!(flags[11] & 0b11, 0, "neither does with slot 5 far away");
    let row = TickRow {
        t: 0,
        ph: Phase::Measured,
        timers: PhaseTimers::default(),
        start_ns: 0,
        out_queue: [0; LOGGED_SLOTS],
        scene: SceneRow {
            everyone: 0b01,
            ..SceneRow::default()
        },
    };
    assert_eq!(
        tick_record(&row, 3).s["everyone_in_view"],
        [1, 0, 0],
        "one flag per slot of the roster, a slot with no client 0"
    );
}

/// The writer never blocks the tick (marks/README.md §1): with the writer stuck, every due tick still runs and each
/// row beyond the queue's room is counted as dropped.
#[test]
fn s5_the_tick_never_waits_on_a_stuck_writer() {
    let extra = 40;
    // The wakes on their own thread, awaited at most 5 s (TR-U6A-7): a row offer that waited fails here, never hangs.
    let (tx, rx) = std::sync::mpsc::channel();
    let wakes = std::thread::spawn(move || {
        let mut b = bench(Scene::course(), 1);
        for k in 0..(ROWS_CAP as u64 + extra) {
            b.wake(due(k));
        }
        let _ = tx.send((b.core.run, b.shared.rows_dropped.load(Ordering::Relaxed)));
    });
    let (run, dropped) = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("the wakes ended within 5 s");
    wakes.join().expect("the wakes' thread");
    assert_eq!(run, ROWS_CAP as u64 + extra, "every due tick ran");
    assert_eq!(dropped, extra, "the rows beyond the queue's room");
}

/// SERIES.md's rule (the advisor's C2): every counter on every record, 0 included: each tick record carries the loop's
/// five names, the seven room counts and one `everyone_in_view` flag per slot; each second record the six per-second
/// counts, the resident memory, the two CPU shares and one queue length per slot.
#[test]
fn s5_every_counter_is_on_every_record() {
    let sink = Sink::default();
    let mut s = settings(Scene::stress(), 2);
    s.log = Some(log_to(&sink));
    let tl = TickLoop::start(s).expect("starts");
    let (server, mut link) = Loopback::pair(1_200);
    assert!(tl.connect(Box::new(server), RttSource::Link));
    let mut e = ClientEndpoint::new(join(0), npt_sim::TICK_HZ);
    // On the loop's own progress (TR-U6A-4 (a)): until it has run 40 ticks (1.3 s of its clock, so past a second's
    // edge) and the client has a snapshot, at most 4 s.
    let start = clock::now_ns();
    while (tl.ticks() < 40 || e.newest_tick().is_none()) && clock::now_ns() - start < 4_000 * MS {
        step_client(&mut e, &mut link, clock::now_ns());
        std::thread::sleep(Duration::from_millis(5));
    }
    assert!(
        tl.ticks() >= 40 && e.newest_tick().is_some(),
        "40 ticks and a snapshot within 4 s"
    );
    assert_eq!(tl.stop().expect("the log ends"), Ended::Clean);
    let text = sink
        .0
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    let log = read_log(text.as_slice()).expect("reads back");
    let tick_names = [
        "work_ns",
        "start_jitter_ns",
        "dropped_ns",
        "overrun",
        "wake_ticks_count",
        "soldiers_count",
        "smallest_team_count",
        "snapshot_clients_count",
        "big_vehicles_count",
        "very_large_units_count",
        "projectiles_count",
        "effects_per_soldier_max_count",
    ];
    assert!(!log.ticks.is_empty());
    for r in &log.ticks {
        for n in tick_names {
            assert!(r.v.contains_key(n), "tick {}: {n}", r.t);
        }
        assert_eq!(r.s["everyone_in_view"].len(), 2, "tick {}", r.t);
        assert_eq!(r.v["soldiers_count"], 48);
    }
    assert!(
        log.ticks.iter().any(|r| r.s["everyone_in_view"][0] == 1),
        "the client saw everyone"
    );
    assert!(log.seconds.len() >= 2);
    for r in &log.seconds {
        for n in PER_SECOND
            .iter()
            .chain(&["rss_bytes", "proc_cpu_permille", "io_cpu_permille"])
        {
            assert!(r.v.contains_key(*n), "second {}: {n}", r.sec);
        }
        assert_eq!(r.s["out_queue_count"].len(), 2);
    }
}

#[test]
fn s1_smoke_two_clients_for_two_seconds_on_the_real_clock() {
    let sink = Sink::default();
    let mut s = settings(Scene::stress(), 2);
    s.log = Some(log_to(&sink));
    let tl = TickLoop::start(s).expect("starts");
    let mut clients: Vec<(Loopback, ClientEndpoint)> = (0..2)
        .map(|slot| {
            let (server, client) = Loopback::pair(1_200);
            assert!(tl.connect(Box::new(server), RttSource::Link));
            (client, ClientEndpoint::new(join(slot), npt_sim::TICK_HZ))
        })
        .collect();
    let start = clock::now_ns();
    while clock::now_ns() - start < 2_000 * MS {
        for (link, endpoint) in &mut clients {
            step_client(endpoint, link, clock::now_ns());
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    let ticks = tl.ticks();
    assert_eq!(tl.stop().expect("the log ends"), Ended::Clean);
    let text = sink
        .0
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    let log = read_log(text.as_slice()).expect("the log reads back");
    let overruns: i64 = log.ticks.iter().map(|t| t.v["overrun"]).sum();
    let wake_max = log.ticks.iter().map(|t| t.v["wake_ticks_count"]).max();
    let jitter_max = log.ticks.iter().map(|t| t.v["start_jitter_ns"]).max();
    let newest: Vec<Option<u32>> = clients.iter().map(|c| c.1.newest_tick()).collect();
    println!(
        "smoke (container reading, not judged): {ticks} ticks in 2 s, {overruns} overruns, most ticks in a wake \
         {wake_max:?}, largest start jitter {jitter_max:?} ns; the clients' newest ticks {newest:?}"
    );
    assert!(ticks > 0);
    assert!(
        log.ticks.len() as u64 >= ticks,
        "one record per tick (ticks run after the count above are logged too)"
    );
    assert_eq!(log.footer.map(|f| f.ended), Some(Ended::Clean));
    for (i, (_, e)) in clients.iter().enumerate() {
        assert_eq!(e.slot(), Some(i as u8), "client {i} joined");
        assert!(
            e.newest_tick().is_some(),
            "client {i} decoded the room's state"
        );
    }
}
