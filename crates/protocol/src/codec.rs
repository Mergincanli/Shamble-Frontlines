//! The first wire form of the five messages (ADR 0001 parts 5 and 6): whole bytes, little-endian, a kind byte first.
//! Every decoder returns an error for bytes it does not take and never panics on any bytes: it reads through
//! `wire::Reader`, checks a count or a length against its cap and against the bytes left before anything is
//! allocated, and does arithmetic on a decoded value only with checked or wrapping operations. A message is taken
//! whole or not at all, and a message that decodes encodes back to the same bytes: nothing has two wire forms.
//!
//! | Message | Bytes after the kind |
//! |---|---|
//! | `Join` 0x01 | token length `u16`, the token, protocol version `u16`, content hash 32 |
//! | `JoinAccepted` 0x02 | slot, team, squad `u8`; mode id length `u8`, the id (UTF-8); mode hash 32; content hash 32; server tick `u32`; tick rate, snapshot rate `u8` |
//! | `JoinRejected` 0x03 | reason `u8` (1 protocol version, 2 content hash, 3 room full) |
//! | `Input` 0x10 | ack count `u8`, per ack: group `u8`, tick `u32`; command count `u8`; the newest command: tick `u32`, sequence `u16`, its fields; each older one: how many ticks earlier `u8` (1 to 255), its fields |
//! | `Snapshot` 0x11 | server tick `u32`; flags `u8`; group `u8`; then, each only with its flag: bit 0 the baseline, as ticks back `u16` (1 or more); bit 1 the newest command received `u16`; bit 2 the last command processed `u16`; bit 3 the own part: buttons `u16`, 16 press counters |
//!
//! A command's fields are 32 bytes: move x, y `i8`; buttons `u16`; 16 press counters; aim x `i16`, y `u16`, z `i16`;
//! view tick `u32`, fraction `u16`. An older command's sequence is one below the command before it, so it is not
//! sent. Commands are sent whole: the delta coding and the narrower widths the network draft plans come before
//! online play (an `Input` at the cap is 292 bytes here, above the draft's 200 per datagram).

use crate::messages::{
    AimPoint, BUTTONS, Command, Input, Join, JoinAccepted, JoinRejected, MAX_ACKS,
    MAX_MODE_ID_BYTES, MAX_TOKEN_BYTES, OwnState, REDUNDANCY_CAP, RejectReason, Snapshot,
    SnapshotAck, ViewTime,
};
use crate::wire::{Reader, put_i8, put_i16, put_u8, put_u16, put_u32};

pub const KIND_JOIN: u8 = 0x01;
pub const KIND_JOIN_ACCEPTED: u8 = 0x02;
pub const KIND_JOIN_REJECTED: u8 = 0x03;
pub const KIND_INPUT: u8 = 0x10;
pub const KIND_SNAPSHOT: u8 = 0x11;

/// The snapshot's flag bits: which of its optional parts follow.
pub const FLAG_BASELINE: u8 = 1;
pub const FLAG_RECEIVED: u8 = 2;
pub const FLAG_PROCESSED: u8 = 4;
pub const FLAG_OWN: u8 = 8;

/// The longest encodings, for whoever sizes a buffer: a `Join` with the longest token, an `Input` at both caps, a
/// `Snapshot` with every part.
pub const MAX_JOIN_BYTES: usize = 1 + 2 + MAX_TOKEN_BYTES + 2 + 32;
pub const MAX_INPUT_BYTES: usize = 1 + 1 + MAX_ACKS * 5 + 1 + 6 + 32 + (REDUNDANCY_CAP - 1) * 33;
pub const MAX_SNAPSHOT_BYTES: usize = 1 + 4 + 1 + 1 + 2 + 2 + 2 + 2 + BUTTONS;

// Each count and length has a fixed width on the wire.
const _: () = assert!(MAX_ACKS <= 255 && REDUNDANCY_CAP >= 1 && REDUNDANCY_CAP <= 255);
const _: () = assert!(MAX_TOKEN_BYTES <= 65_535 && MAX_MODE_ID_BYTES <= 255);

/// Why bytes were not taken as a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    /// The bytes end before the message does.
    Truncated,
    /// Bytes are left after the message's end.
    Trailing,
    /// A kind byte this decoder does not take.
    Kind(u8),
    /// A flag bit that means nothing.
    Flags(u8),
    /// A reject reason that does not exist.
    Reason(u8),
    /// More acks or commands than the cap allows.
    TooMany,
    /// A longer token or mode id than the cap allows.
    TooLong,
    /// A mode id that is not UTF-8.
    Utf8,
    /// A distance of 0 ticks, or a tick that would lie before tick 0.
    Gap,
}

/// Why a message has no wire form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeError {
    /// More acks or commands than the cap allows.
    TooMany,
    /// A longer token or mode id than the cap allows.
    TooLong,
    /// An older command that is not 1 to 255 ticks before the command ahead of it.
    TickGap,
    /// An older command whose sequence is not one below the command ahead of it.
    Sequence,
    /// A baseline that is not 1 to 65,535 ticks before the snapshot's own tick.
    Baseline,
}

/// The three messages of the control channel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Control {
    Join(Join),
    JoinAccepted(JoinAccepted),
    JoinRejected(JoinRejected),
}

/// `value` as the wire's narrower type when it is within `max`, for an encoder.
fn fits<T: TryFrom<usize>>(value: usize, max: usize, error: EncodeError) -> Result<T, EncodeError> {
    T::try_from(value)
        .ok()
        .filter(|_| value <= max)
        .ok_or(error)
}

/// A decoded count or length when it is within `max`.
fn within(value: impl Into<usize>, max: usize, error: DecodeError) -> Result<usize, DecodeError> {
    Some(value.into())
        .filter(|value| *value <= max)
        .ok_or(error)
}

/// The kind byte, when it is the one this decoder takes.
fn kind(reader: &mut Reader<'_>, taken: u8) -> Result<(), DecodeError> {
    match reader.u8()? {
        kind if kind == taken => Ok(()),
        other => Err(DecodeError::Kind(other)),
    }
}

/// Writes a control message into `out`, replacing what it held; `TooLong` for a token or a mode id above its cap,
/// and `out` is then empty.
pub fn encode_control(control: &Control, out: &mut Vec<u8>) -> Result<(), EncodeError> {
    out.clear();
    match control {
        Control::Join(join) => {
            let len = fits::<u16>(join.token.len(), MAX_TOKEN_BYTES, EncodeError::TooLong)?;
            put_u8(out, KIND_JOIN);
            put_u16(out, len);
            out.extend_from_slice(&join.token);
            put_u16(out, join.protocol_version);
            out.extend_from_slice(&join.content_hash);
        }
        Control::JoinAccepted(accepted) => {
            let len = accepted.mode_id.len();
            let len = fits::<u8>(len, MAX_MODE_ID_BYTES, EncodeError::TooLong)?;
            out.extend_from_slice(&[KIND_JOIN_ACCEPTED, accepted.slot, accepted.team]);
            out.extend_from_slice(&[accepted.squad, len]);
            out.extend_from_slice(accepted.mode_id.as_bytes());
            out.extend_from_slice(&accepted.mode_hash);
            out.extend_from_slice(&accepted.content_hash);
            put_u32(out, accepted.server_tick);
            out.extend_from_slice(&[accepted.tick_rate_hz, accepted.snapshot_rate_hz]);
        }
        Control::JoinRejected(rejected) => {
            let reason = match rejected.reason {
                RejectReason::ProtocolVersion => 1,
                RejectReason::ContentHash => 2,
                RejectReason::RoomFull => 3,
            };
            out.extend_from_slice(&[KIND_JOIN_REJECTED, reason]);
        }
    }
    Ok(())
}

/// Reads a control message; an error for any bytes that are not exactly one within its caps. A message that is
/// refused allocates nothing: the token and the mode id are copied only once the whole message has been read.
pub fn decode_control(bytes: &[u8]) -> Result<Control, DecodeError> {
    let mut reader = Reader::new(bytes);
    match reader.u8()? {
        KIND_JOIN => {
            let len = within(reader.u16()?, MAX_TOKEN_BYTES, DecodeError::TooLong)?;
            let token = reader.bytes(len)?;
            let protocol_version = reader.u16()?;
            let content_hash = reader.array()?;
            reader.finish()?;
            Ok(Control::Join(Join {
                token: token.to_vec(),
                protocol_version,
                content_hash,
            }))
        }
        KIND_JOIN_ACCEPTED => {
            let (slot, team, squad) = (reader.u8()?, reader.u8()?, reader.u8()?);
            let len = within(reader.u8()?, MAX_MODE_ID_BYTES, DecodeError::TooLong)?;
            let mode_id =
                core::str::from_utf8(reader.bytes(len)?).map_err(|_| DecodeError::Utf8)?;
            let (mode_hash, content_hash) = (reader.array()?, reader.array()?);
            let server_tick = reader.u32()?;
            let (tick_rate_hz, snapshot_rate_hz) = (reader.u8()?, reader.u8()?);
            reader.finish()?;
            Ok(Control::JoinAccepted(JoinAccepted {
                slot,
                team,
                squad,
                mode_id: mode_id.to_owned(),
                mode_hash,
                content_hash,
                server_tick,
                tick_rate_hz,
                snapshot_rate_hz,
            }))
        }
        KIND_JOIN_REJECTED => {
            let reason = match reader.u8()? {
                1 => RejectReason::ProtocolVersion,
                2 => RejectReason::ContentHash,
                3 => RejectReason::RoomFull,
                other => return Err(DecodeError::Reason(other)),
            };
            reader.finish()?;
            Ok(Control::JoinRejected(JoinRejected { reason }))
        }
        other => Err(DecodeError::Kind(other)),
    }
}

fn put_fields(out: &mut Vec<u8>, command: &Command) {
    put_i8(out, command.move_x);
    put_i8(out, command.move_y);
    put_u16(out, command.buttons);
    out.extend_from_slice(&command.presses);
    put_i16(out, command.aim.x);
    put_u16(out, command.aim.y);
    put_i16(out, command.aim.z);
    put_u32(out, command.view_time.tick);
    put_u16(out, command.view_time.fraction);
}

fn read_fields(reader: &mut Reader<'_>, tick: u32, sequence: u16) -> Result<Command, DecodeError> {
    let (move_x, move_y) = (reader.i8()?, reader.i8()?);
    let (buttons, presses) = (reader.u16()?, reader.array()?);
    let (x, y, z) = (reader.i16()?, reader.u16()?, reader.i16()?);
    let view_time = ViewTime {
        tick: reader.u32()?,
        fraction: reader.u16()?,
    };
    let aim = AimPoint { x, y, z };
    Ok(Command {
        tick,
        sequence,
        move_x,
        move_y,
        buttons,
        presses,
        aim,
        view_time,
    })
}

/// Writes an `Input` into `out`, replacing what it held. An error, and `out` empty, for more acks or commands than
/// their caps, or commands that are not the newest first with consecutive sequences, each 1 to 255 ticks before
/// the one ahead of it.
pub fn encode_input(input: &Input, out: &mut Vec<u8>) -> Result<(), EncodeError> {
    out.clear();
    let written = write_input(input, out);
    if written.is_err() {
        out.clear();
    }
    written
}

fn write_input(input: &Input, out: &mut Vec<u8>) -> Result<(), EncodeError> {
    put_u8(out, KIND_INPUT);
    put_u8(out, fits(input.acks.len(), MAX_ACKS, EncodeError::TooMany)?);
    for ack in &input.acks {
        put_u8(out, ack.group);
        put_u32(out, ack.tick);
    }
    let commands = input.commands.len();
    put_u8(out, fits(commands, REDUNDANCY_CAP, EncodeError::TooMany)?);
    let mut ahead: Option<&Command> = None;
    for command in &input.commands {
        if let Some(ahead) = ahead {
            if command.sequence != ahead.sequence.wrapping_sub(1) {
                return Err(EncodeError::Sequence);
            }
            let gap = ahead.tick.checked_sub(command.tick);
            let gap = gap.and_then(|gap| u8::try_from(gap).ok());
            put_u8(
                out,
                gap.filter(|gap| *gap != 0).ok_or(EncodeError::TickGap)?,
            );
        } else {
            put_u32(out, command.tick);
            put_u16(out, command.sequence);
        }
        put_fields(out, command);
        ahead = Some(command);
    }
    Ok(())
}

/// Reads an `Input` into `into`, replacing what it held; an error, and `into` empty, for any bytes that are not
/// exactly one `Input` within its caps. `into`'s two lists are sized once, to their caps and never to a count a
/// peer sent, so a reused `into` takes memory on its first calls and never again.
pub fn decode_input(bytes: &[u8], into: &mut Input) -> Result<(), DecodeError> {
    into.acks.clear();
    into.commands.clear();
    let read = read_input(bytes, into);
    if read.is_err() {
        into.acks.clear();
        into.commands.clear();
    }
    read
}

fn read_input(bytes: &[u8], into: &mut Input) -> Result<(), DecodeError> {
    let mut reader = Reader::new(bytes);
    kind(&mut reader, KIND_INPUT)?;
    let acks = within(reader.u8()?, MAX_ACKS, DecodeError::TooMany)?;
    into.acks.reserve_exact(MAX_ACKS);
    for _ in 0..acks {
        let (group, tick) = (reader.u8()?, reader.u32()?);
        into.acks.push(SnapshotAck { group, tick });
    }
    let commands = within(reader.u8()?, REDUNDANCY_CAP, DecodeError::TooMany)?;
    into.commands.reserve_exact(REDUNDANCY_CAP);
    let mut ahead: Option<(u32, u16)> = None;
    for _ in 0..commands {
        let (tick, sequence) = match ahead {
            None => (reader.u32()?, reader.u16()?),
            Some((tick, sequence)) => {
                let gap = Some(reader.u8()?).filter(|gap| *gap != 0);
                let tick = gap.and_then(|gap| tick.checked_sub(u32::from(gap)));
                (tick.ok_or(DecodeError::Gap)?, sequence.wrapping_sub(1))
            }
        };
        let command = read_fields(&mut reader, tick, sequence)?;
        into.commands.push(command);
        ahead = Some((tick, sequence));
    }
    reader.finish()
}

/// Writes a `Snapshot` into `out`, replacing what it held; `Baseline`, and `out` empty, for a baseline that is not
/// 1 to 65,535 ticks before the snapshot's own tick.
pub fn encode_snapshot(snapshot: &Snapshot, out: &mut Vec<u8>) -> Result<(), EncodeError> {
    out.clear();
    let back = match snapshot.baseline_tick {
        None => None,
        Some(baseline) => {
            let back = snapshot.server_tick.checked_sub(baseline);
            let back = back.and_then(|back| u16::try_from(back).ok());
            Some(
                back.filter(|back| *back != 0)
                    .ok_or(EncodeError::Baseline)?,
            )
        }
    };
    let words = [
        (back, FLAG_BASELINE),
        (snapshot.newest_command_received, FLAG_RECEIVED),
        (snapshot.last_processed_command, FLAG_PROCESSED),
    ];
    let mut flags = if snapshot.own.is_some() { FLAG_OWN } else { 0 };
    for (_, flag) in words.iter().filter(|(word, _)| word.is_some()) {
        flags |= flag;
    }
    put_u8(out, KIND_SNAPSHOT);
    put_u32(out, snapshot.server_tick);
    out.extend_from_slice(&[flags, snapshot.group]);
    for (word, _) in words {
        if let Some(word) = word {
            put_u16(out, word);
        }
    }
    if let Some(own) = &snapshot.own {
        put_u16(out, own.buttons);
        out.extend_from_slice(&own.presses);
    }
    Ok(())
}

/// Reads a `Snapshot`; an error for any bytes that are not exactly one. It allocates nothing.
pub fn decode_snapshot(bytes: &[u8]) -> Result<Snapshot, DecodeError> {
    let mut reader = Reader::new(bytes);
    kind(&mut reader, KIND_SNAPSHOT)?;
    let server_tick = reader.u32()?;
    let flags = reader.u8()?;
    if flags & !(FLAG_BASELINE | FLAG_RECEIVED | FLAG_PROCESSED | FLAG_OWN) != 0 {
        return Err(DecodeError::Flags(flags));
    }
    let group = reader.u8()?;
    let mut word = |flag: u8| -> Result<Option<u16>, DecodeError> {
        Ok(if flags & flag == 0 {
            None
        } else {
            Some(reader.u16()?)
        })
    };
    let baseline_tick = match word(FLAG_BASELINE)? {
        None => None,
        Some(back) => {
            let baseline = Some(back).filter(|back| *back != 0);
            let baseline = baseline.and_then(|back| server_tick.checked_sub(u32::from(back)));
            Some(baseline.ok_or(DecodeError::Gap)?)
        }
    };
    let newest_command_received = word(FLAG_RECEIVED)?;
    let last_processed_command = word(FLAG_PROCESSED)?;
    let own = if flags & FLAG_OWN == 0 {
        None
    } else {
        let (buttons, presses) = (reader.u16()?, reader.array()?);
        Some(OwnState { buttons, presses })
    };
    reader.finish()?;
    Ok(Snapshot {
        server_tick,
        baseline_tick,
        newest_command_received,
        last_processed_command,
        group,
        own,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command(tick: u32, sequence: u16, salt: u8) -> Command {
        let mut presses = [0; BUTTONS];
        for (at, counter) in (0u8..).zip(presses.iter_mut()) {
            *counter = salt.wrapping_mul(7).wrapping_add(at);
        }
        let (x, y, z) = (-2, 0xFFFE, i16::MIN);
        Command {
            tick,
            sequence,
            move_x: -127,
            move_y: i8::from_le_bytes([salt]),
            buttons: 0x8001 | u16::from(salt) << 4,
            presses,
            aim: AimPoint { x, y, z },
            view_time: ViewTime {
                tick: tick.wrapping_sub(3),
                fraction: 0xABCD,
            },
        }
    }

    fn join(token: &[u8]) -> Control {
        Control::Join(Join {
            token: token.to_vec(),
            protocol_version: 0x0102,
            content_hash: [0xC4; 32],
        })
    }

    fn accepted(mode_id: &str) -> Control {
        Control::JoinAccepted(JoinAccepted {
            slot: 1,
            team: 2,
            squad: 3,
            mode_id: mode_id.to_owned(),
            mode_hash: [0xAA; 32],
            content_hash: [0xBB; 32],
            server_tick: 0x0403_0201,
            tick_rate_hz: 30,
            snapshot_rate_hz: 30,
        })
    }

    fn rejected(reason: RejectReason) -> Control {
        Control::JoinRejected(JoinRejected { reason })
    }

    fn input(acks: u8, commands: Vec<Command>) -> Input {
        let ack = |group| SnapshotAck {
            group,
            tick: 1000 + u32::from(group),
        };
        Input {
            acks: (0..acks).map(ack).collect(),
            commands,
        }
    }

    /// Both caps: 4 acks, and 8 commands whose sequences cross the 16-bit wrap and whose ticks lie 1, 2, ... 6
    /// and 255 apart.
    fn full_input() -> Input {
        let back = [0u32, 1, 3, 6, 10, 15, 21, 276];
        let commands = back.iter().zip(0u16..);
        let commands = commands.map(|(back, at)| command(5000 - back, 3u16.wrapping_sub(at), 9));
        input(4, commands.collect())
    }

    /// A snapshot with the parts `flags` names, each at an extreme.
    fn snapshot(flags: u8) -> Snapshot {
        let own = OwnState {
            buttons: 0x8421,
            presses: [0xF0; BUTTONS],
        };
        Snapshot {
            server_tick: 70_000,
            baseline_tick: (flags & 1 != 0).then_some(70_000 - 65_535),
            newest_command_received: (flags & 2 != 0).then_some(65_535),
            last_processed_command: (flags & 4 != 0).then_some(0),
            group: 7,
            own: (flags & 8 != 0).then_some(own),
        }
    }

    fn control_bytes(control: &Control) -> Vec<u8> {
        let mut bytes = vec![0xEE; 3];
        encode_control(control, &mut bytes).expect("it has a wire form");
        bytes
    }

    fn input_bytes(input: &Input) -> Vec<u8> {
        let mut bytes = vec![0xEE; 3];
        encode_input(input, &mut bytes).expect("it has a wire form");
        bytes
    }

    fn snapshot_bytes(snapshot: &Snapshot) -> Vec<u8> {
        let mut bytes = vec![0xEE; 3];
        encode_snapshot(snapshot, &mut bytes).expect("it has a wire form");
        bytes
    }

    fn decoded_input(bytes: &[u8]) -> Result<Input, DecodeError> {
        let mut into = input(0, Vec::new());
        decode_input(bytes, &mut into).map(|()| into)
    }

    /// The wire form, byte for byte: these pin the table in this module's doc.
    #[test]
    fn each_message_has_the_bytes_the_table_gives() {
        let join_bytes = [&[0x01, 3, 0, 9, 8, 7, 0x02, 0x01][..], &[0xC4; 32]].concat();
        assert_eq!(control_bytes(&join(&[9, 8, 7])), join_bytes);
        let head = [0x02, 1, 2, 3, 2, b'a', b'b'];
        let tail = [1, 2, 3, 4, 30, 30];
        let accepted_bytes = [&head[..], &[0xAA; 32], &[0xBB; 32], &tail].concat();
        assert_eq!(control_bytes(&accepted("ab")), accepted_bytes);
        let reasons = [
            RejectReason::ProtocolVersion,
            RejectReason::ContentHash,
            RejectReason::RoomFull,
        ];
        for (reason, byte) in reasons.into_iter().zip(1..) {
            assert_eq!(control_bytes(&rejected(reason)), [0x03, byte]);
        }

        // Move -127 and the salt, the buttons, the counters, the aim, then the view time: three ticks before the
        // command's own, and the fraction.
        let fields = |salt: u8, view_tick: [u8; 4]| {
            let start = [0x81, salt, 0x01 | salt << 4, 0x80];
            let aim = [0xFE, 0xFF, 0xFE, 0xFF, 0x00, 0x80];
            let presses = command(0, 0, salt).presses;
            [&start[..], &presses, &aim, &view_tick, &[0xCD, 0xAB]].concat()
        };
        let commands = vec![command(0x0102_0304, 0, 1), command(0x0102_0302, 65_535, 2)];
        let mut two = input(1, commands);
        two.acks[0] = SnapshotAck {
            group: 5,
            tick: 0x0A0B_0C0D,
        };
        let head = [
            0x10, 1, 5, 0x0D, 0x0C, 0x0B, 0x0A, 2, 0x04, 0x03, 0x02, 0x01, 0, 0,
        ];
        let (newest, older) = (fields(1, [1, 3, 2, 1]), fields(2, [0xFF, 2, 2, 1]));
        assert_eq!(
            input_bytes(&two),
            [&head[..], &newest, &[2], &older].concat()
        );

        let head = [0x11, 0x70, 0x11, 0x01, 0x00, 0x0F, 7];
        let words = [0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x21, 0x84];
        let full = [&head[..], &words, &[0xF0; 16]].concat();
        assert_eq!(snapshot_bytes(&snapshot(15)), full);
        assert_eq!(snapshot_bytes(&snapshot(0)), [0x11, 0x70, 0x11, 1, 0, 0, 7]);
    }

    #[test]
    fn the_longest_messages_have_the_lengths_the_constants_give() {
        let longest = (MAX_JOIN_BYTES, MAX_INPUT_BYTES, MAX_SNAPSHOT_BYTES);
        assert_eq!(longest, (1061, 292, 31));
        let join_len = control_bytes(&join(&[1; MAX_TOKEN_BYTES])).len();
        let input_len = input_bytes(&full_input()).len();
        assert_eq!(
            (join_len, input_len, snapshot_bytes(&snapshot(15)).len()),
            longest
        );
    }

    /// `bytes` decode to the message they were made from; every proper prefix is `Truncated`, and one byte more
    /// is `Trailing`.
    fn only_whole<T: PartialEq + std::fmt::Debug>(
        bytes: &[u8],
        message: &T,
        decode: impl Fn(&[u8]) -> Result<T, DecodeError>,
    ) {
        assert_eq!(decode(bytes).as_ref(), Ok(message));
        for cut in 0..bytes.len() {
            assert_eq!(decode(&bytes[..cut]), Err(DecodeError::Truncated));
        }
        let longer = [bytes, &[0]].concat();
        assert_eq!(decode(&longer), Err(DecodeError::Trailing));
    }

    #[test]
    fn messages_at_their_extremes_round_trip_and_only_whole_messages_are_taken() {
        let long_id = "é".repeat(MAX_MODE_ID_BYTES / 2);
        let controls = [
            join(&[]),
            join(&[0xFF; MAX_TOKEN_BYTES]),
            accepted(""),
            accepted(&long_id),
            rejected(RejectReason::RoomFull),
        ];
        for control in &controls {
            only_whole(&control_bytes(control), control, decode_control);
        }
        let one = input(0, vec![command(0, 0, 0)]);
        for input in [&input(0, Vec::new()), &one, &full_input()] {
            only_whole(&input_bytes(input), input, decoded_input);
        }
        for flags in 0..16 {
            let bytes = snapshot_bytes(&snapshot(flags));
            assert_eq!(bytes[5], flags);
            only_whole(&bytes, &snapshot(flags), decode_snapshot);
        }
    }

    #[test]
    fn each_decoder_takes_only_its_own_kinds() {
        for kind in 0..=u8::MAX {
            let (bytes, refused) = ([kind, 1], DecodeError::Kind(kind));
            if !matches!(kind, KIND_JOIN | KIND_JOIN_ACCEPTED | KIND_JOIN_REJECTED) {
                assert_eq!(decode_control(&bytes), Err(refused));
            }
            if kind != KIND_INPUT {
                assert_eq!(decoded_input(&bytes), Err(refused));
            }
            if kind != KIND_SNAPSHOT {
                assert_eq!(decode_snapshot(&bytes), Err(refused));
            }
        }
    }

    #[test]
    fn what_lies_outside_a_cap_or_a_range_is_refused_by_name() {
        // A token one byte over the cap, with the bytes there: refused for its length. A length within the cap
        // that the message does not hold: truncated, whatever it claims.
        let long = [&[KIND_JOIN, 0x01, 0x04][..], &[0; 1025 + 34]].concat();
        assert_eq!(decode_control(&long), Err(DecodeError::TooLong));
        let short = [KIND_JOIN, 0x00, 0x04, 1, 2, 3];
        assert_eq!(decode_control(&short), Err(DecodeError::Truncated));
        let mode = [&[KIND_JOIN_ACCEPTED, 0, 0, 0, 65][..], &[b'a'; 65 + 70]].concat();
        assert_eq!(decode_control(&mode), Err(DecodeError::TooLong));
        let mut not_text = control_bytes(&accepted("ab"));
        not_text[5] = 0xFF;
        assert_eq!(decode_control(&not_text), Err(DecodeError::Utf8));
        for reason in [0, 4, 255] {
            let refused = Err(DecodeError::Reason(reason));
            assert_eq!(decode_control(&[KIND_JOIN_REJECTED, reason]), refused);
        }

        for bytes in [
            &[KIND_INPUT, 5][..],
            &[KIND_INPUT, 255],
            &[KIND_INPUT, 0, 9],
        ] {
            assert_eq!(decoded_input(bytes), Err(DecodeError::TooMany));
        }
        // An older command 0 ticks back, and one that would lie before tick 0.
        let mut bytes = input_bytes(&input(0, vec![command(3, 8, 1), command(1, 7, 1)]));
        let gap_at = 3 + 6 + 32;
        assert_eq!(bytes[gap_at], 2);
        let mut oldest_tick = |gap: u8| {
            bytes[gap_at] = gap;
            decoded_input(&bytes).map(|input| input.commands[1].tick)
        };
        assert_eq!(oldest_tick(0), Err(DecodeError::Gap));
        assert_eq!(oldest_tick(4), Err(DecodeError::Gap));
        assert_eq!(oldest_tick(3), Ok(0));

        for flags in 16..=u8::MAX {
            let bytes = [KIND_SNAPSHOT, 0, 0, 0, 0, flags, 0];
            assert_eq!(decode_snapshot(&bytes), Err(DecodeError::Flags(flags)));
        }
        let baseline = |tick: u8, back: u8| {
            let bytes = [KIND_SNAPSHOT, tick, 0, 0, 0, FLAG_BASELINE, 0, back, 0];
            decode_snapshot(&bytes).map(|snapshot| snapshot.baseline_tick)
        };
        assert_eq!(baseline(10, 0), Err(DecodeError::Gap));
        assert_eq!(baseline(10, 11), Err(DecodeError::Gap));
        assert_eq!(baseline(10, 10), Ok(Some(0)));
    }

    #[test]
    fn a_message_with_no_wire_form_is_refused_and_leaves_nothing_behind() {
        let mut out = vec![1, 2, 3];
        let too_long = Err(EncodeError::TooLong);
        assert_eq!(encode_control(&join(&[0; 1025]), &mut out), too_long);
        assert!(out.is_empty());
        assert_eq!(
            encode_control(&accepted(&"a".repeat(65)), &mut out),
            too_long
        );
        let refused = |input: &Input| {
            let mut out = vec![1, 2, 3];
            let result = encode_input(input, &mut out);
            assert!(out.is_empty());
            result
        };
        let mut many = full_input();
        many.acks.push(SnapshotAck { group: 0, tick: 0 });
        assert_eq!(refused(&many), Err(EncodeError::TooMany));
        let mut many = full_input();
        many.commands.push(command(1, 3u16.wrapping_sub(8), 9));
        assert_eq!(refused(&many), Err(EncodeError::TooMany));
        // The same tick, a later tick, and 256 ticks back have no one-byte distance.
        for older_tick in [1000, 1001, 744] {
            let pair = input(0, vec![command(1000, 5, 0), command(older_tick, 4, 0)]);
            assert_eq!(refused(&pair), Err(EncodeError::TickGap));
        }
        // A sequence that is not exactly one below.
        for older_sequence in [5, 6, 3] {
            let pair = input(
                0,
                vec![command(1000, 5, 0), command(999, older_sequence, 0)],
            );
            assert_eq!(refused(&pair), Err(EncodeError::Sequence));
        }
        for baseline_tick in [70_000, 70_001, 70_000 - 65_536].map(Some) {
            let snapshot = Snapshot {
                baseline_tick,
                ..snapshot(15)
            };
            let mut out = vec![1, 2, 3];
            assert_eq!(
                encode_snapshot(&snapshot, &mut out),
                Err(EncodeError::Baseline)
            );
            assert!(out.is_empty());
        }
    }

    /// A reused `Input` is sized on its first decode and its two lists never move again, whatever comes in; after
    /// a refused message it is empty.
    #[test]
    fn a_reused_input_keeps_its_buffers() {
        let mut into = input(0, Vec::new());
        decode_input(&[KIND_INPUT, 0, 0], &mut into).expect("an empty input");
        let sized = |into: &Input| (into.acks.capacity(), into.commands.capacity());
        assert_eq!(sized(&into), (MAX_ACKS, REDUNDANCY_CAP));
        let places = (into.acks.as_ptr(), into.commands.as_ptr());
        let full = input_bytes(&full_input());
        for _ in 0..100 {
            decode_input(&full, &mut into).expect("a full input");
            assert_eq!(into, full_input());
            let cut = decode_input(&full[..full.len() - 1], &mut into);
            assert_eq!(cut, Err(DecodeError::Truncated));
            assert!(into.acks.is_empty() && into.commands.is_empty());
        }
        assert_eq!((into.acks.as_ptr(), into.commands.as_ptr()), places);
        assert_eq!(sized(&into), (MAX_ACKS, REDUNDANCY_CAP));
    }
}
