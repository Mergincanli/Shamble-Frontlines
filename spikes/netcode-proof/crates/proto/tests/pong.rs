//! U4f's `Pong` field (U4F-Q6 (b); the advisor's ruling: the server's time at the `Pong`'s send, exact to 1 µs or
//! better, inside the 32 B limit): its codec round trip, its size against the limit, and the catalogue's row.

use npt_link::Channel;
use npt_proto::{Direction, Limits, Message, Pong, decode, encode};

const DOC: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../../docs/design/netcode-proof-messages.md"
);

/// splitmix64 (bots/src/lib.rs's step).
fn mix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// The field's round trip, exact to the ns: 1,000 seeded values (seed 0x5eed_009f) and the edges 0, 1 and u64::MAX,
/// both ways (the `Pong` travels both ways).
#[test]
fn u4f_the_pongs_server_time_round_trips_exactly() {
    let mut s: u64 = 0x5eed_009f;
    let mut values: Vec<u64> = (0..1_000).map(|_| mix(&mut s)).collect();
    values.extend([0, 1, u64::MAX]);
    for v in values {
        let pong = Pong {
            id: v as u8,
            echo_ns: v.rotate_left(7),
            tick: v as u32,
            hold_us: v as u16,
            server_ns: v,
        };
        let mut out = Vec::new();
        encode(&Message::Pong(pong), &mut out).expect("a Pong encodes");
        for way in [Direction::RoomToClient, Direction::ClientToRoom] {
            assert_eq!(
                decode(way, Channel::Datagram, &out),
                Ok(Message::Pong(pong)),
                "{v}"
            );
        }
    }
}

/// The field fits the `Pong`'s 32 B limit: every `Pong` is 24 B (1 B type + 1 + 8 + 4 + 2 + 8), and 33 B is refused as
/// too large.
#[test]
fn u4f_the_pong_with_its_server_time_is_24_b_inside_the_32_b_limit() {
    let mut out = Vec::new();
    let big = Pong {
        id: u8::MAX,
        echo_ns: u64::MAX,
        tick: u32::MAX,
        hold_us: u16::MAX,
        server_ns: u64::MAX,
    };
    encode(&Message::Pong(big), &mut out).expect("the largest Pong encodes");
    assert_eq!(out.len(), 24);
    assert!(out.len() <= Limits::PONG && Limits::PONG == 32);
    let mut long = out.clone();
    long.resize(33, 0);
    assert!(decode(Direction::RoomToClient, Channel::Datagram, &long).is_err());
}

/// The catalogue's `Pong` row (.claude/rules/netcode.md: every protocol change updates the catalogue) names the field
/// and gives its 24 B sum under the 32 B limit.
#[test]
fn u4f_the_catalogues_pong_row_names_the_server_time_and_its_size() {
    let text = std::fs::read_to_string(DOC).expect("the catalogue");
    let row = text
        .lines()
        .find(|l| l.starts_with("| `Pong` |"))
        .expect("the Pong row");
    for words in [
        "8 B sender's time at the send = 24 B",
        "| 32 B (proposed",
        "a client's `Pong` carries 0 there, and the room never reads it",
    ] {
        assert!(row.contains(words), "the Pong row lacks {words:?}");
    }
}
