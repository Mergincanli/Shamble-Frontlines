//! The client endpoint's side of the message exchange (ADR 0001 parts 5 and 6; ADR 0002 part 6): it asks to join,
//! builds each tick's command from the page's newest post, sends every command the room has not confirmed, and
//! reads the room's snapshots. It moves no bytes itself: whoever holds the transport hands it what arrived and
//! sends what it wrote. It never panics on bytes from the link, allocates nothing per tick, and reads no clock.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::arithmetic_side_effects
    )
)]

use protocol::codec::{
    Control, DecodeError, decode_control, decode_snapshot, encode_control, encode_input,
};
use protocol::messages::{
    AimPoint, Command, Input, Join, PROTOCOL_VERSION, REDUNDANCY_CAP, RejectReason, SnapshotAck,
    ViewTime,
};
use protocol::wrap::{press_edges, seq_newer, ticks_after};
use sim::{BUTTONS, TICK_HZ};

use crate::post::Post;
use crate::press::PressPacer;

/// How many sent commands the endpoint remembers, to check the room's echo against: a power of two, and more than
/// the room can lag behind.
pub const SENT_RING: usize = 64;
const RING_MASK: u16 = 63;
const _: () = assert!(SENT_RING == RING_MASK as usize + 1 && SENT_RING > 2 * REDUNDANCY_CAP);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointState {
    /// The join is asked and not answered.
    Joining,
    Joined,
    Rejected(RejectReason),
}

/// Why a control message was not taken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointError {
    Decode(DecodeError),
    /// A control message that is not the answer to a join that waits for one.
    Unexpected,
    /// The room ticks at another rate than this build.
    TickRate(u8),
    /// The room runs other content than this client loaded.
    ContentHash,
}

/// What the endpoint knows as of the newest snapshot, for the block and for tests. Each count wraps.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Readout {
    /// The newest snapshot's tick.
    pub tick: u32,
    /// The input the room last applied for this player: the buttons it holds as held and its press counters.
    pub echo_buttons: u16,
    pub echo_presses: [u8; BUTTONS],
    /// The number of the post from which the last command the room ran was built.
    pub post_used: u32,
    /// `Input` messages written, and snapshots taken.
    pub sent: u32,
    pub received: u32,
    /// Messages not taken: bytes that were none, a snapshot that was not newer, anything before the join.
    pub refused: u32,
    /// Presses the room's echo lacks, or has too many of, against the command it says it ran.
    pub presses_lost: u32,
    pub presses_doubled: u32,
}

/// A command as it was sent.
#[derive(Clone, Copy)]
struct Sent {
    sequence: u16,
    tick: u32,
    view_tick: u32,
    post: Post,
}

pub struct Endpoint {
    content_hash: [u8; 32],
    state: EndpointState,
    pacer: PressPacer,
    next_sequence: u16,
    /// The tick of the next command: one after the first snapshot's, then one more per command.
    next_tick: Option<u32>,
    newest_snapshot: Option<u32>,
    /// The newest command the room said it received, and the last one whose echo was checked.
    confirmed: Option<u16>,
    checked: Option<u16>,
    /// The sent commands by sequence; none in a place before its first command.
    sent: [Option<Sent>; SENT_RING],
    /// The message under construction, reused every tick.
    input: Input,
    readout: Readout,
}

impl Endpoint {
    /// An endpoint for the content with this hash, whose first command has this sequence (0 in play; near the
    /// 16-bit wrap in the tests of the wraps).
    #[must_use]
    pub fn new(content_hash: [u8; 32], first_sequence: u16) -> Self {
        Self {
            content_hash,
            state: EndpointState::Joining,
            pacer: PressPacer::default(),
            next_sequence: first_sequence,
            next_tick: None,
            newest_snapshot: None,
            confirmed: None,
            checked: None,
            sent: [None; SENT_RING],
            input: Input {
                acks: Vec::with_capacity(1),
                commands: Vec::with_capacity(REDUNDANCY_CAP),
            },
            readout: Readout::default(),
        }
    }

    #[must_use]
    pub const fn state(&self) -> EndpointState {
        self.state
    }

    #[must_use]
    pub const fn readout(&self) -> &Readout {
        &self.readout
    }

    #[must_use]
    pub const fn pacer(&self) -> &PressPacer {
        &self.pacer
    }

    /// Writes the `Join` into `out`, for the control channel. Local play has no token.
    pub fn join(&self, out: &mut Vec<u8>) -> bool {
        let join = Join {
            token: Vec::new(),
            protocol_version: PROTOCOL_VERSION,
            content_hash: self.content_hash,
        };
        encode_control(&Control::Join(join), out).is_ok()
    }

    /// Takes the room's answer to the join.
    ///
    /// # Errors
    /// Bytes that are no control message, a message that is not the answer waited for, or an acceptance by a room
    /// of another tick rate or other content; the endpoint then stays as it was.
    pub fn on_control(&mut self, bytes: &[u8]) -> Result<(), EndpointError> {
        let control = decode_control(bytes).map_err(EndpointError::Decode)?;
        self.state = match (self.state, control) {
            (EndpointState::Joining, Control::JoinAccepted(accepted)) => {
                if u32::from(accepted.tick_rate_hz) != TICK_HZ {
                    return Err(EndpointError::TickRate(accepted.tick_rate_hz));
                }
                if accepted.content_hash != self.content_hash {
                    return Err(EndpointError::ContentHash);
                }
                EndpointState::Joined
            }
            (EndpointState::Joining, Control::JoinRejected(rejected)) => {
                EndpointState::Rejected(rejected.reason)
            }
            _ => return Err(EndpointError::Unexpected),
        };
        Ok(())
    }

    fn sent(&self, sequence: u16) -> Option<&Sent> {
        let place = self.sent.get(usize::from(sequence & RING_MASK))?;
        place.as_ref().filter(|sent| sent.sequence == sequence)
    }

    /// Builds the next tick's command from the page's newest post and writes the `Input` that carries it into
    /// `out`: the newest snapshot as the ack, then that command and every earlier one the room has not confirmed,
    /// the newest first, up to the cap. False, with nothing to send, until the join is accepted and the first
    /// snapshot has said which tick the room runs next.
    pub fn build_input(&mut self, post: &Post, out: &mut Vec<u8>) -> bool {
        let (EndpointState::Joined, Some(tick)) = (self.state, self.next_tick) else {
            return false;
        };
        let sequence = self.next_sequence;
        let command = Sent {
            sequence,
            tick,
            view_tick: self.newest_snapshot.unwrap_or(0),
            post: Post {
                presses: self.pacer.next(&post.presses),
                ..*post
            },
        };
        if let Some(place) = self.sent.get_mut(usize::from(sequence & RING_MASK)) {
            *place = Some(command);
        }
        self.input.acks.clear();
        self.input.commands.clear();
        let ack = |tick| SnapshotAck { group: 0, tick };
        self.input.acks.extend(self.newest_snapshot.map(ack));
        for back in 0..REDUNDANCY_CAP {
            let earlier = sequence.wrapping_sub(u16::try_from(back).unwrap_or(u16::MAX));
            let unconfirmed = self.confirmed.is_none_or(|seen| seq_newer(earlier, seen));
            let Some(sent) = self.sent(earlier).filter(|_| unconfirmed).copied() else {
                break;
            };
            self.input.commands.push(Command {
                tick: sent.tick,
                sequence: sent.sequence,
                move_x: 0,
                move_y: 0,
                buttons: sent.post.buttons,
                presses: sent.post.presses,
                aim: AimPoint { x: 0, y: 0, z: 0 },
                view_time: ViewTime {
                    tick: sent.view_tick,
                    fraction: 0,
                },
            });
        }
        self.next_sequence = sequence.wrapping_add(1);
        // At the last tick there is no next command: the world itself stops there.
        self.next_tick = tick.checked_add(1);
        self.readout.sent = self.readout.sent.wrapping_add(1);
        encode_input(&self.input, out).is_ok()
    }

    /// Takes a datagram from the room: a snapshot. One that is not newer than the newest taken, or that comes
    /// before the join is accepted, is counted and dropped, as are bytes that are no snapshot.
    pub fn on_datagram(&mut self, bytes: &[u8]) {
        let newest = self.newest_snapshot;
        let taken = decode_snapshot(bytes).ok().filter(|snapshot| {
            let newer = |newest| ticks_after(newest, snapshot.server_tick).is_some_and(|by| by > 0);
            self.state == EndpointState::Joined && newest.is_none_or(newer)
        });
        let Some(snapshot) = taken else {
            self.readout.refused = self.readout.refused.wrapping_add(1);
            return;
        };
        self.newest_snapshot = Some(snapshot.server_tick);
        self.readout.tick = snapshot.server_tick;
        self.readout.received = self.readout.received.wrapping_add(1);
        if self.next_tick.is_none() && self.readout.sent == 0 {
            self.next_tick = snapshot.server_tick.checked_add(1);
        }
        if let Some(received) = snapshot.newest_command_received
            && self.confirmed.is_none_or(|seen| seq_newer(received, seen))
        {
            self.confirmed = Some(received);
        }
        let Some(own) = snapshot.own else {
            return;
        };
        (self.readout.echo_buttons, self.readout.echo_presses) = (own.buttons, own.presses);
        // The echo is the command the room last ran, as it applied it: checked once per command against what
        // that command carried.
        let processed = snapshot.last_processed_command;
        if processed == self.checked {
            return;
        }
        let Some(sent) = processed.and_then(|sequence| self.sent(sequence)).copied() else {
            return;
        };
        self.checked = processed;
        self.readout.post_used = sent.post.number;
        for (sent, echoed) in sent.post.presses.iter().zip(own.presses) {
            let more = press_edges(*sent, echoed);
            let (lost, doubled) = (
                &mut self.readout.presses_lost,
                &mut self.readout.presses_doubled,
            );
            if more >= 0x80 {
                *lost = lost.wrapping_add(u32::from(more.wrapping_neg()));
            } else {
                *doubled = doubled.wrapping_add(u32::from(more));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::codec::{decode_input, encode_snapshot};
    use protocol::messages::{JoinAccepted, JoinRejected, OwnState, Snapshot};

    const HASH: [u8; 32] = [0xC4; 32];

    fn control_bytes(control: &Control) -> Vec<u8> {
        let mut bytes = Vec::new();
        encode_control(control, &mut bytes).expect("it has a wire form");
        bytes
    }

    fn accepted(tick_rate_hz: u8, content_hash: [u8; 32]) -> Vec<u8> {
        control_bytes(&Control::JoinAccepted(JoinAccepted {
            slot: 0,
            team: 0,
            squad: 0,
            mode_id: "skeleton".to_owned(),
            mode_hash: [0; 32],
            content_hash,
            server_tick: 9,
            tick_rate_hz,
            snapshot_rate_hz: 30,
        }))
    }

    /// A snapshot at `tick` that confirms and echoes the command `ran` with these press counters.
    fn snapshot(tick: u32, ran: Option<u16>, presses: [u8; BUTTONS]) -> Vec<u8> {
        let own = OwnState {
            buttons: 0x21,
            presses,
        };
        let snapshot = Snapshot {
            server_tick: tick,
            baseline_tick: None,
            newest_command_received: ran,
            last_processed_command: ran,
            group: 0,
            own: Some(own),
        };
        let mut bytes = Vec::new();
        encode_snapshot(&snapshot, &mut bytes).expect("it has a wire form");
        bytes
    }

    fn post(number: u32, first_presses: u8) -> Post {
        let mut presses = [0; BUTTONS];
        presses[0] = first_presses;
        Post {
            number,
            buttons: 0x21,
            presses,
        }
    }

    /// An endpoint that has joined and taken the snapshot of tick 100.
    fn joined(first_sequence: u16) -> Endpoint {
        let mut endpoint = Endpoint::new(HASH, first_sequence);
        endpoint.on_control(&accepted(30, HASH)).expect("accepted");
        endpoint.on_datagram(&snapshot(100, None, [0; BUTTONS]));
        endpoint
    }

    /// The `Input` the endpoint writes for this post, decoded again.
    fn input(endpoint: &mut Endpoint, post: &Post) -> Input {
        let (mut bytes, mut input) = (Vec::new(), Endpoint::new(HASH, 0).input);
        assert!(endpoint.build_input(post, &mut bytes));
        decode_input(&bytes, &mut input).expect("the endpoint writes a valid input");
        input
    }

    #[test]
    fn the_join_exchange_ends_joined_rejected_or_refused_by_name() {
        let mut endpoint = Endpoint::new(HASH, 0);
        let mut bytes = Vec::new();
        assert!(endpoint.join(&mut bytes));
        let join = Join {
            token: Vec::new(),
            protocol_version: PROTOCOL_VERSION,
            content_hash: HASH,
        };
        assert_eq!(decode_control(&bytes), Ok(Control::Join(join)));
        // Nothing to send before the answer, and a snapshot before it is not taken.
        assert!(!endpoint.build_input(&Post::NONE, &mut bytes));
        endpoint.on_datagram(&snapshot(5, None, [0; BUTTONS]));
        assert_eq!(endpoint.readout().refused, 1);
        // What is not the answer waited for leaves the endpoint joining.
        let refusals = [
            (vec![0xFF], EndpointError::Decode(DecodeError::Kind(0xFF))),
            (bytes.clone(), EndpointError::Unexpected),
            (accepted(60, HASH), EndpointError::TickRate(60)),
            (accepted(30, [0; 32]), EndpointError::ContentHash),
        ];
        for (bytes, error) in refusals {
            assert_eq!(endpoint.on_control(&bytes), Err(error));
            assert_eq!(endpoint.state(), EndpointState::Joining);
        }
        assert_eq!(endpoint.on_control(&accepted(30, HASH)), Ok(()));
        assert_eq!(endpoint.state(), EndpointState::Joined);
        // A second answer is not waited for; and still nothing to send before the first snapshot.
        assert_eq!(
            endpoint.on_control(&accepted(30, HASH)),
            Err(EndpointError::Unexpected)
        );
        assert!(!endpoint.build_input(&Post::NONE, &mut bytes));

        let mut refused = Endpoint::new(HASH, 0);
        let reason = RejectReason::RoomFull;
        let rejected = control_bytes(&Control::JoinRejected(JoinRejected { reason }));
        assert_eq!(refused.on_control(&rejected), Ok(()));
        assert_eq!(refused.state(), EndpointState::Rejected(reason));
        assert!(!refused.build_input(&Post::NONE, &mut bytes));
    }

    #[test]
    fn each_command_is_for_the_next_tick_and_unconfirmed_ones_are_sent_again() {
        let mut endpoint = joined(65_534);
        // The first command is for the tick after the first snapshot's, and acks that snapshot.
        let first = input(&mut endpoint, &post(1, 1));
        assert_eq!(
            first.acks,
            [SnapshotAck {
                group: 0,
                tick: 100
            }]
        );
        assert_eq!(first.commands.len(), 1);
        let command = &first.commands[0];
        assert_eq!((command.tick, command.sequence), (101, 65_534));
        assert_eq!(
            (command.buttons, command.presses[0], command.view_time.tick),
            (0x21, 1, 100)
        );
        // Nothing confirmed: each input carries every command so far, the newest first, across the wrap, up to
        // the cap of 8.
        for sent in 2..=10u16 {
            let input = input(&mut endpoint, &post(u32::from(sent), 1));
            let sequences: Vec<u16> = input
                .commands
                .iter()
                .map(|command| command.sequence)
                .collect();
            let newest = 65_533u16.wrapping_add(sent);
            let expected: Vec<u16> = (0..sent.min(8))
                .map(|back| newest.wrapping_sub(back))
                .collect();
            assert_eq!(sequences, expected);
            assert_eq!(input.commands[0].tick, 100 + u32::from(sent));
        }
        assert_eq!(endpoint.readout().sent, 10);
        // The room confirms up to sequence 6 (the ninth command): only the tenth and the new one are sent.
        endpoint.on_datagram(&snapshot(101, Some(6), [1; BUTTONS]));
        let input = input(&mut endpoint, &post(11, 1));
        let sequences: Vec<u16> = input
            .commands
            .iter()
            .map(|command| command.sequence)
            .collect();
        assert_eq!(sequences, [8, 7]);
        assert_eq!(
            input.acks,
            [SnapshotAck {
                group: 0,
                tick: 101
            }]
        );
    }

    #[test]
    fn the_echo_is_checked_once_against_the_command_the_room_ran() {
        let mut endpoint = joined(0);
        input(&mut endpoint, &post(41, 1));
        input(&mut endpoint, &post(42, 2));
        let mut echo = [0; BUTTONS];
        echo[0] = 1;
        endpoint.on_datagram(&snapshot(101, Some(0), echo));
        let readout = *endpoint.readout();
        assert_eq!(
            (readout.tick, readout.received, readout.post_used),
            (101, 2, 41)
        );
        assert_eq!((readout.echo_buttons, readout.echo_presses), (0x21, echo));
        assert_eq!((readout.presses_lost, readout.presses_doubled), (0, 0));
        // The room says it ran sequence 1 and echoes one press too few on button 0 and two too many on button 3.
        echo[3] = 2;
        endpoint.on_datagram(&snapshot(102, Some(1), echo));
        let readout = *endpoint.readout();
        assert_eq!(
            (
                readout.post_used,
                readout.presses_lost,
                readout.presses_doubled
            ),
            (42, 1, 2)
        );
        // The same command echoed again in a later snapshot is not counted twice.
        endpoint.on_datagram(&snapshot(103, Some(1), echo));
        let readout = *endpoint.readout();
        assert_eq!(
            (readout.tick, readout.presses_lost, readout.presses_doubled),
            (103, 1, 2)
        );
    }

    #[test]
    fn a_snapshot_that_is_not_newer_and_bytes_that_are_none_are_counted_and_dropped() {
        let mut endpoint = joined(0);
        let before = *endpoint.readout();
        for bytes in [
            snapshot(100, Some(7), [9; BUTTONS]),
            snapshot(99, None, [9; BUTTONS]),
            vec![1, 2],
        ] {
            endpoint.on_datagram(&bytes);
        }
        let refused = Readout {
            refused: 3,
            ..before
        };
        assert_eq!(*endpoint.readout(), refused);
        // The dropped snapshot's confirmation was not taken either: the first command is still sent again.
        input(&mut endpoint, &post(1, 0));
        assert_eq!(input(&mut endpoint, &post(2, 0)).commands.len(), 2);
    }

    /// A thousand ticks of the exchange: the endpoint's two reused buffers stay where they are.
    #[test]
    fn a_thousand_ticks_move_no_buffer() {
        let mut endpoint = joined(0);
        let mut out = Vec::with_capacity(protocol::codec::MAX_INPUT_BYTES);
        let places = |endpoint: &Endpoint, out: &Vec<u8>| {
            let input = &endpoint.input;
            (
                input.acks.as_ptr(),
                input.commands.as_ptr(),
                out.as_ptr(),
                out.capacity(),
            )
        };
        let before = places(&endpoint, &out);
        for tick in 0..1000u16 {
            assert!(endpoint.build_input(&post(u32::from(tick), tick.to_le_bytes()[0]), &mut out));
            if tick % 3 != 0 {
                let ran = Some(tick);
                endpoint.on_datagram(&snapshot(101 + u32::from(tick), ran, [0; BUTTONS]));
            }
        }
        assert_eq!(places(&endpoint, &out), before);
        assert_eq!(endpoint.readout().sent, 1000);
    }
}
