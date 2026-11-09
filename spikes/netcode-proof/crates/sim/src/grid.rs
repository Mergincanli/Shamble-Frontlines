//! The wire grid (u4-build-plan.md §4.4; the advisor's Q23 ruling): one function each way between the sim's values and
//! the integers the wire carries, per field kind. The sim's own-copy snap, the room's encoder and the client all call
//! it, so the three agree bit for bit; npt-proto's `Quant` packs its integers into their widths.
//!
//! - **To the grid** ([`Config::to_grid`]): one multiply by the field's scale, in f64 (exact for the centimetre fields:
//!   an f32 times 100 needs at most 31 significant bits), then the nearest integer, **a tie rounding away from zero**
//!   (`f64::round`, on ADR 0004's float allow-list), then integers only: a range field saturates at its ends, yaw wraps
//!   around the turn. NaN maps to 0, an infinity to the end of a range field it points past and to one fixed yaw
//!   integer; the sim never holds either (plan §4.5), and no input of any kind panics.
//! - **From the grid** ([`Config::from_grid`]): the integer divided by the same scale in f64, rounded once to f32. Every
//!   integer of a field comes back from the grid as itself (the tests run all of them).
//!
//! Positions are metres, angles radians, velocities metres per second. A field's integers are `min()..=max()`, which
//! fit in `bits()` bits. [`Field::integer`] describes the record fields the sim holds as integers (flag bits, a tick),
//! so one type names every field of an entity's layout (`EntityKind::layout`).
//!
//! **Fixed for wave 1:** the five field constants below are fixed as built while U4a (npt-proto's `Quant` and field
//! table) and U4d (npt-sim) build in parallel; [`VELOCITY`]'s step included. A change to any of them waits for U4d's
//! merge and goes through its own change, so both lanes build against one grid.
//!
//! The grid's one planted defect, bites builds only, is the float side's: `Config::floor_rounding`, chosen through
//! the [`Config`] value a caller passes, never a static (u4-build-plan.md §4.11). It carries no bite id: each bite id
//! lives in the crate of its site, which maps it to the defect (u4-build-plan.md §4.14: "planted where its code
//! lives"): npt-client's PM-0001-17/client-rounding (`npt_client::BITES`), the authority step's own-copy snap's
//! PM-0001-41/server-quant-differs (`npt_sim::OWN_COPY_SNAP_BITES`). PM-0001-57's coarsening bites act on integers, in
//! npt-proto's `Quant` (U4a).

use core::f64::consts::{FRAC_PI_2, TAU};

/// Which values a field carries; part of a field's identity, so two fields of one range but different kinds differ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Position,
    Yaw,
    Pitch,
    Velocity,
    /// A value the sim holds as an integer: no scale.
    Integer,
}

/// One field kind of the wire grid: its scale (grid steps per sim unit), its integers and its width.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Field {
    kind: Kind,
    scale: f64,
    min: i32,
    max: i32,
    bits: u32,
    wraps: bool,
}

impl Field {
    /// A field the sim holds as an integer (flag bits, a tick by its u32 bits, a weapon id): no scale, the integers
    /// `min..=max`, in the fewest bits that hold them, never wrapping. Its value is never a float, so an entity
    /// record's `raw` is 0 for it.
    ///
    /// # Panics
    /// If `min` is not below `max` (a field of one value carries nothing); in a const, a compile error.
    pub const fn integer(min: i32, max: i32) -> Field {
        assert!(min < max, "an integer field holds at least two values");
        let span = (max as i64 - min as i64) as u64;
        Field {
            kind: Kind::Integer,
            scale: 1.0,
            min,
            max,
            bits: u64::BITS - span.leading_zeros(),
            wraps: false,
        }
    }

    /// The field's lowest integer.
    pub const fn min(self) -> i32 {
        self.min
    }

    /// The field's highest integer.
    pub const fn max(self) -> i32 {
        self.max
    }

    /// The field's width on the wire, in bits.
    pub const fn bits(self) -> u32 {
        self.bits
    }

    /// Whether the field wraps around (yaw) rather than saturating at its ends.
    pub const fn wraps(self) -> bool {
        self.wraps
    }

    /// An integer on the grid's line brought into the field: wrapped around the turn (an integer `rem_euclid` over a
    /// power-of-two span, never a float one), or saturated at the field's ends.
    fn fit(self, k: i64) -> i32 {
        let (min, max) = (i64::from(self.min), i64::from(self.max));
        let k = if self.wraps {
            k.wrapping_sub(min).rem_euclid(max - min + 1) + min
        } else {
            k.clamp(min, max)
        };
        k as i32
    }
}

/// Horizontal position (x and y), metres: 1 cm steps in 16 bits, -327.68 m to 327.67 m (ADR 0001 part 6, E3).
pub const POSITION_XY: Field = Field {
    kind: Kind::Position,
    scale: 100.0,
    min: -32_768,
    max: 32_767,
    bits: 16,
    wraps: false,
};

/// Height (z), metres: 1 cm steps in 12 bits, 0 to 40.95 m (ADR 0001 part 6, E3).
pub const POSITION_Z: Field = Field {
    kind: Kind::Position,
    scale: 100.0,
    min: 0,
    max: 4_095,
    bits: 12,
    wraps: false,
};

/// Yaw, radians: 16 bits over the whole turn (steps of about 0.0055 degrees), the integers -32,768 (-π) to 32,767; it
/// wraps, so π and -π are one integer (ADR 0001 part 6, E3; PM-0001-57's bound is 0.01 degrees).
pub const YAW: Field = Field {
    kind: Kind::Yaw,
    scale: 65_536.0 / TAU,
    min: -32_768,
    max: 32_767,
    bits: 16,
    wraps: true,
};

/// Pitch, radians: 10 bits over ±90 degrees, the integers -511 to 511, so both ends are exact (steps of about 0.176
/// degrees, inside PM-0001-57's one 0.18 degree step). The 10-bit code -512 is not a pitch.
pub const PITCH: Field = Field {
    kind: Kind::Pitch,
    scale: 511.0 / FRAC_PI_2,
    min: -511,
    max: 511,
    bits: 10,
    wraps: false,
};

/// Velocity, metres per second: 1 cm/s steps in 16 bits, -327.68 to 327.67 m/s. u4-build-plan.md §4.4 left velocity's
/// step to U4a's field table, and the own-copy snap needs one now: this step is fixed for wave 1 as built (the module
/// text above), so U4a's table states it as given; a different step waits for U4d's merge and changes only this
/// constant.
pub const VELOCITY: Field = Field {
    kind: Kind::Velocity,
    scale: 100.0,
    min: -32_768,
    max: 32_767,
    bits: 16,
    wraps: false,
};

/// The grid a caller uses: in a measured build always the wire grid; in a bites build it may hold the grid's one
/// planted defect, `Config::floor_rounding`, for that caller alone.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Config {
    /// Rounds down in place of to the nearest (bites builds only).
    #[cfg(feature = "bites")]
    floor: bool,
}

impl Config {
    /// `x` on the field's grid: one scale multiply, the nearest integer (a tie away from zero), then the field's
    /// range in integers.
    pub fn to_grid(self, field: Field, x: f32) -> i32 {
        let scaled = f64::from(x) * field.scale;
        #[cfg(feature = "bites")]
        if self.floor {
            return field.fit(scaled.floor() as i64);
        }
        field.fit(scaled.round() as i64)
    }

    /// The value of the field's integer `k`: `k` over the field's scale, rounded once to f32.
    pub fn from_grid(self, field: Field, k: i32) -> f32 {
        (f64::from(k) / field.scale) as f32
    }

    /// `x` snapped to the field's grid: [`Config::from_grid`] of [`Config::to_grid`]. The sim's own-copy snap
    /// (the advisor's Q23 ruling).
    pub fn snap(self, field: Field, x: f32) -> f32 {
        self.from_grid(field, self.to_grid(field, x))
    }
}

#[cfg(feature = "bites")]
impl Config {
    /// The grid's one planted defect (bites builds only): [`Config::to_grid`] rounds down in place of to the nearest,
    /// which moves about half of all values one step off the wire grid; [`Config::from_grid`] is unchanged. It names
    /// no bite: the sites that plant it map their own ids to it (the module text).
    pub const fn floor_rounding() -> Config {
        Config { floor: true }
    }
}
