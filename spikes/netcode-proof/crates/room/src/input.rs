//! The input buffer under OQ-0001-6's option B (ADR 0001:1156-1162: "buffer them and run exactly one per client per
//! tick, repeating the last on a miss, late commands dropped and catch-up limited to the redundancy window"), and the
//! two room-side command checks PM-0001-58's counters pass through (u4-build-plan.md §4.14 :467-468; plan.md:514): the
//! sequence-forward compare and the press-edge compare, each through npt-proto's `Wrap` with its counter's name.
//! Option A, and every other command check with its -21 counter, are U4f's.

use npt_proto::Wrap;
use npt_sim::Command;

/// The sequence-forward compare's counter name in `Wrap` (PM-0001-58/lt-cmd-seq plants a plain `<` on it).
pub(crate) const CMD_SEQ: &str = "cmd_seq";
/// The press-edge compare's counter name (PM-0001-58/lt-press).
pub(crate) const PRESS: &str = "press";

/// One client's input buffer (option B) with its two checks' state.
#[derive(Clone, Debug, Default)]
pub struct InputBuffer {
    /// Accepted commands not yet run, at most [`InputBuffer::CMD_CATCHUP`], in stamped-tick order.
    buffered: Vec<Command>,
    /// The last command run: option B runs it again on a tick with none stamped for it.
    last_run: Option<Command>,
    /// The server tick the last `take` ran.
    ran_through: Option<u32>,
    /// The newest sequence received (the forward compare's reference).
    newest_seq: Option<u16>,
    /// The last command accepted (the press compare's reference).
    accepted: Option<Command>,
    /// How many ticks ahead the newest buffered command was at the last `take` (`buffer_depth_ticks`).
    depth: u32,
}

/// What the buffer did with one received command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Offered {
    /// Accepted and buffered for its tick.
    Buffered,
    /// Its sequence is not after the newest received (a redundant copy): not run again.
    Repeat,
    /// A press counter moved backwards or by more edges than the ticks the command covers: refused.
    PressRefused,
    /// Stamped for a tick already run, or one already buffered: it can never run.
    Late,
    /// The buffer holds [`InputBuffer::CMD_CATCHUP`] commands already.
    Full,
}

/// What `take` ran for one tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Taken {
    /// The command run, if the client has sent any yet.
    pub(crate) command: Option<Command>,
    /// Whether it is the last command run again (no command was stamped for this tick).
    pub(crate) repeated: bool,
    /// Buffered commands stamped before this tick, dropped as late.
    pub(crate) late: u32,
}

impl InputBuffer {
    /// Commands per client the buffer holds ahead, the redundancy cap (u4-build-plan.md §4.3; carry 27): one `Input`
    /// carries at most this many, so a client never needs more buffered. Not derived from the tick rate.
    pub const CMD_CATCHUP: usize = npt_proto::Limits::REDUNDANCY_CAP;

    /// How many ticks ahead of the tick last run the newest buffered command was (0: none ahead).
    pub fn buffer_depth_ticks(&self) -> u32 {
        self.depth
    }

    /// The newest sequence received, which a snapshot's header acknowledges (it ends the client's repeats).
    pub(crate) fn newest_received(&self) -> Option<u16> {
        self.newest_seq
    }

    /// The last command run, whose sequence a snapshot's header names (where the client's replay starts).
    pub(crate) fn last_run(&self) -> Option<&Command> {
        self.last_run.as_ref()
    }

    /// The checks and the buffer for one received command (its `slot` already the session's). A counter that wrapped
    /// on an accepted command pushes its name onto `wraps` (for -58's `event.counter_wrap`).
    pub(crate) fn offer(&mut self, c: Command, w: Wrap, wraps: &mut Vec<&'static str>) -> Offered {
        if let Some(n) = self.newest_seq {
            if !w.after16(CMD_SEQ, n, c.seq) {
                return Offered::Repeat;
            }
            if c.seq < n {
                wraps.push(CMD_SEQ);
            }
        }
        self.newest_seq = Some(c.seq);
        if let Some(prev) = &self.accepted {
            // The ticks the command covers: from the last accepted command's tick to its own.
            let covered = (c.tick.wrapping_sub(prev.tick) as i32).max(0) as u32;
            let mut wrapped = false;
            for (&a, &b) in prev.presses.iter().zip(&c.presses) {
                if a == b {
                    continue;
                }
                if !w.after8(PRESS, a, b) || u32::from(b.wrapping_sub(a)) > covered {
                    return Offered::PressRefused;
                }
                wrapped |= b < a;
            }
            if wrapped {
                wraps.push(PRESS);
            }
        }
        if self
            .ran_through
            .is_some_and(|r| !w.after32("tick", r, c.tick))
            || self.buffered.iter().any(|b| b.tick == c.tick)
        {
            return Offered::Late;
        }
        if self.buffered.len() >= InputBuffer::CMD_CATCHUP {
            return Offered::Full;
        }
        let at = self
            .buffered
            .iter()
            .position(|b| w.after32("tick", c.tick, b.tick))
            .unwrap_or(self.buffered.len());
        self.buffered.insert(at, c);
        self.accepted = Some(c);
        Offered::Buffered
    }

    /// The command for tick `tick` (option B): the one stamped for it, else the last command run again; buffered
    /// commands stamped before it are dropped as late.
    pub(crate) fn take(&mut self, tick: u32) -> Taken {
        let w = Wrap::default();
        self.ran_through = Some(tick);
        let before = self.buffered.len();
        self.buffered.retain(|c| !w.after32("tick", c.tick, tick));
        let late = (before - self.buffered.len()) as u32;
        let (command, repeated) = if self.buffered.first().is_some_and(|c| c.tick == tick) {
            let c = self.buffered.remove(0);
            self.last_run = Some(c);
            (Some(c), false)
        } else {
            (self.last_run, self.last_run.is_some())
        };
        self.depth = self
            .buffered
            .last()
            .map_or(0, |c| c.tick.wrapping_sub(tick));
        Taken {
            command,
            repeated,
            late,
        }
    }
}

// Carry 27 (u4-build-plan.md §4.3; ADR 0001:238-239): the catch-up cap is no larger than the redundancy cap.
const _: () = assert!(InputBuffer::CMD_CATCHUP <= npt_proto::Limits::REDUNDANCY_CAP);

#[cfg(test)]
mod tests {
    use super::*;

    fn cmd(seq: u16, tick: u32, presses: [u8; 4]) -> Command {
        Command {
            slot: 3,
            tick,
            seq,
            presses,
            ..Command::default()
        }
    }

    /// Carry 27: the const assert holds `CMD_CATCHUP` to the redundancy cap, and it is that cap.
    #[test]
    fn carry_27_cmd_catchup_is_the_redundancy_cap() {
        assert_eq!(InputBuffer::CMD_CATCHUP, npt_proto::Limits::REDUNDANCY_CAP);
        let mut b = InputBuffer::default();
        let w = Wrap::default();
        let mut wraps = Vec::new();
        for k in 0..InputBuffer::CMD_CATCHUP as u16 {
            assert_eq!(
                b.offer(cmd(k, 10 + u32::from(k), [0; 4]), w, &mut wraps),
                Offered::Buffered
            );
        }
        assert_eq!(b.offer(cmd(99, 99, [0; 4]), w, &mut wraps), Offered::Full);
    }

    /// e9's first room-side negative: a repeated command is not run twice.
    #[test]
    fn e9_a_repeated_command_is_not_run_twice() {
        let mut b = InputBuffer::default();
        let w = Wrap::default();
        let mut wraps = Vec::new();
        assert_eq!(
            b.offer(cmd(7, 10, [0; 4]), w, &mut wraps),
            Offered::Buffered
        );
        assert_eq!(b.offer(cmd(7, 10, [0; 4]), w, &mut wraps), Offered::Repeat);
        assert_eq!(b.offer(cmd(6, 9, [0; 4]), w, &mut wraps), Offered::Repeat);
        let t = b.take(10);
        assert_eq!((t.command.map(|c| c.seq), t.repeated), (Some(7), false));
        // The next tick has none stamped: option B runs the last command again, flagged as a repeat.
        let t = b.take(11);
        assert_eq!((t.command.map(|c| c.seq), t.repeated), (Some(7), true));
    }

    /// e9's second room-side negative: two press edges for one button inside one covered tick are refused; one edge per
    /// covered tick passes, and a counter moving backwards is refused.
    #[test]
    fn e9_two_press_edges_inside_one_covered_tick_are_refused() {
        let mut b = InputBuffer::default();
        let w = Wrap::default();
        let mut wraps = Vec::new();
        assert_eq!(
            b.offer(cmd(1, 10, [0; 4]), w, &mut wraps),
            Offered::Buffered
        );
        assert_eq!(
            b.offer(cmd(2, 11, [2, 0, 0, 0]), w, &mut wraps),
            Offered::PressRefused
        );
        assert_eq!(
            b.offer(cmd(3, 11, [1, 0, 0, 0]), w, &mut wraps),
            Offered::Buffered
        );
        // Covering two ticks, two edges pass.
        assert_eq!(
            b.offer(cmd(4, 13, [3, 0, 0, 0]), w, &mut wraps),
            Offered::Buffered
        );
        assert_eq!(
            b.offer(cmd(5, 14, [2, 0, 0, 0]), w, &mut wraps),
            Offered::PressRefused
        );
        assert!(wraps.is_empty());
    }

    /// e9's "each counter accepted across its wrap in the clean build": the sequence and a press counter cross their
    /// wraps from injectable start values a few steps below, each wrap reported once by name.
    #[test]
    fn e9_the_sequence_and_the_press_counters_are_accepted_across_their_wraps() {
        let mut b = InputBuffer::default();
        let w = Wrap::default();
        let mut wraps = Vec::new();
        let (seq0, press0) = (u16::MAX - 2, u8::MAX - 1);
        for k in 0..6u16 {
            let seq = seq0.wrapping_add(k);
            let press = press0.wrapping_add(k as u8);
            let got = b.offer(
                cmd(seq, 100 + u32::from(k), [press, 0, 0, 0]),
                w,
                &mut wraps,
            );
            assert_eq!(got, Offered::Buffered, "step {k}");
            b.take(100 + u32::from(k));
        }
        assert_eq!(wraps, [PRESS, CMD_SEQ]);
    }

    #[test]
    fn late_commands_are_dropped_and_the_depth_is_measured() {
        let mut b = InputBuffer::default();
        let w = Wrap::default();
        let mut wraps = Vec::new();
        for (seq, tick) in [(1, 10), (2, 11), (3, 13)] {
            assert_eq!(
                b.offer(cmd(seq, tick, [0; 4]), w, &mut wraps),
                Offered::Buffered
            );
        }
        let t = b.take(11);
        assert_eq!((t.command.map(|c| c.seq), t.late), (Some(2), 1));
        assert_eq!(b.buffer_depth_ticks(), 2);
        assert_eq!(b.offer(cmd(4, 11, [0; 4]), w, &mut wraps), Offered::Late);
        assert_eq!(
            b.offer(cmd(5, 13, [0; 4]), w, &mut wraps),
            Offered::Late,
            "tick 13 is buffered"
        );
    }
}
