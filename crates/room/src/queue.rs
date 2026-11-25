//! The room's command queue for one player (ADR 0001 part 5): the commands that arrived and have not run yet, kept
//! in the order of their sequence numbers in a fixed array. A command that arrives twice is kept once, one that
//! arrives out of order finds its place, and one that never arrives is skipped as soon as a newer one is due: the
//! newer command's press counters carry what the missing one held.

use protocol::wrap::{seq_newer, ticks_after};
use sim::PlayerCommand;

/// How many commands wait at most: twice the most one `Input` carries.
pub const QUEUE_LEN: usize = 16;

/// A command as the queue holds it: what the simulation reads of it, its sequence and the tick it is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Queued {
    pub sequence: u16,
    pub tick: u32,
    pub command: PlayerCommand,
}

/// What became of a command offered to the queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Inserted {
    Queued,
    /// It already waits.
    Duplicate,
    /// It is not newer than the last command that ran: a late repeat, or a sequence half the circle or more
    /// ahead of it; before any command ran, one behind the first command offered.
    Stale,
    /// The queue holds as many as it can.
    Full,
}

/// One player's waiting commands, the oldest sequence first.
pub struct CommandQueue {
    /// The first `len` wait, the oldest sequence first.
    waiting: [Queued; QUEUE_LEN],
    len: usize,
    /// The sequence of the last command that ran; before the first, one below the first command offered.
    newest_run: Option<u16>,
    /// The sequence of the command taken out last, for `ran`.
    taken: Option<u16>,
}

impl Default for CommandQueue {
    fn default() -> Self {
        let empty = Queued {
            sequence: 0,
            tick: 0,
            command: PlayerCommand::NONE,
        };
        Self {
            waiting: [empty; QUEUE_LEN],
            len: 0,
            newest_run: None,
            taken: None,
        }
    }
}

impl CommandQueue {
    /// How many commands wait.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Puts a command in its place by sequence. Before any command ran, the first one offered opens the window:
    /// whatever lies behind it is stale. So every waiting command lies within half the sequence circle of the
    /// last one that ran, and no spread of sequences can turn their order round.
    pub fn insert(&mut self, new: Queued) -> Inserted {
        let newest_run = *self.newest_run.get_or_insert(new.sequence.wrapping_sub(1));
        if !seq_newer(new.sequence, newest_run) {
            return Inserted::Stale;
        }
        let waiting = self.waiting.get(..self.len).unwrap_or(&[]);
        // Its place: before the first waiting command it is not newer than.
        let not_newer = |waits: &Queued| !seq_newer(new.sequence, waits.sequence);
        let at = waiting.iter().position(not_newer).unwrap_or(self.len);
        if waiting
            .get(at)
            .is_some_and(|waits| waits.sequence == new.sequence)
        {
            return Inserted::Duplicate;
        }
        // The commands from `at` on and the free place behind them: the new one goes into the free place and is
        // turned to the front, so each of the others moves one place back. A full queue has no free place.
        let Some(moved) = self.waiting.get_mut(at..=self.len) else {
            return Inserted::Full;
        };
        if let Some(free) = moved.last_mut() {
            *free = new;
        }
        moved.rotate_right(1);
        self.len = self.len.saturating_add(1);
        Inserted::Queued
    }

    /// Takes out the oldest waiting command when its tick is `running` or earlier; one for a later tick waits.
    /// The window does not move: the room says which command ran (`ran`).
    pub fn take_due(&mut self, running: u32) -> Option<Queued> {
        let waiting = self.waiting.get_mut(..self.len)?;
        let first = *waiting.first()?;
        ticks_after(first.tick, running)?;
        waiting.rotate_left(1);
        self.len = self.len.saturating_sub(1);
        self.taken = Some(first.sequence);
        Some(first)
    }

    /// The command taken out last ran: what is not newer is stale from now on. A command that was taken out and
    /// refused does not move the window, so a refused command far ahead cannot shut out the honest ones behind
    /// it, and every command that runs is newer than the one that ran before it. The queue knows which command
    /// it took out last, so no caller can name another. Call it before anything else is offered: a command put
    /// in between could lie behind the new base (checked in debug builds).
    pub fn ran(&mut self) {
        if let Some(taken) = self.taken {
            let mut waiting = self.waiting.iter().take(self.len);
            debug_assert!(
                waiting.all(|waits| seq_newer(waits.sequence, taken)),
                "a command was offered between its taking out and `ran`"
            );
            self.newest_run = Some(taken);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn queued(sequence: u16, tick: u32) -> Queued {
        let command = PlayerCommand {
            buttons: sequence,
            presses: [0; sim::BUTTONS],
        };
        Queued {
            sequence,
            tick,
            command,
        }
    }

    /// Takes out everything that waits, each command running as the room would run it.
    fn drain(queue: &mut CommandQueue) -> Vec<u16> {
        let mut sequences = Vec::new();
        while let Some(taken) = queue.take_due(u32::MAX) {
            queue.ran();
            sequences.push(taken.sequence);
        }
        sequences
    }

    #[test]
    fn commands_come_out_in_sequence_whatever_order_they_came_in() {
        let mut queue = CommandQueue::default();
        assert!(queue.is_empty());
        for sequence in [3, 5, 4, 7, 6] {
            assert_eq!(queue.insert(queued(sequence, 1)), Inserted::Queued);
        }
        assert_eq!(queue.insert(queued(4, 1)), Inserted::Duplicate);
        assert_eq!(queue.insert(queued(7, 1)), Inserted::Duplicate);
        assert_eq!(queue.len(), 5);
        // The whole command comes out, not only its sequence.
        assert_eq!(queue.take_due(1), Some(queued(3, 1)));
        assert_eq!(drain(&mut queue), [4, 5, 6, 7]);
        assert_eq!(queue.take_due(u32::MAX), None);
    }

    #[test]
    fn what_was_taken_out_or_lies_before_it_is_stale() {
        let mut queue = CommandQueue::default();
        queue.insert(queued(10, 1));
        assert_eq!(drain(&mut queue), [10]);
        for sequence in [10, 9, 3] {
            assert_eq!(queue.insert(queued(sequence, 1)), Inserted::Stale);
        }
        // A gap is no reason to wait: 13 comes out while 11 and 12 never arrived, and they are stale afterwards.
        assert_eq!(queue.insert(queued(13, 1)), Inserted::Queued);
        assert_eq!(drain(&mut queue), [13]);
        assert_eq!(queue.insert(queued(12, 1)), Inserted::Stale);
    }

    #[test]
    fn the_order_holds_across_the_sequence_wrap() {
        let mut queue = CommandQueue::default();
        for sequence in [65_534, 1, 0, 65_535] {
            assert_eq!(queue.insert(queued(sequence, 1)), Inserted::Queued);
        }
        assert_eq!(drain(&mut queue), [65_534, 65_535, 0, 1]);
        assert_eq!(queue.insert(queued(65_535, 1)), Inserted::Stale);
        assert_eq!(queue.insert(queued(2, 1)), Inserted::Queued);
    }

    /// A client that spreads its sequences round the circle before the first command runs: the first one offered
    /// opens the window, what lies behind it or half the circle away is stale, and nothing comes out twice.
    #[test]
    fn a_spread_of_sequences_before_the_first_command_runs_nothing_twice() {
        let mut queue = CommandQueue::default();
        let offered = [0, 40_000, 20_000, 0, 65_535, 32_767, 32_768];
        let kept = offered.map(|sequence| queue.insert(queued(sequence, 1)));
        let (yes, stale) = (Inserted::Queued, Inserted::Stale);
        let twice = Inserted::Duplicate;
        assert_eq!(kept, [yes, stale, yes, twice, stale, stale, stale]);
        assert_eq!(drain(&mut queue), [0, 20_000]);
    }

    /// A command that is taken out and does not run (the room refused it) leaves the window where it was: the
    /// commands behind it still come in, and a second copy of it is offered to the room again, never kept twice.
    #[test]
    fn a_command_taken_out_and_refused_does_not_move_the_window() {
        let mut queue = CommandQueue::default();
        queue.insert(queued(10, 1));
        assert_eq!(drain(&mut queue), [10]);
        assert_eq!(queue.insert(queued(30_010, 1)), Inserted::Queued);
        assert_eq!(queue.take_due(1).map(|taken| taken.sequence), Some(30_010));
        // Not run. Sequence 11 is still newer than the last command that ran, and so is the refused one again.
        assert_eq!(queue.insert(queued(11, 1)), Inserted::Queued);
        assert_eq!(queue.insert(queued(30_010, 1)), Inserted::Queued);
        assert_eq!(queue.insert(queued(10, 1)), Inserted::Stale);
        // More than half the circle from the last command that ran: stale, whatever was taken out in between.
        assert_eq!(queue.insert(queued(40_010, 1)), Inserted::Stale);
        assert_eq!(drain(&mut queue), [11, 30_010]);
    }

    /// `ran` belongs right behind `take_due`: a command offered in between could lie behind the new base, and a
    /// debug build says so.
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "between its taking out and `ran`")]
    fn a_command_offered_between_the_taking_out_and_ran_is_caught() {
        let mut queue = CommandQueue::default();
        queue.insert(queued(3, 1));
        assert_eq!(drain(&mut queue), [3]);
        queue.insert(queued(5, 1));
        queue.insert(queued(7, 1));
        assert_eq!(queue.take_due(1).map(|taken| taken.sequence), Some(5));
        assert_eq!(queue.insert(queued(4, 1)), Inserted::Queued);
        queue.ran();
    }

    #[test]
    fn a_command_for_a_later_tick_waits_and_holds_back_the_ones_behind_it() {
        let mut queue = CommandQueue::default();
        queue.insert(queued(1, 10));
        queue.insert(queued(2, 12));
        queue.insert(queued(3, 12));
        assert_eq!(queue.take_due(9), None);
        assert_eq!(queue.take_due(10).map(|taken| taken.sequence), Some(1));
        assert_eq!(queue.take_due(11), None);
        // Late is still due.
        assert_eq!(queue.take_due(40).map(|taken| taken.sequence), Some(2));
        assert_eq!(queue.take_due(12).map(|taken| taken.sequence), Some(3));
    }

    #[test]
    fn a_full_queue_refuses_and_keeps_what_it_holds() {
        let mut queue = CommandQueue::default();
        for sequence in (0..32).step_by(2) {
            assert_eq!(queue.insert(queued(sequence, 1)), Inserted::Queued);
        }
        assert_eq!(queue.len(), QUEUE_LEN);
        for sequence in [5, 31, 32] {
            assert_eq!(queue.insert(queued(sequence, 1)), Inserted::Full);
        }
        assert_eq!(queue.insert(queued(4, 1)), Inserted::Duplicate);
        let even: Vec<u16> = (0..32).step_by(2).collect();
        assert_eq!(drain(&mut queue), even);
    }
}
