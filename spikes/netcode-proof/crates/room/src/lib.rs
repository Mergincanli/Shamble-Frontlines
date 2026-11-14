//! npt-room: the room library of the netcode proof test (plan §4.1, §4.4, §4.6; ADR 0002:189).
//!
//! Slots and roster, every tick-counted command check, the input buffer under both OQ-0001-6 options, interest, hit
//! history and the rewind query, per-client encoding and the event queue; it accepts only decoded messages and talks
//! to the sim through one seam (`SimCore`). No I/O, no async runtime, no clock; it builds for wasm32. Built in unit U4.
//!
//! U4e (u4-build-plan.md §5 U4e) builds [`Room`] over the seam alone, with its [`Roster`] and constant-time token
//! compare, each client's [`ClientSlot`] ([`AckRing`], [`Encoder`], [`EventQueue`], [`InputBuffer`] under option B,
//! [`Priority`]), the room's [`InterestGrid`] and [`AreaFlood`], [`rewind_at`]'s integer half, and the room-side checks
//! PM-0001-58's counters pass through. Every value is an integer: the room holds no float (u4-build-plan.md §4.11; the
//! lint below and check.mjs's float grep).
#![deny(clippy::float_arithmetic)]

mod ack;
mod encoder;
mod events;
mod input;
mod interest;
mod rewind;
mod room;
mod roster;

pub use ack::AckRing;
pub use encoder::Encoder;
pub use events::EventQueue;
pub use input::InputBuffer;
pub use interest::{AreaFlood, InterestGrid, Priority};
pub use rewind::rewind_at;
pub use room::{ClientCounters, ClientSlot, Room};
pub use roster::Roster;

/// The bite ids npt-room plants (bites builds only; U4E-Q3 (a): a bites-only list of the room's own ids), each chosen
/// per instance through [`Room::with_bite`] (u4-build-plan.md §4.11): -18's baseline choice, -58's three room-side
/// compares, -62's event routing, -22's event-queue cap, and catalogue item 5's class filter, whose id is outside the
/// `PM-<ADR>-<row>/<name>` form (U4E-Q13 (a): no mark judges it). npt-proto's ids reach the room's encoder through
/// `npt_proto::Quant::with_bite`, never spelled here.
#[cfg(feature = "bites")]
pub const BITES: [&str; 7] = [
    "PM-0001-18/no-fallback",
    "PM-0001-58/lt-ack",
    "PM-0001-58/lt-cmd-seq",
    "PM-0001-58/lt-press",
    "PM-0001-62/server-echo",
    "PM-0001-22/unbounded",
    "catalogue-5/owner-to-all",
];
