//! C-U6-17's test (plan.md:1254; u6a-brief.md Block 7 and condition 5): "one 600 ms pong and a 10 ms path, under
//! option B's buffer of 8, with no kind-10 refusal on the honest client". The honest client is a real endpoint under
//! U4f's controller (max lead 9, the lab's), against a room and its decoder on an injected clock, over a test-local
//! delaying `Link` on Loopback (common/mod.rs's `Gate` pattern: Loopback itself neither delays nor loses), never L1.
//!
//! The 600 ms exchange is run both ways round: slow on its way down (the `Pong` late) and slow on its way up (its
//! `Ping` late). Both lift the client's upper one-way estimate past the controller's cap of 9; the slow way down also
//! lifts its clock offset by about 15 ms (300 ms in the mean of about 20 samples), which by itself pulls its stamps
//! back about half a tick on this path, so only the slow way up shows what the cap prevents (U6a's notes, P7: without
//! the cap the slow way down still passes). Both ways run under option A as well, each option set on the room
//! (TR-U6A-3).

use std::collections::VecDeque;

use npt_client::ClientEndpoint;
use npt_link::{Channel, Link, LinkState, Loopback, Recv, SendResult};
use npt_proto::{Direction, Join, Message, Quant, channel_of, decode};
use npt_room::{CmdTiming, Room};
use npt_server::{Decoder, RttSource};
use npt_sim::replay::Recording;
use npt_sim::{Scene, World};

const MS: u64 = 1_000_000;
const VERSION: u16 = 1;
const CONTENT: [u8; 32] = [7; 32];

/// A Loopback end whose received messages each arrive `delay` later, and the `slow`-th message of type `ty` (read in
/// direction `way`) 600 ms later still, in arrival order (so the slow one holds nothing behind it).
struct Delay {
    inner: Loopback,
    delay: u64,
    way: Direction,
    ty: &'static str,
    slow: Option<usize>,
    seen: usize,
    held: VecDeque<Recv>,
}

impl Delay {
    fn new(inner: Loopback, way: Direction, ty: &'static str, slow: Option<usize>) -> Delay {
        Delay {
            inner,
            delay: 5 * MS,
            way,
            ty,
            slow,
            seen: 0,
            held: VecDeque::new(),
        }
    }
}

impl Link for Delay {
    fn send_unreliable(&mut self, bytes: &[u8], now_ns: u64) -> SendResult {
        self.inner.send_unreliable(bytes, now_ns)
    }
    fn send_reliable(&mut self, channel: Channel, bytes: &[u8], now_ns: u64) -> SendResult {
        self.inner.send_reliable(channel, bytes, now_ns)
    }
    fn poll_recv(&mut self, now_ns: u64) -> Option<Recv> {
        // Every message sent so far (the lab is one thread), each given its arrival.
        while let Some(mut r) = self.inner.poll_recv(u64::MAX) {
            r.arrival_ns += self.delay;
            if decode(self.way, r.channel, &r.bytes).is_ok_and(|m| m.name() == self.ty) {
                self.seen += 1;
                if Some(self.seen) == self.slow {
                    r.arrival_ns += 600 * MS;
                }
            }
            let at = self
                .held
                .iter()
                .position(|h| h.arrival_ns > r.arrival_ns)
                .unwrap_or(self.held.len());
            self.held.insert(at, r);
        }
        if self.held.front().is_some_and(|r| r.arrival_ns <= now_ns) {
            self.held.pop_front()
        } else {
            None
        }
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

/// One run of 5 s under `timing`: the 3rd exchange's `Pong` 600 ms late (`up` false) or its `Ping` (`up` true).
/// Returns the commands given and run, the kind-10 refusals, the largest lead and the slow messages' type count seen.
fn run(timing: CmdTiming, up: bool) -> (usize, u64, u64, i64, usize) {
    let scene = Scene::course();
    let first = 1_000;
    let world = World::new(&scene, 5, first);
    let greybox = world.greybox().clone();
    let token = [0x5a; 16];
    let join = Join {
        token,
        bind: [0xa5; 16],
        version: VERSION,
        content_hash: CONTENT,
    };
    // Option A or B, set explicitly (TR-U6A-3).
    let mut room = Room::new(world, &scene, &greybox, vec![token], 1, npt_sim::TICK_HZ)
        .with_hashes(CONTENT, [0; 32])
        .with_cmd_timing(timing);
    let (s, c) = Loopback::pair(1_200);
    // A 10 ms path: 5 ms each way; the 3rd exchange 600 ms slow, down or up.
    let (down, rise) = if up { (None, Some(3)) } else { (Some(3), None) };
    let mut server = Delay::new(s, Direction::ClientToRoom, "Ping", rise);
    let mut client = Delay::new(c, Direction::RoomToClient, "Pong", down);
    let base = 1_000_000_000;
    let mut decoder = Decoder::new(
        Quant::default(),
        npt_sim::TICK_HZ,
        base,
        VERSION,
        CONTENT,
        RttSource::Link,
    );
    let mut e = ClientEndpoint::new(join, npt_sim::TICK_HZ).with_clock(9);
    let commands: Vec<npt_sim::Command> = Recording::synthetic(&scene, 5, first, 200, 2_000)
        .ticks
        .iter()
        .flat_map(|t| t.commands.iter().map(|(c, _)| *c))
        .filter(|c| c.slot == 0)
        .collect();
    let mut given = 0usize;
    let mut slot = None;
    let mut lead_max = 0i64;
    let tick_ns = |k: u64| base + k * 1_000_000_000 / u64::from(npt_sim::TICK_HZ);
    for k in 0..150u64 {
        let now = tick_ns(k);
        let tick = room.last_tick().wrapping_add(1);
        let mut msgs = Vec::new();
        decoder.pump(&mut server, now, tick, &mut msgs);
        for (m, _) in msgs {
            match (&m, slot) {
                (Message::Join(j), _) => {
                    let (s, _) = room.join(j).expect("the roster takes it");
                    decoder.accept();
                    slot = Some(s);
                }
                (_, Some(s)) => room.receive(s, &m),
                _ => {}
            }
        }
        if let Some(s) = slot {
            room.pump_report(s, decoder.take_sent_bytes(), server.max_datagram_size());
        }
        room.tick();
        if let Some(s) = slot {
            for d in room.take_datagrams(s) {
                server.send_unreliable(&d, now);
            }
            for b in room.take_control(s) {
                server.send_reliable(channel_of(&b).unwrap_or(Channel::Control), &b, now);
            }
        }
        // The client steps 1 ms after each server tick.
        let at = now + MS;
        let mut got = Vec::new();
        while let Some(r) = client.poll_recv(at) {
            got.push((r.channel, r.bytes, r.arrival_ns));
        }
        e.input(None, &got, at);
        for _ in 0..e.commands_due() {
            e.input(Some(commands[given % commands.len()]), &[], at);
            given += 1;
        }
        let out = e.step();
        for d in &out.datagrams {
            client.send_unreliable(d, at);
        }
        for (ch, b) in &out.reliable {
            client.send_reliable(*ch, b, at);
        }
        lead_max = lead_max.max(e.lead_now().unwrap_or(0));
    }
    let k = room.client(0).expect("joined").counters().clone();
    (
        given,
        k.commands_run,
        k.buffer_full,
        lead_max,
        client.seen + server.seen,
    )
}

/// Both legs under option B (C-U6-17's words) and option A (TR-U6A-3: C-U6-16 (b) keeps A selectable); every leg runs
/// before the first assert, so a failure names each leg's refusals.
#[test]
fn c_u6_17_one_600_ms_pong_on_a_10_ms_path_brings_no_kind_10_refusal_under_option_b() {
    let legs: Vec<_> = [CmdTiming::A, CmdTiming::B]
        .into_iter()
        .flat_map(|t| [(t, false), (t, true)])
        .map(|(timing, up)| {
            let way = if up {
                "its ping slow (up)"
            } else {
                "the pong slow (down)"
            };
            (format!("{timing:?}, {way}"), run(timing, up))
        })
        .collect();
    for (leg, (given, ran, refused, lead, seen)) in &legs {
        println!(
            "C-U6-17, {leg}: {given} commands given, {ran} run, {refused} refused as kind 10, largest lead {lead}, \
             {seen} of the slow message's type seen"
        );
    }
    let refusals: Vec<(&str, u64)> = legs.iter().map(|(l, r)| (l.as_str(), r.2)).collect();
    for (leg, (_, ran, refused, _, seen)) in &legs {
        assert!(*seen > 3, "{leg}: the slow one came and went");
        assert!(*ran > 60, "{leg}: the honest client played");
        assert_eq!(
            *refused, 0,
            "{leg}: no kind-10 refusal on the honest client (each leg's refusals: {refusals:?})"
        );
    }
}
