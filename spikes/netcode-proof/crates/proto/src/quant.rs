//! `Quant`: the codec as one caller chose it. It packs each grid integer into its field's width
//! (u4-build-plan.md §4.4: the grid's integers come from `npt_sim::grid`, one function each way for the sim's own-copy
//! snap and both codec ends; npt-proto only packs them), and it is the one value through which a caller of
//! [`Quant::encode`] or [`Quant::decode`] chooses npt-proto's planted defects, per instance and never through a static
//! (u4-build-plan.md §4.11). In a measured build every `Quant` is the wire's, `Quant::default()`.
//!
//! **A field's code** is its integer in the field's [`Field::bits`], read from the field's own `min()`, `max()` and
//! `bits()` and never from a copy (u4s-carries.md carry 1): two's complement when the field has negative integers and
//! they fit (positions, yaw, pitch, velocity, a tick by its u32 bits), plain binary when it has none and they fit
//! (height, flag bits), and the offset from `min()` otherwise (an integer field such as `Field::integer(100, 101)`).
//! A code that is no integer of its field is refused on decode, never clamped: pitch's 10-bit code -512
//! (`grid::PITCH` is -511 to 511) is [`DecodeError::OutOfRange`] (u4s-carries.md carry 4).
//!
//! **The planted defects** (bites builds only; `BITES` names each, and no other crate names these ids:
//! u4-build-plan.md §4.14):
//! - PM-0001-57's three, as integer coarsening of the grid integer before it is packed (u4s-carries.md carry 2): to
//!   the nearest multiple of the coarse step in integers (a tie away from zero, as the grid rounds), inside the
//!   field's range; `pos-4cm` every 4th integer of `POSITION_XY` and `POSITION_Z` (errors up to 2 cm),
//!   `yaw-12bit` every 16th of `YAW` (0.088 degree steps, wrapping around the turn), `pitch-8bit` every 4th of
//!   `PITCH` (0.70 degree steps). Every other field, `VELOCITY` included (one range with `POSITION_XY`, another kind),
//!   is untouched. At a non-wrapping field's end, where the nearest multiple lies outside the range, the coarse value
//!   is the nearest multiple inside it (`POSITION_XY`'s 32,767 codes as 32,764).
//! - PM-0001-17/tick16 and PM-0001-58/tick16: the snapshot header's writer keeps only the tick's low 16 bits.
//! - PM-0001-21/no-trailing: the decoder accepts bytes after the last field and set reserved or pad bits.
//! - PM-0001-21/no-channel: the decoder accepts a message on a channel its type does not travel on.

use npt_sim::grid::Field;

use crate::bits::{BitReader, BitWriter};
use crate::error::{DecodeError, EncodeError};

/// The bite ids npt-proto plants (bites builds only), each chosen through [`Quant::with_bite`].
#[cfg(feature = "bites")]
pub const BITES: [&str; 7] = [
    "PM-0001-57/pos-4cm",
    "PM-0001-57/yaw-12bit",
    "PM-0001-57/pitch-8bit",
    "PM-0001-17/tick16",
    "PM-0001-58/tick16",
    "PM-0001-21/no-trailing",
    "PM-0001-21/no-channel",
];

#[cfg(feature = "bites")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Bite {
    Pos4cm,
    Yaw12bit,
    Pitch8bit,
    Tick16,
    NoTrailing,
    NoChannel,
}

/// The codec as one caller chose it: in a measured build always the wire's; in a bites build it may carry one of
/// npt-proto's planted defects (`BITES`) for that caller alone (the module text).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Quant {
    #[cfg(feature = "bites")]
    bite: Option<Bite>,
}

/// How a field's integers become its code (the module text).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Coding {
    Unsigned,
    Signed,
    Offset,
}

fn coding(field: Field) -> Coding {
    let top = 1i64 << field.bits();
    let (min, max) = (i64::from(field.min()), i64::from(field.max()));
    if min >= 0 && max < top {
        Coding::Unsigned
    } else if min >= -(top / 2) && max < top / 2 {
        Coding::Signed
    } else {
        Coding::Offset
    }
}

/// The field's `bits()` low bits set.
fn mask(field: Field) -> u64 {
    (1u64 << field.bits()) - 1
}

/// The multiple of `s` nearest `k`, a tie away from zero (the grid's tie rule: npt_sim::grid).
fn nearest_multiple(k: i64, s: i64) -> i64 {
    let q = if k >= 0 {
        (k + s / 2) / s
    } else {
        -((-k + s / 2) / s)
    };
    q * s
}

impl Quant {
    /// The `Quant` that plants `bite`, one of [`BITES`] (bites builds only); `None` for an id npt-proto does not
    /// plant.
    #[cfg(feature = "bites")]
    pub fn with_bite(bite: &str) -> Option<Quant> {
        let b = match bite {
            "PM-0001-57/pos-4cm" => Bite::Pos4cm,
            "PM-0001-57/yaw-12bit" => Bite::Yaw12bit,
            "PM-0001-57/pitch-8bit" => Bite::Pitch8bit,
            "PM-0001-17/tick16" | "PM-0001-58/tick16" => Bite::Tick16,
            "PM-0001-21/no-trailing" => Bite::NoTrailing,
            "PM-0001-21/no-channel" => Bite::NoChannel,
            _ => return None,
        };
        Some(Quant { bite: Some(b) })
    }

    /// Whether the decoder refuses trailing bytes and set reserved or pad bits (always, but under no-trailing).
    pub(crate) fn strict(self) -> bool {
        #[cfg(feature = "bites")]
        if self.bite == Some(Bite::NoTrailing) {
            return false;
        }
        true
    }

    /// Whether the decoder refuses a message on the wrong channel (always, but under no-channel).
    pub(crate) fn checks_channel(self) -> bool {
        #[cfg(feature = "bites")]
        if self.bite == Some(Bite::NoChannel) {
            return false;
        }
        true
    }

    /// The tick the snapshot header's writer writes: the tick itself, or under tick16 its low 16 bits.
    pub(crate) fn header_tick(self, tick: u32) -> u32 {
        #[cfg(feature = "bites")]
        if self.bite == Some(Bite::Tick16) {
            return tick & 0xffff;
        }
        tick
    }

    /// The coarse step this `Quant` applies to `field`'s integers before packing: 1 (none) but under -57's bites.
    fn step(self, field: Field) -> i64 {
        #[cfg(feature = "bites")]
        {
            use npt_sim::grid::{PITCH, POSITION_XY, POSITION_Z, YAW};
            match self.bite {
                Some(Bite::Pos4cm) if field == POSITION_XY || field == POSITION_Z => return 4,
                Some(Bite::Yaw12bit) if field == YAW => return 16,
                Some(Bite::Pitch8bit) if field == PITCH => return 4,
                _ => {}
            }
        }
        let _ = field;
        1
    }

    /// The integer this `Quant` sends for `k`, an integer of `field`: `k` itself, or under -57's bites the nearest
    /// multiple of the coarse step inside the field's range (the module text).
    pub(crate) fn coarse(self, field: Field, k: i32) -> i32 {
        let s = self.step(field);
        if s == 1 {
            return k;
        }
        let (min, max) = (i64::from(field.min()), i64::from(field.max()));
        let mut c = nearest_multiple(i64::from(k), s);
        if field.wraps() {
            c = (c - min).rem_euclid(max - min + 1) + min;
        } else {
            if c > max {
                c -= s;
            }
            if c < min {
                c += s;
            }
        }
        c as i32
    }

    /// `field`'s code for its integer `k` (after this `Quant`'s coarsening): [`EncodeError::OutOfRange`] for an
    /// integer the field does not hold.
    pub(crate) fn code(self, field: Field, k: i32) -> Result<u64, EncodeError> {
        if k < field.min() || k > field.max() {
            return Err(EncodeError::OutOfRange);
        }
        let k = i64::from(self.coarse(field, k));
        let code = match coding(field) {
            Coding::Unsigned => k,
            Coding::Signed => k & mask(field) as i64,
            Coding::Offset => k - i64::from(field.min()),
        };
        Ok(code as u64)
    }

    /// The integer `field`'s code `code` (its low `bits()` bits) stands for: [`DecodeError::OutOfRange`] for a code
    /// that is no integer of the field, such as pitch's -512, never a clamp.
    pub(crate) fn value(self, field: Field, code: u64) -> Result<i32, DecodeError> {
        let code = code & mask(field);
        let bits = field.bits();
        let k = match coding(field) {
            Coding::Unsigned => code as i64,
            Coding::Signed => {
                let sign = 1u64 << (bits - 1);
                if code & sign != 0 {
                    code as i64 - (1i64 << bits)
                } else {
                    code as i64
                }
            }
            Coding::Offset => code as i64 + i64::from(field.min()),
        };
        if k < i64::from(field.min()) || k > i64::from(field.max()) {
            return Err(DecodeError::OutOfRange);
        }
        Ok(k as i32)
    }

    /// Writes `k`, an integer of `field`, in the field's width.
    pub(crate) fn put(
        self,
        w: &mut BitWriter<'_>,
        field: Field,
        k: i32,
    ) -> Result<(), EncodeError> {
        let code = self.code(field, k)?;
        w.put(code, field.bits());
        Ok(())
    }

    /// Reads one integer of `field`.
    pub(crate) fn get(self, r: &mut BitReader<'_>, field: Field) -> Result<i32, DecodeError> {
        let code = r.get(field.bits())?;
        self.value(field, code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use npt_sim::grid::{PITCH, POSITION_XY, POSITION_Z, VELOCITY, YAW};

    const FIELDS: [(&str, Field); 5] = [
        ("POSITION_XY", POSITION_XY),
        ("POSITION_Z", POSITION_Z),
        ("YAW", YAW),
        ("PITCH", PITCH),
        ("VELOCITY", VELOCITY),
    ];

    /// Every integer of `field`, packed and read back in its width.
    fn round_trip(q: Quant, field: Field, k: i32) -> i32 {
        let mut out = Vec::new();
        let mut w = BitWriter::new(&mut out);
        q.put(&mut w, field, k).expect("an integer of the field");
        assert_eq!(out.len(), (field.bits() as usize).div_ceil(8));
        q.get(&mut BitReader::new(&out), field)
            .expect("its own code")
    }

    #[test]
    fn a4_each_packed_width_holds_its_range_ends_and_every_integer_between() {
        let q = Quant::default();
        for (name, f) in FIELDS {
            // The widths the plan states (u4-build-plan.md §4.4: position 16/16/12 bits, yaw 16, pitch 10; velocity
            // 1 cm/s in 16, a given from npt-sim: u4s-carries.md carry 1).
            let want = match name {
                "POSITION_Z" => 12,
                "PITCH" => 10,
                _ => 16,
            };
            assert_eq!(f.bits(), want, "{name}");
            for k in f.min()..=f.max() {
                assert_eq!(round_trip(q, f, k), k, "{name}: {k}");
            }
            // The ends code as the width's own extremes or inside them, and one past an end is refused.
            assert_eq!(
                q.code(f, f.min() - 1),
                Err(EncodeError::OutOfRange),
                "{name}"
            );
            assert_eq!(
                q.code(f, f.max().saturating_add(1)),
                Err(EncodeError::OutOfRange),
                "{name}"
            );
        }
        // The codes at the ends: XY, yaw and velocity two's complement in 16 bits, Z plain in 12, pitch in 10.
        assert_eq!(q.code(POSITION_XY, -32_768), Ok(0x8000));
        assert_eq!(q.code(POSITION_XY, 32_767), Ok(0x7fff));
        assert_eq!(q.code(POSITION_Z, 4_095), Ok(0xfff));
        assert_eq!(q.code(PITCH, -511), Ok(0x201));
        assert_eq!(q.code(PITCH, 511), Ok(0x1ff));
        // Integer fields of every shape: a tick by its u32 bits, flag bits, and one whose integers need the offset.
        for f in [
            Field::integer(i32::MIN, i32::MAX),
            Field::integer(0, 255),
            Field::integer(0, 1),
            Field::integer(100, 101),
            Field::integer(-1, 300),
        ] {
            for k in [f.min(), f.min() + 1, f.max() - 1, f.max()] {
                assert_eq!(round_trip(q, f, k), k, "{f:?}: {k}");
            }
        }
        assert_eq!(
            q.code(Field::integer(i32::MIN, i32::MAX), -1),
            Ok(0xffff_ffff)
        );
        assert_eq!(q.code(Field::integer(100, 101), 101), Ok(1));
    }

    #[test]
    fn the_pitch_code_minus_512_is_a_decode_error_never_a_panic_or_a_clamp() {
        // u4s-carries.md carry 4: grid::PITCH is -511..=511, so the 10-bit code -512 (0x200) is no pitch.
        let q = Quant::default();
        assert_eq!(q.value(PITCH, 0x200), Err(DecodeError::OutOfRange));
        assert_eq!(
            q.get(&mut BitReader::new(&[0x00, 0x02]), PITCH),
            Err(DecodeError::OutOfRange)
        );
        // Its neighbours are the two ends.
        assert_eq!(q.value(PITCH, 0x201), Ok(-511));
        assert_eq!(q.value(PITCH, 0x1ff), Ok(511));
        // Every other field's unused codes are refused the same way: Z holds every 12-bit code, and an offset field
        // its span only.
        assert_eq!(
            q.value(Field::integer(100, 102), 3),
            Err(DecodeError::OutOfRange)
        );
        assert_eq!(
            q.value(Field::integer(-1, 300), 302),
            Err(DecodeError::OutOfRange)
        );
    }

    /// The worst error of `q`'s coarsening of `field` over its integers, as (the worst over the integers whose
    /// nearest multiple lies inside the range, or every integer when the field wraps, measured around the turn; the
    /// worst at the range's own ends).
    #[cfg(feature = "bites")]
    fn worst(q: Quant, field: Field, step: i64) -> (i64, i64) {
        let span = i64::from(field.max()) - i64::from(field.min()) + 1;
        let (mut inside, mut ends) = (0i64, 0i64);
        for k in field.min()..=field.max() {
            let c = q.coarse(field, k);
            assert!(
                (field.min()..=field.max()).contains(&c),
                "{field:?}: {k} → {c}"
            );
            assert_eq!(
                c.rem_euclid(step as i32),
                0,
                "{field:?}: {k} → {c}, not a multiple of {step}"
            );
            // It decodes as itself: the coarse integer is what the client gets.
            assert_eq!(round_trip(q, field, k), c);
            let mut e = (i64::from(c) - i64::from(k)).abs();
            if field.wraps() {
                e = e.min(span - e);
            }
            let nearest = nearest_multiple(i64::from(k), step);
            let nearest_outside = !field.wraps()
                && (nearest > i64::from(field.max()) || nearest < i64::from(field.min()));
            if nearest_outside {
                ends = ends.max(e);
            } else {
                inside = inside.max(e);
            }
        }
        (inside, ends)
    }

    #[cfg(feature = "bites")]
    #[test]
    fn a4_pm_0001_57s_three_bites_coarsen_their_fields_alone_by_half_the_coarse_step() {
        for (id, fields, step) in [
            ("PM-0001-57/pos-4cm", &[POSITION_XY, POSITION_Z][..], 4i64),
            ("PM-0001-57/yaw-12bit", &[YAW][..], 16),
            ("PM-0001-57/pitch-8bit", &[PITCH][..], 4),
        ] {
            let q = Quant::with_bite(id).expect("npt-proto plants it");
            for &f in fields {
                let (inside, ends) = worst(q, f, step);
                // Within half the coarse step, and above half the step less one: the defect is really there.
                assert!(
                    inside <= step / 2 && inside > step / 2 - 1,
                    "{id} on {f:?}: worst {inside}"
                );
                // At a non-wrapping field's ends the nearest multiple inside the range is less than one step off.
                assert!(ends < step, "{id} on {f:?}: at the ends {ends}");
                // The range ends themselves.
                for k in [f.min(), f.max()] {
                    let c = q.coarse(f, k);
                    assert!((f.min()..=f.max()).contains(&c), "{id}: {k} → {c}");
                }
            }
            // Every other field is untouched, VELOCITY included (POSITION_XY's range, another kind).
            for (name, f) in FIELDS {
                if fields.contains(&f) {
                    continue;
                }
                for k in [f.min(), -3, -2, -1, 0, 1, 2, 3, 5, 7, f.max()] {
                    if (f.min()..=f.max()).contains(&k) {
                        assert_eq!(q.coarse(f, k), k, "{id} touched {name} at {k}");
                    }
                }
            }
        }
        // The pinned ends: 32,767 to 32,764 (the nearest multiple of 4 inside), 4,095 to 4,092, pitch ±511 to ±508,
        // yaw 32,767 around the turn to -32,768 (one step of 16 off, wrapped: an error of 1).
        let pos = Quant::with_bite("PM-0001-57/pos-4cm").expect("planted");
        assert_eq!(pos.coarse(POSITION_XY, 32_767), 32_764);
        assert_eq!(pos.coarse(POSITION_XY, -32_768), -32_768);
        assert_eq!(pos.coarse(POSITION_XY, 2), 4);
        assert_eq!(pos.coarse(POSITION_XY, -2), -4);
        assert_eq!(pos.coarse(POSITION_Z, 4_095), 4_092);
        let yaw = Quant::with_bite("PM-0001-57/yaw-12bit").expect("planted");
        assert_eq!(yaw.coarse(YAW, 32_767), -32_768);
        assert_eq!(yaw.coarse(YAW, 8), 16);
        let pitch = Quant::with_bite("PM-0001-57/pitch-8bit").expect("planted");
        assert_eq!(pitch.coarse(PITCH, 511), 508);
        assert_eq!(pitch.coarse(PITCH, -511), -508);
    }

    #[cfg(feature = "bites")]
    #[test]
    fn npt_proto_plants_its_seven_ids_and_no_other_sites_id() {
        for id in BITES {
            assert!(Quant::with_bite(id).is_some(), "{id}");
            assert_ne!(Quant::with_bite(id), Some(Quant::default()), "{id}");
        }
        for other in [
            "PM-0001-17/client-rounding",
            "PM-0001-41/server-quant-differs",
            "PM-0001-58/lt-cmd-seq",
            "PM-0001-57/no-such-bite",
        ] {
            assert!(Quant::with_bite(other).is_none(), "{other}");
        }
    }
}
