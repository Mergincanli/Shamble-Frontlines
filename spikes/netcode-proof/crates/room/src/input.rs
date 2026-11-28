//! The input buffer under both of OQ-0001-6's options (ADR 0001:1156-1167) and every room-side command check of ADR
//! 0001:231-239 with its -21 counter (u4f-brief.md 2.3, 2.4): option B buffers and runs exactly one command per client
//! per tick, repeating the last on a miss (U4e's); option A runs each command at the first room tick after it arrives,
//! up to `CMD_CATCHUP` a tick (U4f's). -58's two room-side compares (the sequence-forward and the press-edge) keep their
//! sites and their `Wrap` counter names.
//!
//! **The order** (U4F-Q9 (a)): a sequence not after the newest received goes to the repeat filter, never counted as
//! forged: an equal copy of the command kept under it is a redundant copy (`Repeat`); a changed copy of an *accepted*
//! command kept under it is kind 3; any copy of a *refused* sequence is a repeat; a sequence with no command kept
//! under it (older than the kept window, or a gap) is `OutsideWindow`. A new sequence becomes the newest received
//! (U4F-Q13 (b)), then meets kind 1 (the lead bound), kind 2 (strictly newer than the newest accepted, or on a fresh slot
//! after the join's tick: the join anchor), the rejoin overlap (U4F-Q18 (ii) (d)), kinds 4, 5 and 6, then option B's
//! late rule or option A's past window W = `CMD_CATCHUP` (U4F-Q18 (a)), then kind 10 (the catch-up cap). The first
//! check that refuses decides; one kind per refusal. Every tick compare is a signed distance (`dist`), never `t + n` or
//! `t - n` in u32 (SA31-3).

use std::collections::VecDeque;

use npt_proto::Wrap;
use npt_sim::Command;

/// The sequence-forward compare's counter name in `Wrap` (PM-0001-58/lt-cmd-seq plants a plain `<` on it).
pub(crate) const CMD_SEQ: &str = "cmd_seq";
/// The press-edge compare's counter name (PM-0001-58/lt-press).
pub(crate) const PRESS: &str = "press";
/// The kept window's reorder allowance beyond the redundancy cap (U4F-Q9 (a): K = 8 + 8 = 16), in commands.
const REORDER_ALLOWANCE: usize = 8;
/// Accepted stamps held for `tick.s.cmd_on_time` (a bound on memory only: kind 1 keeps them within the lead bound).
const STAMPS_HELD: usize = 64;
/// Under PM-0001-21/no-catchup-cap the cap is gone, yet the memory stays bounded (no unbounded buffer).
const BITE_HELD: usize = 4 * npt_proto::Limits::REDUNDANCY_CAP;

/// `a`'s signed distance from `b` in ticks (positive: `a` after `b`), wrap-aware (SA31-3).
pub(crate) fn dist(a: u32, b: u32) -> i64 {
    i64::from(a.wrapping_sub(b) as i32)
}

/// OQ-0001-6's two command-timing options: a start-up setting, recorded in every header as `params.cmd_timing`
/// (README :191-192), never changed in a run.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CmdTiming {
    /// (a) Each command runs at the first room tick after it arrives, several in one tick within `CMD_CATCHUP`.
    A,
    /// (b) Buffered, exactly one per client per tick (the one stamped for it), the last run again on a miss.
    #[default]
    B,
}

/// A forged-command kind the room refuses and counts (ADR 0001:231-239; u4f-brief.md 2.4's table). Kind 7 is the
/// sim's (its `reports()`, the sim's own `Refusal` records), kinds 9 and 11 the pump's (C-U4F2-3: named apart from
/// the sim's records).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandRefusal {
    /// Kind 1: the tick too far ahead of the room's (the lead bound).
    LeadBound,
    /// Kind 2: the tick not strictly newer than the newest accepted (on a fresh slot: at or before the join's tick).
    NotNewer,
    /// Kind 3: a resent copy, with changed content, of a command the room accepted (the sequence not forward).
    SeqNotForward,
    /// Kind 4: the move vector longer than 1 (x² + y² > 127²).
    MoveLength,
    /// Kind 5: more press edges than the ticks the command covers, or a counter moving backwards.
    PressEdge,
    /// Kind 6: the aim point outside the map's x and y extent.
    AimBounds,
    /// Kind 8: the datagram's acknowledgement names a snapshot never sent (its commands refused whole).
    AckNeverSent,
    /// Kind 10: more commands in a tick than the catch-up cap (A), or than the buffer holds (B).
    Catchup,
}

impl CommandRefusal {
    /// The kind's number in u4f-brief.md 2.4's table.
    pub fn kind(self) -> u8 {
        match self {
            CommandRefusal::LeadBound => 1,
            CommandRefusal::NotNewer => 2,
            CommandRefusal::SeqNotForward => 3,
            CommandRefusal::MoveLength => 4,
            CommandRefusal::PressEdge => 5,
            CommandRefusal::AimBounds => 6,
            CommandRefusal::AckNeverSent => 8,
            CommandRefusal::Catchup => 10,
        }
    }
}

/// The checks a planted -21 bite removes or relaxes (bites builds only set any of them: `Room::with_bite`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Relax {
    pub(crate) no_lead_bound: bool,
    pub(crate) no_newer_tick: bool,
    pub(crate) not_older_than: bool,
    pub(crate) no_seq_forward: bool,
    pub(crate) no_move_length: bool,
    pub(crate) no_press_edge: bool,
    pub(crate) no_aim_bounds: bool,
    pub(crate) no_catchup_cap: bool,
}

/// The room's side of one offer: the tick it runs next, the lead bound in ticks, the map's x and y extent (cm: [x0,
/// y0, x1, y1]), the arrival number (option A's run order) and any relaxed check. The option is the buffer's own.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Checks {
    pub(crate) next_tick: u32,
    pub(crate) lead_bound: u32,
    pub(crate) extent: [i32; 4],
    pub(crate) arrival: u64,
    pub(crate) relax: Relax,
}

/// One client's input buffer, under its option, with its checks' state.
#[derive(Clone, Debug, Default)]
pub struct InputBuffer {
    timing: CmdTiming,
    /// Accepted commands not yet run, with their arrival numbers: under B for their stamped ticks, in stamped-tick
    /// order, at most [`InputBuffer::CMD_CATCHUP`]; under A for the coming tick, in arrival order, at most the same.
    buffered: Vec<(u64, Command)>,
    /// The last command run: option B runs it again on a tick with none stamped for it.
    last_run: Option<Command>,
    /// The server tick the last `take` ran.
    ran_through: Option<u32>,
    /// The newest sequence received (the forward compare's reference).
    newest_seq: Option<u16>,
    /// The newest command accepted (kind 2's and the press compare's reference).
    accepted: Option<Command>,
    /// How many ticks ahead of the tick last run the newest buffered command was (`buffer_depth_ticks`; 0 under A).
    depth: u32,
    /// The kept window: the last K commands received under a new sequence, each with whether it was accepted.
    kept: VecDeque<(Command, bool)>,
    /// The tick the join's `JoinAccepted` names (the join anchor: kind 2's reference on a fresh slot).
    join_tick: u32,
    /// The slot's newest accepted stamp before this join, kept by the room across joins (U4F-Q18 (ii) (d)).
    kept_stamp: Option<u32>,
    /// Accepted stamps not yet due, oldest first (they rise strictly): `tick.s.cmd_on_time` by stamp.
    stamps: VecDeque<u32>,
    /// The first accepted stamp: from its tick on, the client has an entry in `tick.s.cmd_on_time` every tick.
    first_stamp: Option<u32>,
}

/// What the buffer did with one received command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Offered {
    /// Accepted: buffered for its tick (B), or held for the coming tick (A).
    Accepted,
    /// A redundant copy (equal to the accepted command kept under its sequence), or any copy of a refused sequence:
    /// not run again, never forged.
    Repeat,
    /// A sequence not after the newest received with no command kept under it (older than the kept window, or a
    /// gap): dropped, never forged (`seq_outside_window`).
    OutsideWindow,
    /// After the join's tick but at or before the slot's kept stamp from an earlier session: dropped, never forged
    /// (`rejoin_overlap`; U4F-Q18 (ii) (d)).
    RejoinOverlap,
    /// It can never run: under B stamped for a tick already run; under A stamped more than W ticks before the tick it
    /// would run at (U4F-Q18 (a)). Dropped, counted `late`, never forged.
    Late,
    /// Refused under a forged kind.
    Refused(CommandRefusal),
}

/// What `take` ran for one tick (option B).
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
    /// carries at most this many, so a client never needs more buffered. Under option A it is also the past window W
    /// (U4F-Q18 (a): a count of commands, at one stamp per tick a count of ticks). Not derived from the tick rate.
    pub const CMD_CATCHUP: usize = npt_proto::Limits::REDUNDANCY_CAP;

    /// The kept window's length K (U4F-Q9 (a)): the redundancy cap plus a reorder allowance of 8.
    pub(crate) const KEPT_WINDOW: usize = npt_proto::Limits::REDUNDANCY_CAP + REORDER_ALLOWANCE;

    /// A fresh buffer for a join under `timing`, anchored at the join's tick `join_tick` (its `JoinAccepted` tick), with
    /// the slot's newest accepted stamp from its earlier sessions, if any.
    pub(crate) fn new(timing: CmdTiming, join_tick: u32, kept_stamp: Option<u32>) -> InputBuffer {
        InputBuffer {
            timing,
            join_tick,
            kept_stamp,
            ..InputBuffer::default()
        }
    }

    /// How many ticks ahead of the tick last run the newest buffered command was (0: none ahead; always 0 under
    /// option A, which buffers nothing: U4F-Q3 (c)).
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

    /// The checks and the buffer for one received command (its `slot` already the session's), in 2.4's order. A counter
    /// that wrapped on an accepted command pushes its name onto `wraps` (for -58's `event.counter_wrap`).
    pub(crate) fn offer(
        &mut self,
        c: Command,
        w: Wrap,
        wraps: &mut Vec<&'static str>,
        k: &Checks,
    ) -> Offered {
        let mut seq_wrapped = false;
        if let Some(n) = self.newest_seq {
            if !w.after16(CMD_SEQ, n, c.seq) {
                return self.filter(c, k);
            }
            seq_wrapped = c.seq < n;
        }
        self.newest_seq = Some(c.seq);
        let out = self.check(c, w, wraps, k);
        if seq_wrapped && out == Offered::Accepted {
            wraps.push(CMD_SEQ);
        }
        if self.kept.len() >= InputBuffer::KEPT_WINDOW {
            self.kept.pop_front();
        }
        self.kept.push_back((c, out == Offered::Accepted));
        out
    }

    /// The repeat filter: a sequence not after the newest received.
    fn filter(&mut self, c: Command, k: &Checks) -> Offered {
        let Some(i) = self.kept.iter().position(|(kc, _)| kc.seq == c.seq) else {
            return Offered::OutsideWindow;
        };
        let (kc, accepted) = self.kept[i];
        if !accepted || kc == c {
            return Offered::Repeat;
        }
        if k.relax.no_seq_forward {
            // PM-0001-21/no-seq-forward: the check removed, the changed copy takes the command's place where it waits,
            // else it runs as one more command.
            self.kept[i].0 = c;
            if let Some(b) = self.buffered.iter_mut().find(|(_, b)| b.seq == c.seq) {
                b.1 = c;
            } else if self.buffered.len() < BITE_HELD {
                self.buffered.push((k.arrival, c));
            } else {
                return Offered::Refused(CommandRefusal::Catchup);
            }
            return Offered::Accepted;
        }
        Offered::Refused(CommandRefusal::SeqNotForward)
    }

    /// A new sequence's checks (kinds 1, 2, the rejoin overlap, 4, 5, 6, late or the past window, 10), then the buffer.
    fn check(&mut self, c: Command, w: Wrap, wraps: &mut Vec<&'static str>, k: &Checks) -> Offered {
        let r = k.relax;
        // Kind 1: no more than the lead bound ahead of the tick the room runs next.
        if !r.no_lead_bound && dist(c.tick, k.next_tick) > i64::from(k.lead_bound) {
            return Offered::Refused(CommandRefusal::LeadBound);
        }
        // Kind 2: strictly newer than the newest accepted (run or buffered: NA-13), or on a fresh slot strictly after
        // the join's tick (the join anchor, SA3-1).
        let reference = self.accepted.map_or(self.join_tick, |a| a.tick);
        let d = dist(c.tick, reference);
        let newer = if r.not_older_than { d >= 0 } else { d > 0 };
        if !r.no_newer_tick && !newer {
            return Offered::Refused(CommandRefusal::NotNewer);
        }
        // The rejoin overlap: a fresh session's command at or before the slot's kept stamp (U4F-Q18 (ii) (d)).
        if self.accepted.is_none() && self.kept_stamp.is_some_and(|s| dist(c.tick, s) <= 0) {
            return Offered::RejoinOverlap;
        }
        // Kind 4: x² + y² ≤ 127², each part widened to i32 before it is squared (SA3-3).
        let (x, y) = (i32::from(c.move_xy[0]), i32::from(c.move_xy[1]));
        if !r.no_move_length && x * x + y * y > 127 * 127 {
            return Offered::Refused(CommandRefusal::MoveLength);
        }
        // Kind 5: at most one edge per button for each tick the command covers, counters never backwards.
        let mut press_wrapped = false;
        if let Some(prev) = &self.accepted {
            let covered = dist(c.tick, prev.tick).max(0);
            for (&a, &b) in prev.presses.iter().zip(&c.presses) {
                if a == b {
                    continue;
                }
                if !r.no_press_edge
                    && (!w.after8(PRESS, a, b) || i64::from(b.wrapping_sub(a)) > covered)
                {
                    return Offered::Refused(CommandRefusal::PressEdge);
                }
                press_wrapped |= b < a;
            }
        }
        // Kind 6: the aim's x and y inside the map's extent (z is the codec's and the clamp's: SA-14).
        let [x0, y0, x1, y1] = k.extent;
        let inside = (x0..=x1).contains(&c.aim[0]) && (y0..=y1).contains(&c.aim[1]);
        if !r.no_aim_bounds && !inside {
            return Offered::Refused(CommandRefusal::AimBounds);
        }
        // B's late rule; A's past window W (more than W ticks before the tick it would run at: U4F-Q18 (a)).
        let late = match self.timing {
            CmdTiming::B => self.ran_through.is_some_and(|t| dist(c.tick, t) <= 0),
            CmdTiming::A => dist(k.next_tick, c.tick) > InputBuffer::CMD_CATCHUP as i64,
        };
        if late {
            return Offered::Late;
        }
        // Kind 10: A's per-tick cap, on arrival, once CMD_CATCHUP are held for the coming tick (SA3-8); B's `Full`. An
        // honest client under U4f's controller cap of 9 stays inside B's 8 only while its up delay is at least about 2
        // ticks whenever the cap binds, which L1's bounded jitter always gives; other links are C-U6-17's (NC-U4F-3).
        let cap = if self.timing == CmdTiming::A && r.no_catchup_cap {
            BITE_HELD
        } else {
            InputBuffer::CMD_CATCHUP
        };
        if self.buffered.len() >= cap {
            return Offered::Refused(CommandRefusal::Catchup);
        }
        match self.timing {
            CmdTiming::B => {
                let at = self
                    .buffered
                    .iter()
                    .position(|(_, b)| dist(c.tick, b.tick) < 0)
                    .unwrap_or(self.buffered.len());
                self.buffered.insert(at, (k.arrival, c));
            }
            CmdTiming::A => self.buffered.push((k.arrival, c)),
        }
        if press_wrapped {
            wraps.push(PRESS);
        }
        self.accepted = Some(c);
        if self.stamps.len() >= STAMPS_HELD {
            self.stamps.pop_front();
        }
        self.stamps.push_back(c.tick);
        self.first_stamp.get_or_insert(c.tick);
        Offered::Accepted
    }

    /// `tick.s.cmd_on_time` and `tick.s.input_missing` for tick `tick`, by stamp (NA-14; the advisor's A3), before it
    /// runs: `None` until the client's first accepted stamp is at or before `tick`; then whether a command stamped for
    /// `tick` had arrived (been accepted) before it ran.
    pub(crate) fn due(&mut self, tick: u32) -> Option<bool> {
        while self.stamps.front().is_some_and(|&s| dist(s, tick) < 0) {
            self.stamps.pop_front();
        }
        let first = self.first_stamp?;
        (dist(tick, first) >= 0).then(|| self.stamps.front() == Some(&tick))
    }

    /// The command for tick `tick` (option B): the one stamped for it, else the last command run again; buffered
    /// commands stamped before it are dropped as late.
    pub(crate) fn take(&mut self, tick: u32) -> Taken {
        self.ran_through = Some(tick);
        let before = self.buffered.len();
        self.buffered.retain(|(_, c)| dist(c.tick, tick) >= 0);
        let late = (before - self.buffered.len()) as u32;
        let (command, repeated) = if self.buffered.first().is_some_and(|(_, c)| c.tick == tick) {
            let (_, c) = self.buffered.remove(0);
            self.last_run = Some(c);
            (Some(c), false)
        } else {
            (self.last_run, self.last_run.is_some())
        };
        self.depth = self
            .buffered
            .last()
            .map_or(0, |(_, c)| c.tick.wrapping_sub(tick));
        Taken {
            command,
            repeated,
            late,
        }
    }

    /// Option A's commands for tick `tick`: every one held, with its arrival number, onto `out` (in arrival order).
    pub(crate) fn drain(&mut self, tick: u32, out: &mut Vec<(u64, Command)>) {
        self.ran_through = Some(tick);
        if let Some((_, c)) = self.buffered.last() {
            self.last_run = Some(*c);
        }
        out.append(&mut self.buffered);
        self.depth = 0;
    }

    /// The commands held now (B: buffered ahead; A: for the coming tick).
    #[cfg(test)]
    pub(crate) fn held(&self) -> usize {
        self.buffered.len()
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

    /// Checks for a room whose next tick is `next`, a wide lead bound, the greybox's extent.
    fn at(_timing: CmdTiming, next: u32) -> Checks {
        Checks {
            next_tick: next,
            lead_bound: 1_000,
            extent: [-12_800, -12_800, 12_800, 12_800],
            arrival: 0,
            relax: Relax::default(),
        }
    }

    /// Carry 27: the const assert holds `CMD_CATCHUP` to the redundancy cap, and it is that cap; one more is kind 10.
    #[test]
    fn carry_27_cmd_catchup_is_the_redundancy_cap() {
        assert_eq!(InputBuffer::CMD_CATCHUP, npt_proto::Limits::REDUNDANCY_CAP);
        let mut b = InputBuffer::default();
        let w = Wrap::default();
        let k = at(CmdTiming::B, 10);
        let mut wraps = Vec::new();
        for i in 0..InputBuffer::CMD_CATCHUP as u16 {
            assert_eq!(
                b.offer(cmd(i, 10 + u32::from(i), [0; 4]), w, &mut wraps, &k),
                Offered::Accepted
            );
        }
        assert_eq!(
            b.offer(cmd(99, 99, [0; 4]), w, &mut wraps, &k),
            Offered::Refused(CommandRefusal::Catchup)
        );
    }

    /// e9's first room-side negative: a repeated command is not run twice; a sequence never received is outside the
    /// kept window (U4F-Q9 (a)), never a repeat.
    #[test]
    fn e9_a_repeated_command_is_not_run_twice() {
        let mut b = InputBuffer::default();
        let w = Wrap::default();
        let k = at(CmdTiming::B, 10);
        let mut wraps = Vec::new();
        assert_eq!(
            b.offer(cmd(7, 10, [0; 4]), w, &mut wraps, &k),
            Offered::Accepted
        );
        assert_eq!(
            b.offer(cmd(7, 10, [0; 4]), w, &mut wraps, &k),
            Offered::Repeat
        );
        assert_eq!(
            b.offer(cmd(6, 9, [0; 4]), w, &mut wraps, &k),
            Offered::OutsideWindow
        );
        let t = b.take(10);
        assert_eq!((t.command.map(|c| c.seq), t.repeated), (Some(7), false));
        // The next tick has none stamped: option B runs the last command again, flagged as a repeat.
        let t = b.take(11);
        assert_eq!((t.command.map(|c| c.seq), t.repeated), (Some(7), true));
    }

    /// e9's second room-side negative: two press edges for one button inside one covered tick are refused (kind 5); one
    /// edge per covered tick passes, and a counter moving backwards is refused.
    #[test]
    fn e9_two_press_edges_inside_one_covered_tick_are_refused() {
        let mut b = InputBuffer::default();
        let w = Wrap::default();
        let k = at(CmdTiming::B, 10);
        let mut wraps = Vec::new();
        let press = Offered::Refused(CommandRefusal::PressEdge);
        assert_eq!(
            b.offer(cmd(1, 10, [0; 4]), w, &mut wraps, &k),
            Offered::Accepted
        );
        assert_eq!(b.offer(cmd(2, 11, [2, 0, 0, 0]), w, &mut wraps, &k), press);
        assert_eq!(
            b.offer(cmd(3, 11, [1, 0, 0, 0]), w, &mut wraps, &k),
            Offered::Accepted
        );
        // Covering two ticks, two edges pass.
        assert_eq!(
            b.offer(cmd(4, 13, [3, 0, 0, 0]), w, &mut wraps, &k),
            Offered::Accepted
        );
        assert_eq!(b.offer(cmd(5, 14, [2, 0, 0, 0]), w, &mut wraps, &k), press);
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
        for i in 0..6u16 {
            let seq = seq0.wrapping_add(i);
            let press = press0.wrapping_add(i as u8);
            let k = at(CmdTiming::B, 100 + u32::from(i));
            let got = b.offer(
                cmd(seq, 100 + u32::from(i), [press, 0, 0, 0]),
                w,
                &mut wraps,
                &k,
            );
            assert_eq!(got, Offered::Accepted, "step {i}");
            b.take(100 + u32::from(i));
        }
        assert_eq!(wraps, [PRESS, CMD_SEQ]);
    }

    /// Late commands are dropped and the depth is measured; a stamp equal to a buffered one, or older than the newest
    /// accepted, is kind 2 (NA-13: 2.4's order refuses it before the late rule).
    #[test]
    fn late_commands_are_dropped_and_the_depth_is_measured() {
        let mut b = InputBuffer::default();
        let w = Wrap::default();
        let k = at(CmdTiming::B, 10);
        let mut wraps = Vec::new();
        for (seq, tick) in [(1, 10), (2, 11), (3, 13)] {
            assert_eq!(
                b.offer(cmd(seq, tick, [0; 4]), w, &mut wraps, &k),
                Offered::Accepted
            );
        }
        let t = b.take(11);
        assert_eq!((t.command.map(|c| c.seq), t.late), (Some(2), 1));
        assert_eq!(b.buffer_depth_ticks(), 2);
        let kind2 = Offered::Refused(CommandRefusal::NotNewer);
        assert_eq!(b.offer(cmd(4, 11, [0; 4]), w, &mut wraps, &k), kind2);
        assert_eq!(
            b.offer(cmd(5, 13, [0; 4]), w, &mut wraps, &k),
            kind2,
            "tick 13 is buffered"
        );
        // Stamped after the newest accepted but for a tick already run: late (option B).
        let mut b = InputBuffer::default();
        assert_eq!(
            b.offer(cmd(1, 5, [0; 4]), w, &mut wraps, &k),
            Offered::Accepted
        );
        b.take(5);
        b.take(6);
        b.take(7);
        assert_eq!(b.offer(cmd(2, 7, [0; 4]), w, &mut wraps, &k), Offered::Late);
        assert_eq!(b.held(), 0);
    }

    /// U4F-Q9 (a) (neg; TE-U4F-2): a flood of 10,000 new sequences from 60,000, across the u16 wrap, each one tick
    /// later: the kept window never holds more than K (16), and exactly K from the 16th on.
    #[test]
    fn q9_a_the_kept_window_never_holds_more_than_k_under_a_flood_of_new_sequences() {
        let mut b = InputBuffer::default();
        let mut wraps = Vec::new();
        for i in 0..10_000u32 {
            let k = at(CmdTiming::B, 10 + i);
            let seq = 60_000u16.wrapping_add(i as u16);
            b.offer(cmd(seq, 10 + i, [0; 4]), Wrap::default(), &mut wraps, &k);
            b.take(10 + i);
            let (n, most) = (b.kept.len(), InputBuffer::KEPT_WINDOW);
            assert!(n <= most && (i < 15 || n == most), "{i}: {n}");
        }
    }

    /// f38 (SA3-1; U4F-Q18 (a)): under option A a client that idles N ticks after a stamp of the join's tick + 1 (N = 60
    /// s of ticks at 30 Hz, and N = 100,000), then gives 8 commands a tick stamped one apart from its last stamp + 1:
    /// over any span at most W + the lead bound (8 + 12) more commands run than ticks pass, and every command the past
    /// window drops is counted `late`, exactly those more than W ticks before the tick they would run at; a case at
    /// room tick 1 (no underflow).
    #[test]
    fn f38_option_a_after_an_idle_runs_at_most_w_plus_the_bound_more_commands_than_ticks() {
        let w = Wrap::default();
        let mut wraps = Vec::new();
        for (join, idle) in [(1u32, 1_800u32), (1, 100_000), (4_000_000_000, 1_800)] {
            let mut b = InputBuffer::new(CmdTiming::A, join, None);
            let mut k = at(CmdTiming::A, join);
            k.lead_bound = 12;
            let mut out = Vec::new();
            assert_eq!(
                b.offer(cmd(0, join.wrapping_add(1), [0; 4]), w, &mut wraps, &k),
                Offered::Accepted
            );
            b.drain(join, &mut out);
            let mut next = join.wrapping_add(1).wrapping_add(idle);
            let (mut last, mut seq) = (join.wrapping_add(1), 1u16);
            let (mut late, mut late_expected) = (0u64, 0u64);
            let mut excess = Vec::new();
            // The stamps close 7 ticks a tick on the room's: idle / 7 ticks to catch up, then 400 more.
            for _ in 0..(idle / 7 + 400) {
                k.next_tick = next;
                for _ in 0..8 {
                    last = last.wrapping_add(1);
                    if dist(next, last) > 8 {
                        late_expected += 1;
                    }
                    match b.offer(cmd(seq, last, [0; 4]), w, &mut wraps, &k) {
                        Offered::Late => late += 1,
                        Offered::Accepted | Offered::Refused(_) => {}
                        other => panic!("{other:?}"),
                    }
                    seq = seq.wrapping_add(1);
                }
                out.clear();
                b.drain(next, &mut out);
                excess.push(out.len() as i64 - 1);
                next = next.wrapping_add(1);
            }
            // The most any span runs beyond its ticks (Kadane's maximum subarray sum).
            let (mut best, mut run) = (0i64, 0i64);
            for e in excess {
                run = (run + e).max(0);
                best = best.max(run);
            }
            assert!(best <= 8 + 12, "join {join}, idle {idle}: {best}");
            assert!(best > 0, "the window let some bank through");
            assert_eq!(late, late_expected, "join {join}, idle {idle}");
            assert!(late > 0);
        }
    }
}
