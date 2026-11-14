//! npt-server: the room process of the netcode proof test, as a library (plan §4.1, §4.4, §4.6; ADR 0004 part 5).
//!
//! The S17 loop through the pure function `due_ticks`, the tick thread and the I/O runtime (where every inbound message
//! is decoded), the wall-time limits from an injected `now`, the per-slot local control sockets, metrics and logs, and
//! the panic catch. npt-lab imports this library so L1 runs the real server code on simulated time. Built in U4 and U6.
//!
//! Built so far: the loop policy (U4b): [`Schedule`] and [`due_ticks`], tested on a fake clock with PM-0004-67's three
//! bites; and the I/O step (U4e): [`Decoder::pump`] with its [`ConnLimits`] and [`Counters`]. Each item has one path,
//! at the crate root (W1-PUB's nit: the modules are private).

mod pump;
mod schedule;

pub use pump::{ConnLimits, Counters, Decoder, RttSource};
#[cfg(feature = "bites")]
pub use schedule::BITES;
pub use schedule::{Dropped, Due, LoopCounters, Schedule, due_ticks};
