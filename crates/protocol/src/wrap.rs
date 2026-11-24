//! Compares for counters that came off the wire (ADR 0001 part 5). A decoded tick, sequence or press counter is
//! never compared with `<` or subtracted with `-`: a sequence and a press counter wrap by design, and overflow
//! checks are on in every build, so a plain minus on a value a peer chose would be a crash a peer can cause.

/// Whether sequence `a` is newer than `b`: ahead of it by 1 to 32,767 steps around the 16-bit circle. Exactly half
/// way round (32,768) is neither newer nor older, and counts as not newer.
#[must_use]
pub const fn seq_newer(a: u16, b: u16) -> bool {
    seq_ahead(b, a).is_some()
}

/// How many steps `to` lies ahead of `from`, 1 to 32,767; none when it is the same, behind, or half way round.
#[must_use]
pub const fn seq_ahead(from: u16, to: u16) -> Option<u16> {
    let steps = to.wrapping_sub(from);
    if steps != 0 && steps < 0x8000 {
        Some(steps)
    } else {
        None
    }
}

/// How many ticks `later` lies after `earlier`; none when it lies before. A tick has 32 bits and never wraps in a
/// match, so two ticks out of order are an answer of their own, never a huge number.
#[must_use]
pub const fn ticks_after(earlier: u32, later: u32) -> Option<u32> {
    later.checked_sub(earlier)
}

/// How many presses a button's counter moved from `old` to `new`, around the 8-bit circle.
#[must_use]
pub const fn press_edges(old: u8, new: u8) -> u8 {
    new.wrapping_sub(old)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_sequence_is_newer_up_to_half_way_round_and_never_at_half_way() {
        assert!(seq_newer(1, 0));
        assert!(!seq_newer(0, 1));
        assert!(!seq_newer(7, 7));
        // Across the wrap.
        assert!(seq_newer(0, 65_535));
        assert!(seq_newer(2, 65_533));
        assert!(!seq_newer(65_535, 0));
        // The edges of the window: 32,767 ahead is newer, 32,768 is neither way, 32,769 is older.
        assert!(seq_newer(0x7FFF, 0));
        assert!(!seq_newer(0x8000, 0));
        assert!(!seq_newer(0, 0x8000));
        assert!(!seq_newer(0x8001, 0));
        assert!(seq_newer(0, 0x8001));
        assert_eq!(seq_ahead(0, 0x7FFF), Some(0x7FFF));
        assert_eq!(seq_ahead(0, 0x8000), None);
        assert_eq!(seq_ahead(65_533, 2), Some(5));
        assert_eq!(seq_ahead(2, 65_533), None);
        assert_eq!(seq_ahead(9, 9), None);
    }

    /// Every pair on a coarse grid and around every start: exactly one of "a newer", "b newer", "same" or "half way
    /// round" holds, and the distance is the wrapped difference.
    #[test]
    fn newer_is_one_sided_for_every_pair_tried() {
        for a in (0..=u16::MAX).step_by(251) {
            for delta in [0, 1, 2, 0x7FFE, 0x7FFF, 0x8000, 0x8001, 0xFFFE, 0xFFFF] {
                let b = a.wrapping_add(delta);
                let (forward, back) = (seq_newer(b, a), seq_newer(a, b));
                assert!(!(forward && back), "{a} and {b} are each newer");
                assert_eq!(forward || back, delta != 0 && delta != 0x8000, "{a} {b}");
                assert_eq!(seq_ahead(a, b), forward.then_some(delta), "{a} {b}");
            }
        }
    }

    #[test]
    fn ticks_out_of_order_are_none_and_never_a_huge_number() {
        assert_eq!(ticks_after(10, 10), Some(0));
        assert_eq!(ticks_after(10, 13), Some(3));
        assert_eq!(ticks_after(13, 10), None);
        assert_eq!(ticks_after(0, u32::MAX), Some(u32::MAX));
        assert_eq!(ticks_after(u32::MAX, 0), None);
    }

    #[test]
    fn press_counters_wrap_at_256() {
        assert_eq!(press_edges(3, 5), 2);
        assert_eq!(press_edges(5, 5), 0);
        assert_eq!(press_edges(254, 1), 3);
        // A counter that went back by one reads as 255 presses: the caller bounds what it accepts.
        assert_eq!(press_edges(5, 4), 255);
    }
}
