//! U4e's pump tests (u4-build-plan.md §5 U4e; u4e-brief.md 2.2, e9, e10): `Decoder::pump` over a Loopback session on a
//! fake clock, every time in ns; the room joins where a test needs its half of the `Join` checks.

mod common;

use common::{CONTENT, VERSION, bind, join, token};
use npt_link::{Channel, Link, LinkState, Loopback, Recv, SendResult};
use npt_proto::{
    Direction, Input, Join, JoinRejected, Kick, Limits, Message, Ping, Pong, Quant, Resync, decode,
    encode,
};
use npt_server::{ConnLimits, Decoder, RttSource};
use npt_sim::{Scene, World};

fn bytes(m: &Message) -> Vec<u8> {
    let mut out = Vec::new();
    encode(m, &mut out).unwrap_or_else(|e| panic!("{}: {e}", m.name()));
    out
}

/// A `wt` or L1 session's decoder (its RTT the link's), accepted at 0 at 30 Hz.
fn decoder(quant: Quant) -> Decoder {
    Decoder::new(quant, 30, 0, VERSION, CONTENT, RttSource::Link)
}

/// Everything the client end has received by `now`, decoded.
fn received(c: &mut Loopback, now: u64) -> Vec<Message> {
    let mut out = Vec::new();
    while let Some(r) = c.poll_recv(now) {
        out.push(
            decode(Direction::RoomToClient, r.channel, &r.bytes)
                .expect("the room's messages decode"),
        );
    }
    out
}

/// A session whose `Join` the pump took and passed on (the room's answer stood in for by `accept`).
fn joined(quant: Quant) -> (Loopback, Loopback, Decoder) {
    let (mut s, mut c) = Loopback::pair(1_200);
    let mut d = decoder(quant);
    c.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
    let mut out = Vec::new();
    d.pump(&mut s, 0, 0, &mut out);
    assert!(matches!(out.as_slice(), [(Message::Join(_), 0)]));
    d.accept();
    (s, c, d)
}

fn input() -> Vec<u8> {
    bytes(&Message::Input(Input::default()))
}

/// e8 (§4.11 :401-402): the `Decoder` is `Send` (it holds no `Link`).
#[test]
fn e8_the_decoder_is_send() {
    fn send<T: Send>() {}
    send::<Decoder>();
}

/// e9: each `JoinRejected` reason, end to end on Loopback (one table test naming the failing reason): the pump's
/// version and content hash, the room's token, bind and roster.
#[test]
fn e9_each_join_rejected_reason_reaches_the_client() {
    let scene = Scene::course();
    let world = World::new(&scene, 1, 0);
    let greybox = world.greybox().clone();
    let mut room = npt_room::Room::new(world, &scene, &greybox, vec![token(0), token(1)], 1, 30);
    // Slot 0 joins and leaves, binding its token; then joins again and stays (the roster's N is 1).
    assert_eq!(room.join(&join(0)).map(|j| j.0), Ok(0));
    room.leave(0);
    assert_eq!(room.join(&join(0)).map(|j| j.0), Ok(0));
    let table = [
        (
            "wrong version",
            Join {
                version: VERSION + 1,
                ..join(1)
            },
            JoinRejected::WrongVersion,
        ),
        (
            "wrong content hash",
            Join {
                content_hash: [8; 32],
                ..join(1)
            },
            JoinRejected::WrongContentHash,
        ),
        (
            "bad token",
            Join {
                token: [0xee; 16],
                ..join(1)
            },
            JoinRejected::BadToken,
        ),
        (
            "bind mismatch",
            Join {
                bind: bind(9),
                ..join(0)
            },
            JoinRejected::BindMismatch,
        ),
        ("roster full", join(1), JoinRejected::RosterFull),
    ];
    for (name, j, want) in table {
        let (mut s, mut c) = Loopback::pair(1_200);
        let mut d = decoder(Quant::default());
        c.send_reliable(Channel::Control, &bytes(&Message::Join(j)), 5);
        let mut out = Vec::new();
        d.pump(&mut s, 5, 0, &mut out);
        for (m, _) in out {
            if let Message::Join(j) = m {
                match room.join(&j) {
                    Ok(_) => panic!("{name}: admitted"),
                    Err(r) => d.reject(&mut s, r, 5),
                }
            }
        }
        assert!(d.is_closed(), "{name}: the session closes");
        assert_eq!(received(&mut c, 5), [Message::JoinRejected(want)], "{name}");
    }
    assert_eq!(room.roster().live(), 1, "only slot 0's first session");
}

/// e9: a datagram before `Join` is dropped and counted (ADR 0001:446).
#[test]
fn e9_a_datagram_before_join_is_dropped_and_counted() {
    let (mut s, mut c) = Loopback::pair(1_200);
    let mut d = decoder(Quant::default());
    c.send_unreliable(&input(), 0);
    let mut out = Vec::new();
    d.pump(&mut s, 0, 0, &mut out);
    assert!(out.is_empty());
    assert_eq!(d.counters().before_join, 1);
    assert!(!d.is_closed());
}

/// e9: a control message before `Join` closes the session as malformed, counted under its reason, never INVALID (the
/// run verdict is the judge's): the reason's count, `Kick(malformed)`, and a roster that admitted no session.
#[test]
fn e9_a_control_message_before_join_closes_the_session_as_malformed() {
    let scene = Scene::course();
    let world = World::new(&scene, 1, 0);
    let greybox = world.greybox().clone();
    let room = npt_room::Room::new(world, &scene, &greybox, vec![token(0)], 1, 30);
    let (mut s, mut c) = Loopback::pair(1_200);
    let mut d = decoder(Quant::default());
    c.send_reliable(Channel::Control, &bytes(&Message::Resync(Resync)), 0);
    c.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
    let mut out = Vec::new();
    d.pump(&mut s, 0, 0, &mut out);
    assert!(out.is_empty() && d.is_closed());
    assert_eq!(d.counters().first_not_join, 1);
    assert_eq!(received(&mut c, 0), [Message::Kick(Kick::Malformed)]);
    assert_eq!(room.roster().live(), 0, "the roster admitted no session");
}

/// e9 and e10: the Input bucket (8, refilled at 2 × the tick rate a second) on a fake clock, either side of a refill
/// instant (10^9 / 60 ns is not a whole number), and its rate pinned at 30 and 60 Hz.
#[test]
fn e9_e10_the_input_bucket_holds_8_and_refills_at_2_times_the_tick_rate() {
    for (hz, per_second) in [(30, 60), (60, 120)] {
        assert_eq!(ConnLimits::new(hz, 0).input_refill_per_second(), per_second);
    }
    assert_eq!(
        ConnLimits::new(npt_sim::TICK_HZ, 0).input_refill_per_second(),
        2 * u64::from(npt_sim::TICK_HZ)
    );
    let (mut s, mut c, mut d) = joined(Quant::default());
    let mut out = Vec::new();
    let mut at = |now: u64, n: usize, out: &mut Vec<(Message, u64)>| {
        out.clear();
        for _ in 0..n {
            c.send_unreliable(&input(), now);
        }
        d.pump(&mut s, now, 0, out);
        (out.len(), d.counters().input_limited)
    };
    assert_eq!(at(0, 9, &mut out), (8, 1), "8 taken, the ninth refused");
    // The next token comes at 10^9 / 60 = 16,666,666.67 ns.
    assert_eq!(
        at(16_666_666, 1, &mut out),
        (0, 2),
        "1 ns before the refill"
    );
    assert_eq!(at(16_666_667, 1, &mut out), (1, 2), "1 ns after it");
}

/// e9: the Ping limit (10 a second per session) on a fake clock, either side of a refill instant (10^8 ns).
#[test]
fn e9_the_ping_limit_answers_10_a_second() {
    let (mut s, mut c, mut d) = joined(Quant::default());
    let ping = |id| bytes(&Message::Ping(Ping { id, sent_ns: 0 }));
    let mut out = Vec::new();
    let mut pongs = |now: u64, n: u8| {
        for id in 0..n {
            c.send_unreliable(&ping(id), now);
        }
        d.pump(&mut s, now, 0, &mut out);
        let got = received(&mut c, now)
            .into_iter()
            .filter(|m| matches!(m, Message::Pong(_)))
            .count();
        (got, d.counters().ping_limited)
    };
    assert_eq!(pongs(0, 11), (10, 1));
    assert_eq!(pongs(100_000_000 - 1, 1), (0, 2), "1 ns before the refill");
    assert_eq!(pongs(100_000_000 + 1, 1), (1, 2), "1 ns after it");
}

/// e9: the 2 s Join deadline on a fake clock.
#[test]
fn e9_a_session_with_no_join_within_2_s_is_closed() {
    let (mut s, _c) = Loopback::pair(1_200);
    let mut d = decoder(Quant::default());
    let mut out = Vec::new();
    d.pump(&mut s, 1_999_999_999, 0, &mut out);
    assert!(!d.is_closed());
    d.pump(&mut s, 2_000_000_000, 0, &mut out);
    assert!(d.is_closed());
    assert_eq!(d.counters().join_deadline, 1);
}

/// A Loopback end whose link reports no RTT, so the pump's own ping samples decide (a `ws` session's case).
struct NoRtt(Loopback);

impl Link for NoRtt {
    fn send_unreliable(&mut self, b: &[u8], now: u64) -> SendResult {
        self.0.send_unreliable(b, now)
    }
    fn send_reliable(&mut self, ch: Channel, b: &[u8], now: u64) -> SendResult {
        self.0.send_reliable(ch, b, now)
    }
    fn poll_recv(&mut self, now: u64) -> Option<Recv> {
        self.0.poll_recv(now)
    }
    fn state(&self) -> LinkState {
        self.0.state()
    }
    fn rtt_ns(&self) -> Option<u64> {
        None
    }
    fn max_datagram_size(&self) -> usize {
        self.0.max_datagram_size()
    }
    fn unsent_unreliable(&self) -> usize {
        0
    }
}

/// e10 and 2.2: the `Pong`'s hold time counts up to one tick period (pinned at 30 and 60 Hz), the pump sends a few
/// pings a second and counts their bytes, a `Pong` naming no outstanding ping is dropped and counted (plan silent:
/// built and tested here), and a `ws` session's RTT is the lowest ping sample; a `wt` or L1 session's is the link's.
#[test]
fn e10_the_hold_time_counts_one_tick_period_and_unknown_pongs_are_dropped() {
    assert_eq!(ConnLimits::new(30, 0).hold_cap_ns(), 33_333_333);
    assert_eq!(ConnLimits::new(60, 0).hold_cap_ns(), 16_666_666);
    let (s, mut c) = Loopback::pair(1_200);
    let mut s = NoRtt(s);
    let mut d = Decoder::new(Quant::default(), 30, 0, VERSION, CONTENT, RttSource::Pings);
    c.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
    let mut out = Vec::new();
    d.pump(&mut s, 0, 0, &mut out);
    d.accept();
    d.pump(&mut s, 1_000, 7, &mut out);
    assert_eq!(d.counters().pings_sent, 1);
    assert_eq!(d.take_sent_bytes(), 10 + 28, "a ping's 10 B plus 28");
    let Some(Message::Ping(p)) = received(&mut c, 1_000).pop() else {
        panic!("a ping");
    };
    let at = 1_000 + 100_000_000;
    let pong = |id, hold_us| {
        bytes(&Message::Pong(Pong {
            id,
            echo_ns: p.sent_ns,
            tick: 0,
            hold_us,
        }))
    };
    c.send_unreliable(&pong(p.id.wrapping_add(77), 0), at);
    c.send_unreliable(&pong(p.id, 50_000), at);
    d.pump(&mut s, at, 7, &mut out);
    assert_eq!(d.counters().pong_unknown, 1);
    // 100 ms less the hold time counted up to 33.33 ms (it claimed 50 ms).
    assert_eq!(d.rtt_ns(&s), Some(100_000_000 - 33_333_333));
    let (s2, _c2) = Loopback::pair(1_200);
    assert_eq!(
        decoder(Quant::default()).rtt_ns(&s2),
        Some(0),
        "Loopback's own RTT for an L1 or wt session"
    );
}

/// One RTT sample on a session of kind `rtt` over `s` (its client end `c`): joined at 0, the room's ping sent at
/// 1,000 ns, the client's `Pong` (hold 0) sent at 1,000 ns + 100 ms, the pump run `wait_ns` after it arrived.
fn one_sample<L: Link>(mut s: L, c: &mut Loopback, rtt: RttSource, wait_ns: u64) -> (Decoder, L) {
    let mut d = Decoder::new(Quant::default(), 30, 0, VERSION, CONTENT, rtt);
    c.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
    let mut out = Vec::new();
    d.pump(&mut s, 0, 0, &mut out);
    d.accept();
    d.pump(&mut s, 1_000, 7, &mut out);
    let Some(Message::Ping(p)) = received(c, 1_000).pop() else {
        panic!("a ping");
    };
    let at = 1_000 + 100_000_000;
    let pong = Pong {
        id: p.id,
        echo_ns: p.sent_ns,
        tick: 0,
        hold_us: 0,
    };
    c.send_unreliable(&bytes(&Message::Pong(pong)), at);
    d.pump(&mut s, at + wait_ns, 7, &mut out);
    assert_eq!(
        (d.counters().pings_sent, d.counters().pong_unknown),
        (1, 0),
        "the Pong is the ping's: one sample"
    );
    (d, s)
}

/// NC-M3 (advisor-u4e-r1 R2): the RTT sample is taken at the `Pong`'s arrival, not when the pump runs: a `Pong` sent
/// 100 ms after its ping (hold 0) and pumped 20 ms after it arrived gives 100 ms, not 120 ms.
#[test]
fn e10_the_rtt_sample_is_taken_at_the_pongs_arrival_not_at_the_pump() {
    let (s, mut c) = Loopback::pair(1_200);
    let (d, s) = one_sample(NoRtt(s), &mut c, RttSource::Pings, 20_000_000);
    assert_eq!(d.rtt_ns(&s), Some(100_000_000));
}

/// A2 (advisor-u4e-r1 R2; u4-build-plan.md §4.9 :352-354): a `ws` session's RTT is its ping samples', never the
/// link's, even over a link that answers one (Loopback's 0).
#[test]
fn a2_a_pings_session_takes_its_ping_sample_never_the_links_rtt() {
    let (s, mut c) = Loopback::pair(1_200);
    assert_eq!(s.rtt_ns(), Some(0), "the link answers");
    let (d, s) = one_sample(s, &mut c, RttSource::Pings, 0);
    assert_eq!(d.rtt_ns(&s), Some(100_000_000));
}

/// A2: a `wt` or L1 session's RTT is the link's, `None` while the link has none though a ping sample is held, never
/// the sample.
#[test]
fn a2_a_link_session_takes_the_links_rtt_never_a_ping_sample() {
    let (s, mut c) = Loopback::pair(1_200);
    let (d, s) = one_sample(NoRtt(s), &mut c, RttSource::Link, 0);
    assert_eq!(d.rtt_ns(&s), None);
    let (s2, _c2) = Loopback::pair(1_200);
    assert_eq!(d.rtt_ns(&s2), Some(0), "a link with an estimate: its own");
}

/// TE-5 (advisor-u4e-r1 R6; U4E-Q11 (a)'s size check): an `Input` datagram one byte over `Limits::INPUT` is refused
/// and counted as too large; the room gets nothing and the session stays open.
#[test]
fn e9_an_input_one_byte_over_its_limit_is_counted_too_large() {
    let (mut s, mut c, mut d) = joined(Quant::default());
    let mut big = input();
    big.resize(Limits::INPUT + 1, 0);
    c.send_unreliable(&big, 1);
    let mut out = Vec::new();
    d.pump(&mut s, 1, 0, &mut out);
    assert_eq!(
        (d.counters().too_large, out.len(), d.is_closed()),
        (1, 0, false)
    );
}

/// TE-5: a control-stream message one byte over the stream's cap (the largest client-to-room control type's limit,
/// its 2 B length included) is refused as too large and closes the session as malformed, with `Kick(malformed)`.
#[test]
fn e9_a_control_message_one_byte_over_the_streams_cap_is_too_large_and_closes_the_session() {
    let cap = Limits::TYPES
        .iter()
        .filter(|r| r.1 == Channel::Control && r.2 != Direction::RoomToClient)
        .map(|r| r.3)
        .max()
        .expect("client-to-room control types");
    let (mut s, mut c, mut d) = joined(Quant::default());
    let mut big = bytes(&Message::Join(join(0)));
    big.resize(cap + 1, 0);
    let length = u16::try_from(big.len() - 2).expect("a 2 B length");
    big[..2].copy_from_slice(&length.to_le_bytes());
    c.send_reliable(Channel::Control, &big, 1);
    let mut out = Vec::new();
    d.pump(&mut s, 1, 0, &mut out);
    assert!(out.is_empty() && d.is_closed());
    assert_eq!(
        (d.counters().too_large, d.counters().stream_malformed),
        (1, 1)
    );
    // The joined session's first pump sent its first ping before it read; the close's Kick comes last.
    assert_eq!(
        received(&mut c, 1).last(),
        Some(&Message::Kick(Kick::Malformed))
    );
}

/// SEC-U4E-1 (advisor-u4e-r1 R5; catalogue :93, "any other type first on the stream closes the session as
/// malformed"): before the `Join`, a known type on the wrong channel first on the stream (an `Input` framed on the
/// control stream) closes the session as malformed; the `Join` after it is never taken.
#[test]
fn sec_u4e_1_a_forged_kind_first_on_the_stream_closes_the_session() {
    let (mut s, mut c) = Loopback::pair(1_200);
    let mut d = decoder(Quant::default());
    let body = input();
    let mut framed = (body.len() as u16).to_le_bytes().to_vec();
    framed.extend_from_slice(&body);
    c.send_reliable(Channel::Control, &framed, 0);
    c.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
    let mut out = Vec::new();
    d.pump(&mut s, 0, 0, &mut out);
    assert!(out.is_empty() && d.is_closed());
    assert_eq!(
        (d.counters().first_not_join, d.counters().wrong_channel),
        (1, 1)
    );
    assert_eq!(received(&mut c, 0), [Message::Kick(Kick::Malformed)]);
}

/// SEC-U4E-3 (advisor-u4e-r1 R5): a stream message while the session's `Join` waits for the room's answer is dropped
/// and counted under its own name; the session stays open.
#[test]
fn sec_u4e_3_a_stream_message_while_joining_is_counted() {
    let (mut s, mut c) = Loopback::pair(1_200);
    let mut d = decoder(Quant::default());
    c.send_reliable(Channel::Control, &bytes(&Message::Join(join(0))), 0);
    let mut out = Vec::new();
    d.pump(&mut s, 0, 0, &mut out);
    assert!(matches!(out.as_slice(), [(Message::Join(_), 0)]));
    out.clear();
    c.send_reliable(Channel::Control, &bytes(&Message::Resync(Resync)), 1);
    d.pump(&mut s, 1, 0, &mut out);
    assert!(out.is_empty() && !d.is_closed());
    assert_eq!(d.counters().joining_dropped, 1);
}

/// The socket checks through the pump (A26; -21's two decode sites): an `Input` with a byte after its last field is
/// refused and counted as trailing bytes, one on the control stream as the wrong channel (refused and counted, never
/// kicked: §4.10); the session stays open and the room gets neither.
fn socket_case(quant: Quant) -> (u64, u64, usize) {
    let (mut s, mut c, mut d) = joined(quant);
    let mut trailing = input();
    trailing.push(0);
    c.send_unreliable(&trailing, 1);
    // An `Input` framed as a control-stream message: its 2 B length, then its type and body.
    let body = input();
    let mut framed = (body.len() as u16).to_le_bytes().to_vec();
    framed.extend_from_slice(&body);
    c.send_reliable(Channel::Control, &framed, 1);
    let mut out = Vec::new();
    d.pump(&mut s, 1, 0, &mut out);
    assert!(!d.is_closed());
    (
        d.counters().trailing_bytes,
        d.counters().wrong_channel,
        out.len(),
    )
}

#[test]
fn pm_0001_21_the_pump_counts_trailing_bytes_and_the_wrong_channel() {
    assert_eq!(socket_case(Quant::default()), (1, 1, 0));
}

/// The pump's two npt-proto bites (bites builds only; their ids are npt-proto's, chosen through `Quant::with_bite`):
/// the clean test's counts stay 0 under each.
#[cfg(feature = "bites")]
mod bites {
    use super::*;

    #[test]
    fn pm_0001_21_no_trailing_leaves_the_pumps_trailing_count_at_0() {
        let q = Quant::with_bite("PM-0001-21/no-trailing").expect("npt-proto plants it");
        assert_eq!(socket_case(q).0, 0);
    }

    #[test]
    fn pm_0001_21_no_channel_leaves_the_pumps_wrong_channel_count_at_0() {
        let q = Quant::with_bite("PM-0001-21/no-channel").expect("npt-proto plants it");
        assert_eq!(socket_case(q).1, 0);
    }
}
