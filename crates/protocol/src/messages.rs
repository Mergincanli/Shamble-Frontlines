//! The first message types (ADR 0001, the message table and parts 5 and 6): what each message holds, as plain Rust
//! types. Their first wire form is `codec`'s: whole bytes, every command sent whole; the narrower widths and the
//! delta coding come before online play. A test writes their TypeScript form into `packages/types` (`ts_export`).
//!
//! A field marked "provisional" is open in the decision drafts. What may still move it: how commands are timed at
//! the server (OQ-0001-6), which Slice 3 picks; the netcode proof test's results (OQ-0001-7, R2-XU-OQ4); the
//! definition of the mode hash and the join token's format. A counter (a sequence, a press counter) wraps at its
//! wire width: two values are compared only with `wrap`'s compares, never with `<`.

/// The protocol's version, which a `Join` carries and the room checks.
pub const PROTOCOL_VERSION: u16 = 1;

/// The buttons a command has room for: one held bit and one press counter each.
pub const BUTTONS: usize = 16;

/// The most commands one `Input` carries: the newest and its repeats (ADR 0001 part 5: "data, proposed 8"; a
/// constant here until the content data exists).
pub const REDUNDANCY_CAP: usize = 8;

/// The most snapshot acknowledgements one `Input` carries: one per entity group, and there is one group so far.
pub const MAX_ACKS: usize = 4;

/// The longest join token, in bytes. Provisional with the token's format.
pub const MAX_TOKEN_BYTES: usize = 1024;

/// The longest mode id, in bytes of UTF-8.
pub const MAX_MODE_ID_BYTES: usize = 64;

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
    /// The player's slot in the room, which is the soldier's index in the simulation (ADR 0004 part 2). Provisional:
    /// its width follows the wire's entity index.
    pub slot: u8,
    /// Provisional: its width.
    pub team: u8,
    /// Provisional: its width, and whether squads are numbered per team or across the room.
    pub squad: u8,
    /// The mode's id; content ids are strings (ADR 0005).
    pub mode_id: String,
    /// Provisional: only ADR 0001's message table names it, and what it hashes is not set.
    pub mode_hash: [u8; 32],
    /// SHA-256 of the content the room runs (ADR 0005).
    pub content_hash: [u8; 32],
    /// The room's tick when it accepted the client.
    pub server_tick: u32,
    /// Ticks per second. The rate is fixed when the game is built (ADR 0004 part 4): the client checks this against
    /// its own and never sets its clock from it. Provisional: its width.
    pub tick_rate_hz: u8,
    /// Snapshots per second, a nominal rate: under pressure the room sends fewer (ADR 0001 part 10), so the client
    /// takes the time between snapshots from their ticks. Provisional: its width.
    pub snapshot_rate_hz: u8,
}

/// Room to client, the answer to a `Join` it refuses.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct JoinRejected {
    pub reason: RejectReason,
}

/// Why a room refused a `Join`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum RejectReason {
    /// The client speaks another version of the protocol.
    ProtocolVersion,
    /// The client loaded other content than the room runs.
    ContentHash,
    /// Every slot is taken.
    RoomFull,
}

/// Client to room, every tick: the newest snapshot the client decoded per entity group, and every command the room
/// has not acknowledged yet, up to `REDUNDANCY_CAP` (ADR 0001 part 5).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct Input {
    /// One per entity group the client has decoded a snapshot of.
    pub acks: Vec<SnapshotAck>,
    /// The commands not acknowledged yet, the newest first: each one's sequence number is one below the command
    /// before it (wrapping), and its tick 1 to 255 ticks earlier.
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
    /// Moves forward only and wraps. Provisional: its width (the drafts say 16 bits or narrower).
    pub sequence: u16,
    /// The move vector's x; the vector's length is at most 1, and a room refuses a longer one. Provisional: its scale
    /// and width.
    pub move_x: i8,
    /// The move vector's y. Provisional: its scale and width.
    pub move_y: i8,
    /// One bit per button held. Provisional: 16 is a capacity, and the buttons enter with the systems that read them.
    pub buttons: u16,
    /// A press counter per button, so a resent command neither loses nor doubles a press; each wraps. Provisional:
    /// its width.
    pub presses: [u8; BUTTONS],
    /// The world point the player aims at.
    pub aim: AimPoint,
    /// The time the player's view showed when the command was made.
    pub view_time: ViewTime,
}

/// The world point the player aims at (ADR 0001 part 5: "at most 80 bits"). Provisional as a whole: the step of
/// 1 cm and the widths are borrowed from the snapshot encoding of entity positions (part 6, E3); the codec sets them
/// under PM-0001-51. The encoder clamps to the map: E3's height is 12 bits (0 to 40.95 m), and `y` here holds more.
/// x -32,768 with y 0 and z 0 means "no aim in this command" ([`AimPoint::NONE`]): a room takes it without asking the
/// map's bounds, and refuses any other aim outside them.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct AimPoint {
    /// In centimetres. Provisional, as the type says.
    pub x: i16,
    /// Height, in centimetres. Provisional, as the type says; that y is up is fixed by no decision draft yet either.
    pub y: u16,
    /// In centimetres. Provisional, as the type says.
    pub z: i16,
}

impl AimPoint {
    /// No aim in this command: x is its type's least, outside every map's bounds, so no real aim is ever this point.
    pub const NONE: Self = Self {
        x: i16::MIN,
        y: 0,
        z: 0,
    };
}

/// The time the player's view showed: the interpolation tick plus a fraction of a tick.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ViewTime {
    /// The interpolation tick.
    pub tick: u32,
    /// In 1/65,536 of a tick. Provisional: the simulation's time unit is set with the skeleton.
    pub fraction: u16,
}

/// The receiving player's own part of a snapshot (ADR 0001 part 6's owner-only section). Provisional: so far it is
/// the input the room last applied for the player, which is what the page's key diagram shows and what the client
/// checks its sent presses against; the soldier's own state joins with the soldier.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct OwnState {
    /// One bit per button the room holds as held.
    pub buttons: u16,
    /// The press counter per button, as the room last applied it; each wraps.
    pub presses: [u8; BUTTONS],
}

/// Room to client, every tick: the snapshot's header (ADR 0001 part 6) and the receiving player's own part. Its
/// other sections enter with the data they hold.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct Snapshot {
    /// The full 32-bit server tick; the wire's other ticks are 16-bit offsets from it.
    pub server_tick: u32,
    /// The tick of the snapshot this one is a delta against; none when it is coded against the defaults.
    pub baseline_tick: Option<u32>,
    /// The sequence of the newest command the room received, which ends its repeats; none before the first.
    /// Provisional: its form (a sequence or a tick) and its width.
    pub newest_command_received: Option<u16>,
    /// The sequence of the last command the room ran, where the client's replay starts; none before the first.
    /// Provisional: its form (a sequence or a tick) and its width.
    pub last_processed_command: Option<u16>,
    /// The entity group this datagram holds. Provisional: its width.
    pub group: u8,
    /// The receiving player's own part; none before the room has a player in that slot.
    pub own: Option<OwnState>,
}
