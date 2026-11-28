//! The Loopback lab for U4e's tests (u4-build-plan.md §5 Lanes: in npt-server's own tests, through its npt-link and its
//! dev dependency on npt-client; never in crates/lab): a `Room<World>` behind one `Decoder` per session, each session a
//! Loopback pair to a real `ClientEndpoint`, on an injected clock. The server ticks at U4b's schedule
//! (base + ⌊k × 10^9 / tick_hz⌋ ns); each client steps at its own offset after each server tick. A test-only `Link`
//! wrapper, [`Gate`], drops every datagram in a window both ways (2.5's gap; Loopback itself neither delays nor loses).
#![allow(dead_code, reason = "each test binary uses part of the lab")]

use std::cell::Cell;
use std::collections::BTreeMap;
use std::rc::Rc;

use npt_client::ClientEndpoint;
use npt_link::{Channel, Link, LinkState, Loopback, Recv, SendResult};
use npt_proto::{Join, Limits, Message, Quant};
use npt_room::Room;
use npt_server::{Decoder, RttSource};
use npt_sim::replay::Recording;
use npt_sim::{Command, Scene, World};

pub const VERSION: u16 = 1;
pub const CONTENT: [u8; 32] = [7; 32];
/// Loopback's datagram limit for the lab's sessions (the link's own; the room's groups stay within 1,000 B).
pub const MAX_DATAGRAM: usize = 1_200;

pub fn token(slot: u8) -> [u8; 16] {
    let mut t = [0x5a; 16];
    t[0] = slot;
    t
}

pub fn bind(slot: u8) -> [u8; 16] {
    [slot ^ 0xa5; 16]
}

pub fn join(slot: u8) -> Join {
    Join {
        token: token(slot),
        bind: bind(slot),
        version: VERSION,
        content_hash: CONTENT,
    }
}

/// The stream a reliable message the room encoded travels on: its type byte's row of `Limits::TYPES` (npt-proto's rule,
/// limits.rs:55), the type byte read after the stream's length prefix (2 B on the control stream, 4 B on the
/// full-state stream: the catalogue's §1).
pub fn stream_of(bytes: &[u8]) -> Channel {
    [(Channel::Control, 2), (Channel::FullState, 4)]
        .into_iter()
        .find(|&(channel, at)| {
            bytes
                .get(at)
                .and_then(|&t| usize::from(t).checked_sub(1))
                .and_then(|i| Limits::TYPES.get(i))
                .is_some_and(|r| r.1 == channel)
        })
        .map_or(Channel::Control, |(channel, _)| channel)
}

/// A window of ns in which a [`Gate`] drops every datagram it is asked to send.
#[derive(Clone, Default)]
pub struct Window(Rc<Cell<(u64, u64)>>);

impl Window {
    pub fn set(&self, from_ns: u64, to_ns: u64) {
        self.0.set((from_ns, to_ns));
    }
    fn holds(&self, now_ns: u64) -> bool {
        let (a, b) = self.0.get();
        now_ns >= a && now_ns < b
    }
}

/// A test-only `Link` over a Loopback end: datagrams sent inside its window are dropped (counted), every other call is
/// the Loopback's.
pub struct Gate {
    inner: Loopback,
    window: Window,
    pub dropped: Rc<Cell<u64>>,
}

impl Link for Gate {
    fn send_unreliable(&mut self, bytes: &[u8], now_ns: u64) -> SendResult {
        if self.window.holds(now_ns) {
            self.dropped.set(self.dropped.get() + 1);
            return SendResult::Sent;
        }
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
        self.inner.rtt_ns()
    }
    fn max_datagram_size(&self) -> usize {
        self.inner.max_datagram_size()
    }
    fn unsent_unreliable(&self) -> usize {
        self.inner.unsent_unreliable()
    }
}

/// What a client's command is, from its slot and the tick it will stamp.
pub type Script = Box<dyn FnMut(u8, u32) -> Option<Command>>;

/// The commands of a synthetic recording (npt-sim's seeded generator), by (slot, tick): each slot's first command of
/// each tick, as the page's command for the tick the client stamps it with.
pub fn recorded(scene: &Scene, seed: u64, first_tick: u32, ticks: u32) -> Script {
    let rec = Recording::synthetic(scene, seed, first_tick, ticks, 2_000);
    let mut by: BTreeMap<(u8, u32), Command> = BTreeMap::new();
    for t in &rec.ticks {
        for (c, _) in &t.commands {
            by.entry((c.slot, c.tick)).or_insert(*c);
        }
    }
    Box::new(move |slot, tick| by.get(&(slot, tick)).copied())
}

/// One session: the server's end with its decoder, the client's end with its endpoint, and what each side filed.
pub struct Conn {
    pub server: Gate,
    pub client: Gate,
    pub decoder: Decoder,
    pub endpoint: ClientEndpoint,
    pub slot: Option<u8>,
    pub offset_ns: u64,
    pub step: u64,
    /// Every (tick, hash) the client filed, in order (0 the sentinel).
    pub filed: Vec<(u32, u64)>,
    /// The runs of ticks the client resolved lost.
    pub lost: Vec<(u32, u32)>,
    pub wraps: Vec<&'static str>,
}

/// The lab (the module text).
pub struct Lab {
    pub room: Room<World>,
    pub conns: Vec<Conn>,
    pub hz: u32,
    pub base_ns: u64,
    pub k: u64,
    pub now_ns: u64,
    pub script: Script,
    /// The room's hash per (slot, tick).
    pub server: BTreeMap<(u8, u32), u64>,
    /// The client hashes per (slot, tick): a second, different hash makes it the sentinel (marks/README.md §1).
    pub client: BTreeMap<(u8, u32), u64>,
    /// When each server tick ran, ns.
    pub tick_at: BTreeMap<u32, u64>,
    /// The window every session's gates share (both ways).
    pub window: Window,
    /// The bytes the pump reported to the room per (slot, tick) before that tick ran.
    pub pumped: BTreeMap<(u8, u32), u32>,
    /// Drops the room's datagram (slot, tick, group) when it answers true (e2's and e3's lost group).
    pub tamper: Option<Tamper>,
}

/// Which of the room's datagrams the lab drops, by (slot, tick, group).
pub type Tamper = Box<dyn FnMut(u8, u32, usize) -> bool>;

impl Lab {
    /// A lab of `endpoints.len()` sessions on `room`, each client stepping `offsets[i]` ns after each server tick.
    pub fn new(
        room: Room<World>,
        endpoints: Vec<ClientEndpoint>,
        offsets: &[u64],
        script: Script,
    ) -> Lab {
        Lab::sized(room, endpoints, offsets, script, MAX_DATAGRAM)
    }

    /// The same, each session's Loopback taking datagrams up to `max_datagram` bytes.
    pub fn sized(
        room: Room<World>,
        endpoints: Vec<ClientEndpoint>,
        offsets: &[u64],
        script: Script,
        max_datagram: usize,
    ) -> Lab {
        let hz = npt_sim::TICK_HZ;
        let base_ns = 1_000_000_000;
        let window = Window::default();
        let conns = endpoints
            .into_iter()
            .enumerate()
            .map(|(i, endpoint)| {
                let (s, c) = Loopback::pair(max_datagram);
                Conn {
                    server: Gate {
                        inner: s,
                        window: window.clone(),
                        dropped: Rc::default(),
                    },
                    client: Gate {
                        inner: c,
                        window: window.clone(),
                        dropped: Rc::default(),
                    },
                    decoder: Decoder::new(
                        Quant::default(),
                        hz,
                        base_ns,
                        VERSION,
                        CONTENT,
                        RttSource::Link,
                    ),
                    endpoint,
                    slot: None,
                    offset_ns: offsets.get(i).copied().unwrap_or(1_000_000),
                    step: 0,
                    filed: Vec::new(),
                    lost: Vec::new(),
                    wraps: Vec::new(),
                }
            })
            .collect();
        Lab {
            room,
            conns,
            hz,
            base_ns,
            k: 0,
            now_ns: base_ns,
            script,
            server: BTreeMap::new(),
            client: BTreeMap::new(),
            tick_at: BTreeMap::new(),
            window,
            pumped: BTreeMap::new(),
            tamper: None,
        }
    }

    /// The same lab with session `i`'s pump decoding with `quant`.
    pub fn with_decoder(mut self, i: usize, quant: Quant) -> Lab {
        self.conns[i].decoder = Decoder::new(
            quant,
            self.hz,
            self.base_ns,
            VERSION,
            CONTENT,
            RttSource::Link,
        );
        self
    }

    /// The time of server tick `k` (U4b's schedule).
    pub fn tick_ns(&self, k: u64) -> u64 {
        self.base_ns + (u128::from(k) * 1_000_000_000 / u128::from(self.hz)) as u64
    }

    fn server_tick(&mut self, now: u64) {
        let tick = self.room.last_tick().wrapping_add(1);
        for conn in &mut self.conns {
            loop {
                let mut msgs = Vec::new();
                conn.decoder.pump(&mut conn.server, now, tick, &mut msgs);
                let mut joined = false;
                for (m, _) in msgs {
                    match (&m, conn.slot) {
                        (Message::Join(j), _) => {
                            joined = true;
                            match self.room.join(j) {
                                Ok((slot, _)) => {
                                    conn.decoder.accept();
                                    conn.slot = Some(slot);
                                }
                                Err(r) => conn.decoder.reject(&mut conn.server, r, now),
                            }
                        }
                        (_, Some(slot)) => self.room.receive(slot, &m),
                        _ => {}
                    }
                }
                if !joined {
                    break;
                }
            }
            if let Some(slot) = conn.slot {
                let sent = conn.decoder.take_sent_bytes();
                self.pumped.insert((slot, tick), sent);
                self.room
                    .pump_report(slot, sent, conn.server.max_datagram_size());
            }
        }
        self.room.tick();
        let t = self.room.last_tick();
        self.tick_at.insert(t, now);
        for conn in &mut self.conns {
            let Some(slot) = conn.slot else { continue };
            for (g, d) in self.room.take_datagrams(slot).into_iter().enumerate() {
                if self.tamper.as_mut().is_some_and(|f| f(slot, t, g)) {
                    continue;
                }
                conn.server.send_unreliable(&d, now);
            }
            // The room encoded each once (A1); the lab sends the bytes as they are, on their type's stream.
            for b in self.room.take_control(slot) {
                conn.server.send_reliable(stream_of(&b), &b, now);
            }
            if let Some(h) = self.room.client(slot).and_then(|c| c.server_hash()) {
                self.server.insert((slot, t), h);
            }
        }
    }

    fn client_step(&mut self, i: usize, now: u64) {
        let conn = &mut self.conns[i];
        let mut received = Vec::new();
        while let Some(r) = conn.client.poll_recv(now) {
            received.push((r.channel, r.bytes, r.arrival_ns));
        }
        conn.endpoint.input(None, &received, now);
        // The page's command for the tick the endpoint stamps it with (its newest tick plus its lead of 1).
        let stamp = conn.endpoint.newest_tick().map(|n| n.wrapping_add(1));
        if let (Some(slot), Some(t)) = (conn.endpoint.slot(), stamp)
            && let Some(c) = (self.script)(slot, t)
        {
            conn.endpoint.input(Some(c), &[], now);
        }
        let out = conn.endpoint.step();
        for d in &out.datagrams {
            conn.client.send_unreliable(d, now);
        }
        for (ch, b) in &out.reliable {
            conn.client.send_reliable(*ch, b, now);
        }
        let slot = conn.endpoint.slot().unwrap_or(u8::MAX);
        for &(t, h) in &out.hashes {
            conn.filed.push((t, h));
            let e = self.client.entry((slot, t)).or_insert(h);
            if *e != h {
                *e = 0;
            }
        }
        conn.lost.extend_from_slice(&out.lost);
        conn.wraps.extend_from_slice(&out.wraps);
        conn.step += 1;
    }

    /// Runs `ticks` server ticks, each client stepping at its offset after each; `after` sees the lab after each
    /// server tick.
    pub fn run(&mut self, ticks: u64, mut after: impl FnMut(&Lab)) {
        for _ in 0..ticks {
            let at = self.tick_ns(self.k);
            let next = self.tick_ns(self.k + 1);
            self.now_ns = at;
            self.server_tick(at);
            after(self);
            let mut order: Vec<(u64, usize)> = (0..self.conns.len())
                .map(|i| (at + self.conns[i].offset_ns.min(next - at - 1), i))
                .collect();
            order.sort_unstable();
            for (t, i) in order {
                self.now_ns = t;
                self.client_step(i, t);
            }
            self.k += 1;
        }
    }

    /// Every (slot, tick) both sides hashed, and how many pairs differ or are the sentinel.
    pub fn compare(&self) -> (usize, usize) {
        let mut compared = 0;
        let mut bad = 0;
        for (k, h) in &self.client {
            compared += 1;
            if *h == 0 || self.server.get(k) != Some(h) {
                bad += 1;
            }
        }
        (compared, bad)
    }
}

/// `n` endpoints that join as slots 0 to n-1.
pub fn endpoints(n: u8) -> Vec<ClientEndpoint> {
    (0..n)
        .map(|s| ClientEndpoint::new(join(s), npt_sim::TICK_HZ))
        .collect()
}

/// A room on `scene` from `seed`, its first tick `first_tick`, with a roster of `n` slots.
pub fn room(scene: &Scene, seed: u64, first_tick: u32, n: u8) -> Room<World> {
    let world = World::new(scene, seed, first_tick);
    let greybox = world.greybox().clone();
    Room::new(
        world,
        scene,
        &greybox,
        (0..n).map(token).collect(),
        usize::from(n),
        npt_sim::TICK_HZ,
    )
    .with_hashes(CONTENT, [0; 32])
}
