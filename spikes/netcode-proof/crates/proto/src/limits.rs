//! Every size limit and cap the wire has, as the catalogue gives them (docs/design/netcode-proof-messages.md §2's
//! Size limit column, §1's framing, §4 and §5): catalogue item 3 compares [`Limits::TYPES`] with that column.

use npt_link::Channel;

use crate::msg::Direction;

/// The wire's limits and caps (plan §4.7; catalogue §2). Each message type's limit counts what the catalogue's Size
/// limit column counts: a datagram whole; a control-stream message with its 2 B length, its type and its body; a
/// full state's type and body, after its 4 B length (catalogue §1, "Framing and channels").
#[derive(Clone, Copy, Debug)]
pub struct Limits;

impl Limits {
    /// Commands one `Input` carries: the newest and every unacknowledged one before it, up to this cap (ADR 0001
    /// part 5, proposed 8). npt-room's `CMD_CATCHUP` is set from it (u4-build-plan.md §4.3).
    pub const REDUNDANCY_CAP: usize = 8;
    /// Groups one tick's snapshot may split into: the width of the `Input`'s acknowledgement mask, one bit per group
    /// (u4-build-plan.md §4.5).
    pub const GROUPS: usize = 8;
    /// `Input` (166 B: the catalogue's proposal; the codec's maxima, 5 + 23 + 7 × 19 = 161 B, are inside it).
    pub const INPUT: usize = 166;
    /// `Snapshot` at the send side: the datagram cap until PM-0001-36 measures (catalogue §2; plan.md E4); `encode`
    /// refuses a larger one.
    pub const SNAPSHOT: usize = 1_000;
    /// `Snapshot` at the receive side: npt-client refuses a datagram above it (catalogue §2); `decode` takes one up
    /// to it.
    pub const SNAPSHOT_RECEIVE: usize = 1_150;
    /// `FullState`, its type and body, after the 4 B length (proposed: about 10 × the stress scene's whole world).
    pub const FULL_STATE: usize = 65_536;
    pub const RESYNC: usize = 8;
    pub const PING: usize = 32;
    pub const PONG: usize = 32;
    pub const JOIN: usize = 128;
    pub const JOIN_ACCEPTED: usize = 256;
    pub const JOIN_REJECTED: usize = 16;
    pub const CLIENT_SETTINGS: usize = 8;
    pub const ROOM_ENDED: usize = 64;
    pub const LEAVE: usize = 16;
    pub const KICK: usize = 16;
    /// `SessionControl`, the largest client message (its `RecordStep` variant is 504 B at most).
    pub const SESSION_CONTROL: usize = 512;
    pub const SESSION_STATUS: usize = 32;
    /// A room-wide control on its local socket ([`crate::RoomControl`]; catalogue §5).
    pub const ROOM_CONTROL: usize = 1_024;
    /// `RecordStep`'s note, in bytes of UTF-8.
    pub const NOTE: usize = 480;
    /// The checklist's steps, `RecordStep`'s `step` 1 to this (held as data: catalogue §4).
    pub const RECORD_STEPS: u8 = 14;
    /// Addresses in one room-wide control's peer set (catalogue §5's size line).
    pub const PEERS: usize = 16;

    /// Each built message type, in the catalogue's order (plan §4.7's 13, `SessionControl`, then `SessionStatus`):
    /// its name, channel, direction and size limit (the Size limit column's leading number; `Snapshot`'s is its
    /// send-side cap). The type byte on the wire is the row's position, from 1.
    pub const TYPES: [(&'static str, Channel, Direction, usize); 15] = [
        (
            "Input",
            Channel::Datagram,
            Direction::ClientToRoom,
            Limits::INPUT,
        ),
        (
            "Snapshot",
            Channel::Datagram,
            Direction::RoomToClient,
            Limits::SNAPSHOT,
        ),
        (
            "FullState",
            Channel::FullState,
            Direction::RoomToClient,
            Limits::FULL_STATE,
        ),
        (
            "Resync",
            Channel::Control,
            Direction::ClientToRoom,
            Limits::RESYNC,
        ),
        ("Ping", Channel::Datagram, Direction::BothWays, Limits::PING),
        ("Pong", Channel::Datagram, Direction::BothWays, Limits::PONG),
        (
            "Join",
            Channel::Control,
            Direction::ClientToRoom,
            Limits::JOIN,
        ),
        (
            "JoinAccepted",
            Channel::Control,
            Direction::RoomToClient,
            Limits::JOIN_ACCEPTED,
        ),
        (
            "JoinRejected",
            Channel::Control,
            Direction::RoomToClient,
            Limits::JOIN_REJECTED,
        ),
        (
            "ClientSettings",
            Channel::Control,
            Direction::ClientToRoom,
            Limits::CLIENT_SETTINGS,
        ),
        (
            "RoomEnded",
            Channel::Control,
            Direction::RoomToClient,
            Limits::ROOM_ENDED,
        ),
        (
            "Leave",
            Channel::Control,
            Direction::ClientToRoom,
            Limits::LEAVE,
        ),
        (
            "Kick",
            Channel::Control,
            Direction::RoomToClient,
            Limits::KICK,
        ),
        (
            "SessionControl",
            Channel::Control,
            Direction::ClientToRoom,
            Limits::SESSION_CONTROL,
        ),
        (
            "SessionStatus",
            Channel::Control,
            Direction::RoomToClient,
            Limits::SESSION_STATUS,
        ),
    ];
}
