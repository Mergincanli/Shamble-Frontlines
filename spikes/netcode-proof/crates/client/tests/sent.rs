//! U7 R2's endpoint traces (C-U7-3; NC-U4F-4): `ClientEndpoint::sent` reaches the clock as each ping's t0, the wire
//! keeps the step's time, and a host that never calls it (or calls it with the step's own time) changes nothing.

use npt_client::{ClientEndpoint, HONEST_MAX_LEAD};
use npt_link::Channel;
use npt_proto::{Direction, Join, JoinAccepted, Message, Pong, decode, encode};

const MS: u64 = 1_000_000;
/// The room's clock runs 3 s ahead of the host's (offset, host minus server, -3 s), on a symmetric 80 ms path.
const OFFSET: i64 = -3_000 * MS as i64;
const ONE_WAY: u64 = 40 * MS;

fn bytes(m: &Message) -> Vec<u8> {
    let mut out = Vec::new();
    encode(m, &mut out).expect("encodes");
    out
}

/// What a run gave: each step's datagrams and reliable messages, the clock's estimate at its end, and each `Ping`'s
/// wire stamp less its step's time.
type Run = (
    Vec<(Vec<Vec<u8>>, Vec<(Channel, Vec<u8>)>)>,
    Option<i64>,
    Vec<u64>,
);

/// One endpoint under the controller, joined at once, stepped at 30 Hz for 3 s; each step's output leaves `lag` ns
/// after the step, and with `with_sent` the host passes that time to `sent` before the next step; the room answers each
/// `Ping` over the path (several steps per round trip).
fn run(lag: u64, with_sent: bool) -> Run {
    let mut e = ClientEndpoint::new(Join::default(), 30).with_clock(HONEST_MAX_LEAD);
    let joined = Message::JoinAccepted(JoinAccepted {
        tick: 100,
        ..JoinAccepted::default()
    });
    let mut inbox = vec![(Channel::Control, bytes(&joined), 0)];
    let (mut steps, mut wire) = (Vec::new(), Vec::new());
    for k in 0..90u64 {
        let now = k * 1_000_000_000 / 30;
        let (due, later): (Vec<_>, Vec<_>) = std::mem::take(&mut inbox)
            .into_iter()
            .partition(|m| m.2 <= now);
        inbox = later;
        e.input(None, &due, now);
        let out = e.step();
        let left = now + lag;
        for d in &out.datagrams {
            if let Ok(Message::Ping(p)) = decode(Direction::ClientToRoom, Channel::Datagram, d) {
                wire.push(p.sent_ns - now);
                let server = ((left + ONE_WAY) as i64 - OFFSET) as u64;
                let pong = Message::Pong(Pong {
                    id: p.id,
                    echo_ns: p.sent_ns,
                    tick: 100,
                    hold_us: 0,
                    server_ns: server,
                });
                inbox.push((Channel::Datagram, bytes(&pong), left + 2 * ONE_WAY));
            }
        }
        if with_sent {
            e.sent(left);
        }
        steps.push((out.datagrams, out.reliable));
    }
    (steps, e.clock().offset_ns(), wire)
}

/// U7 R2 (C-U7-3): the endpoint passes the host's send time to its clock: with a 5 ms send lag the estimate is the true
/// offset with `sent` and 2.5 ms low without it.
#[test]
fn r2_the_endpoint_gives_its_clock_the_send_time() {
    assert_eq!(run(5 * MS, true).1, Some(OFFSET));
    assert_eq!(run(5 * MS, false).1, Some(OFFSET - 2_500_000));
}

/// U7 R2 (C-U7-3): no `sent` call, or one with the step's own time, leaves every stamp as before: the same bytes at
/// every step and the same estimate; and with a lag the wire still carries the step's time (each `Ping`'s stamp).
#[test]
fn r2_no_sent_call_leaves_every_stamp_as_today() {
    let (plain, plain_est, plain_wire) = run(0, false);
    let (same, same_est, _) = run(0, true);
    assert_eq!(plain, same);
    assert_eq!(plain_est, same_est);
    assert!(plain_wire.len() >= 25 && plain_wire.iter().all(|&w| w == 0));
    let (lagged, _, lagged_wire) = run(5 * MS, true);
    assert_eq!(
        lagged,
        run(5 * MS, false).0,
        "the bytes sent do not depend on `sent`"
    );
    assert!(
        lagged_wire.iter().all(|&w| w == 0),
        "the wire keeps the step's time"
    );
}
