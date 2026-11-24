//! The press pacer: from the page's running press counts to the counters a command carries (ADR 0001 part 5). The
//! room takes at most one press per button per tick, so a button's counter moves by at most one per command and
//! the rest waits its turn: a second tap inside one tick is applied one tick later and never dropped (Mustafa's
//! answer PRESS-PACE (a), 2026-11-23). A tap shorter than a frame is not lost, because the page counted it, and a
//! command that is sent twice does not double it, because it carries counters and not edges.
#![cfg_attr(
    not(test),
    deny(clippy::indexing_slicing, clippy::arithmetic_side_effects)
)]

use protocol::wrap::press_edges;
use sim::BUTTONS;

/// The most presses of one button that wait their turn; a person never reaches it.
pub const MAX_BACKLOG: u16 = 255;

#[derive(Default)]
pub struct PressPacer {
    /// The page's counts as of the last call.
    seen: [u8; BUTTONS],
    /// Presses the page counted that no command has carried yet.
    waiting: [u16; BUTTONS],
    /// The counters of the last command.
    counters: [u8; BUTTONS],
    flooded: u32,
    backwards: u32,
}

impl PressPacer {
    /// The counters of the next command, given the page's counts now: each button with a press waiting moves by
    /// one. A count that went backwards is ignored and counted (it is 128 or more presses ahead, which no honest
    /// page sends between two ticks); presses above `MAX_BACKLOG` waiting are dropped and counted.
    pub fn next(&mut self, page: &[u8; BUTTONS]) -> [u8; BUTTONS] {
        let buttons = self.seen.iter_mut().zip(&mut self.waiting);
        for ((seen, waiting), (counter, page)) in buttons.zip(self.counters.iter_mut().zip(page)) {
            let new = press_edges(*seen, *page);
            if new >= 0x80 {
                self.backwards = self.backwards.wrapping_add(1);
            } else {
                *seen = *page;
                let total = waiting.saturating_add(u16::from(new));
                let over = total.saturating_sub(MAX_BACKLOG);
                self.flooded = self.flooded.wrapping_add(u32::from(over));
                *waiting = total.min(MAX_BACKLOG);
            }
            if let Some(left) = waiting.checked_sub(1) {
                *waiting = left;
                *counter = counter.wrapping_add(1);
            }
        }
        self.counters
    }

    /// How many presses wait their turn, over all buttons.
    #[must_use]
    pub fn waiting(&self) -> u32 {
        self.waiting.iter().map(|waiting| u32::from(*waiting)).sum()
    }

    /// Presses dropped above `MAX_BACKLOG`, and counts that went backwards; each wraps.
    #[must_use]
    pub const fn refused(&self) -> (u32, u32) {
        (self.flooded, self.backwards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The page's counts with button 0 at `first` and button 15 at `last`.
    fn page(first: u8, last: u8) -> [u8; BUTTONS] {
        let mut counts = [0; BUTTONS];
        (counts[0], counts[BUTTONS - 1]) = (first, last);
        counts
    }

    #[test]
    fn each_button_moves_by_at_most_one_per_command_and_the_rest_waits() {
        let mut pacer = PressPacer::default();
        assert_eq!(pacer.next(&page(0, 0)), page(0, 0));
        // Three taps of button 0 and one of button 15 inside one tick: one each now, the other two in turn.
        assert_eq!(pacer.next(&page(3, 1)), page(1, 1));
        assert_eq!(pacer.waiting(), 2);
        assert_eq!(pacer.next(&page(3, 1)), page(2, 1));
        // A new tap joins the queue behind the ones waiting.
        assert_eq!(pacer.next(&page(4, 1)), page(3, 1));
        assert_eq!(pacer.next(&page(4, 1)), page(4, 1));
        assert_eq!(pacer.next(&page(4, 1)), page(4, 1));
        assert_eq!((pacer.waiting(), pacer.refused()), (0, (0, 0)));
    }

    #[test]
    fn the_counters_wrap_at_256_with_the_page_s() {
        let mut pacer = PressPacer::default();
        let mut made = 0u8;
        for tap in 1..=1000u32 {
            made = made.wrapping_add(1);
            let sent = pacer.next(&page(made, 0));
            assert_eq!(u32::from(sent[0]), tap % 256);
        }
        assert_eq!((pacer.waiting(), pacer.refused()), (0, (0, 0)));
    }

    #[test]
    fn a_count_that_goes_back_is_ignored_and_a_flood_is_cut() {
        let mut pacer = PressPacer::default();
        pacer.next(&page(10, 0));
        assert_eq!(pacer.waiting(), 9);
        // An older post's count: nothing is added, nothing is taken back, and the queue goes on.
        assert_eq!(pacer.next(&page(9, 0)), page(2, 0));
        assert_eq!((pacer.waiting(), pacer.refused()), (8, (0, 1)));
        // 127 more per call is the most that counts; the queue stops at 255 and the rest is dropped and counted.
        let mut made = 10u8;
        for _ in 0..3 {
            made = made.wrapping_add(127);
            pacer.next(&page(made, 0));
        }
        assert_eq!(pacer.waiting(), 254);
        assert_eq!(pacer.refused(), (8 + 3 * 127 - 255 - 2, 1));
    }
}
