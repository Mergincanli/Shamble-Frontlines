//! `channel_of` (C-U6-16 (c); U6a): the stream a framed reliable message travels on, read from its own framing.

use npt_link::Channel;
use npt_proto::{
    EndReason, FullState, JoinAccepted, Kick, Limits, Message, RoomEnded, Snapshot, channel_of,
    encode,
};

fn bytes(m: &Message) -> Vec<u8> {
    let mut out = Vec::new();
    encode(m, &mut out).unwrap_or_else(|e| panic!("{}: {e}", m.name()));
    out
}

/// Every room → client stream message the room queues (`JoinAccepted`, `Kick`, `RoomEnded` on the control stream, a
/// `FullState` on its own) is read on its row's channel; a datagram, a cut message and a frame whose length does not
/// count its bytes are none; and a full-state frame above 262,144 B (built by hand: the room's own stay within
/// `Limits::FULL_STATE`), whose length's third byte is a control type's (4, `Resync`), is still the full-state stream.
#[test]
fn channel_of_reads_each_stream_message_by_its_framing_at_any_length() {
    let control = [
        Message::JoinAccepted(JoinAccepted::default()),
        Message::Kick(Kick::Replaced),
        Message::RoomEnded(RoomEnded {
            reason: EndReason::Crash,
            final_tick: 70_000,
        }),
    ];
    for m in &control {
        assert_eq!(
            channel_of(&bytes(m)),
            Some(Channel::Control),
            "{}",
            m.name()
        );
    }
    let full = bytes(&Message::FullState(FullState {
        tick: 9,
        last_run: None,
        own: vec![],
        entities: vec![],
    }));
    assert_eq!(channel_of(&full), Some(Channel::FullState));
    let snap = bytes(&Message::Snapshot(Snapshot::default()));
    assert_eq!(channel_of(&snap), None, "a datagram has no length prefix");
    let kick = bytes(&Message::Kick(Kick::Overflow));
    assert_eq!(channel_of(&kick[..kick.len() - 1]), None, "cut short");
    assert_eq!(channel_of(&[]), None);
    let mut longer = kick.clone();
    longer.push(0);
    assert_eq!(
        channel_of(&longer),
        None,
        "a byte the length does not count"
    );
    // A full state of 262,148 B after its 4 B length: its length's bytes are [0x04, 0x00, 0x04, 0x00].
    let body = 262_144 + 4;
    let mut big = (body as u32).to_le_bytes().to_vec();
    big.push(3);
    big.resize(4 + body, 0);
    assert_eq!(big[2], 4, "the third byte names Resync, a control type");
    assert_eq!(Limits::TYPES[3].1, Channel::Control);
    assert_eq!(channel_of(&big), Some(Channel::FullState));
}
