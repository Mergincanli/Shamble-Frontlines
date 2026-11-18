//! The first message types (ADR 0001, the message table and parts 5 and 6): what each message holds, as plain Rust
//! types. There is no codec yet: the wire form (bit widths, delta coding, 16-bit tick offsets) comes with it, in the
//! protocol's first-messages step. A test writes their TypeScript form into `packages/types` (`ts_export`).
//!
//! A field marked "provisional" is open in the decision drafts. What may still move it: the netcode proof test's
//! results (OQ-0001-6, OQ-0001-7, R2-XU-OQ4), the codec, the definition of the mode hash and the join token's format.

/// Client to room, the first message on the control stream.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct Join {
    /// The join token's bytes. Provisional: its format comes with the lobby.
    pub token: Vec<u8>,
    /// Provisional: its width.
    pub protocol_version: u16,
    /// SHA-256 of the content the client loaded (ADR 0005).
    pub content_hash: [u8; 32],
}

/// Room to client, the answer to a `Join` it accepts.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct JoinAccepted {
    /// The player's slot in the room. Provisional: its width.
    pub slot: u8,
    /// Provisional: its width.
    pub team: u8,
    /// Provisional: its width.
    pub squad: u8,
    /// The mode's id; content ids are strings (ADR 0005).
    pub mode_id: String,
    /// Provisional: only ADR 0001's message table names it, and what it hashes is not set.
    pub mode_hash: [u8; 32],
    /// SHA-256 of the content the room runs (ADR 0005).
    pub content_hash: [u8; 32],
    /// The room's tick when it accepted the client.
    pub server_tick: u32,
    /// Ticks per second. Provisional: its width.
    pub tick_rate_hz: u8,
    /// Snapshots per second. Provisional: its width.
    pub snapshot_rate_hz: u8,
}

/// Client to room, every tick: the newest snapshot the client decoded per entity group, and every command the room
/// has not acknowledged yet, up to a cap that is data (ADR 0001 part 5).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct Input {
    pub acks: Vec<SnapshotAck>,
    pub commands: Vec<Command>,
}

/// The newest snapshot the client decoded for one entity group.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SnapshotAck {
    /// The entity group's index. Provisional: its width.
    pub group: u8,
    /// That snapshot's server tick.
    pub tick: u32,
}

/// One tick of one player's input (ADR 0001 part 5).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct Command {
    /// The tick the command is for.
    pub tick: u32,
    /// Moves forward only and wraps.
    pub sequence: u16,
    /// The move vector's x; the vector's length is at most 1. Provisional: its scale and width.
    pub move_x: i8,
    /// The move vector's y. Provisional: its scale and width.
    pub move_y: i8,
    /// One bit per button held. Provisional: 16 is a capacity, and the buttons enter with the systems that read them.
    pub buttons: u16,
    /// A press counter per button, so a resent command neither loses nor doubles a press. Provisional: its width.
    pub presses: [u8; 16],
    pub aim: AimPoint,
    pub view_time: ViewTime,
}

/// The world point the player aims at, in centimetres (ADR 0001 part 5: "at most 80 bits").
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct AimPoint {
    pub x: i16,
    /// Height. Provisional: that y is up is fixed by no decision draft yet.
    pub y: u16,
    pub z: i16,
}

/// The time the player's view showed: the interpolation tick plus a fraction of a tick.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ViewTime {
    pub tick: u32,
    /// In 1/65,536 of a tick. Provisional: the simulation's time unit is set with the skeleton.
    pub fraction: u16,
}

/// Room to client, every tick: the snapshot's header (ADR 0001 part 6). Its sections enter with the data they hold.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct Snapshot {
    /// The full 32-bit server tick; the wire's other ticks are 16-bit offsets from it.
    pub server_tick: u32,
    /// The tick of the snapshot this one is a delta against; none when it is coded against the defaults.
    pub baseline_tick: Option<u32>,
    /// The sequence of the newest command the room received, which ends its repeats. Provisional: its form.
    pub newest_command_received: Option<u16>,
    /// The sequence of the last command the room ran, where the client's replay starts. Provisional: its form.
    pub last_processed_command: Option<u16>,
    /// The entity group this datagram holds. Provisional: its width.
    pub group: u8,
}
