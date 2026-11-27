//! The simulation's time unit (ADR 0004 part 4): timers count whole units, of which a tick is a whole number. The
//! content crate turns a duration written in seconds into units (`content::time`). The two crates hold the same
//! number, and `crates/devtools/tests/time_unit.rs` says so, until this crate depends on that one (4b-2, with the
//! collision library): from then on `UNITS_PER_SECOND` here is `pub use content::time::UNITS_PER_SECOND`, that
//! test's first check goes, and the constant check below stays.

use crate::TICK_HZ;

/// Units in a second: the least number that a millisecond and a 60th of a second both divide.
pub const UNITS_PER_SECOND: u32 = 3_000;
/// Units in a tick of this build.
pub const UNITS_PER_TICK: u32 = UNITS_PER_SECOND / TICK_HZ;

// A tick is a whole number of units at this build's rate and at the two rates the decision drafts name.
const _: () = assert!(
    UNITS_PER_SECOND.is_multiple_of(TICK_HZ)
        && UNITS_PER_SECOND.is_multiple_of(30)
        && UNITS_PER_SECOND.is_multiple_of(60)
);
