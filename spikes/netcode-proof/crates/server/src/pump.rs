//! The I/O step (u4-build-plan.md §4.1, §4.4 A26, §4.5, §4.9; catalogue §1 and §2): [`Decoder::pump`] decodes every
//! inbound message of one session through npt-proto's `Quant::decode` with the `Quant` it was given, counts each
//! refusal by kind, applies the size and channel checks at the socket ([`DecodeError`]'s socket kinds), the Input
//! bucket, the Ping limit and the Join deadline on the injected `now`, sends the room's pings and answers the client's
//! with their hold time, and keeps the session's RTT and the bytes the server side sent it since the last tick. It
//! holds no `Link` (§4.11: so it can be `Send`); the caller lends one per call.

use std::collections::VecDeque;

use npt_link::{Channel, Link};
use npt_proto::{DecodeError, Direction, Join, JoinRejected, Kick, Message, Ping, Pong, Quant};

/// The Join deadline: 2 s from the connection's accept (catalogue :93), in ns.
const JOIN_DEADLINE_NS: u64 = 2_000_000_000;
/// The room's ping interval: a few a second (catalogue :91), in ns.
const PING_INTERVAL_NS: u64 = 250_000_000;
/// One second, in ns.
const SECOND_NS: u64 = 1_000_000_000;
/// The Input bucket's size (catalogue :87: "a bucket of 8 datagrams").
const INPUT_BUCKET: u64 = 8;
/// The Ping limit (catalogue :91: "10 a second per session"), as a bucket of 10 refilled at 10 a second.
const PINGS_A_SECOND: u64 = 10;
/// Outstanding pings kept, at most; an older one's `Pong` is then not outstanding.
const OUTSTANDING: usize = 8;
/// RTT samples kept for the lowest-recent estimate.
const SAMPLES: usize = 8;
/// What each datagram costs the window beyond its payload (catalogue :88).
const DATAGRAM_OVERHEAD: u32 = 28;

/// A token bucket on the injected clock, exact in integers: its level counts tokens in units of 10^-9 token, so a
/// refill of `rate` tokens a second adds `rate` units per ns.
#[derive(Clone, Copy, Debug)]
struct Bucket {
    cap: u64,
    rate: u64,
    level: u64,
    at_ns: u64,
}

impl Bucket {
    fn full(cap: u64, rate: u64, now_ns: u64) -> Bucket {
        Bucket {
            cap,
            rate,
            level: cap.saturating_mul(SECOND_NS),
            at_ns: now_ns,
        }
    }

    /// Takes one token at `now_ns` if the bucket holds one.
    fn take(&mut self, now_ns: u64) -> bool {
        let elapsed = now_ns.saturating_sub(self.at_ns);
        self.at_ns = self.at_ns.max(now_ns);
        self.level = self
            .level
            .saturating_add(elapsed.saturating_mul(self.rate))
            .min(self.cap.saturating_mul(SECOND_NS));
        if self.level >= SECOND_NS {
            self.level -= SECOND_NS;
            true
        } else {
            false
        }
    }
}

/// One connection's wall-time limits on the injected clock (catalogue :87, :91, :93): the Input bucket (8 datagrams,
/// refilled at 2 × the tick rate a second), the Ping limit (10 a second) and the Join deadline (2 s from the accept).
/// The tick rate is an argument (the F2 ruling): production callers pass `npt_sim::TICK_HZ`.
#[derive(Clone, Copy, Debug)]
pub struct ConnLimits {
    tick_hz: u32,
    input: Bucket,
    ping: Bucket,
    deadline_ns: u64,
}

impl ConnLimits {
    /// The limits of a connection accepted at `accept_ns`, at `tick_hz` ticks a second.
    pub fn new(tick_hz: u32, accept_ns: u64) -> ConnLimits {
        let tick_hz = tick_hz.max(1);
        ConnLimits {
            tick_hz,
            input: Bucket::full(INPUT_BUCKET, 2 * u64::from(tick_hz), accept_ns),
            ping: Bucket::full(PINGS_A_SECOND, PINGS_A_SECOND, accept_ns),
            deadline_ns: accept_ns.saturating_add(JOIN_DEADLINE_NS),
        }
    }

    /// The Input bucket's refill, datagrams a second: 2 × the tick rate (60 at 30 Hz, 120 at 60 Hz).
    pub fn input_refill_per_second(&self) -> u64 {
        self.input.rate
    }

    /// The most of a `Pong`'s hold time the RTT subtracts: one tick period (catalogue :92: "33.3 ms at 30 Hz"), ns.
    pub fn hold_cap_ns(&self) -> u64 {
        SECOND_NS / u64::from(self.tick_hz)
    }

    /// When a connection with no `Join` is closed, ns.
    #[allow(
        dead_code,
        reason = "no consumer yet (advisor-u4e-r1 R2, A5): crate-private until a unit needs it public"
    )]
    pub(crate) fn join_deadline_ns(&self) -> u64 {
        self.deadline_ns
    }
}

/// One session's refusal and traffic counts (PM-0001-21's counters; U4f adds their -21 series: U4E-Q11).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Counters {
    /// `decode` refusals by kind: above the size the receiving end takes ...
    pub too_large: u64,
    /// ... ending inside a field ...
    pub truncated: u64,
    /// ... a stream message whose length prefix does not count its bytes ...
    pub bad_length: u64,
    /// ... a type byte no message has ...
    pub unknown_type: u64,
    /// ... a room → client type from a client (forged) ...
    pub wrong_direction: u64,
    /// ... a known type on a channel it does not travel on ...
    pub wrong_channel: u64,
    /// ... bytes after the last field ...
    pub trailing_bytes: u64,
    /// ... a reserved or pad bit set ...
    pub unknown_field: u64,
    /// ... a code outside its field ...
    pub out_of_range: u64,
    /// ... and well framed but not a message.
    pub malformed: u64,
    /// Datagrams before the session's `Join` was accepted: dropped (ADR 0001:446).
    pub before_join: u64,
    /// `Input` datagrams over the bucket: dropped.
    pub input_limited: u64,
    /// `Ping`s over the limit: dropped.
    pub ping_limited: u64,
    /// `Pong`s naming no outstanding ping of this session: dropped.
    pub pong_unknown: u64,
    /// Sessions closed as malformed because their first stream message was not a `Join` (a decodable other type, or
    /// a known type on the wrong channel or from the wrong direction: catalogue :93), never INVALID (the run verdict
    /// is the judge's).
    pub first_not_join: u64,
    /// Stream messages received while the session's `Join` waited for the room's answer: dropped (SEC-U4E-3).
    pub joining_dropped: u64,
    /// Sessions closed as malformed for a stream message that failed its framing, size or type checks.
    pub stream_malformed: u64,
    /// Sessions closed for no `Join` within 2 s of the accept.
    pub join_deadline: u64,
    /// `Join`s refused for the protocol version ...
    pub wrong_version: u64,
    /// ... or the content hash.
    pub wrong_content_hash: u64,
    /// A second `Join` on a session already joining or joined: dropped.
    pub join_again: u64,
    /// Pings the room sent.
    pub pings_sent: u64,
    /// Pongs the room sent.
    pub pongs_sent: u64,
}

impl Counters {
    fn refused(&mut self, e: DecodeError) {
        match e {
            DecodeError::TooLarge => self.too_large += 1,
            DecodeError::Truncated => self.truncated += 1,
            DecodeError::BadLength => self.bad_length += 1,
            DecodeError::UnknownType => self.unknown_type += 1,
            DecodeError::WrongDirection => self.wrong_direction += 1,
            DecodeError::WrongChannel => self.wrong_channel += 1,
            DecodeError::TrailingBytes => self.trailing_bytes += 1,
            DecodeError::UnknownField => self.unknown_field += 1,
            DecodeError::OutOfRange => self.out_of_range += 1,
            DecodeError::Malformed => self.malformed += 1,
        }
    }
}

/// Where a session's RTT comes from (u4-build-plan.md §4.9 :352-354): by the session's kind, which the pump's caller
/// learns from the listener that accepted it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RttSource {
    /// A `wt` or L1 session: the link's smoothed estimate (`Link::rtt_ns`), `None` until it has one; never the room's
    /// ping samples.
    Link,
    /// A `ws` session: the lowest of the last 8 samples of the room's own pings, each less the client's hold time up
    /// to one tick period; never the link's estimate.
    Pings,
}

/// Where a session is in its life.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    AwaitJoin,
    /// The `Join` went to the room; its answer (`Decoder::accept` or `Decoder::reject`) is due.
    Joining,
    Joined,
    Closed,
}

/// One session's decoder and I/O step (the module text).
#[derive(Clone, Debug)]
pub struct Decoder {
    quant: Quant,
    limits: ConnLimits,
    counters: Counters,
    version: u16,
    content_hash: [u8; 32],
    rtt: RttSource,
    state: State,
    next_ping_ns: u64,
    ping_id: u8,
    outstanding: VecDeque<(u8, u64)>,
    samples: VecDeque<u64>,
    sent_bytes: u32,
}

impl Decoder {
    /// The decoder of a session accepted at `accept_ns`, decoding with `quant` (the wire's, `Quant::default()`; in a
    /// bites build one of npt-proto's planted defects through `Quant::with_bite`), at `tick_hz` ticks a second
    /// (production callers pass `npt_sim::TICK_HZ`), taking `Join`s of protocol `version` and `content_hash` only, its
    /// RTT from `rtt` (the session's kind).
    pub fn new(
        quant: Quant,
        tick_hz: u32,
        accept_ns: u64,
        version: u16,
        content_hash: [u8; 32],
        rtt: RttSource,
    ) -> Decoder {
        Decoder {
            quant,
            limits: ConnLimits::new(tick_hz, accept_ns),
            counters: Counters::default(),
            version,
            content_hash,
            rtt,
            state: State::AwaitJoin,
            next_ping_ns: accept_ns,
            ping_id: 0,
            outstanding: VecDeque::new(),
            samples: VecDeque::new(),
            sent_bytes: 0,
        }
    }

    /// The session's counts.
    pub fn counters(&self) -> &Counters {
        &self.counters
    }

    /// The session's limits.
    #[allow(
        dead_code,
        reason = "no consumer yet (advisor-u4e-r1 R2, A5): crate-private until a unit needs it public"
    )]
    pub(crate) fn limits(&self) -> &ConnLimits {
        &self.limits
    }

    /// Whether the pump closed the session (the caller drops its link).
    pub fn is_closed(&self) -> bool {
        self.state == State::Closed
    }

    /// Whether the room accepted the session's `Join`.
    #[allow(
        dead_code,
        reason = "no consumer yet (advisor-u4e-r1 R2, A5): crate-private until a unit needs it public"
    )]
    pub(crate) fn is_joined(&self) -> bool {
        self.state == State::Joined
    }

    /// The room accepted the `Join` the pump passed on: datagrams are taken from now on.
    pub fn accept(&mut self) {
        if self.state == State::Joining {
            self.state = State::Joined;
        }
    }

    /// The room refused the `Join`: `JoinRejected` with its reason, then the session closes (catalogue :96).
    pub fn reject(&mut self, link: &mut dyn Link, reason: JoinRejected, now_ns: u64) {
        self.send_control(link, &Message::JoinRejected(reason), now_ns);
        self.state = State::Closed;
    }

    /// The bytes the server side sent this session since the last call (pings and pongs, payload plus 28 B each: the
    /// room's window counts them, u4-build-plan.md §4.5).
    pub fn take_sent_bytes(&mut self) -> u32 {
        core::mem::take(&mut self.sent_bytes)
    }

    /// The session's RTT (u4-build-plan.md §4.9 :352-354), from its [`RttSource`]: for an L1 or `wt` session the link's
    /// smoothed estimate (Loopback's is 0), for a `ws` session the lowest recent sample of the room's own pings, each
    /// less the client's hold time up to one tick period. One session never mixes the two.
    pub fn rtt_ns(&self, link: &dyn Link) -> Option<u64> {
        match self.rtt {
            RttSource::Link => link.rtt_ns(),
            RttSource::Pings => self.samples.iter().copied().min(),
        }
    }

    fn send_control(&mut self, link: &mut dyn Link, msg: &Message, now_ns: u64) {
        let mut out = Vec::new();
        if npt_proto::encode(msg, &mut out).is_ok() {
            link.send_reliable(Channel::Control, &out, now_ns);
        }
    }

    fn send_datagram(&mut self, link: &mut dyn Link, msg: &Message, now_ns: u64) {
        let mut out = Vec::new();
        if npt_proto::encode(msg, &mut out).is_ok() {
            link.send_unreliable(&out, now_ns);
            self.sent_bytes = self
                .sent_bytes
                .saturating_add(out.len() as u32 + DATAGRAM_OVERHEAD);
        }
    }

    /// Closes the session as malformed, with `Kick(malformed)` while the stream still works (catalogue §1).
    fn close_malformed(&mut self, link: &mut dyn Link, now_ns: u64) {
        self.send_control(link, &Message::Kick(Kick::Malformed), now_ns);
        self.state = State::Closed;
    }

    /// The I/O step at `now_ns` (the room at `tick`): the deadline, the ping sender, then every message that has
    /// arrived, decoded and checked; what the room takes goes onto `out` with its arrival time. After a `Join` the
    /// step returns, so the room answers it before the session's next message is read.
    pub fn pump(
        &mut self,
        link: &mut dyn Link,
        now_ns: u64,
        tick: u32,
        out: &mut Vec<(Message, u64)>,
    ) {
        if self.state == State::Closed {
            return;
        }
        if self.state == State::AwaitJoin && now_ns >= self.limits.deadline_ns {
            self.counters.join_deadline += 1;
            self.state = State::Closed;
            return;
        }
        if self.state == State::Joined && now_ns >= self.next_ping_ns {
            let id = self.ping_id;
            self.ping_id = id.wrapping_add(1);
            self.outstanding.push_back((id, now_ns));
            if self.outstanding.len() > OUTSTANDING {
                self.outstanding.pop_front();
            }
            self.send_datagram(
                link,
                &Message::Ping(Ping {
                    id,
                    sent_ns: now_ns,
                }),
                now_ns,
            );
            self.counters.pings_sent += 1;
            self.next_ping_ns = now_ns.saturating_add(PING_INTERVAL_NS);
        }
        while let Some(r) = link.poll_recv(now_ns) {
            let stream = r.channel != Channel::Datagram;
            let msg = match self
                .quant
                .decode(Direction::ClientToRoom, r.channel, &r.bytes)
            {
                Ok(m) => m,
                Err(e) => {
                    self.counters.refused(e);
                    // A forged kind of a known type within its limits is refused and counted, never kicked, on a
                    // joined session (u4-build-plan.md §4.10); any other bad stream message closes the session.
                    if stream
                        && !matches!(e, DecodeError::WrongChannel | DecodeError::WrongDirection)
                    {
                        self.counters.stream_malformed += 1;
                        self.close_malformed(link, now_ns);
                        return;
                    }
                    // Before the `Join`, a forged kind first on the stream is "any other type first on the stream"
                    // (catalogue :93; advisor-u4e-r1 R5, SEC-U4E-1): the session closes as malformed.
                    if stream && self.state == State::AwaitJoin {
                        self.counters.first_not_join += 1;
                        self.close_malformed(link, now_ns);
                        return;
                    }
                    continue;
                }
            };
            match (self.state, msg) {
                (State::AwaitJoin, Message::Join(j)) => {
                    if !self.join_ok(link, &j, now_ns) {
                        return;
                    }
                    self.state = State::Joining;
                    out.push((Message::Join(j), r.arrival_ns));
                    return;
                }
                (State::AwaitJoin, _) if stream => {
                    self.counters.first_not_join += 1;
                    self.close_malformed(link, now_ns);
                    return;
                }
                (State::AwaitJoin | State::Joining, _) if !stream => self.counters.before_join += 1,
                (_, Message::Join(_)) => self.counters.join_again += 1,
                (State::Joined, Message::Input(i)) => {
                    if self.limits.input.take(now_ns) {
                        out.push((Message::Input(i), r.arrival_ns));
                    } else {
                        self.counters.input_limited += 1;
                    }
                }
                (State::Joined, Message::Ping(p)) => {
                    if self.limits.ping.take(now_ns) {
                        let hold = now_ns.saturating_sub(r.arrival_ns) / 1_000;
                        let pong = Pong {
                            id: p.id,
                            echo_ns: p.sent_ns,
                            tick,
                            hold_us: u16::try_from(hold).unwrap_or(u16::MAX),
                        };
                        self.send_datagram(link, &Message::Pong(pong), now_ns);
                        self.counters.pongs_sent += 1;
                    } else {
                        self.counters.ping_limited += 1;
                    }
                }
                (State::Joined, Message::Pong(p)) => self.pong(&p, r.arrival_ns),
                (State::Joined, m) => out.push((m, r.arrival_ns)),
                // Only a stream message other than `Join` in the Joining state is left (a closed session returns
                // before the loop, and every close returns): dropped and counted (SEC-U4E-3).
                _ => self.counters.joining_dropped += 1,
            }
        }
    }

    /// The server's half of the `Join` checks (catalogue :93): the protocol version and the content hash.
    fn join_ok(&mut self, link: &mut dyn Link, j: &Join, now_ns: u64) -> bool {
        let reason = if j.version != self.version {
            self.counters.wrong_version += 1;
            JoinRejected::WrongVersion
        } else if j.content_hash != self.content_hash {
            self.counters.wrong_content_hash += 1;
            JoinRejected::WrongContentHash
        } else {
            return true;
        };
        self.reject(link, reason, now_ns);
        false
    }

    /// A `Pong` of one of the room's outstanding pings, arrived at `arrival_ns`: one RTT sample, from its arrival (not
    /// the pump's later run: NC-M3), the hold time subtracted up to one tick period (catalogue :92); any other id is
    /// dropped and counted.
    fn pong(&mut self, p: &Pong, arrival_ns: u64) {
        let Some(at) = self
            .outstanding
            .iter()
            .position(|&(id, sent)| id == p.id && sent == p.echo_ns)
        else {
            self.counters.pong_unknown += 1;
            return;
        };
        self.outstanding.remove(at);
        let hold = (u64::from(p.hold_us) * 1_000).min(self.limits.hold_cap_ns());
        let sample = arrival_ns.saturating_sub(p.echo_ns).saturating_sub(hold);
        self.samples.push_back(sample);
        if self.samples.len() > SAMPLES {
            self.samples.pop_front();
        }
    }
}
