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
    AimPoint, Input, JoinAccepted, JoinRejected, OwnState, PROTOCOL_VERSION, RejectReason, Snapshot,
};
use protocol::wrap::{press_edges, seq_ahead, seq_newer, ticks_after};
use queue::{CommandQueue, Inserted, Queued};
use sim::aim::AimCm;
use sim::collision::StaticWorld;
use sim::rules::Rules;
use sim::{
    BUTTONS, MatchSetup, PLAYER_SLOT, PLAYER_TEAM, PlayerCommand, SpawnPoint, TICK_HZ, World,
    move_in_reach,
};

const _: () = assert!(protocol::messages::BUTTONS == BUTTONS);
// "No aim yet" has one form, on the wire and in the simulation.
const _: () = assert!(
    AimPoint::NONE.x == AimCm::NONE.x
        && AimPoint::NONE.y == AimCm::NONE.y
        && AimPoint::NONE.z == AimCm::NONE.z
);

/// The tick rate as the join answer carries it.
const TICK_RATE: u8 = {
    assert!(TICK_HZ <= 255);
    TICK_HZ as u8
};

/// The one player's slot as the join answer carries it: the world's own (`sim::PLAYER_SLOT`), which must fit a byte.
const JOIN_SLOT: u8 = {
    let [low, high] = PLAYER_SLOT.to_le_bytes();
    assert!(high == 0, "the join answer carries a slot in one byte");
    low
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
    /// counter that moved by more than one press per tick since the command run before; a move longer than the
    /// wire's reach (`sim::MOVE_REACH`); an aim that is not "no aim yet" and lies outside the map's bounds.
    pub refused_ahead: u32,
    pub refused_not_newer: u32,
    pub refused_presses: u32,
    pub refused_move: u32,
    pub refused_aim: u32,
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
        let wishes = self.refused_move.wrapping_add(self.refused_aim);
        let messages = self.decode_errors.wrapping_add(self.refused_ack);
        commands
            .wrapping_add(self.refused_presses)
            .wrapping_add(wishes)
            .wrapping_add(messages)
    }
}

fn count(counter: &mut u32) {
    *counter = counter.wrapping_add(1);
}

/// One match's server side: the world, the one player's slot and that player's commands.
pub struct Room {
    world: World,
    /// The match's static world, built once from its map: the aim check reads its bounds.
    statics: StaticWorld,
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
    /// How many commands the last tick handed the world, for the room's own tests: a tick whose queue gives nothing
    /// due hands none, and no older command is fed again.
    #[cfg(test)]
    handed: usize,
}

impl Room {
    /// A room for one match on the content with this hash, with the match's rules, its static world and its spawn
    /// table. The world's start reads the rules and the table (his soldier at the first spawn); the room keeps the
    /// static world and neither of the others.
    #[must_use]
    pub fn new(
        setup: &MatchSetup,
        content_hash: [u8; 32],
        rules: &Rules,
        statics: StaticWorld,
        spawns: &[SpawnPoint],
    ) -> Self {
        let (acks, commands) = (Vec::new(), Vec::new());
        Self {
            world: World::new(setup, rules, spawns),
            statics,
            content_hash,
            joined_at: None,
            queue: CommandQueue::default(),
            input: Input { acks, commands },
            last_run: None,
            newest_received: None,
            newest_sent: None,
            counters: RoomCounters::default(),
            #[cfg(test)]
            handed: 0,
        }
    }

    #[must_use]
    pub const fn world(&self) -> &World {
        &self.world
    }

    /// The match's static world: the map's shapes, and its bounds in whole centimetres.
    #[must_use]
    pub const fn statics(&self) -> &StaticWorld {
        &self.statics
    }

    /// The hash of the world's shared state as it stands: the one number a replay compares tick by tick, the same
    /// on every target. It covers no seed byte, changes nothing and takes no memory.
    #[must_use]
    pub fn shared_hash(&self) -> u64 {
        self.world.shared_hash()
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
                slot: JOIN_SLOT,
                team: PLAYER_TEAM,
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
    /// Checked here and in `admit`: the acks, the lead, the move's length (at most `sim::MOVE_REACH`), the aim
    /// inside the map's bounds in whole centimetres or "no aim yet", a strictly later tick and the press pace. Not
    /// checked yet, each with what brings it: the aim's reach against a weapon (change-set 8); the view time (lag
    /// compensation); presses against cooldowns and ammunition (weapons); how many messages a client may send, the
    /// join's deadline and its token (the connection, Slice 3); a lower bound on an ack's tick; and how late a
    /// command may still run, so that a client cannot save ticks up and spend them two a tick (decided with the
    /// command timing, ADR 0001's open question 6, before online play).
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
            if !move_in_reach(command.move_x, command.move_y) {
                count(&mut self.counters.refused_move);
                continue;
            }
            let aim = AimCm {
                x: command.aim.x,
                y: command.aim.y,
                z: command.aim.z,
            };
            // "No aim yet" asks no bounds; any other aim lies inside the map's, the faces included.
            if aim != AimCm::NONE && !self.statics.holds_cm(aim) {
                count(&mut self.counters.refused_aim);
                continue;
            }
            let queued = Queued {
                sequence,
                tick: command.tick,
                command: PlayerCommand {
                    buttons: command.buttons,
                    presses: command.presses,
                    move_x: command.move_x,
                    move_y: command.move_y,
                    aim,
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
        let handed = run.get(..ran).unwrap_or(&[]);
        #[cfg(test)]
        {
            self.handed = handed.len();
        }
        self.world.authority_step(handed);
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
    use content::map::{Bounds, MapDocument, Spawn};
    use protocol::codec::{decode_snapshot, encode_input};
    use protocol::messages::{Command, Join, REDUNDANCY_CAP, SnapshotAck, ViewTime};
    use sim::soldiers::{OCCUPIED, Soldier, spawn_points};

    const HASH: [u8; 32] = [0xC4; 32];

    /// The test bundle's rules and its map (content's made-up test texts: bounds -50 to 50 m on x and z and 0 to 20 m
    /// up, one spawn "a" at (0, 0, -5) facing 90).
    fn test_bundle() -> (Rules, MapDocument) {
        let set = content::load(content::bundle::test_bundle().as_bytes())
            .expect("the test bundle loads");
        let rules = Rules::take(&set.movements()[0]).expect("its movement table plays");
        (rules, set.map("test-scene").expect("its map").clone())
    }

    /// A room for one match on this map, with the test bundle's rules.
    fn room_on(setup: &MatchSetup, map: &MapDocument) -> Room {
        let (rules, _) = test_bundle();
        let statics = StaticWorld::build(map).expect("the map builds");
        Room::new(setup, HASH, &rules, statics, &spawn_points(map))
    }

    fn room(start_tick: u32) -> Room {
        let seed = [7; sim::SEED_BYTES];
        room_on(&MatchSetup { seed, start_tick }, &test_bundle().1)
    }

    /// A map built in code with these bounds and one spawn "a" at `spawn`, facing 0, and no shape.
    fn bounded(min: [f64; 3], max: [f64; 3], spawn: [f64; 3]) -> MapDocument {
        MapDocument {
            id: "bounded".to_owned(),
            bounds: Bounds { min, max },
            spawns: vec![Spawn {
                id: "a".to_owned(),
                at: spawn,
                facing_deg: 0.0,
            }],
            shapes: Vec::new(),
            regions: Vec::new(),
        }
    }

    /// The player's soldier's row.
    fn row(room: &Room) -> Soldier {
        room.world().soldiers().rows()[usize::from(PLAYER_SLOT)]
    }

    /// The same command with this move and this aim.
    fn wishing(mut command: Command, move_x: i8, move_y: i8, aim: AimPoint) -> Command {
        (command.move_x, command.move_y, command.aim) = (move_x, move_y, aim);
        command
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

    /// The room with its player joined.
    fn with_player(mut room: Room) -> Room {
        assert!(matches!(
            answer(&mut room, &join(PROTOCOL_VERSION, HASH)),
            Some(Control::JoinAccepted(_))
        ));
        room
    }

    /// A room at `start_tick` with its player joined.
    fn joined(start_tick: u32) -> Room {
        with_player(room(start_tick))
    }

    /// A room on this map at tick 0 with its player joined.
    fn joined_on(map: &MapDocument) -> Room {
        let seed = [7; sim::SEED_BYTES];
        with_player(room_on(
            &MatchSetup {
                seed,
                start_tick: 0,
            },
            map,
        ))
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

    /// The room's shared hash is the world's own, read through a shared borrow: two rooms whose seeds differ in every
    /// byte agree on it tick by tick, and differ on the whole state's hash.
    #[test]
    fn the_shared_hash_is_the_world_s_and_covers_no_seed_byte() {
        let seeds = [[0x5A; sim::SEED_BYTES], [0xA5; sim::SEED_BYTES]];
        let map = test_bundle().1;
        let [mut a, mut b] = seeds.map(|seed| {
            let setup = MatchSetup {
                seed,
                start_tick: 3,
            };
            room_on(&setup, &map)
        });
        let mut snapshot = Vec::new();
        for _ in 0..40 {
            let (shared, read_only): (u64, &Room) = (a.shared_hash(), &a);
            assert_eq!(shared, read_only.shared_hash());
            assert_eq!(shared, b.shared_hash());
            assert_eq!(shared, a.world().shared_hash());
            assert_ne!(a.world().state_hash(), b.world().state_hash());
            assert!(!a.step(&mut snapshot));
            assert!(!b.step(&mut snapshot));
        }
        assert_eq!(a.world().tick(), 43);
    }

    /// The move's length and the aim inside the map (netcode-architect's T2-6): a move up to the wire's reach is
    /// taken and a longer one refused; an aim on a face of the map's bounds is taken and one 1 cm past it refused;
    /// each refusal is counted by its own counter and in `refused()`. A refused command never runs, and its sequence
    /// counts as missing when the next command runs.
    #[test]
    fn a_move_past_the_reach_and_an_aim_outside_the_map_are_refused_and_counted() {
        let mut room = joined(0);
        step(&mut room);
        let aim = |x, y, z| AimPoint { x, y, z };
        let none = AimPoint::NONE;
        // The test bundle's bounds in whole centimetres: -5,000 to 5,000 on x and z, 0 to 2,000 up.
        #[rustfmt::skip]
        let cases = [
            (127, 0, none.clone(), true), (90, 90, none.clone(), false), (0, -127, none.clone(), true),
            (-128, 0, none.clone(), false), (89, 89, none.clone(), true), (127, 127, none.clone(), false),
            (-89, -89, none.clone(), true),
            (0, 0, aim(-5_000, 0, 0), true), (0, 0, aim(-5_001, 0, 0), false),
            (0, 0, aim(5_000, 2_000, 5_000), true), (0, 0, aim(5_001, 0, 0), false), (0, 0, aim(0, 2_001, 0), false),
            (0, 0, aim(0, 0, -5_000), true), (0, 0, aim(0, 0, -5_001), false), (0, 0, aim(0, 0, 5_001), false),
            (0, 0, aim(i16::MIN, 1, 0), false), (0, 0, none, true),
        ];
        let (mut moves, mut aims, mut held) = (0, 0, AimCm::NONE);
        for (at, (move_x, move_y, aim, taken)) in cases.into_iter().enumerate() {
            let (sequence, tick) = (at as u16, 2 + at as u32);
            let sent = wishing(command(sequence, tick, 0), move_x, move_y, aim.clone());
            offer(&mut room, &[sent], None);
            let ran = step(&mut room).last_processed_command == Some(sequence);
            assert_eq!(ran, taken, "{move_x} {move_y} {aim:?}");
            let real = AimCm {
                x: aim.x,
                y: aim.y,
                z: aim.z,
            };
            if !taken && sim::move_in_reach(move_x, move_y) {
                aims += 1;
            } else if !taken {
                moves += 1;
            } else {
                assert_eq!(row(&room).held_move, [move_x, move_y]);
                if real != AimCm::NONE {
                    held = real;
                }
            }
            assert_eq!(row(&room).aim_cm, held);
        }
        let counters = *room.counters();
        assert_eq!((counters.refused_move, counters.refused_aim), (3, 6));
        assert_eq!((moves, aims), (3, 6));
        assert_eq!(counters.refused(), 9);
        // Each refused command's sequence was skipped by the next one that ran.
        assert_eq!((counters.missing, counters.commands_run), (9, 8));
        assert_eq!(row(&room).held_move, [0, 0]);
    }

    /// "No aim yet" (netcode-architect's N-1): on a map whose bounds leave the origin out, a command that carries it
    /// is never refused, and an aim with x -32,768 and any other y or z is refused and counted. A tick that runs an
    /// aim A and then "no aim yet" leaves the row on A, and a later "no aim yet" alone keeps it.
    #[test]
    fn no_aim_yet_is_taken_on_any_map_and_keeps_the_aim_the_row_holds() {
        let map = bounded([10.0, 0.0, 10.0], [20.0, 5.0, 20.0], [15.0, 0.0, 15.0]);
        let mut room = joined_on(&map);
        assert!(!room.statics().holds_cm(AimCm { x: 0, y: 0, z: 0 }));
        for _ in 0..4 {
            step(&mut room);
        }
        // Tick 5 runs two commands: 0 (late, for tick 4) aims at A, then 1 carries no aim.
        let (a, held) = (
            AimPoint {
                x: 1_500,
                y: 100,
                z: 1_600,
            },
            AimCm {
                x: 1_500,
                y: 100,
                z: 1_600,
            },
        );
        let none = |sequence, tick| wishing(command(sequence, tick, 0), 0, 0, AimPoint::NONE);
        offer(
            &mut room,
            &[none(1, 5), wishing(command(0, 4, 0), 0, 0, a)],
            None,
        );
        assert_eq!(step(&mut room).last_processed_command, Some(1));
        assert_eq!(row(&room).aim_cm, held);
        offer(&mut room, &[none(2, 6)], None);
        assert_eq!(step(&mut room).last_processed_command, Some(2));
        assert_eq!(row(&room).aim_cm, held);
        // Two aims with the least x that are not "no aim yet": refused, counted, never run.
        for (sequence, tick, y, z) in [(3, 7, 1, 0), (4, 8, 0, 5)] {
            let malformed = AimPoint { x: i16::MIN, y, z };
            let sent = wishing(command(sequence, tick, 0), 0, 0, malformed);
            offer(&mut room, &[sent], None);
            assert_eq!(step(&mut room).last_processed_command, Some(2));
        }
        let counters = *room.counters();
        let counted = (counters.refused_aim, counters.refused());
        assert_eq!((counted, counters.commands_run), ((2, 2), 3));
        assert_eq!(row(&room).aim_cm, held);
    }

    /// His soldier exists from the room's start, before anyone joins, and stands (netcode-architect's W-3, the half
    /// of it before the movement): ten ticks with no join leave his row as the start made it, the join's slot and
    /// team are that row's, and thirty inputs with the move (127, 0) and no aim reach the row's held move and leave
    /// its aim as it was.
    #[test]
    fn his_soldier_stands_before_the_join_and_the_wire_s_move_reaches_his_row() {
        let mut room = room(0);
        let start = row(&room);
        assert_ne!(start.flags & OCCUPIED, 0);
        let mut bytes = Vec::new();
        for _ in 0..10 {
            assert!(!room.step(&mut bytes));
        }
        assert_eq!(row(&room), start);
        let Some(Control::JoinAccepted(accepted)) =
            answer(&mut room, &join(PROTOCOL_VERSION, HASH))
        else {
            panic!("the join is accepted");
        };
        let occupied = |soldier: &Soldier| soldier.flags & OCCUPIED != 0;
        let rows = room.world().soldiers().rows();
        assert_eq!(
            rows.iter().position(occupied),
            Some(usize::from(accepted.slot))
        );
        assert_eq!(rows.iter().filter(|soldier| occupied(soldier)).count(), 1);
        assert_eq!(
            (u16::from(accepted.slot), accepted.team),
            (PLAYER_SLOT, PLAYER_TEAM)
        );
        for at in 0..30u16 {
            let tick = 11 + u32::from(at);
            let sent = wishing(command(at, tick, 0), 127, 0, AimPoint::NONE);
            offer(&mut room, &[sent], None);
            step(&mut room);
        }
        let now = row(&room);
        assert_eq!((now.held_move, now.aim_cm), ([127, 0], AimCm::NONE));
        assert_eq!(room.counters().commands_run, 30);
        assert_eq!((now.position, now.facing), (start.position, start.facing));
    }

    /// The room's pin (netcode-architect's 7.2): commands for ticks 1 to 30 with the move (127, 0), then the
    /// datagrams for ticks 31 to 33 lost, so nothing is due at those ticks, and tick 34's datagram repeats them late.
    /// At the starved ticks the world is handed no command at all (no older command fed again) and his row keeps
    /// its held move; when the late ones run, `late` counts them and the newest's move is taken.
    #[test]
    fn a_starved_tick_hands_the_world_no_command_and_the_row_keeps_the_held_move() {
        let mut room = joined(0);
        let mut sent: Vec<Command> = Vec::new();
        for tick in 1..=34u32 {
            let move_x = if tick == 34 { -127 } else { 127 };
            let sequence = tick as u16;
            sent.insert(
                0,
                wishing(command(sequence, tick, 0), move_x, 0, AimPoint::NONE),
            );
            sent.truncate(REDUNDANCY_CAP);
            if !(31..=33).contains(&tick) {
                offer(&mut room, &sent, None);
            }
            let ran_before = room.counters().commands_run;
            step(&mut room);
            let ran = room.counters().commands_run - ran_before;
            let expected = match tick {
                31..=33 => 0,
                34 => 2,
                _ => 1,
            };
            assert_eq!(
                (room.handed, ran as usize),
                (expected, expected),
                "tick {tick}"
            );
            assert_eq!(row(&room).held_move, [127, 0], "tick {tick}");
        }
        // Tick 35 runs the two left, 33 and then 34: the newest's move is the row's.
        step(&mut room);
        assert_eq!(room.handed, 2);
        assert_eq!(row(&room).held_move, [-127, 0]);
        let counters = *room.counters();
        assert_eq!(
            (counters.late, counters.missing, counters.refused()),
            (4, 0, 0)
        );
        assert_eq!(counters.commands_run, 34);
    }

    /// A world built in code wholly past the aim's reach on x (engine-architect's 6.2): the room still starts, its
    /// static world holds no box for the aim, every real aim is refused and counted, and "no aim yet" runs.
    #[test]
    fn on_a_world_past_the_aim_s_reach_every_real_aim_is_refused_and_no_aim_yet_runs() {
        let map = bounded([400.0, 0.0, 0.0], [500.0, 10.0, 10.0], [450.0, 0.0, 5.0]);
        let mut room = joined_on(&map);
        assert_eq!(room.statics().bounds_cm(), None);
        assert_ne!(row(&room).flags & OCCUPIED, 0);
        step(&mut room);
        let aims = [
            AimPoint {
                x: 32_767,
                y: 0,
                z: 5,
            },
            AimPoint { x: 0, y: 0, z: 0 },
            AimPoint {
                x: -32_767,
                y: 65_535,
                z: 32_767,
            },
        ];
        for (at, aim) in aims.into_iter().enumerate() {
            let sent = wishing(command(at as u16, 2 + at as u32, 0), 0, 0, aim);
            offer(&mut room, &[sent], None);
            assert_eq!(step(&mut room).last_processed_command, None);
        }
        let none = wishing(command(3, 5, 0), 0, 0, AimPoint::NONE);
        offer(&mut room, &[none], None);
        assert_eq!(step(&mut room).last_processed_command, Some(3));
        let counters = *room.counters();
        assert_eq!((counters.refused_aim, counters.commands_run), (3, 1));
    }
}
