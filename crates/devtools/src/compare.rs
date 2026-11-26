//! The comparer (ADR 0002 part 8; PM-0004-21): where two runs of one recording first differ, by tick, and then
//! which fields of the state differ there, by name.

use std::fmt;

use crate::recording::TickHashes;

/// Which of a line's two hashes is compared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Column {
    /// The shared state's: every target gives it.
    Shared,
    /// The whole state's: native runs only.
    Whole,
}

impl Column {
    const fn of(self, line: &TickHashes) -> u64 {
        match self {
            Self::Shared => line.shared,
            Self::Whole => line.whole,
        }
    }
}

/// The first step at which two runs differ. `step` counts from 1; a side that has no such step is `None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Difference {
    pub step: usize,
    pub expected: Option<TickHashes>,
    pub got: Option<TickHashes>,
}

impl fmt::Display for Difference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = |line: Option<TickHashes>| match line {
            Some(line) => format!(
                "tick {} shared {:016x} whole {:016x}",
                line.tick, line.shared, line.whole
            ),
            None => "no such step".to_owned(),
        };
        write!(
            f,
            "step {}: expected {}; got {}",
            self.step,
            side(self.expected),
            side(self.got)
        )
    }
}

/// The first step at which `got` differs from `expected` in its tick or in the hash of `column`, or at which one of
/// the two ends before the other; `None` when they agree step for step.
#[must_use]
pub fn first_difference(
    expected: &[TickHashes],
    got: &[TickHashes],
    column: Column,
) -> Option<Difference> {
    let same = |a: &TickHashes, b: &TickHashes| a.tick == b.tick && column.of(a) == column.of(b);
    let steps = expected.len().max(got.len());
    (0..steps)
        .find(|&index| match (expected.get(index), got.get(index)) {
            (Some(a), Some(b)) => !same(a, b),
            _ => true,
        })
        .map(|index| Difference {
            step: index + 1,
            expected: expected.get(index).copied(),
            got: got.get(index).copied(),
        })
}

/// The lines of two state dumps (`replay::state_lines`) that differ, each as `name: expected ..., got ...`. Both
/// dumps come from the one visitor of the state, so their lines are the same fields in the same order; a dump with
/// more lines than the other shows as lines the other lacks.
#[must_use]
pub fn field_diff(expected: &str, got: &str) -> Vec<String> {
    let (mut a, mut b) = (expected.lines(), got.lines());
    let mut differing = Vec::new();
    loop {
        let (left, right) = (a.next(), b.next());
        if left.is_none() && right.is_none() {
            return differing;
        }
        if left != right {
            let part = |line: Option<&str>| match line.map(|line| line.split_once(" = ")) {
                Some(Some((name, value))) => (name.to_owned(), value.to_owned()),
                Some(None) => (line.unwrap_or_default().to_owned(), String::new()),
                None => (String::new(), "(no such line)".to_owned()),
            };
            let ((name, was), (other, is)) = (part(left), part(right));
            let name = if name.is_empty() { other } else { name };
            differing.push(format!("{name}: expected {was}, got {is}"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn line(tick: u32, shared: u64, whole: u64) -> TickHashes {
        TickHashes {
            tick,
            shared,
            whole,
        }
    }

    #[test]
    fn the_first_difference_is_the_first_step_whose_tick_or_hash_differs() {
        let expected = [line(1, 10, 100), line(2, 20, 200), line(3, 30, 300)];
        let same = expected;
        assert_eq!(first_difference(&expected, &same, Column::Shared), None);
        assert_eq!(first_difference(&expected, &same, Column::Whole), None);
        assert_eq!(first_difference(&[], &[], Column::Shared), None);
        // A whole hash that differs is seen in its own column only.
        let whole = [line(1, 10, 100), line(2, 20, 201), line(3, 31, 300)];
        let at = |step: usize, got: &[TickHashes]| {
            Some(Difference {
                step,
                expected: expected.get(step - 1).copied(),
                got: got.get(step - 1).copied(),
            })
        };
        assert_eq!(
            first_difference(&expected, &whole, Column::Whole),
            at(2, &whole)
        );
        assert_eq!(
            first_difference(&expected, &whole, Column::Shared),
            at(3, &whole)
        );
        // A tick that differs counts in either column.
        let tick = [line(1, 10, 100), line(3, 20, 200), line(3, 30, 300)];
        assert_eq!(
            first_difference(&expected, &tick, Column::Shared),
            at(2, &tick)
        );
        // A run that ends early, and one that goes on.
        assert_eq!(
            first_difference(&expected, &same[..2], Column::Shared),
            at(3, &same[..2])
        );
        let longer = [&expected[..], &[line(4, 40, 400)]].concat();
        let more = first_difference(&expected, &longer, Column::Shared).expect("one more step");
        assert_eq!(
            (more.step, more.expected, more.got),
            (4, None, Some(line(4, 40, 400)))
        );
        assert_eq!(
            more.to_string(),
            "step 4: expected no such step; got tick 4 shared 0000000000000028 whole 0000000000000190"
        );
    }

    #[test]
    fn a_field_diff_names_each_field_that_differs() {
        let a = "tick = 600\nplayer.buttons = 1\nplayer.presses = [0, 1]\n";
        assert_eq!(field_diff(a, a), Vec::<String>::new());
        let b = "tick = 600\nplayer.buttons = 3\nplayer.presses = [0, 2]\n";
        assert_eq!(
            field_diff(a, b),
            [
                "player.buttons: expected 1, got 3",
                "player.presses: expected [0, 1], got [0, 2]"
            ]
        );
        // A field one side lacks.
        let longer = format!("{a}soldier.0.x = 7\n");
        assert_eq!(
            field_diff(a, &longer),
            ["soldier.0.x: expected (no such line), got 7"]
        );
        assert_eq!(
            field_diff(&longer, a),
            ["soldier.0.x: expected 7, got (no such line)"]
        );
    }
}
