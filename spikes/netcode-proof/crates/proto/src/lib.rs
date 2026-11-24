//! npt-proto: the netcode proof test's wire format (plan E3, E4, §4.7; ADR 0001:654-678 and its catalogue).
//!
//! Messages and session controls, each with its channel, class, authority and size limit as
//! docs/design/netcode-proof-messages.md lists them; E3 quantisation; the delta codec. Built in unit U4a
//! (u4-build-plan.md §5): the 15 built types and `SessionControl`'s four variants, [`RoomControl`], framing per
//! channel, [`Limits`], [`Quant`], [`Wrap`], the `Input` codec with U2's delta repeats, the `Snapshot` container and
//! its field-group layout table, the `EntityRecord` ↔ wire mapping ([`EntityDelta`]), [`client_hash`], [`encode`] and
//! [`decode`].
//!
//! - **One grid, from npt-sim:** every float becomes a grid integer through `npt_sim::grid` (the sim's own-copy snap,
//!   the room's encoder and the client call the same function); npt-proto packs the integers ([`Quant`]) and reads
//!   each entity kind's fields from `EntityKind::layout()`, never from a copy.
//! - **Decoders of network input** check the size first, refuse unknown types, wrong directions and channels,
//!   trailing bytes and set reserved bits, and every value outside its field; they return an error and never panic
//!   (ADR 0001:446; .claude/rules/netcode.md).
//! - **Planted defects** (bites builds only): `BITES`, each chosen per caller through `Quant::with_bite`, and
//!   `Wrap::plain_lt`, whose ids live at their sites (msg.rs, quant.rs and wrap.rs say which).
//! - No transport, no clock; builds for wasm32. Its clippy.toml is npt-sim's, below its header (check.mjs compares
//!   them).

mod bits;
mod control;
mod error;
mod hash;
mod input;
mod limits;
mod msg;
mod quant;
mod snapshot;
mod wrap;

pub use control::{ProfileId, RoomControl, SessionControl, SessionStatus, StepResult, TokenKind};
pub use error::{DecodeError, EncodeError};
pub use hash::client_hash;
pub use input::Input;
pub use limits::Limits;
pub use msg::{
    ClientSettings, Direction, EndReason, Join, JoinAccepted, JoinRejected, Kick, Leave, Message,
    Ping, Pong, Resync, RoomEnded, channel_of, decode, encode,
};
#[cfg(feature = "bites")]
pub use quant::BITES;
pub use quant::Quant;
pub use snapshot::{Class, EntityDelta, FullState, Snapshot};
pub use wrap::Wrap;

/// The protocol version every `Join` must carry (plan §10.3). One home for the room, U6b's clients and U7's page
/// (advisor-u7-brief.md C2): the value U6a's binary held, unchanged.
pub const PROTOCOL_VERSION: u16 = 1;
/// The content hash every `Join` must carry and every `JoinAccepted` names (no plan line names the content's files yet:
/// U6a's report, open items); U6a's binary's value, unchanged.
pub const CONTENT_HASH: [u8; 32] = *b"npt-server room content hash v1 ";
/// The mode hash every `JoinAccepted` names (one mode at the spike: all zero); U6a's binary's value, unchanged.
pub const MODE_HASH: [u8; 32] = [0; 32];
