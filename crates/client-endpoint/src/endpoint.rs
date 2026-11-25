//! The client endpoint's side of the message exchange (ADR 0001 parts 5 and 6; ADR 0002 part 6): it asks to join,
//! holds the page's newest post, builds each tick's command from it, sends every command the room has not
//! confirmed, reads the room's snapshots and writes the block for the page. It moves no bytes itself: whoever holds
//! the transport hands it what arrived and sends what it wrote. It never panics on bytes from the link, allocates
//! nothing per tick, and reads no clock.
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

use crate::block::BlockWriter;
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
    /// Datagrams that were no snapshot: refused.
    pub refused: u32,
    /// Snapshots that were well formed and not taken: not newer than the newest taken (a link may deliver one
    /// late or twice), or before the join was answered. Not a refusal: an honest room over a lossy link causes it.
    pub late: u32,
    /// Inputs the endpoint could not write. None can occur: every command it sends fits the wire form.
    pub unsent: u32,
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

/// One client's side of a match: the join, the commands, the snapshots, and the block the page reads.
pub struct Endpoint {
    content_hash: [u8; 32],
    state: EndpointState,
    /// The page's newest post, as the host handed it over.
    post: Post,
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
    writer: BlockWriter,
}

impl Endpoint {
    /// An endpoint for the content with this hash, whose first command has this sequence (0 in play; near the
    /// 16-bit wrap in the tests of the wraps), with a block padded by `pad_bytes` (0 in play). The block is whole
    /// from the start. `None` when `pad_bytes` is more padding than the block writer takes.
    #[must_use]
    pub fn new(content_hash: [u8; 32], first_sequence: u16, pad_bytes: u32) -> Option<Self> {
        let readout = Readout::default();
        let mut writer = BlockWriter::new(pad_bytes)?;
        writer.write(&readout, 0);
        Some(Self {
            content_hash,
            state: EndpointState::Joining,
            post: Post::NONE,
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
            readout,
            writer,
        })
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

    /// The page's newest post, from the host. The host keeps the newest post by its number and hands that one
    /// over once per tick; the endpoint takes what it is given.
    pub const fn set_post(&mut self, post: Post) {
        self.post = post;
    }

    /// The block's bytes. Their address does not change for the life of the endpoint.
    #[must_use]
    pub fn block(&self) -> &[u8] {
        self.writer.bytes()
    }

    #[must_use]
    pub const fn block_len(&self) -> u32 {
        self.writer.byte_len()
    }

    /// Writes the block for the page from what the endpoint knows now. `refused_elsewhere` is added to the block's
    /// count of refused messages: in local play, what the local server and the link refused.
    pub fn write_block(&mut self, refused_elsewhere: u32) {
        let refused = self.readout.refused.wrapping_add(refused_elsewhere);
        self.writer.write(&self.readout, refused);
    }

    /// Writes the `Join` into `out`, for the control channel, and says whether it did. Local play has no token.
    #[must_use]
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
    /// `out`: the newest snapshot as the ack, then that command, always, and after it every earlier one the room
    /// has not confirmed, the newest first, up to the cap. False, with nothing to send, until the join is accepted
    /// and the first snapshot has said which tick the room runs next.
    #[must_use]
    pub fn build_input(&mut self, out: &mut Vec<u8>) -> bool {
        let (EndpointState::Joined, Some(tick)) = (self.state, self.next_tick) else {
            return false;
        };
        let sequence = self.next_sequence;
        let newest = Sent {
            sequence,
            tick,
            view_tick: self.newest_snapshot.unwrap_or(0),
            post: Post {
                presses: self.pacer.next(&self.post.presses),
                ..self.post
            },
        };
        if let Some(place) = self.sent.get_mut(usize::from(sequence & RING_MASK)) {
            *place = Some(newest);
        }
        self.input.acks.clear();
        self.input.commands.clear();
        let ack = |tick| SnapshotAck { group: 0, tick };
        self.input.acks.extend(self.newest_snapshot.map(ack));
        // The newest command goes whatever the room confirmed; a repeat goes while the room has not confirmed it
        // and it lies 1 to 255 ticks before the command ahead of it, which is what the wire can say.
        let mut ahead = newest;
        for _ in 0..REDUNDANCY_CAP {
            self.input.commands.push(Command {
                tick: ahead.tick,
                sequence: ahead.sequence,
                move_x: 0,
                move_y: 0,
                buttons: ahead.post.buttons,
                presses: ahead.post.presses,
                aim: AimPoint { x: 0, y: 0, z: 0 },
                view_time: ViewTime {
                    tick: ahead.view_tick,
                    fraction: 0,
                },
            });
            let earlier = ahead.sequence.wrapping_sub(1);
            let unconfirmed = self.confirmed.is_none_or(|seen| seq_newer(earlier, seen));
            let fits = |sent: &&Sent| {
                let gap = ticks_after(sent.tick, ahead.tick);
                unconfirmed && gap.is_some_and(|gap| (1..=255).contains(&gap))
            };
            let Some(sent) = self.sent(earlier).filter(fits).copied() else {
                break;
            };
            ahead = sent;
        }
        self.next_sequence = sequence.wrapping_add(1);
        // At the last tick there is no next command: the world itself stops there.
        self.next_tick = tick.checked_add(1);
        let written = encode_input(&self.input, out).is_ok();
        let count = if written {
            &mut self.readout.sent
        } else {
            &mut self.readout.unsent
        };
        *count = count.wrapping_add(1);
        written
    }

    /// Takes a datagram from the room: a snapshot. Bytes that are no snapshot are refused and counted. A snapshot
    /// that is not newer than the newest taken, or that comes before the join is accepted, is dropped and counted
    /// apart, as late: a link that delivers late or twice causes it, and nobody did wrong.
    pub fn on_datagram(&mut self, bytes: &[u8]) {
        let Ok(snapshot) = decode_snapshot(bytes) else {
            self.readout.refused = self.readout.refused.wrapping_add(1);
            return;
        };
        let newer = |newest| ticks_after(newest, snapshot.server_tick).is_some_and(|by| by > 0);
        if self.state != EndpointState::Joined || !self.newest_snapshot.is_none_or(newer) {
            self.readout.late = self.readout.late.wrapping_add(1);
            return;
        }
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

    fn endpoint(first_sequence: u16) -> Endpoint {
        Endpoint::new(HASH, first_sequence, 0).expect("no padding is allowed")
    }

    /// An endpoint that has joined and taken the snapshot of tick 100.
    fn joined(first_sequence: u16) -> Endpoint {
        let mut endpoint = endpoint(first_sequence);
        endpoint.on_control(&accepted(30, HASH)).expect("accepted");
        endpoint.on_datagram(&snapshot(100, None, [0; BUTTONS]));
        endpoint
    }

    /// The `Input` the endpoint writes for this post, decoded again.
    fn input(endpoint: &mut Endpoint, post: &Post) -> Input {
        let (mut bytes, mut input) = (Vec::new(), self::endpoint(0).input);
        endpoint.set_post(*post);
        assert!(endpoint.build_input(&mut bytes));
        decode_input(&bytes, &mut input).expect("the endpoint writes a valid input");
        input
    }

    #[test]
    fn the_join_exchange_ends_joined_rejected_or_refused_by_name() {
        let mut endpoint = endpoint(0);
        let mut bytes = Vec::new();
        assert!(endpoint.join(&mut bytes));
        let join = Join {
            token: Vec::new(),
            protocol_version: PROTOCOL_VERSION,
            content_hash: HASH,
        };
        assert_eq!(decode_control(&bytes), Ok(Control::Join(join)));
        // Nothing to send before the answer, and a snapshot before it is not taken: late, not refused.
        assert!(!endpoint.build_input(&mut bytes));
        endpoint.on_datagram(&snapshot(5, None, [0; BUTTONS]));
        let readout = endpoint.readout();
        assert_eq!((readout.late, readout.refused, readout.received), (1, 0, 0));
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
        assert!(!endpoint.build_input(&mut bytes));

        let mut refused = self::endpoint(0);
        let reason = RejectReason::RoomFull;
        let rejected = control_bytes(&Control::JoinRejected(JoinRejected { reason }));
        assert_eq!(refused.on_control(&rejected), Ok(()));
        assert_eq!(refused.state(), EndpointState::Rejected(reason));
        assert!(!refused.build_input(&mut bytes));
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
        // Two honest late snapshots and one piece of nonsense: counted apart, and nothing else moves.
        let after = Readout {
            late: 2,
            refused: 1,
            ..before
        };
        assert_eq!(*endpoint.readout(), after);
        // The dropped snapshot's confirmation was not taken either: the first command is still sent again.
        input(&mut endpoint, &post(1, 0));
        assert_eq!(input(&mut endpoint, &post(2, 0)).commands.len(), 2);
    }

    /// The room confirms a sequence the endpoint has not sent yet (a room that counts wrong, or bytes changed on
    /// the way that still decode): the newest command goes all the same, alone.
    #[test]
    fn the_newest_command_goes_whatever_the_room_confirmed() {
        let mut endpoint = joined(0);
        input(&mut endpoint, &post(1, 0));
        endpoint.on_datagram(&snapshot(101, Some(500), [0; BUTTONS]));
        for sequence in 1..=3u16 {
            let input = input(&mut endpoint, &post(u32::from(sequence) + 1, 0));
            let sequences: Vec<u16> = input
                .commands
                .iter()
                .map(|command| command.sequence)
                .collect();
            assert_eq!(sequences, [sequence]);
        }
        assert_eq!((endpoint.readout().sent, endpoint.readout().unsent), (4, 0));
    }

    /// The endpoint writes the page's block itself: whole before the first snapshot, and after `write_block` it
    /// holds the newest readout, with what was refused elsewhere added to the endpoint's own count.
    #[test]
    fn the_block_is_whole_from_the_start_and_follows_the_readout() {
        use crate::layout::{BLOCK_BYTES, field, section_offset};
        let read = |endpoint: &Endpoint, section: &str, name: &str| {
            let start = section_offset(section).expect("the layout names the section");
            let at = (start + field(section, name).expect("and the field").offset) as usize;
            let bytes: [u8; 4] = endpoint.block()[at..at + 4].try_into().expect("four bytes");
            u32::from_le_bytes(bytes)
        };
        let mut endpoint = endpoint(0);
        assert_eq!(endpoint.block().len(), BLOCK_BYTES as usize);
        assert_eq!(endpoint.block_len(), BLOCK_BYTES);
        let place = endpoint.block().as_ptr();
        // Whole before anything arrived: the layout's hash is there, so the page can check its first read.
        assert_eq!(
            read(&endpoint, "header", "layout_hash"),
            crate::layout::LAYOUT_HASH
        );
        assert_eq!(read(&endpoint, "header", "tick"), 0);
        assert!(Endpoint::new(HASH, 0, u32::MAX).is_none());

        endpoint.on_control(&accepted(30, HASH)).expect("accepted");
        endpoint.on_datagram(&snapshot(100, None, [0; BUTTONS]));
        endpoint.on_datagram(&[1, 2]);
        input(&mut endpoint, &post(1, 0));
        // Nothing reaches the page until the block is written.
        assert_eq!(read(&endpoint, "header", "tick"), 0);
        endpoint.write_block(40);
        assert_eq!(read(&endpoint, "header", "tick"), 100);
        assert_eq!(read(&endpoint, "link", "sent"), 1);
        assert_eq!(read(&endpoint, "link", "received"), 1);
        assert_eq!(read(&endpoint, "link", "refused"), 41);
        assert_eq!(endpoint.block().as_ptr(), place);
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
            endpoint.set_post(post(u32::from(tick), tick.to_le_bytes()[0]));
            assert!(endpoint.build_input(&mut out));
            if tick % 3 != 0 {
                let ran = Some(tick);
                endpoint.on_datagram(&snapshot(101 + u32::from(tick), ran, [0; BUTTONS]));
            }
        }
        assert_eq!(places(&endpoint, &out), before);
        assert_eq!(endpoint.readout().sent, 1000);
    }
}
