//! The room's match logic, with no I/O, no async runtime and no clock (ADR 0002 part 3; ADR 0001 parts 5 and 6).
//! So far: one player's slot, the join exchange, the checks on that player's commands, their queue, the world's
//! step and the snapshot the player gets back. Whoever holds the transport hands the room what arrived and sends
//! what it wrote; nothing here has a duration yet, so there is no timer.
//!
//! The room never panics on bytes from the link and allocates nothing per tick. A check that fails drops the
//! command or the message and counts it; the counters are what a test, and later a metric, reads.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::arithmetic_side_effects
    )
)]

pub mod queue;

use protocol::codec::{Control, decode_control, decode_input, encode_control, encode_snapshot};
use protocol::messages::{
    Input, JoinAccepted, JoinRejected, OwnState, PROTOCOL_VERSION, RejectReason, Snapshot,
};
use protocol::wrap::{press_edges, seq_ahead, seq_newer, ticks_after};
use queue::{CommandQueue, Inserted, Queued};
use sim::{BUTTONS, MatchSetup, PlayerCommand, TICK_HZ, World};

const _: () = assert!(protocol::messages::BUTTONS == BUTTONS);

/// The tick rate as the join answer carries it.
const TICK_RATE: u8 = {
    assert!(TICK_HZ <= 255);
    TICK_HZ as u8
};

/// How far ahead of the tick being run a command's tick may lie; one further ahead is dropped.
pub const LEAD_MAX_TICKS: u32 = 8;

/// The most commands of one player the room runs in one tick, when several are due at once.
pub const CATCH_UP_CAP: usize = 2;

/// The one mode so far. Provisional, as the mode hash is: both come with the content data.
pub const MODE_ID: &str = "skeleton";

/// What the room counted. Each wraps.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RoomCounters {
    pub joins: u32,
    pub rejections: u32,
    /// Bytes that were no message of their channel.
    pub decode_errors: u32,
    /// Inputs dropped whole: an ack of a snapshot the room never sent.
    pub refused_ack: u32,
    /// Commands dropped: for a tick too far ahead; not for a later tick than the command run before; a press
    /// counter that moved by more than one press per tick since the command run before.
    pub refused_ahead: u32,
    pub refused_not_newer: u32,
    pub refused_presses: u32,
    /// Commands that came again, came after a newer one had run, or found the queue full: dropped, not refused.
    pub duplicates: u32,
    pub stale: u32,
    pub queue_full: u32,
    /// Commands run after their tick, and sequences that never ran.
    pub late: u32,
    pub missing: u32,
    pub commands_run: u32,
    pub snapshots_sent: u32,
}

impl RoomCounters {
    /// Everything the room refused: what an honest client over a clean link never causes.
    #[must_use]
    pub const fn refused(&self) -> u32 {
        let commands = self.refused_ahead.wrapping_add(self.refused_not_newer);
        let messages = self.decode_errors.wrapping_add(self.refused_ack);
        commands
            .wrapping_add(self.refused_presses)
            .wrapping_add(messages)
    }
}

fn count(counter: &mut u32) {
    *counter = counter.wrapping_add(1);
}

/// One match's server side: the world, the one player's slot and that player's commands.
pub struct Room {
    world: World,
    content_hash: [u8; 32],
    /// The world's tick when the one slot was taken.
    joined_at: Option<u32>,
    queue: CommandQueue,
    /// The message being read, reused for every datagram.
    input: Input,
    /// The last command that ran, the newest sequence that arrived, and the newest snapshot's tick.
    last_run: Option<Queued>,
    newest_received: Option<u16>,
    newest_sent: Option<u32>,
    counters: RoomCounters,
}

impl Room {
    /// A room for one match on the content with this hash.
    #[must_use]
    pub fn new(setup: &MatchSetup, content_hash: [u8; 32]) -> Self {
        let (acks, commands) = (Vec::new(), Vec::new());
        Self {
            world: World::new(setup),
            content_hash,
            joined_at: None,
            queue: CommandQueue::default(),
            input: Input { acks, commands },
            last_run: None,
            newest_received: None,
            newest_sent: None,
            counters: RoomCounters::default(),
        }
    }

    #[must_use]
    pub const fn world(&self) -> &World {
        &self.world
    }

    #[must_use]
    pub const fn counters(&self) -> &RoomCounters {
        &self.counters
    }

    /// Takes a message from the control channel and, when it is a `Join`, writes the answer into `reply` and
    /// returns true: accepted, or rejected for another protocol version, other content, or a slot already
    /// taken. Local play needs no token; the token's check comes with online play. When it returns false,
    /// `reply` holds nothing to send.
    #[must_use]
    pub fn on_control(&mut self, bytes: &[u8], reply: &mut Vec<u8>) -> bool {
        let Ok(Control::Join(join)) = decode_control(bytes) else {
            count(&mut self.counters.decode_errors);
            return false;
        };
        let refusal = if join.protocol_version != PROTOCOL_VERSION {
            Some(RejectReason::ProtocolVersion)
        } else if join.content_hash != self.content_hash {
            Some(RejectReason::ContentHash)
        } else {
            self.joined_at.map(|_| RejectReason::RoomFull)
        };
        let answer = if let Some(reason) = refusal {
            count(&mut self.counters.rejections);
            Control::JoinRejected(JoinRejected { reason })
        } else {
            count(&mut self.counters.joins);
            self.joined_at = Some(self.world.tick());
            Control::JoinAccepted(JoinAccepted {
                slot: 0,
                team: 0,
                squad: 0,
                mode_id: MODE_ID.to_owned(),
                mode_hash: [0; 32],
                content_hash: self.content_hash,
                server_tick: self.world.tick(),
                tick_rate_hz: TICK_RATE,
                snapshot_rate_hz: TICK_RATE,
            })
        };
        encode_control(&answer, reply).is_ok()
    }

    /// Takes a datagram: an `Input`. Its commands go to the queue, the oldest first.
    ///
    /// Checked here and in `admit`: the acks, the lead, a strictly later tick and the press pace. Not checked yet,
    /// each with what brings it: the length of the move and the aim inside the map (the soldier and the map,
    /// change-set 4); the view time (lag compensation); presses against cooldowns and ammunition (weapons); how
    /// many messages a client may send, the join's deadline and its token (the connection, Slice 3); a lower
    /// bound on an ack's tick; and how late a command may still run, so that a client cannot save ticks up and
    /// spend them two a tick (decided with the command timing, ADR 0001's open question 6, before online play).
    pub fn on_datagram(&mut self, bytes: &[u8]) {
        if self.joined_at.is_none() || decode_input(bytes, &mut self.input).is_err() {
            count(&mut self.counters.decode_errors);
            return;
        }
        // An ack names a snapshot the client decoded. One for a group that does not exist, or for a later tick
        // than the newest snapshot sent, is not true, and the message is dropped whole.
        let sent = |tick| {
            self.newest_sent
                .is_some_and(|sent| ticks_after(tick, sent).is_some())
        };
        if !self
            .input
            .acks
            .iter()
            .all(|ack| ack.group == 0 && sent(ack.tick))
        {
            count(&mut self.counters.refused_ack);
            return;
        }
        let running = self.world.tick().saturating_add(1);
        for command in self.input.commands.iter().rev() {
            let sequence = command.sequence;
            if self
                .newest_received
                .is_none_or(|newest| seq_newer(sequence, newest))
            {
                self.newest_received = Some(sequence);
            }
            if ticks_after(running, command.tick).is_some_and(|lead| lead > LEAD_MAX_TICKS) {
                count(&mut self.counters.refused_ahead);
                continue;
            }
            let queued = Queued {
                sequence,
                tick: command.tick,
                command: PlayerCommand {
                    buttons: command.buttons,
                    presses: command.presses,
                },
            };
            match self.queue.insert(queued) {
                Inserted::Queued => {}
                Inserted::Duplicate => count(&mut self.counters.duplicates),
                Inserted::Stale => count(&mut self.counters.stale),
                Inserted::Full => count(&mut self.counters.queue_full),
            }
        }
    }

    /// Whether a command that is due may run: its tick lies after the tick of the command run before it (before
    /// the first, after the tick of the join), and no press counter moved by more than one press per tick in
    /// between. Counts what it refuses. (That its sequence is newer than the last command's is the queue's rule.)
    fn admit(&mut self, next: &Queued) -> bool {
        let (base_tick, base) = match &self.last_run {
            Some(last) => (last.tick, last.command.presses),
            None => (self.joined_at.unwrap_or(0), [0; BUTTONS]),
        };
        let Some(covered) = ticks_after(base_tick, next.tick).filter(|covered| *covered > 0) else {
            count(&mut self.counters.refused_not_newer);
            return false;
        };
        let mut moved = base.iter().zip(next.command.presses);
        let slow = |(old, new): (&u8, u8)| u32::from(press_edges(*old, new)) <= covered;
        if !moved.all(slow) {
            count(&mut self.counters.refused_presses);
            return false;
        }
        true
    }

    /// Runs one tick: up to `CATCH_UP_CAP` due commands that pass the checks, then the world's step. With a player
    /// in the slot it writes that player's snapshot of the new tick into `snapshot` and returns true; when it
    /// returns false, `snapshot` holds nothing to send.
    #[must_use]
    pub fn step(&mut self, snapshot: &mut Vec<u8>) -> bool {
        let running = self.world.tick().saturating_add(1);
        let mut run = [PlayerCommand::NONE; CATCH_UP_CAP];
        let mut ran = 0;
        while let Some(place) = run.get_mut(ran)
            && let Some(next) = self.queue.take_due(running)
        {
            if !self.admit(&next) {
                continue;
            }
            let skipped = self
                .last_run
                .and_then(|last| seq_ahead(last.sequence, next.sequence));
            let skipped = u32::from(skipped.unwrap_or(1)).saturating_sub(1);
            self.counters.missing = self.counters.missing.wrapping_add(skipped);
            if ticks_after(next.tick, running).is_some_and(|late| late > 0) {
                count(&mut self.counters.late);
            }
            count(&mut self.counters.commands_run);
            self.queue.ran();
            *place = next.command;
            self.last_run = Some(next);
            ran = ran.saturating_add(1);
        }
        self.world.authority_step(run.get(..ran).unwrap_or(&[]));
        if self.joined_at.is_none() {
            return false;
        }
        let player = self.world.player();
        let message = Snapshot {
            server_tick: self.world.tick(),
            baseline_tick: None,
            newest_command_received: self.newest_received,
            last_processed_command: self.last_run.map(|last| last.sequence),
            group: 0,
            own: Some(OwnState {
                buttons: player.buttons,
                presses: player.presses,
            }),
        };
        self.newest_sent = Some(self.world.tick());
        count(&mut self.counters.snapshots_sent);
        encode_snapshot(&message, snapshot).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::codec::{decode_snapshot, encode_input};
    use protocol::messages::{AimPoint, Command, Join, SnapshotAck, ViewTime};

    const HASH: [u8; 32] = [0xC4; 32];

    fn room(start_tick: u32) -> Room {
        let seed = [7; sim::SEED_BYTES];
        Room::new(&MatchSetup { seed, start_tick }, HASH)
    }

    fn join(protocol_version: u16, content_hash: [u8; 32]) -> Vec<u8> {
        let join = Join {
            token: Vec::new(),
            protocol_version,
            content_hash,
        };
        let mut bytes = Vec::new();
        encode_control(&Control::Join(join), &mut bytes).expect("it has a wire form");
        bytes
    }

    /// The room's answer to these control bytes, decoded; none when it wrote none.
    fn answer(room: &mut Room, bytes: &[u8]) -> Option<Control> {
        let mut reply = Vec::new();
        let replied = room.on_control(bytes, &mut reply);
        replied.then(|| decode_control(&reply).expect("the room writes a valid answer"))
    }

    /// A room at `start_tick` with its player joined.
    fn joined(start_tick: u32) -> Room {
        let mut room = room(start_tick);
        assert!(matches!(
            answer(&mut room, &join(PROTOCOL_VERSION, HASH)),
            Some(Control::JoinAccepted(_))
        ));
        room
    }

    /// A command whose buttons are its sequence and whose every press counter is `presses`.
    fn command(sequence: u16, tick: u32, presses: u8) -> Command {
        Command {
            tick,
            sequence,
            move_x: 0,
            move_y: 0,
            buttons: sequence,
            presses: [presses; BUTTONS],
            aim: AimPoint { x: 0, y: 0, z: 0 },
            view_time: ViewTime {
                tick: 0,
                fraction: 0,
            },
        }
    }

    /// Hands the room an `Input` with these commands (the newest first) and this ack, if any.
    fn offer(room: &mut Room, commands: &[Command], acked: Option<u32>) {
        let ack = |tick| SnapshotAck { group: 0, tick };
        let input = Input {
            acks: acked.into_iter().map(ack).collect(),
            commands: commands.to_vec(),
        };
        let mut bytes = Vec::new();
        encode_input(&input, &mut bytes).expect("it has a wire form");
        room.on_datagram(&bytes);
    }

    /// Runs one tick and returns the snapshot it wrote.
    fn step(room: &mut Room) -> Snapshot {
        let mut bytes = Vec::new();
        assert!(room.step(&mut bytes));
        decode_snapshot(&bytes).expect("the room writes a valid snapshot")
    }

    #[test]
    fn a_join_is_accepted_once_and_refused_by_name_otherwise() {
        let mut room = room(40);
        let mut bytes = Vec::new();
        assert!(
            !room.step(&mut bytes),
            "no snapshot while nobody has joined"
        );
        let rejected = |reason| Some(Control::JoinRejected(JoinRejected { reason }));
        assert_eq!(
            answer(&mut room, &join(2, HASH)),
            rejected(RejectReason::ProtocolVersion)
        );
        assert_eq!(
            answer(&mut room, &join(PROTOCOL_VERSION, [0; 32])),
            rejected(RejectReason::ContentHash)
        );
        let Some(Control::JoinAccepted(accepted)) =
            answer(&mut room, &join(PROTOCOL_VERSION, HASH))
        else {
            panic!("the join is accepted");
        };
        assert_eq!(
            (
                accepted.slot,
                accepted.mode_id.as_str(),
                accepted.server_tick
            ),
            (0, MODE_ID, 41)
        );
        assert_eq!((accepted.tick_rate_hz, accepted.content_hash), (30, HASH));
        assert_eq!(
            answer(&mut room, &join(PROTOCOL_VERSION, HASH)),
            rejected(RejectReason::RoomFull)
        );
        // Bytes that are no join get no answer.
        assert_eq!(answer(&mut room, &[0xFF, 1]), None);
        let mut reply = Vec::new();
        encode_control(&rejected(RejectReason::RoomFull).expect("some"), &mut reply).expect("ok");
        assert_eq!(answer(&mut room, &reply), None);
        let counters = *room.counters();
        assert_eq!(
            (counters.joins, counters.rejections, counters.decode_errors),
            (1, 3, 2)
        );
    }

    #[test]
    fn a_command_runs_at_its_tick_and_the_snapshot_echoes_what_was_applied() {
        let mut room = joined(40);
        let first = step(&mut room);
        assert_eq!(
            (first.server_tick, first.group, first.baseline_tick),
            (41, 0, None)
        );
        assert_eq!(
            (first.newest_command_received, first.last_processed_command),
            (None, None)
        );
        let none = OwnState {
            buttons: 0,
            presses: [0; BUTTONS],
        };
        assert_eq!(first.own, Some(none));
        // The command for tick 42 arrives before the room runs tick 42, acking the snapshot of tick 41.
        offer(&mut room, &[command(9, 42, 1)], Some(41));
        let second = step(&mut room);
        assert_eq!(second.server_tick, 42);
        assert_eq!(
            (
                second.newest_command_received,
                second.last_processed_command
            ),
            (Some(9), Some(9))
        );
        let applied = OwnState {
            buttons: 9,
            presses: [1; BUTTONS],
        };
        assert_eq!(second.own, Some(applied.clone()));
        assert_eq!(room.world().player().buttons, 9);
        // No command for tick 43: the record holds, and the snapshot says so.
        assert_eq!(step(&mut room).own, Some(applied));
        let counters = *room.counters();
        assert_eq!(
            (
                counters.commands_run,
                counters.snapshots_sent,
                counters.late
            ),
            (1, 3, 0)
        );
        assert_eq!(counters.refused(), 0);
    }

    #[test]
    fn repeats_reordering_and_gaps_never_run_a_command_twice_or_out_of_order() {
        let mut room = joined(0);
        step(&mut room);
        // Commands 0 and 1 arrive together, the newest first as the wire has them; 1 is for the next tick.
        offer(&mut room, &[command(1, 3, 0), command(0, 2, 0)], None);
        assert_eq!(step(&mut room).last_processed_command, Some(0));
        // The same input again, late: 0 is stale, 1 already waits.
        offer(&mut room, &[command(1, 3, 0), command(0, 2, 0)], None);
        assert_eq!(step(&mut room).last_processed_command, Some(1));
        // 2 and 3 never arrive, and 4 and 5 arrive late, 5 before 4: both run in one tick, in order.
        for _ in 4..=7 {
            assert_eq!(step(&mut room).last_processed_command, Some(1));
        }
        offer(&mut room, &[command(5, 7, 0)], None);
        offer(&mut room, &[command(4, 6, 0)], None);
        let caught_up = step(&mut room);
        let ran = (caught_up.server_tick, caught_up.last_processed_command);
        assert_eq!(ran, (8, Some(5)));
        assert_eq!(caught_up.newest_command_received, Some(5));
        assert_eq!(room.world().player().buttons, 5);
        let counters = *room.counters();
        assert_eq!(
            (counters.commands_run, counters.duplicates, counters.stale),
            (4, 1, 1)
        );
        let gaps = (counters.missing, counters.late, counters.refused());
        assert_eq!(gaps, (2, 2, 0));
    }

    /// A refused command does not shut out the honest ones behind it, and no sequence comes round the circle
    /// behind it. The room runs sequence 10; a command 30,000 ahead is taken out and refused (its tick is not
    /// later). Sequence 11 still runs. A command 40,000 ahead of the last one that ran is stale. The refused one
    /// again is refused again.
    #[test]
    fn a_command_that_runs_is_newer_than_the_last_one_that_ran_whatever_was_refused() {
        let mut room = joined(100);
        offer(&mut room, &[command(10, 101, 0)], None);
        assert_eq!(step(&mut room).last_processed_command, Some(10));
        offer(&mut room, &[command(30_010, 101, 0)], None);
        assert_eq!(step(&mut room).last_processed_command, Some(10));
        offer(&mut room, &[command(11, 103, 0)], None);
        assert_eq!(step(&mut room).last_processed_command, Some(11));
        offer(&mut room, &[command(40_011, 104, 0)], None);
        offer(&mut room, &[command(30_010, 103, 0)], None);
        assert_eq!(step(&mut room).last_processed_command, Some(11));
        let counters = *room.counters();
        let dropped = (counters.refused_not_newer, counters.stale);
        assert_eq!(
            (dropped, counters.commands_run, counters.missing),
            ((2, 1), 2, 0)
        );
        // A command inside the window runs, and the count of missing sequences is the distance it skipped.
        offer(&mut room, &[command(30_011, 105, 0)], None);
        assert_eq!(step(&mut room).last_processed_command, Some(30_011));
        assert_eq!(room.counters().missing, 29_999);
    }

    #[test]
    fn at_most_two_commands_run_in_one_tick_and_the_rest_the_next() {
        let mut room = joined(0);
        let ticks = [5, 4, 3, 2, 1].map(|tick| command(tick, u32::from(tick), 0));
        for _ in 0..5 {
            step(&mut room);
        }
        offer(&mut room, &ticks, None);
        assert_eq!(step(&mut room).last_processed_command, Some(2));
        assert_eq!(step(&mut room).last_processed_command, Some(4));
        assert_eq!(step(&mut room).last_processed_command, Some(5));
        assert_eq!((room.counters().commands_run, room.counters().late), (5, 5));
    }

    #[test]
    fn each_check_drops_its_command_or_message_and_counts_it() {
        let mut room = joined(100);
        step(&mut room);
        // The room runs tick 102 next: a command for tick 110 is 8 ahead and waits; one for 111 is dropped.
        offer(&mut room, &[command(0, 111, 0)], None);
        assert_eq!(room.counters().refused_ahead, 1);
        // An ack for a later tick than the newest snapshot (101), or for another group, drops the whole input.
        offer(&mut room, &[command(1, 102, 0)], Some(102));
        let mut bytes = Vec::new();
        let other_group = Input {
            acks: vec![SnapshotAck {
                group: 1,
                tick: 101,
            }],
            commands: vec![command(1, 102, 0)],
        };
        encode_input(&other_group, &mut bytes).expect("it has a wire form");
        room.on_datagram(&bytes);
        assert_eq!(room.counters().refused_ack, 2);
        room.on_datagram(&[0x10, 9]);
        assert_eq!(room.counters().decode_errors, 1);
        assert_eq!(step(&mut room).last_processed_command, None);
        // One press per tick since the join at tick 100: by tick 103, three presses are honest and four are not.
        offer(&mut room, &[command(1, 103, 4)], Some(102));
        assert_eq!(step(&mut room).last_processed_command, None);
        assert_eq!(room.counters().refused_presses, 1);
        offer(&mut room, &[command(2, 104, 4)], Some(103));
        assert_eq!(step(&mut room).last_processed_command, Some(2));
        // A newer sequence for a tick that is not later than the last run command's: dropped. A counter that
        // moves back reads as 255 presses: dropped.
        offer(&mut room, &[command(3, 104, 4)], None);
        offer(&mut room, &[command(4, 105, 3)], None);
        assert_eq!(step(&mut room).last_processed_command, Some(2));
        let counters = *room.counters();
        assert_eq!(
            (counters.refused_not_newer, counters.refused_presses),
            (1, 2)
        );
        assert_eq!((counters.refused(), counters.commands_run), (7, 1));
        // What was refused is still received: the client stops sending it again.
        assert_eq!(step(&mut room).newest_command_received, Some(4));
        // Before the join no datagram is taken.
        let mut empty = super::tests::room(0);
        offer(&mut empty, &[command(0, 1, 0)], None);
        assert_eq!(empty.counters().decode_errors, 1);
    }

    #[test]
    fn the_sequence_wraps_and_the_tick_starts_anywhere() {
        let mut room = joined(131_069);
        let mut tick = step(&mut room).server_tick;
        for sequence in [65_534, 65_535, 0, 1] {
            tick += 1;
            offer(&mut room, &[command(sequence, tick, 0)], Some(tick - 1));
            let snapshot = step(&mut room);
            assert_eq!(
                (snapshot.server_tick, snapshot.last_processed_command),
                (tick, Some(sequence))
            );
        }
        assert_eq!(tick, 131_074);
        assert_eq!(
            (room.counters().commands_run, room.counters().refused()),
            (4, 0)
        );
    }

    /// A thousand ticks with a command each: the room's reused buffers and the caller's stay where they are.
    #[test]
    fn a_thousand_ticks_move_no_buffer() {
        let mut room = joined(0);
        let (mut datagram, mut snapshot) = (Vec::with_capacity(512), Vec::with_capacity(64));
        let mut input = Input {
            acks: Vec::with_capacity(1),
            commands: Vec::with_capacity(1),
        };
        assert!(room.step(&mut snapshot));
        let warm = |room: &Room| (room.input.acks.as_ptr(), room.input.commands.as_ptr());
        let mut places = None;
        for tick in 2..1002u16 {
            input.commands.clear();
            input.commands.push(command(tick, u32::from(tick), 0));
            encode_input(&input, &mut datagram).expect("it has a wire form");
            room.on_datagram(&datagram);
            assert!(room.step(&mut snapshot));
            let now = (
                warm(&room),
                datagram.as_ptr(),
                snapshot.as_ptr(),
                snapshot.capacity(),
            );
            assert_eq!(*places.get_or_insert(now), now);
        }
        assert_eq!(room.counters().commands_run, 1000);
    }
}
