//! The 15 built message types (plan §4.7: 13, `SessionControl`, and `SessionStatus` by the advisor's A1 ruling), their
//! framing per channel, and [`encode`] and [`decode`] (catalogue §1, "Framing and channels"; §2's rows).
//!
//! **Framing:** a datagram is one message, its 1 B type, then its body. A control-stream message is a 2 B
//! little-endian length of what follows, then the type, then the body. A full-state stream carries a 4 B length, then
//! one `FullState` (its type, then its body). Every field is little-endian; packed fields go least significant bit
//! first (bits.rs), and every pad bit is zero.
//!
//! **Decoding, in this order** ([`decode`]): the size against the largest message the receiving end takes on that
//! channel; the type byte; the size against that type's own receive limit (so a buffer one byte over is
//! [`DecodeError::TooLarge`] whatever else it holds: catalogue item 3's size half); a stream's length prefix; the
//! direction (npt-server decodes only client → room types: a room type from a client is refused as forged); the
//! channel (catalogue item 2); then the body, which must end exactly at the last byte with every pad and reserved bit
//! zero (ADR 0001:238: trailing bytes or unknown fields drop the message whole). Nothing is allocated before the size
//! checks pass, and nothing panics.
//!
//! **Encoding** ([`encode`]) refuses a message above its type's send limit, its row's in [`Limits::TYPES`], with
//! [`EncodeError::TooLarge`]. A type's send and receive limits are one number, but `Snapshot`'s: it is sent up to
//! [`Limits::SNAPSHOT`] (1,000 B) and taken up to [`Limits::SNAPSHOT_RECEIVE`] (1,150 B; catalogue §2).

use npt_link::Channel;

use crate::bits::{BitReader, BitWriter};
use crate::control::{Redacted, SessionControl, SessionStatus};
use crate::error::{DecodeError, EncodeError};
use crate::input::Input;
use crate::limits::Limits;
use crate::quant::Quant;
use crate::snapshot::{FullState, Snapshot};

/// Which way a message travels (the catalogue's Direction column). [`decode`] takes the way the bytes came: the
/// room decodes with `ClientToRoom`, a client with `RoomToClient`, and each refuses a type that never travels that way.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    ClientToRoom,
    RoomToClient,
    /// `Ping` and `Pong`. Passed to [`decode`], it takes these two types alone.
    BothWays,
}

/// `Resync`: a joined client that has lost its state asks for a `FullState` (catalogue §2; no body).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Resync;

/// `Ping`: either end of a joined session, a few times a second (catalogue §2).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Ping {
    /// The sender's id for this ping, echoed by its `Pong`.
    pub id: u8,
    /// The sender's time when it sent it, in ns on its own clock.
    pub sent_ns: u64,
}

/// `Pong`: the answer to a `Ping` (catalogue §2; ADR 0001 part 5).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pong {
    /// The `Ping`'s id.
    pub id: u8,
    /// The `Ping`'s `sent_ns`, echoed.
    pub echo_ns: u64,
    /// The answering end's tick.
    pub tick: u32,
    /// The hold time from the ping's arrival to this answer, in µs (the room counts at most one tick period of it).
    pub hold_us: u16,
}

/// `Join`: a connection's first control-stream message (catalogue §2, with its notes on `bind`). Its `Debug` shows the
/// token's and the bind value's lengths only.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Join {
    /// The slot's test token (plan §10.3), compared in constant time by npt-room.
    pub token: [u8; 16],
    /// The client's bind value (the catalogue's note on `bind`).
    pub bind: [u8; 16],
    /// The protocol version.
    pub version: u16,
    /// The content hash: the scene data and the build.
    pub content_hash: [u8; 32],
}

impl core::fmt::Debug for Join {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Join")
            .field("token", &Redacted(self.token.len()))
            .field("bind", &Redacted(self.bind.len()))
            .field("version", &self.version)
            .field("content_hash", &self.content_hash)
            .finish()
    }
}

/// `JoinAccepted`: the room's answer to an accepted `Join` (catalogue §2).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct JoinAccepted {
    pub slot: u8,
    pub team: u8,
    pub squad: u8,
    /// The mode id: it names the scene (the catalogue's note).
    pub mode_id: u16,
    pub mode_hash: [u8; 32],
    pub content_hash: [u8; 32],
    /// The server's tick.
    pub tick: u32,
    /// The tick rate, ticks per second.
    pub tick_hz: u16,
    /// The snapshot rate, snapshots per second.
    pub snapshot_hz: u16,
}

/// `JoinRejected`: why the room refused a `Join` (the catalogue's proposed codes); a code, never the offending bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinRejected {
    BadToken,
    BindMismatch,
    RosterFull,
    WrongVersion,
    WrongContentHash,
}

/// `ClientSettings`: the client's announced interpolation delay, sent when it changes (catalogue §2).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClientSettings {
    /// The delay in 256ths of a snapshot interval (512 is 2 intervals, 1,280 is 5), integers so the float-free room
    /// clamps it to the mode's range and moves it by at most one interval a second exactly. Any value decodes: the
    /// room clamps, never refuses.
    pub delay: u16,
}

/// Why a room ended (`RoomEnded`'s reason: catalogue §2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndReason {
    /// The run reached its end.
    Ended,
    /// The operator stopped it.
    Stopped,
    /// The tick thread's top-level catch sent it.
    Crash,
}

/// `RoomEnded`: sent to every session at the run's end, or with the crash reason (catalogue §2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoomEnded {
    pub reason: EndReason,
    /// The room's last tick.
    pub final_tick: u32,
}

/// `Leave`: a joined client ends its own session (catalogue §2; reason codes proposed here: the catalogue names none).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leave {
    /// The player left.
    Quit,
    /// The client stops on an error of its own.
    Error,
}

/// `Kick`: the room ends a session, then closes it (the catalogue's proposed reasons).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kick {
    /// A newer `Join` took the slot.
    Replaced,
    /// A stream message failed its framing, size or type checks.
    Malformed,
    /// The client's event queue overflowed again, or its resync went unacknowledged (u4-build-plan.md §4.7).
    Overflow,
    /// The operator stopped the session.
    Stopped,
}

/// Every built message, as [`encode`] takes it and [`decode`] returns it.
#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    Input(Input),
    Snapshot(Snapshot),
    FullState(FullState),
    Resync(Resync),
    Ping(Ping),
    Pong(Pong),
    Join(Join),
    JoinAccepted(JoinAccepted),
    JoinRejected(JoinRejected),
    ClientSettings(ClientSettings),
    RoomEnded(RoomEnded),
    Leave(Leave),
    Kick(Kick),
    SessionControl(SessionControl),
    SessionStatus(SessionStatus),
}

impl Message {
    /// The message's type name, as the catalogue's Type column and [`Limits::TYPES`] give it.
    pub fn name(&self) -> &'static str {
        Limits::TYPES[self.ty() - 1].0
    }

    /// The type byte: the type's position in [`Limits::TYPES`], from 1. An exhaustive match, so a new variant must
    /// be given its row.
    fn ty(&self) -> usize {
        match self {
            Message::Input(_) => 1,
            Message::Snapshot(_) => 2,
            Message::FullState(_) => 3,
            Message::Resync(_) => 4,
            Message::Ping(_) => 5,
            Message::Pong(_) => 6,
            Message::Join(_) => 7,
            Message::JoinAccepted(_) => 8,
            Message::JoinRejected(_) => 9,
            Message::ClientSettings(_) => 10,
            Message::RoomEnded(_) => 11,
            Message::Leave(_) => 12,
            Message::Kick(_) => 13,
            Message::SessionControl(_) => 14,
            Message::SessionStatus(_) => 15,
        }
    }
}

/// The framing bytes before a message's type on `channel`: none, the 2 B length or the 4 B length.
fn framing(channel: Channel) -> usize {
    match channel {
        Channel::Datagram => 0,
        Channel::Control => 2,
        Channel::FullState => 4,
    }
}

/// Type `t`'s row (t from 1).
fn row(t: usize) -> (&'static str, Channel, Direction, usize) {
    Limits::TYPES[t - 1]
}

/// The most bytes type `t` may have after its framing (its type and body) when it is sent: its row's limit in
/// [`Limits::TYPES`] (`Snapshot`'s the 1,000 B send cap), less the 2 B length a control-stream limit counts.
fn send_limit(t: usize) -> usize {
    let (_, channel, _, limit) = row(t);
    after_length(channel, limit)
}

/// The same at the receiving end: every type's send limit but `Snapshot`'s, which a client takes up to
/// [`Limits::SNAPSHOT_RECEIVE`] (catalogue §2: the send cap rises to at most that once PM-0001-36 measures).
fn receive_limit(t: usize) -> usize {
    let (name, channel, _, limit) = row(t);
    let limit = if name == "Snapshot" {
        Limits::SNAPSHOT_RECEIVE
    } else {
        limit
    };
    after_length(channel, limit)
}

/// A limit as the catalogue counts it, less the 2 B length a control-stream limit includes.
fn after_length(channel: Channel, limit: usize) -> usize {
    if channel == Channel::Control {
        limit - 2
    } else {
        limit
    }
}

/// Whether type `t` travels the way `way` says.
fn travels(t: usize, way: Direction) -> bool {
    let d = row(t).2;
    d == way || d == Direction::BothWays
}

/// The largest message, framing included, the end receiving `way` takes on `channel`: the largest of the types that
/// travel that way on that channel, or, on a channel none of them uses (a client's full-state stream), of every type
/// that travels that way, framed for that channel.
pub(crate) fn channel_max(way: Direction, channel: Channel) -> usize {
    let ways = 1..=Limits::TYPES.len();
    let on = ways
        .clone()
        .filter(|&t| travels(t, way) && row(t).1 == channel)
        .map(receive_limit)
        .max();
    let any = ways
        .filter(|&t| travels(t, way))
        .map(receive_limit)
        .max()
        .unwrap_or(0);
    framing(channel) + on.unwrap_or(any)
}

/// Codes `msg` with the wire's [`Quant`], appending its framed bytes to `out` (`Quant::encode` for a chosen one).
///
/// # Errors
/// [`EncodeError`] for a value the wire does not carry; nothing is appended then.
pub fn encode(msg: &Message, out: &mut Vec<u8>) -> Result<(), EncodeError> {
    Quant::default().encode(msg, out)
}

/// Decodes one received message with the wire's [`Quant`]: `way` is the direction the bytes came (`ClientToRoom` at
/// the room), `channel` the channel they came on, `bytes` the whole message as the link delivered it (the module text
/// gives the order of the checks).
///
/// # Errors
/// [`DecodeError`] for anything that is not one valid message of a type that travels that way on that channel.
pub fn decode(way: Direction, channel: Channel, bytes: &[u8]) -> Result<Message, DecodeError> {
    Quant::default().decode(way, channel, bytes)
}

impl Quant {
    /// [`encode`] with this `Quant`'s choices (its packing, and in a bites build its planted defect).
    ///
    /// # Errors
    /// As [`encode`].
    pub fn encode(self, msg: &Message, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        let start = out.len();
        let t = msg.ty();
        let r = self.encode_into(msg, out).and_then(|()| {
            if out.len() - start - framing(row(t).1) > send_limit(t) {
                Err(EncodeError::TooLarge)
            } else {
                Ok(())
            }
        });
        if r.is_err() {
            out.truncate(start);
        }
        r
    }

    /// `msg` framed and appended to `out`, at any size: [`Quant::encode`] then checks the size.
    fn encode_into(self, msg: &Message, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        let start = out.len();
        let t = msg.ty();
        let (_, channel, _, _) = row(t);
        let frame = framing(channel);
        out.resize(start + frame, 0);
        out.push(t as u8);
        {
            let mut w = BitWriter::new(out);
            match msg {
                Message::Input(m) => m.put(self, &mut w)?,
                Message::Snapshot(m) => m.put(self, &mut w)?,
                Message::FullState(m) => m.put(self, &mut w)?,
                Message::Resync(Resync) => {}
                Message::Ping(m) => {
                    w.put(u64::from(m.id), 8);
                    w.put(m.sent_ns, 64);
                }
                Message::Pong(m) => {
                    w.put(u64::from(m.id), 8);
                    w.put(m.echo_ns, 64);
                    w.put(u64::from(m.tick), 32);
                    w.put(u64::from(m.hold_us), 16);
                }
                Message::Join(m) => {
                    w.bytes(&m.token);
                    w.bytes(&m.bind);
                    w.put(u64::from(m.version), 16);
                    w.bytes(&m.content_hash);
                }
                Message::JoinAccepted(m) => {
                    w.put(u64::from(m.slot), 8);
                    w.put(u64::from(m.team), 8);
                    w.put(u64::from(m.squad), 8);
                    w.put(u64::from(m.mode_id), 16);
                    w.bytes(&m.mode_hash);
                    w.bytes(&m.content_hash);
                    w.put(u64::from(m.tick), 32);
                    w.put(u64::from(m.tick_hz), 16);
                    w.put(u64::from(m.snapshot_hz), 16);
                }
                Message::JoinRejected(m) => w.put(*m as u64, 8),
                Message::ClientSettings(m) => w.put(u64::from(m.delay), 16),
                Message::RoomEnded(m) => {
                    w.put(m.reason as u64, 8);
                    w.put(u64::from(m.final_tick), 32);
                }
                Message::Leave(m) => w.put(*m as u64, 8),
                Message::Kick(m) => w.put(*m as u64, 8),
                Message::SessionControl(m) => m.put(&mut w)?,
                Message::SessionStatus(m) => m.put(&mut w)?,
            }
        }
        let length = out.len() - start - frame;
        match frame {
            2 => out[start..start + 2].copy_from_slice(&(length as u16).to_le_bytes()),
            4 => out[start..start + 4].copy_from_slice(&(length as u32).to_le_bytes()),
            _ => {}
        }
        Ok(())
    }

    /// [`decode`] with this `Quant`'s choices (in a bites build, its planted defect).
    ///
    /// # Errors
    /// As [`decode`].
    pub fn decode(
        self,
        way: Direction,
        channel: Channel,
        bytes: &[u8],
    ) -> Result<Message, DecodeError> {
        let frame = framing(channel);
        if bytes.len() > channel_max(way, channel) {
            return Err(DecodeError::TooLarge);
        }
        let code = *bytes.get(frame).ok_or(DecodeError::Truncated)?;
        let t = usize::from(code);
        if !(1..=Limits::TYPES.len()).contains(&t) {
            return Err(DecodeError::UnknownType);
        }
        if bytes.len() > frame + receive_limit(t) {
            return Err(DecodeError::TooLarge);
        }
        let length = match frame {
            2 => usize::from(u16::from_le_bytes([bytes[0], bytes[1]])),
            4 => u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize,
            _ => bytes.len(),
        };
        if length != bytes.len() - frame {
            return Err(DecodeError::BadLength);
        }
        if !travels(t, way) {
            return Err(DecodeError::WrongDirection);
        }
        if row(t).1 != channel && self.checks_channel() {
            return Err(DecodeError::WrongChannel);
        }
        let strict = self.strict();
        let mut r = BitReader::new(&bytes[frame + 1..]);
        let msg = match t {
            1 => Message::Input(Input::get(self, &mut r)?),
            2 => Message::Snapshot(Snapshot::get(self, &mut r)?),
            3 => Message::FullState(FullState::get(self, &mut r)?),
            4 => Message::Resync(Resync),
            5 => Message::Ping(Ping {
                id: r.u8()?,
                sent_ns: r.u64()?,
            }),
            6 => Message::Pong(Pong {
                id: r.u8()?,
                echo_ns: r.u64()?,
                tick: r.u32()?,
                hold_us: r.u16()?,
            }),
            7 => Message::Join(Join {
                token: r.array()?,
                bind: r.array()?,
                version: r.u16()?,
                content_hash: r.array()?,
            }),
            8 => Message::JoinAccepted(JoinAccepted {
                slot: r.u8()?,
                team: r.u8()?,
                squad: r.u8()?,
                mode_id: r.u16()?,
                mode_hash: r.array()?,
                content_hash: r.array()?,
                tick: r.u32()?,
                tick_hz: r.u16()?,
                snapshot_hz: r.u16()?,
            }),
            9 => Message::JoinRejected(match r.u8()? {
                0 => JoinRejected::BadToken,
                1 => JoinRejected::BindMismatch,
                2 => JoinRejected::RosterFull,
                3 => JoinRejected::WrongVersion,
                4 => JoinRejected::WrongContentHash,
                _ => return Err(DecodeError::OutOfRange),
            }),
            10 => Message::ClientSettings(ClientSettings { delay: r.u16()? }),
            11 => {
                let reason = match r.u8()? {
                    0 => EndReason::Ended,
                    1 => EndReason::Stopped,
                    2 => EndReason::Crash,
                    _ => return Err(DecodeError::OutOfRange),
                };
                Message::RoomEnded(RoomEnded {
                    reason,
                    final_tick: r.u32()?,
                })
            }
            12 => Message::Leave(match r.u8()? {
                0 => Leave::Quit,
                1 => Leave::Error,
                _ => return Err(DecodeError::OutOfRange),
            }),
            13 => Message::Kick(match r.u8()? {
                0 => Kick::Replaced,
                1 => Kick::Malformed,
                2 => Kick::Overflow,
                3 => Kick::Stopped,
                _ => return Err(DecodeError::OutOfRange),
            }),
            14 => Message::SessionControl(SessionControl::get(&mut r)?),
            _ => Message::SessionStatus(SessionStatus::get(&mut r)?),
        };
        r.finish(strict)?;
        Ok(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_type_table_and_the_message_enum_agree_both_ways() {
        // Every row is a variant's: ty() is an exhaustive match (a new variant does not compile without a row), and
        // each row's position is the type byte of the variant whose name it carries.
        let one_of_each = [
            Message::Input(Input::default()),
            Message::Snapshot(Snapshot::default()),
            Message::FullState(FullState::default()),
            Message::Resync(Resync),
            Message::Ping(Ping::default()),
            Message::Pong(Pong::default()),
            Message::Join(Join::default()),
            Message::JoinAccepted(JoinAccepted::default()),
            Message::JoinRejected(JoinRejected::BadToken),
            Message::ClientSettings(ClientSettings::default()),
            Message::RoomEnded(RoomEnded {
                reason: EndReason::Ended,
                final_tick: 0,
            }),
            Message::Leave(Leave::Quit),
            Message::Kick(Kick::Replaced),
            Message::SessionControl(SessionControl::TriggerBurst { cap: [0; 16] }),
            Message::SessionStatus(SessionStatus::default()),
        ];
        assert_eq!(one_of_each.len(), Limits::TYPES.len());
        for (i, m) in one_of_each.iter().enumerate() {
            assert_eq!(m.ty(), i + 1, "{m:?}");
            let mut out = Vec::new();
            encode(m, &mut out).unwrap_or_else(|e| panic!("{}: {e}", m.name()));
            let (_, channel, way, _) = Limits::TYPES[i];
            let way = if way == Direction::BothWays {
                Direction::ClientToRoom
            } else {
                way
            };
            assert_eq!(decode(way, channel, &out).as_ref(), Ok(m), "{}", m.name());
        }
    }

    #[test]
    fn each_receiving_end_takes_the_largest_message_of_its_way_on_each_channel() {
        use Direction::*;
        // The catalogue's numbers: any client datagram above 166 B, any client stream message above 512 B, any
        // room datagram above 1,150 B; a room stream message above JoinAccepted's 256 B; a full state's 4 B more.
        assert_eq!(channel_max(ClientToRoom, Channel::Datagram), 166);
        assert_eq!(channel_max(ClientToRoom, Channel::Control), 512);
        assert_eq!(channel_max(ClientToRoom, Channel::FullState), 4 + 510);
        assert_eq!(channel_max(RoomToClient, Channel::Datagram), 1_150);
        assert_eq!(channel_max(RoomToClient, Channel::Control), 256);
        assert_eq!(channel_max(RoomToClient, Channel::FullState), 4 + 65_536);
        assert_eq!(channel_max(BothWays, Channel::Datagram), 32);
    }

    /// `msg` framed as [`encode`] frames it, at any size (no size check), so a test can build one of any length.
    fn uncapped(msg: &Message) -> Vec<u8> {
        let mut out = Vec::new();
        Quant::default()
            .encode_into(msg, &mut out)
            .unwrap_or_else(|e| panic!("{}: {e}", msg.name()));
        out
    }

    /// A message from `build(n, k)` whose framing and bytes, uncapped, are exactly `len` bytes: `n` small items and `k`
    /// fillers of another width, `k` from 0 and `n` by bisection (the length grows with `n`), so the search follows
    /// whatever widths the layouts have.
    fn exactly(len: usize, build: impl Fn(usize, usize) -> Message) -> Message {
        for k in 0..64 {
            let (mut lo, mut hi) = (0, len);
            while lo < hi {
                let mid = (lo + hi) / 2;
                if uncapped(&build(mid, k)).len() < len {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            let m = build(lo, k);
            if uncapped(&m).len() == len {
                return m;
            }
        }
        panic!("no message of exactly {len} B")
    }

    #[test]
    fn a3_a_snapshot_is_taken_up_to_1_150_b_and_sent_up_to_1_000_b() {
        use crate::snapshot::EntityDelta;
        use npt_sim::{EntityKind, SimEvent};
        // Removals (21 bits each) and shots (50 bits each) against a baseline: valid at every length.
        let snapshot = |n: usize, k: usize| {
            Message::Snapshot(Snapshot {
                tick: 70_000,
                baseline_age: 1,
                entities: (0..n)
                    .map(|i| EntityDelta::Remove {
                        kind: EntityKind::ALL[0],
                        index: i as u16,
                    })
                    .collect(),
                events: (0..k)
                    .map(|i| (i as u16, 0, SimEvent::Shot { slot: 1, seq: 2 }))
                    .collect(),
                ..Snapshot::default()
            })
        };
        // The receive limit (Limits::SNAPSHOT_RECEIVE, catalogue §2: npt-client refuses a datagram above 1,150 B):
        // a client takes 1,001 B and 1,150 B, and refuses 1,151 B by its size.
        for len in [Limits::SNAPSHOT + 1, Limits::SNAPSHOT_RECEIVE] {
            let m = exactly(len, snapshot);
            let b = uncapped(&m);
            assert_eq!(
                decode(Direction::RoomToClient, Channel::Datagram, &b),
                Ok(m),
                "{len} B"
            );
        }
        let b = uncapped(&exactly(Limits::SNAPSHOT_RECEIVE + 1, snapshot));
        assert_eq!(
            decode(Direction::RoomToClient, Channel::Datagram, &b),
            Err(DecodeError::TooLarge)
        );
        // The send cap (Limits::SNAPSHOT, its row's limit: 1,000 B at the send side until PM-0001-36 measures):
        // 1,000 B encodes, 1,001 B is TooLarge and appends nothing.
        let at = exactly(Limits::SNAPSHOT, snapshot);
        let mut out = vec![7];
        assert_eq!(encode(&at, &mut out), Ok(()));
        assert_eq!(out.len(), 1 + 1_000);
        let over = exactly(Limits::SNAPSHOT + 1, snapshot);
        let mut out = vec![7];
        assert_eq!(encode(&over, &mut out), Err(EncodeError::TooLarge));
        assert_eq!(out, [7], "nothing appended");
    }

    #[test]
    fn a3_a_full_state_is_sent_and_taken_up_to_65_536_b_after_its_length() {
        use npt_sim::{EntityKind, EntityRecord};
        // The one other type whose encoder can reach its limit (every other type's largest encoding is below its
        // row's limit), and its send and receive limits are one number: walls, and doors as the fillers, whole.
        let record = |kind: EntityKind, i: usize| {
            let mut r = EntityRecord {
                kind,
                index: i as u16,
                generation: 0,
                area: 0,
                grid: [0; EntityRecord::FIELDS],
                raw: [0; EntityRecord::FIELDS],
            };
            for (g, f) in r.grid.iter_mut().zip(kind.layout()) {
                *g = f.min();
            }
            r
        };
        let full_state = |n: usize, k: usize| {
            Message::FullState(FullState {
                entities: (0..n)
                    .map(|i| record(EntityKind::Wall, i))
                    .chain((0..k).map(|i| record(EntityKind::Door, i)))
                    .collect(),
                ..FullState::default()
            })
        };
        let at = exactly(4 + Limits::FULL_STATE, full_state);
        let over = exactly(4 + Limits::FULL_STATE + 1, full_state);
        let way = Direction::RoomToClient;
        assert_eq!(
            decode(way, Channel::FullState, &uncapped(&at)),
            Ok(at.clone())
        );
        assert_eq!(
            decode(way, Channel::FullState, &uncapped(&over)),
            Err(DecodeError::TooLarge)
        );
        let mut out = Vec::new();
        assert_eq!(encode(&at, &mut out), Ok(()));
        assert_eq!(out.len(), 4 + 65_536);
        let mut out = vec![7];
        assert_eq!(encode(&over, &mut out), Err(EncodeError::TooLarge));
        assert_eq!(out, [7], "nothing appended");
    }
}
