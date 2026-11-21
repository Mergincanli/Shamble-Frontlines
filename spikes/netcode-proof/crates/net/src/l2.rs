//! L2: the impairment layer under quinn's socket on RS (plan E15; unit U3b; the advisor's M1-M5, M9-M15, M19 in
//! `.checkpoints/netcode-proof-test/advisor-u3q.md`). Its public surface is plan §4.7's: [`ImpairSocket`] and
//! [`L2Config`]; everything else here is pub(crate).
//!
//! - `config`: [`L2Config`], reusing npt-l1's `Profile` and `ReorderStep` (U3-Q7).
//! - `impair`: the pure impairment core (peers, fates, the reorder step, the one release queue).
//! - `shaper`: PM-0001-23/-24's FIFO with its drop-tail queue (M19).
//! - `stats`: lateness, counters, the fate log; `names`: the names they are reported under (SERIES.md).
//! - `thread`: the release thread's slack, policy and CPU through /proc (M14).
//! - `socket`: [`ImpairSocket`] itself.
//! - `wt`: each WebTransport crate's server on a quinn endpoint over ImpairSocket (M7, M8, M9), used by the loopback
//!   tests here and by WtServer at U6.
//! - `delay`: [`DelayStream`], the WebSocket fallback's wrapper over the TCP model (U3c), with its release clock
//!   (`delay_clock`), its request and frame scanner (`ws_frame`) and its names (`delay_names`).
//! - `tls_test` (tests only): the test certificate and the pinning client both TLS paths' loopback tests use.

mod config;
mod delay;
mod delay_clock;
mod delay_names;
mod impair;
mod names;
mod shaper;
mod socket;
mod stats;
mod thread;
#[cfg(test)]
pub(crate) mod tls_test;
mod ws_frame;
// WtServer joins at U6; until then only the loopback tests use these.
#[cfg(any(feature = "wt-wtransport", feature = "wt-wtq"))]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) mod wt;

pub use config::L2Config;
pub use delay::DelayStream;
pub use socket::ImpairSocket;

/// Both servers' QUIC migration setting (M9; Mustafa's U3-Q6 (a)): off, so a peer's per-address profile cannot be
/// lost to a new address mid-run. The header records it (`l2_migration`). Whether the game allows migration is a later
/// ADR 0001 question.
pub(crate) const MIGRATION: bool = false;
