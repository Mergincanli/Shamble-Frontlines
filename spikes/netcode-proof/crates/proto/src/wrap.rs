//! `Wrap`: the wrap-aware compares, one per counter width, each called with its counter's name (u4-build-plan.md §4.4;
//! PM-0001-58). Every counter that wraps in the proof test goes through one of these: press counters (8 bits);
//! command sequences, the 16-bit tick and event numbers on the wire and the acknowledgement (16 bits); ticks
//! (32 bits). The counters themselves, each with its injectable start value, are their owners' (npt-room's and
//! npt-client's, U4e); this type is the one compare they share.
//!
//! **The rule:** `b` comes after `a` when `b - a`, wrapped to the width and read as a signed number, is above 0: up to
//! half the counter's range ahead. At exactly half (`b - a` = 2^(width - 1)) neither comes after the other.
//!
//! **The planted defect** (bites builds only): `Wrap::plain_lt(counter)` compares that one named counter with a plain
//! `<` and widens it without carrying across the wrap; every other counter keeps the rule. It names no bite: PM-0001-58's
//! ids live at their sites, the room's and the client's compares (u4-build-plan.md §4.14, as `npt_sim::grid`'s one
//! defect is planted by ids in npt-client and npt-sim), which map each id to its counter's name.

/// The wrap-aware compares (the module text). In a measured build every `Wrap` is the rule, `Wrap::default()`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Wrap {
    /// The one counter compared with a plain `<` (bites builds only).
    #[cfg(feature = "bites")]
    plain: Option<&'static str>,
}

impl Wrap {
    /// The `Wrap` whose compares of `counter` use a plain `<` (bites builds only): the module text's defect.
    #[cfg(feature = "bites")]
    pub const fn plain_lt(counter: &'static str) -> Wrap {
        Wrap {
            plain: Some(counter),
        }
    }

    /// Whether this `Wrap` compares `counter` with a plain `<`.
    fn plain(self, counter: &str) -> bool {
        #[cfg(feature = "bites")]
        if self.plain == Some(counter) {
            return true;
        }
        let _ = counter;
        false
    }

    /// Whether `b` comes after `a` on the 8-bit counter `counter` (press counters).
    pub fn after8(self, counter: &str, a: u8, b: u8) -> bool {
        if self.plain(counter) {
            return a < b;
        }
        b.wrapping_sub(a) as i8 > 0
    }

    /// Whether `b` comes after `a` on the 16-bit counter `counter` (command sequences, wire tick and event numbers,
    /// acknowledgements).
    pub fn after16(self, counter: &str, a: u16, b: u16) -> bool {
        if self.plain(counter) {
            return a < b;
        }
        b.wrapping_sub(a) as i16 > 0
    }

    /// Whether `b` comes after `a` on the 32-bit counter `counter` (ticks).
    pub fn after32(self, counter: &str, a: u32, b: u32) -> bool {
        if self.plain(counter) {
            return a < b;
        }
        b.wrapping_sub(a) as i32 > 0
    }

    /// The 32-bit value of the 16-bit counter `counter` whose low 16 bits are `low`, the one nearest `near` (a value
    /// already known in full: the newest tick, the newest event number): from 32,768 before `near` to 32,767 after
    /// it. How a 16-bit acknowledgement or event number is rebuilt.
    pub fn widen16(self, counter: &str, near: u32, low: u16) -> u32 {
        if self.plain(counter) {
            return (near & !0xffff) | u32::from(low);
        }
        let step = low.wrapping_sub(near as u16) as i16;
        near.wrapping_add_signed(i32::from(step))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// a5: each width across its wrap, from a start value a few steps below it (the injectable start of PM-0001-58's
    /// counters), and every counter name alike.
    #[test]
    fn a5_each_width_compares_and_widens_across_its_wrap_for_every_counter() {
        let w = Wrap::default();
        for counter in [
            "cmd_seq",
            "press",
            "tick_offset",
            "event_seq",
            "ack",
            "tick",
        ] {
            // 8 bits: 253, 254, 255, 0, 1, 2 each after the one before; 127 ahead is after, 128 is neither.
            let mut a = 253u8;
            for _ in 0..6 {
                let b = a.wrapping_add(1);
                assert!(
                    w.after8(counter, a, b) && !w.after8(counter, b, a),
                    "{counter}: {a} {b}"
                );
                a = b;
            }
            assert!(w.after8(counter, 200, 200u8.wrapping_add(127)));
            assert!(
                !w.after8(counter, 200, 200u8.wrapping_add(128))
                    && !w.after8(counter, 200u8.wrapping_add(128), 200)
            );
            assert!(!w.after8(counter, 7, 7));
            // 16 bits.
            let mut a = 65_533u16;
            for _ in 0..6 {
                let b = a.wrapping_add(1);
                assert!(
                    w.after16(counter, a, b) && !w.after16(counter, b, a),
                    "{counter}: {a} {b}"
                );
                a = b;
            }
            assert!(w.after16(counter, 65_000, 65_000u16.wrapping_add(32_767)));
            assert!(!w.after16(counter, 65_000, 65_000u16.wrapping_add(32_768)));
            // 32 bits.
            let mut a = u32::MAX - 2;
            for _ in 0..6 {
                let b = a.wrapping_add(1);
                assert!(
                    w.after32(counter, a, b) && !w.after32(counter, b, a),
                    "{counter}: {a} {b}"
                );
                a = b;
            }
            assert!(w.after32(counter, 5, 5u32.wrapping_add(i32::MAX as u32)));
            assert!(!w.after32(counter, 5, 5u32.wrapping_add(1 << 31)));
            // Widening: across the 16-bit wrap in both directions, and across the 32-bit one.
            for (near, low, want) in [
                (65_534u32, 1u16, 65_537u32),
                (65_537, 65_534, 65_534),
                (65_536, 0, 65_536),
                (65_536 + 40_000, 7_000, 131_072 + 7_000),
                (u32::MAX - 1, 2, 2),
                (3, 65_530, u32::MAX - 5),
                (100, 100 + 32_767, 100 + 32_767),
                (40_000, 40_000 - 32_768, 40_000 - 32_768),
            ] {
                assert_eq!(
                    w.widen16(counter, near, low),
                    want,
                    "{counter}: {near} {low}"
                );
            }
            for near in [0u32, 1, 30_000, 65_535, 65_536, 1 << 31, u32::MAX] {
                for d in [-32_768i32, -1, 0, 1, 32_767] {
                    let v = near.wrapping_add_signed(d);
                    assert_eq!(
                        w.widen16(counter, near, v as u16),
                        v,
                        "{counter}: {near} {d}"
                    );
                }
            }
        }
    }

    /// a5's bite: the plain `<` for one named counter fails a5 for that counter, across its wrap, and for no other.
    #[cfg(feature = "bites")]
    #[test]
    fn a5_the_plain_lt_defect_breaks_its_one_named_counter_across_the_wrap() {
        let bitten = Wrap::plain_lt("cmd_seq");
        let rule = Wrap::default();
        assert!(
            !bitten.after16("cmd_seq", 65_535, 0),
            "the defect: 0 is not after 65,535"
        );
        assert!(
            bitten.after16("event_seq", 65_535, 0),
            "another counter keeps the rule"
        );
        assert!(!bitten.after8("cmd_seq", 255, 0) && !bitten.after32("cmd_seq", u32::MAX, 0));
        assert_eq!(
            bitten.widen16("cmd_seq", 65_534, 1),
            1,
            "no carry across the wrap"
        );
        assert_eq!(bitten.widen16("ack", 65_534, 1), 65_537);
        // Away from the wrap the defect agrees with the rule, so it shows only where the counter wraps.
        assert_eq!(
            bitten.after16("cmd_seq", 10, 11),
            rule.after16("cmd_seq", 10, 11)
        );
        assert_eq!(
            bitten.widen16("cmd_seq", 70_000, 4_470),
            rule.widen16("cmd_seq", 70_000, 4_470)
        );
    }
}
