//! npt-lab: ADR 0002's devtools analogue for the netcode proof test (plan §4.7; engine EA-3, EA-11).
//!
//! This library holds the native-against-Wasm hash comparer ([`HashCompare`], U4d2) and its [`TickHash`]. The npt-lab
//! binary holds `SimClock` (integer ns) and the L1 runner, plan §4.7's `L1Scenario`, built as `scenario::run` with its
//! `Server` (one server and 48 headless clients on one simulated clock, in a fixed order per instant: due packets, the
//! server's I/O step, the server tick if due, due client frames, due client ticks), and the x86-64 re-decode and replay
//! tool for the Arm rule (`replay::ReplayX86`). Nothing depends on this crate. Built in units U2 and U4.

/// One tick's entry in a leg's per-tick hash list: the recording both legs replay (its manifest name), the tick and the
/// state hash.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TickHash {
    /// The recording both legs replay (the same name on both legs), as its manifest names it.
    pub recording: String,
    /// The tick the hash is of.
    pub tick: u32,
    /// The state hash after that tick.
    pub hash: u64,
}

/// Two legs' per-tick hash lists compared, entry by entry in order (u4-build-plan.md §5 U4d2: the native-against-Wasm
/// check, and U4r's -30 pre-check, which feeds it two legs' per-tick server hashes; plan.md §4.7). It reads no file and
/// knows no recording: the lists are its whole input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HashCompare {
    /// Both lists hold the same entries in the same order, this many (at least one).
    Equal { entries: usize },
    /// Both lists are empty: nothing was compared, which is never "equal".
    Empty,
    /// The lists differ. `index` is the first entry where they do: there the two entries differ in their recording,
    /// tick or hash, or one list has ended (`None`). `compared` is the longer list's length, and `mismatching` how
    /// many of those positions differ (an entry with no partner counts).
    Mismatch {
        index: usize,
        ours: Option<TickHash>,
        theirs: Option<TickHash>,
        compared: usize,
        mismatching: usize,
    },
}

impl HashCompare {
    /// Compares `ours` with `theirs`.
    pub fn of(ours: &[TickHash], theirs: &[TickHash]) -> HashCompare {
        let compared = ours.len().max(theirs.len());
        if compared == 0 {
            return HashCompare::Empty;
        }
        let differs = |i: usize| ours.get(i) != theirs.get(i);
        let mismatching = (0..compared).filter(|&i| differs(i)).count();
        match (0..compared).find(|&i| differs(i)) {
            None => HashCompare::Equal { entries: compared },
            Some(index) => HashCompare::Mismatch {
                index,
                ours: ours.get(index).cloned(),
                theirs: theirs.get(index).cloned(),
                compared,
                mismatching,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    //! h4, `HashCompare` (u4d2-brief.md 2.6).

    use super::*;

    fn list(n: u32) -> Vec<TickHash> {
        (0..n)
            .map(|k| TickHash {
                recording: if k < 20 { "SS-s1" } else { "ES-s2" }.to_string(),
                tick: 1_000 + k,
                hash: 0x9e37_79b9_7f4a_7c15_u64.wrapping_mul(u64::from(k) + 1),
            })
            .collect()
    }

    #[test]
    fn h4_equal_lists_are_equal_over_every_entry() {
        assert_eq!(
            HashCompare::of(&list(40), &list(40)),
            HashCompare::Equal { entries: 40 }
        );
    }

    #[test]
    fn h4_the_first_mismatch_is_found_above_index_0_with_both_entries_and_the_count() {
        let mut theirs = list(40);
        theirs[17].hash ^= 1;
        theirs[30].hash ^= 1;
        let ours = list(40);
        assert_eq!(
            HashCompare::of(&ours, &theirs),
            HashCompare::Mismatch {
                index: 17,
                ours: Some(ours[17].clone()),
                theirs: Some(theirs[17].clone()),
                compared: 40,
                mismatching: 2,
            }
        );
    }

    #[test]
    fn h4_lists_of_different_lengths_mismatch_where_the_shorter_ends() {
        let ours = list(40);
        let theirs = list(25);
        assert_eq!(
            HashCompare::of(&ours, &theirs),
            HashCompare::Mismatch {
                index: 25,
                ours: Some(ours[25].clone()),
                theirs: None,
                compared: 40,
                mismatching: 15,
            }
        );
        assert!(matches!(
            HashCompare::of(&[], &theirs),
            HashCompare::Mismatch {
                index: 0,
                ours: None,
                compared: 25,
                mismatching: 25,
                ..
            }
        ));
    }

    #[test]
    fn h4_a_tick_or_recording_out_of_step_is_a_mismatch_with_equal_hashes() {
        let ours = list(10);
        let mut tick = list(10);
        tick[4].tick += 1;
        assert!(matches!(
            HashCompare::of(&ours, &tick),
            HashCompare::Mismatch {
                index: 4,
                mismatching: 1,
                ..
            }
        ));
        let mut name = list(10);
        name[6].recording = "TC-s3".to_string();
        assert!(matches!(
            HashCompare::of(&ours, &name),
            HashCompare::Mismatch {
                index: 6,
                mismatching: 1,
                ..
            }
        ));
    }

    #[test]
    fn h4_0_entries_compared_is_its_own_result_never_equal() {
        assert_eq!(HashCompare::of(&[], &[]), HashCompare::Empty);
        assert_ne!(HashCompare::of(&[], &[]), HashCompare::Equal { entries: 0 });
    }
}
