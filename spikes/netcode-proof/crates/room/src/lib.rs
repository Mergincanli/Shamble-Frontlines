//! npt-room: the room library of the netcode proof test (plan §4.1, §4.4, §4.6; ADR 0002:189).
//!
//! Slots and roster, every tick-counted command check, the input buffer under both OQ-0001-6 options, interest, hit
//! history and the rewind query, per-client encoding and the event queue; it accepts only decoded messages and talks
//! to the sim through one seam (`SimCore`). No I/O, no async runtime, no clock; it builds for wasm32. Built in unit U4.
