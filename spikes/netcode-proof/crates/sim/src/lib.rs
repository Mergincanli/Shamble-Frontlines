//! npt-sim: the netcode proof test's stand-in sim (plan E1, §4.5, §4.6; ADR 0001:1428-1435).
//!
//! Movement and hitscan, its soldiers driven by protocol clients (no bot logic here), the stress scene's counts and
//! S1's stand-ins as data behind one narrow seam, and a per-tick state hash. It follows ADR 0004 part 7's float
//! allow-list (enforced by this crate's clippy.toml) and builds for wasm32. Built in unit U4.
//!
//! U4s (u4-build-plan.md §5) builds what every other U4 crate needs first: the tick constant and the values derived
//! from it ([`TICK_HZ`], [`LOOP_CATCHUP`] and the rest: §4.3), the wire grid ([`grid`]: §4.4), and the seam
//! ([`SimCore`], its plain-data types, each entity kind's field layout and [`HitHistory`]: §4.2). `World`, the one
//! implementation, is U4d's.

pub mod grid;
mod history;
mod seam;
mod ticks;

pub use history::HitHistory;
pub use seam::{
    ClampReport, Command, CoverVariant, EntityKind, EntityRecord, HitVolumes, InputReports,
    PortalStates, Rewind, SimCore, SimEvent, TickInput,
};
pub use ticks::{
    ACK_RING_TICKS, FULL_STATE_ACK_TICKS, LOOP_CATCHUP, SETTINGS_STEP_TICKS, TICK_HZ,
    ack_ring_ticks_at, full_state_ack_ticks_at, loop_catchup_at, settings_step_ticks_at,
    ticks_for_ms, ticks_for_ms_at,
};

/// The bite ids the authority step's own-copy snap plants (bites builds only). u4-build-plan.md §4.14 plants each bite
/// where its code lives, and the snap is the sim's, so its id lives here and not in [`grid`]: U4d's `World::with_bite`
/// takes its snap's grid from [`own_copy_snap_with_bite`] for these ids, while the grid the encoder codes with stays
/// the wire grid.
#[cfg(feature = "bites")]
pub const OWN_COPY_SNAP_BITES: [&str; 1] = ["PM-0001-41/server-quant-differs"];

/// The grid the authority step's own-copy snap uses under the planted defect `bite` (bites builds only): for
/// PM-0001-41/server-quant-differs ("the server quantising its own copy differently from what it sends") the grid's one
/// defect, [`grid::Config::floor_rounding`]; `None` for an id the snap does not plant.
#[cfg(feature = "bites")]
pub fn own_copy_snap_with_bite(bite: &str) -> Option<grid::Config> {
    OWN_COPY_SNAP_BITES
        .contains(&bite)
        .then(grid::Config::floor_rounding)
}
