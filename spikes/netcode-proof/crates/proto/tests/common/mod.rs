//! Fixed-seed generators of valid messages for npt-proto's tests (u4-build-plan.md §5 U4a: a1, a2, a3), from
//! ChaCha8Rng, the workspace's pinned generator. Every value is one the wire carries; entity fields are read from
//! `EntityKind::layout()`, so U4d's completed lists need no edit here.

#![allow(
    dead_code,
    reason = "each test file uses its own part of the generators"
)]

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use npt_link::Channel;
use npt_proto::{
    ClientSettings, Direction, EndReason, EntityDelta, FullState, Input, Join, JoinAccepted,
    JoinRejected, Kick, Leave, Limits, Message, Ping, Pong, ProfileId, Resync, RoomControl,
    RoomEnded, SessionControl, SessionStatus, Snapshot, StepResult, TokenKind, encode,
};
use npt_sim::grid::Field;
use npt_sim::{Command, EntityKind, EntityRecord, SimEvent};
use rand_chacha::ChaCha8Rng;
use rand_core::{Rng, SeedableRng};

pub struct Gen(pub ChaCha8Rng);

impl Gen {
    pub fn new(seed: u64) -> Gen {
        Gen(ChaCha8Rng::seed_from_u64(seed))
    }

    pub fn u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    /// 0 to n - 1.
    pub fn below(&mut self, n: u64) -> u64 {
        self.0.next_u64() % n
    }

    pub fn coin(&mut self) -> bool {
        self.below(2) == 0
    }

    pub fn u8(&mut self) -> u8 {
        self.0.next_u32() as u8
    }

    pub fn u16(&mut self) -> u16 {
        self.0.next_u32() as u16
    }

    pub fn u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    pub fn bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut b = [0u8; N];
        self.0.fill_bytes(&mut b);
        b
    }

    pub fn fill(&mut self, b: &mut [u8]) {
        self.0.fill_bytes(b);
    }

    /// An integer of `field`, its ends one time in four.
    pub fn int(&mut self, field: Field) -> i32 {
        let (min, max) = (i64::from(field.min()), i64::from(field.max()));
        match self.below(8) {
            0 => field.min(),
            1 => field.max(),
            _ => (min + self.below((max - min + 1) as u64) as i64) as i32,
        }
    }

    pub fn kind(&mut self) -> EntityKind {
        EntityKind::ALL[self.below(EntityKind::ALL.len() as u64) as usize]
    }

    /// A record as the wire carries it: every layout field in range, every entry past the layout, `area` and `raw` 0.
    pub fn record(&mut self, kind: EntityKind) -> EntityRecord {
        let mut grid = [0; EntityRecord::FIELDS];
        for (g, f) in grid.iter_mut().zip(kind.layout()) {
            *g = self.int(*f);
        }
        EntityRecord {
            kind,
            index: self.u16(),
            generation: self.u16(),
            area: 0,
            grid,
            raw: [0; EntityRecord::FIELDS],
        }
    }

    /// A delta; against the defaults (`from_defaults`) always an add.
    pub fn delta(&mut self, from_defaults: bool) -> EntityDelta {
        let kind = self.kind();
        let n = kind.layout().len();
        match if from_defaults { 0 } else { self.below(3) } {
            1 if n > 0 => {
                let mut changed = 0u16;
                while changed == 0 {
                    changed = (self.u32() & ((1 << n) - 1)) as u16;
                }
                let mut grid = [0; EntityRecord::FIELDS];
                for (i, f) in kind.layout().iter().enumerate() {
                    if changed & (1 << i) != 0 {
                        grid[i] = self.int(*f);
                    }
                }
                EntityDelta::Update {
                    kind,
                    index: self.u16(),
                    changed,
                    grid,
                }
            }
            2 => EntityDelta::Remove {
                kind,
                index: self.u16(),
            },
            _ => EntityDelta::Add(self.record(kind)),
        }
    }

    pub fn event(&mut self) -> SimEvent {
        match self.below(3) {
            0 => SimEvent::Shot {
                slot: self.u8(),
                seq: self.u16(),
            },
            1 => SimEvent::Hit {
                shooter: self.u8(),
                seq: self.u16(),
                victim: self.u8(),
                damage: self.u16(),
            },
            _ => SimEvent::Kill {
                killer: self.u8(),
                victim: self.u8(),
            },
        }
    }

    /// A command the wire carries: aim on the position grid, the view time at most 65,535 ticks back.
    pub fn command(&mut self) -> Command {
        let tick = self.u32();
        Command {
            slot: 0,
            tick,
            seq: self.u16(),
            buttons: self.u16(),
            presses: self.bytes(),
            move_xy: [self.u8() as i8, self.u8() as i8],
            aim: [
                self.int(npt_sim::grid::POSITION_XY),
                self.int(npt_sim::grid::POSITION_XY),
                self.int(npt_sim::grid::POSITION_Z),
            ],
            view_tick: tick.wrapping_sub(u32::from(self.u16())),
            view_fraction: self.u8(),
        }
    }

    /// `count` commands, newest first, each older one consecutive in sequence, its fields kept or changed at random.
    pub fn commands(&mut self, count: usize) -> Vec<Command> {
        let mut out: Vec<Command> = Vec::new();
        for _ in 0..count {
            let c = match out.last() {
                None => self.command(),
                Some(n) => {
                    let n = *n;
                    let fresh = self.command();
                    let tick = n.tick.wrapping_sub(self.below(256) as u32);
                    let offset = if self.coin() {
                        n.tick.wrapping_sub(n.view_tick)
                    } else {
                        u32::from(self.u16())
                    };
                    Command {
                        slot: 0,
                        tick,
                        seq: n.seq.wrapping_sub(1),
                        buttons: if self.coin() {
                            n.buttons
                        } else {
                            fresh.buttons
                        },
                        presses: if self.coin() {
                            n.presses
                        } else {
                            fresh.presses
                        },
                        move_xy: if self.coin() {
                            n.move_xy
                        } else {
                            fresh.move_xy
                        },
                        aim: if self.coin() { n.aim } else { fresh.aim },
                        view_tick: tick.wrapping_sub(offset),
                        view_fraction: if self.coin() {
                            n.view_fraction
                        } else {
                            fresh.view_fraction
                        },
                    }
                }
            };
            out.push(c);
        }
        out
    }

    pub fn input(&mut self) -> Input {
        let count = self.below(Limits::REDUNDANCY_CAP as u64 + 1) as usize;
        let (tick, ack_groups) = (self.u16(), self.u8());
        Input {
            // No group decoded: no tick (the one encoding of "no snapshot yet").
            ack_tick: if ack_groups == 0 { 0 } else { tick },
            ack_groups,
            commands: self.commands(count),
        }
    }

    /// A snapshot group that fits its datagram: entities are dropped until it does (U4d's wider layouts included).
    pub fn snapshot(&mut self) -> Snapshot {
        let baseline_age = if self.below(4) == 0 { 0 } else { self.u16() };
        let from_defaults = baseline_age == 0;
        let groups = 1 + self.below(Limits::GROUPS as u64) as u8;
        let mut s = Snapshot {
            tick: self.u32(),
            baseline_age,
            newest_received: self.coin().then(|| self.u16()),
            last_run: self.coin().then(|| self.u16()),
            group: self.below(u64::from(groups)) as u8,
            groups,
            own: (0..self.below(5))
                .map(|_| self.delta(from_defaults))
                .collect(),
            entities: (0..self.below(25))
                .map(|_| self.delta(from_defaults))
                .collect(),
            events: (0..self.below(13))
                .map(|_| (self.u16(), self.u8(), self.event()))
                .collect(),
        };
        while encoded(&Message::Snapshot(s.clone())).is_none() {
            if s.entities.pop().is_none() && s.own.pop().is_none() {
                s.events.pop();
            }
        }
        s
    }

    pub fn full_state(&mut self) -> FullState {
        FullState {
            tick: self.u32(),
            last_run: self.coin().then(|| self.u16()),
            own: (0..self.below(10))
                .map(|_| self.record(EntityKind::ALL[0]))
                .collect(),
            entities: (0..self.below(60))
                .map(|_| {
                    let k = self.kind();
                    self.record(k)
                })
                .collect(),
        }
    }

    pub fn profile(&mut self) -> ProfileId {
        [
            ProfileId::P0,
            ProfileId::P1,
            ProfileId::P2,
            ProfileId::P3,
            ProfileId::P4,
            ProfileId::Default,
        ][self.below(6) as usize]
    }

    /// A note the form takes: up to 480 B of UTF-8, letters from several scripts, no control or format character.
    pub fn note(&mut self) -> String {
        const POOL: [char; 12] = ['a', 'Z', '0', ' ', '.', ',', 'é', 'ß', 'Ж', 'ж', '中', '🙂'];
        let mut s = String::new();
        let want = self.below(Limits::NOTE as u64 + 1) as usize;
        loop {
            let c = POOL[self.below(POOL.len() as u64) as usize];
            if s.len() + c.len_utf8() > want {
                break;
            }
            s.push(c);
        }
        s
    }

    pub fn session_control(&mut self) -> SessionControl {
        let cap = self.bytes();
        match self.below(4) {
            0 => SessionControl::SetProfile {
                cap,
                profile: self.profile(),
            },
            1 => SessionControl::TriggerBurst { cap },
            2 => SessionControl::BlockUdp {
                cap,
                on: self.coin(),
            },
            _ => SessionControl::RecordStep {
                cap,
                step: 1 + self.below(u64::from(Limits::RECORD_STEPS)) as u8,
                result: [
                    StepResult::Achieved,
                    StepResult::NotYet,
                    StepResult::Observation,
                ][self.below(3) as usize],
                note: self.note(),
            },
        }
    }

    /// A message of type `t` (its position in `Limits::TYPES`, from 1).
    pub fn message(&mut self, t: usize) -> Message {
        match t {
            1 => Message::Input(self.input()),
            2 => Message::Snapshot(self.snapshot()),
            3 => Message::FullState(self.full_state()),
            4 => Message::Resync(Resync),
            5 => Message::Ping(Ping {
                id: self.u8(),
                sent_ns: self.u64(),
            }),
            6 => Message::Pong(Pong {
                id: self.u8(),
                echo_ns: self.u64(),
                tick: self.u32(),
                hold_us: self.u16(),
            }),
            7 => Message::Join(Join {
                token: self.bytes(),
                bind: self.bytes(),
                version: self.u16(),
                content_hash: self.bytes(),
            }),
            8 => Message::JoinAccepted(JoinAccepted {
                slot: self.u8(),
                team: self.u8(),
                squad: self.u8(),
                mode_id: self.u16(),
                mode_hash: self.bytes(),
                content_hash: self.bytes(),
                tick: self.u32(),
                tick_hz: self.u16(),
                snapshot_hz: self.u16(),
            }),
            9 => Message::JoinRejected(
                [
                    JoinRejected::BadToken,
                    JoinRejected::BindMismatch,
                    JoinRejected::RosterFull,
                    JoinRejected::WrongVersion,
                    JoinRejected::WrongContentHash,
                ][self.below(5) as usize],
            ),
            10 => Message::ClientSettings(ClientSettings { delay: self.u16() }),
            11 => Message::RoomEnded(RoomEnded {
                reason: [EndReason::Ended, EndReason::Stopped, EndReason::Crash]
                    [self.below(3) as usize],
                final_tick: self.u32(),
            }),
            12 => Message::Leave([Leave::Quit, Leave::Error][self.below(2) as usize]),
            13 => Message::Kick(
                [
                    Kick::Replaced,
                    Kick::Malformed,
                    Kick::Overflow,
                    Kick::Stopped,
                ][self.below(4) as usize],
            ),
            14 => Message::SessionControl(self.session_control()),
            _ => Message::SessionStatus(SessionStatus {
                profile: self.profile(),
                rtt_us: self.u32(),
                rewind_cap: self.u16(),
                cmd_timing: self.below(2) as u8,
                clipped: self.u32(),
                last_step: self.below(u64::from(Limits::RECORD_STEPS) + 1) as u8,
            }),
        }
    }

    /// 1 to 16 addresses: IPv4, or IPv6 /64 prefixes.
    pub fn peers(&mut self) -> Vec<IpAddr> {
        (0..1 + self.below(Limits::PEERS as u64))
            .map(|_| {
                if self.coin() {
                    IpAddr::V4(Ipv4Addr::from(self.u32()))
                } else {
                    let mut b = [0u8; 16];
                    self.fill(&mut b[..8]);
                    IpAddr::V6(Ipv6Addr::from(b))
                }
            })
            .collect()
    }

    pub fn room_control(&mut self) -> RoomControl {
        match self.below(11) {
            0 => RoomControl::MintToken {
                slot: self.u8(),
                kind: [TokenKind::Bot, TokenKind::Browser, TokenKind::Runner]
                    [self.below(3) as usize],
            },
            1 => RoomControl::MintCapability { slot: self.u8() },
            2 => RoomControl::SetRewindCap {
                ticks: 1 + self.below(u64::from(u16::MAX)) as u16,
            },
            3 => RoomControl::RaiseAddrCap {
                cap: self.u16(),
                addrs: self.peers(),
            },
            4 => RoomControl::InjectPanic {
                target: self.coin().then(|| self.below(255) as u8),
                at: self.u32(),
            },
            5 => RoomControl::InjectStall {
                ms: self.below(1_001) as u16,
                at: self.u32(),
            },
            6 => RoomControl::SetPeerProfile {
                peers: self.coin().then(|| self.peers()),
                profile: [
                    ProfileId::P0,
                    ProfileId::P1,
                    ProfileId::P2,
                    ProfileId::P3,
                    ProfileId::P4,
                ][self.below(5) as usize],
            },
            7 => RoomControl::SetPeerThrottle {
                peers: self.peers(),
                kbps: 1 + self.below(u64::from(u16::MAX)) as u16,
                secs: self.below(601) as u16,
            },
            8 => RoomControl::SetPeerBottleneck {
                peers: self.peers(),
                percent: 1 + self.below(100) as u8,
                secs: self.below(601) as u16,
            },
            9 => RoomControl::PeerBurst {
                peers: self.peers(),
                ms: self.below(1_001) as u16,
            },
            _ => RoomControl::BlockPeerUdp {
                peers: self.coin().then(|| self.peers()),
                on: self.coin(),
            },
        }
    }
}

/// `msg`'s encoding, or `None` when it is above its limit.
pub fn encoded(msg: &Message) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    encode(msg, &mut out).ok().map(|()| out)
}

/// Type `t`'s row: name, channel, the direction its receiver decodes with (a both-ways type at the room), limit.
pub fn row(t: usize) -> (&'static str, Channel, Direction, usize) {
    let (name, channel, way, limit) = Limits::TYPES[t - 1];
    let way = if way == Direction::BothWays {
        Direction::ClientToRoom
    } else {
        way
    };
    (name, channel, way, limit)
}

/// The framing bytes before a message's type on `channel`.
pub fn framing(channel: Channel) -> usize {
    match channel {
        Channel::Datagram => 0,
        Channel::Control => 2,
        Channel::FullState => 4,
    }
}

/// The most bytes type `t`'s receiver takes, framing included (the catalogue's counting: catalogue §1).
pub fn framed_limit(t: usize) -> usize {
    let (name, channel, _, limit) = Limits::TYPES[t - 1];
    let limit = if name == "Snapshot" {
        Limits::SNAPSHOT_RECEIVE
    } else {
        limit
    };
    if channel == Channel::FullState {
        limit + 4
    } else {
        limit
    }
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
