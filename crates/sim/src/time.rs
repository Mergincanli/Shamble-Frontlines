//! The simulation's time unit (ADR 0004 part 4): timers count whole units, of which a tick is a whole number. The
//! content crate turns a duration written in seconds into units (`content::time`), and the number of units in a
//! second is that crate's own: one number, in one place.

use crate::TICK_HZ;

/// Units in a second: the least number that a millisecond and a 60th of a second both divide.
pub use content::time::UNITS_PER_SECOND;
/// Units in a tick of this build.
pub const UNITS_PER_TICK: u32 = UNITS_PER_SECOND / TICK_HZ;

// A tick is a whole number of units at this build's rate and at the two rates the decision drafts name.
const _: () = assert!(
    UNITS_PER_SECOND.is_multiple_of(TICK_HZ)
        && UNITS_PER_SECOND.is_multiple_of(30)
        && UNITS_PER_SECOND.is_multiple_of(60)
);
