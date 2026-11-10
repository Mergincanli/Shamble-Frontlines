//! The controls: `SessionControl` and its four variants, which a browser holding its slot's capability sends on its
//! control stream, `SessionStatus`, the room's answer for that browser's overlay (catalogue §2, §4; plan §10.2), and
//! `RoomControl`, the room-wide controls that never travel on the wire, only on the room's local sockets (catalogue
//! §5; the advisor's Q7 ruling: built here, unfeatured, for catalogue item 6 and U5's fuzz target).
//!
//! **`SessionControl`'s body** (after the type): variant 1 B (0 `SetProfile`, 1 `TriggerBurst`, 2 `BlockUdp`,
//! 3 `RecordStep`), the 16 B capability, then the variant's fields: a profile 1 B (0 to 4 for P0 to P4, 5 for
//! `default`); nothing; `on` 1 B (0 or 1); step 1 B (1 to 14), result 1 B (0 achieved, 1 not yet, 2 observation), the
//! note's length 2 B and the note, at most 480 B of UTF-8 with no control (Cc) or format (Cf) character (catalogue §4:
//! part 11's rule for display names). The capability is compared in constant time by npt-server, never here.
//!
//! **`SessionStatus`'s body:** the profile in force 1 B (as above), the server's round trip in µs 4 B, the rewind cap
//! in ticks 2 B, the command-timing option 1 B (0 for OQ-0001-6's (a), 1 for (b)), the clipped-shot count 4 B, the
//! form's last saved step 1 B (0 for none, else 1 to 14): 16 B with its framing, inside its 32 B.
//!
//! **A room-wide control** on its socket: a 2 B little-endian length of what follows, a 1 B control code (1 to 11 in
//! [`RoomControl`]'s order), then its fields, at most [`Limits::ROOM_CONTROL`] bytes in all, decoded with the same
//! checks as a network message and never a panic. A peer set is a count 1 B (1 to 16; 255 for `default` or `all`
//! where the control takes one), then per address a family 1 B (4 or 6) and 16 B: an IPv4 address in its first 4,
//! an IPv6 /64 prefix in its first 8, every other byte zero. Replies (a status code, and the minted token or
//! capability) are npt-server's `ControlSocket`'s, at U6.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::bits::{BitReader, BitWriter};
use crate::error::{DecodeError, EncodeError};
use crate::limits::Limits;

/// A network profile a control names: P0 to P4 (npt-l1's and L2's profiles), or `Default` to clear the holder's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProfileId {
    P0,
    P1,
    P2,
    P3,
    P4,
    /// The profile each new peer gets (`SetProfile`'s `default`).
    Default,
}

/// `RecordStep`'s result (the checklist's form: catalogue §4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepResult {
    Achieved,
    NotYet,
    Observation,
}

/// A secret's `Debug` form: its length, never its bytes (catalogue §2's notes: a log records a length or a hash of
/// client data, never the data; plan E22). `Join`'s token and bind and `SessionControl`'s capability print as this.
pub(crate) struct Redacted(pub(crate) usize);

impl core::fmt::Debug for Redacted {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{} B]", self.0)
    }
}

/// `SessionControl`: a control from a joined browser session holding its slot's capability (catalogue §4). Its
/// `Debug` shows the capability's length only.
#[derive(Clone, PartialEq, Eq)]
pub enum SessionControl {
    /// Applies `profile` to every connection from the holder's source address.
    SetProfile { cap: [u8; 16], profile: ProfileId },
    /// One second of total loss on this live session, both ways.
    TriggerBurst { cap: [u8; 16] },
    /// Every UDP datagram to and from the holder's address dropped while `on`.
    BlockUdp { cap: [u8; 16], on: bool },
    /// One form record: the checklist's `step` (1 to 14), its `result` and a `note`.
    RecordStep {
        cap: [u8; 16],
        step: u8,
        result: StepResult,
        note: String,
    },
}

impl core::fmt::Debug for SessionControl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SessionControl::SetProfile { cap, profile } => f
                .debug_struct("SetProfile")
                .field("cap", &Redacted(cap.len()))
                .field("profile", profile)
                .finish(),
            SessionControl::TriggerBurst { cap } => f
                .debug_struct("TriggerBurst")
                .field("cap", &Redacted(cap.len()))
                .finish(),
            SessionControl::BlockUdp { cap, on } => f
                .debug_struct("BlockUdp")
                .field("cap", &Redacted(cap.len()))
                .field("on", on)
                .finish(),
            SessionControl::RecordStep {
                cap,
                step,
                result,
                note,
            } => f
                .debug_struct("RecordStep")
                .field("cap", &Redacted(cap.len()))
                .field("step", step)
                .field("result", result)
                .field("note", note)
                .finish(),
        }
    }
}

/// The overlay's room-side values for the capability holder (catalogue §2, the advisor's A1 ruling).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SessionStatus {
    pub profile: ProfileId,
    /// The server's round trip to this client, in µs.
    pub rtt_us: u32,
    /// The rewind cap in force, in ticks.
    pub rewind_cap: u16,
    /// OQ-0001-6's command-timing option: 0 for (a), 1 for (b).
    pub cmd_timing: u8,
    /// Shots whose rewind the cap clipped.
    pub clipped: u32,
    /// The form's last saved step: 0 for none, else 1 to 14.
    pub last_step: u8,
}

impl Default for SessionStatus {
    fn default() -> SessionStatus {
        SessionStatus {
            profile: ProfileId::Default,
            rtt_us: 0,
            rewind_cap: 0,
            cmd_timing: 0,
            clipped: 0,
            last_step: 0,
        }
    }
}

fn profile_code(p: ProfileId) -> u64 {
    p as u64
}

fn profile_of(code: u8) -> Result<ProfileId, DecodeError> {
    Ok(match code {
        0 => ProfileId::P0,
        1 => ProfileId::P1,
        2 => ProfileId::P2,
        3 => ProfileId::P3,
        4 => ProfileId::P4,
        5 => ProfileId::Default,
        _ => return Err(DecodeError::OutOfRange),
    })
}

fn bool_of(code: u8) -> Result<bool, DecodeError> {
    match code {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(DecodeError::OutOfRange),
    }
}

/// Unicode's format characters, general category Cf (Unicode 15.1.0, as Python 3's unicodedata lists them).
const FORMAT: [(u32, u32); 21] = [
    (0x00ad, 0x00ad),
    (0x0600, 0x0605),
    (0x061c, 0x061c),
    (0x06dd, 0x06dd),
    (0x070f, 0x070f),
    (0x0890, 0x0891),
    (0x08e2, 0x08e2),
    (0x180e, 0x180e),
    (0x200b, 0x200f),
    (0x202a, 0x202e),
    (0x2060, 0x2064),
    (0x2066, 0x206f),
    (0xfeff, 0xfeff),
    (0xfff9, 0xfffb),
    (0x110bd, 0x110bd),
    (0x110cd, 0x110cd),
    (0x13430, 0x1343f),
    (0x1bca0, 0x1bca3),
    (0x1d173, 0x1d17a),
    (0xe0001, 0xe0001),
    (0xe0020, 0xe007f),
];

/// Whether `note` is one the form takes: at most 480 B, with no control (Cc) or format (Cf) character.
fn note_ok(note: &str) -> bool {
    note.len() <= Limits::NOTE
        && note.chars().all(|c| {
            let u = u32::from(c);
            !c.is_control() && !FORMAT.iter().any(|&(a, b)| (a..=b).contains(&u))
        })
}

impl SessionControl {
    /// The variants' names, in the catalogue's "Session controls" order.
    pub const VARIANTS: [&'static str; 4] =
        ["SetProfile", "TriggerBurst", "BlockUdp", "RecordStep"];

    /// This control's variant name, one of [`SessionControl::VARIANTS`].
    pub fn variant(&self) -> &'static str {
        Self::VARIANTS[self.code()]
    }

    fn code(&self) -> usize {
        match self {
            SessionControl::SetProfile { .. } => 0,
            SessionControl::TriggerBurst { .. } => 1,
            SessionControl::BlockUdp { .. } => 2,
            SessionControl::RecordStep { .. } => 3,
        }
    }

    pub(crate) fn put(&self, w: &mut BitWriter<'_>) -> Result<(), EncodeError> {
        w.put(self.code() as u64, 8);
        match self {
            SessionControl::SetProfile { cap, profile } => {
                w.bytes(cap);
                w.put(profile_code(*profile), 8);
            }
            SessionControl::TriggerBurst { cap } => w.bytes(cap),
            SessionControl::BlockUdp { cap, on } => {
                w.bytes(cap);
                w.put(u64::from(*on), 8);
            }
            SessionControl::RecordStep {
                cap,
                step,
                result,
                note,
            } => {
                if !(1..=Limits::RECORD_STEPS).contains(step) || !note_ok(note) {
                    return Err(EncodeError::OutOfRange);
                }
                w.bytes(cap);
                w.put(u64::from(*step), 8);
                w.put(*result as u64, 8);
                w.put(note.len() as u64, 16);
                w.bytes(note.as_bytes());
            }
        }
        Ok(())
    }

    pub(crate) fn get(r: &mut BitReader<'_>) -> Result<SessionControl, DecodeError> {
        let variant = r.u8()?;
        let cap = r.array()?;
        Ok(match variant {
            0 => SessionControl::SetProfile {
                cap,
                profile: profile_of(r.u8()?)?,
            },
            1 => SessionControl::TriggerBurst { cap },
            2 => SessionControl::BlockUdp {
                cap,
                on: bool_of(r.u8()?)?,
            },
            3 => {
                let step = r.u8()?;
                if !(1..=Limits::RECORD_STEPS).contains(&step) {
                    return Err(DecodeError::OutOfRange);
                }
                let result = match r.u8()? {
                    0 => StepResult::Achieved,
                    1 => StepResult::NotYet,
                    2 => StepResult::Observation,
                    _ => return Err(DecodeError::OutOfRange),
                };
                let len = usize::from(r.u16()?);
                if len > Limits::NOTE {
                    return Err(DecodeError::OutOfRange);
                }
                let bytes = r.take(len)?;
                let note = core::str::from_utf8(bytes).map_err(|_| DecodeError::Malformed)?;
                if !note_ok(note) {
                    return Err(DecodeError::Malformed);
                }
                SessionControl::RecordStep {
                    cap,
                    step,
                    result,
                    note: note.to_owned(),
                }
            }
            _ => return Err(DecodeError::OutOfRange),
        })
    }
}

impl SessionStatus {
    pub(crate) fn put(&self, w: &mut BitWriter<'_>) -> Result<(), EncodeError> {
        if self.cmd_timing > 1 || self.last_step > Limits::RECORD_STEPS {
            return Err(EncodeError::OutOfRange);
        }
        w.put(profile_code(self.profile), 8);
        w.put(u64::from(self.rtt_us), 32);
        w.put(u64::from(self.rewind_cap), 16);
        w.put(u64::from(self.cmd_timing), 8);
        w.put(u64::from(self.clipped), 32);
        w.put(u64::from(self.last_step), 8);
        Ok(())
    }

    pub(crate) fn get(r: &mut BitReader<'_>) -> Result<SessionStatus, DecodeError> {
        let s = SessionStatus {
            profile: profile_of(r.u8()?)?,
            rtt_us: r.u32()?,
            rewind_cap: r.u16()?,
            cmd_timing: r.u8()?,
            clipped: r.u32()?,
            last_step: r.u8()?,
        };
        if s.cmd_timing > 1 || s.last_step > Limits::RECORD_STEPS {
            return Err(DecodeError::OutOfRange);
        }
        Ok(s)
    }
}

/// What kind of client a minted token is for (`MintToken`'s `kind`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Bot,
    Browser,
    Runner,
}

/// A room-wide control (catalogue §5), from the run script on RS or the laptop session over SSH, on the room slot's
/// local socket. Each field's check that needs no room state is the decoder's; the rest (a slot below the room's N,
/// a tick in the future, an address on the run's lists) are npt-server's and npt-room's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoomControl {
    MintToken {
        slot: u8,
        kind: TokenKind,
    },
    MintCapability {
        slot: u8,
    },
    /// At least 1.
    SetRewindCap {
        ticks: u16,
    },
    /// IPv4 addresses and IPv6 /64 prefixes, 1 to 16.
    RaiseAddrCap {
        cap: u16,
        addrs: Vec<IpAddr>,
    },
    /// `target` `None` for the tick thread, else that slot's connection task (a slot below 255).
    InjectPanic {
        target: Option<u8>,
        at: u32,
    },
    /// At most 1,000 ms.
    InjectStall {
        ms: u16,
        at: u32,
    },
    /// `peers` `None` for `default`; `profile` P0 to P4.
    SetPeerProfile {
        peers: Option<Vec<IpAddr>>,
        profile: ProfileId,
    },
    /// `kbps` above 0, `secs` at most 600.
    SetPeerThrottle {
        peers: Vec<IpAddr>,
        kbps: u16,
        secs: u16,
    },
    /// `percent` 1 to 100, `secs` at most 600.
    SetPeerBottleneck {
        peers: Vec<IpAddr>,
        percent: u8,
        secs: u16,
    },
    /// At most 1,000 ms.
    PeerBurst {
        peers: Vec<IpAddr>,
        ms: u16,
    },
    /// `peers` `None` for `all`.
    BlockPeerUdp {
        peers: Option<Vec<IpAddr>>,
        on: bool,
    },
}

/// A peer-set count that stands for `default` or `all`.
const EVERY_PEER: u8 = 255;

fn put_peers(w: &mut BitWriter<'_>, peers: Option<&[IpAddr]>) -> Result<(), EncodeError> {
    let Some(peers) = peers else {
        w.put(u64::from(EVERY_PEER), 8);
        return Ok(());
    };
    if peers.is_empty() {
        return Err(EncodeError::OutOfRange);
    }
    if peers.len() > Limits::PEERS {
        return Err(EncodeError::TooMany);
    }
    w.put(peers.len() as u64, 8);
    for p in peers {
        let mut bytes = [0u8; 16];
        match p {
            IpAddr::V4(a) => {
                w.put(4, 8);
                bytes[..4].copy_from_slice(&a.octets());
            }
            IpAddr::V6(a) => {
                let o = a.octets();
                if o[8..].iter().any(|&b| b != 0) {
                    return Err(EncodeError::OutOfRange);
                }
                w.put(6, 8);
                bytes = o;
            }
        }
        w.bytes(&bytes);
    }
    Ok(())
}

/// A peer set: `None` for the count that stands for every peer, where `every` allows it.
fn get_peers(r: &mut BitReader<'_>, every: bool) -> Result<Option<Vec<IpAddr>>, DecodeError> {
    let n = r.u8()?;
    if n == EVERY_PEER && every {
        return Ok(None);
    }
    if n == 0 || usize::from(n) > Limits::PEERS {
        return Err(DecodeError::OutOfRange);
    }
    let mut out = Vec::with_capacity(usize::from(n));
    for _ in 0..n {
        let family = r.u8()?;
        let b: [u8; 16] = r.array()?;
        let addr = match family {
            4 if b[4..].iter().all(|&x| x == 0) => {
                IpAddr::V4(Ipv4Addr::new(b[0], b[1], b[2], b[3]))
            }
            6 if b[8..].iter().all(|&x| x == 0) => IpAddr::V6(Ipv6Addr::from(b)),
            4 | 6 => return Err(DecodeError::UnknownField),
            _ => return Err(DecodeError::OutOfRange),
        };
        out.push(addr);
    }
    Ok(Some(out))
}

fn listed(r: &mut BitReader<'_>) -> Result<Vec<IpAddr>, DecodeError> {
    get_peers(r, false)?.ok_or(DecodeError::OutOfRange)
}

impl RoomControl {
    /// The controls' names, in the catalogue's "Room-wide controls" order less `npt-load-ctl` (not a room socket's);
    /// the control code is the position, from 1.
    pub const NAMES: [&'static str; 11] = [
        "MintToken",
        "MintCapability",
        "SetRewindCap",
        "RaiseAddrCap",
        "InjectPanic",
        "InjectStall",
        "SetPeerProfile",
        "SetPeerThrottle",
        "SetPeerBottleneck",
        "PeerBurst",
        "BlockPeerUdp",
    ];

    /// This control's name, one of [`RoomControl::NAMES`].
    pub fn name(&self) -> &'static str {
        Self::NAMES[self.code() - 1]
    }

    fn code(&self) -> usize {
        match self {
            RoomControl::MintToken { .. } => 1,
            RoomControl::MintCapability { .. } => 2,
            RoomControl::SetRewindCap { .. } => 3,
            RoomControl::RaiseAddrCap { .. } => 4,
            RoomControl::InjectPanic { .. } => 5,
            RoomControl::InjectStall { .. } => 6,
            RoomControl::SetPeerProfile { .. } => 7,
            RoomControl::SetPeerThrottle { .. } => 8,
            RoomControl::SetPeerBottleneck { .. } => 9,
            RoomControl::PeerBurst { .. } => 10,
            RoomControl::BlockPeerUdp { .. } => 11,
        }
    }

    /// Appends this control, framed for its socket, to `out`.
    ///
    /// # Errors
    /// [`EncodeError`] for a value its field does not take (the module text); nothing is appended then.
    pub fn encode(&self, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        let start = out.len();
        let r = self.encode_into(out);
        match r {
            Ok(()) if out.len() - start <= Limits::ROOM_CONTROL => {
                let length = (out.len() - start - 2) as u16;
                out[start..start + 2].copy_from_slice(&length.to_le_bytes());
                Ok(())
            }
            Ok(()) => {
                out.truncate(start);
                Err(EncodeError::TooLarge)
            }
            Err(e) => {
                out.truncate(start);
                Err(e)
            }
        }
    }

    fn encode_into(&self, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        out.extend_from_slice(&[0, 0, self.code() as u8]);
        let mut w = BitWriter::new(out);
        match self {
            RoomControl::MintToken { slot, kind } => {
                w.put(u64::from(*slot), 8);
                w.put(*kind as u64, 8);
            }
            RoomControl::MintCapability { slot } => w.put(u64::from(*slot), 8),
            RoomControl::SetRewindCap { ticks } => {
                if *ticks == 0 {
                    return Err(EncodeError::OutOfRange);
                }
                w.put(u64::from(*ticks), 16);
            }
            RoomControl::RaiseAddrCap { cap, addrs } => {
                w.put(u64::from(*cap), 16);
                put_peers(&mut w, Some(addrs))?;
            }
            RoomControl::InjectPanic { target, at } => {
                let t = match target {
                    None => EVERY_PEER,
                    Some(s) if *s != EVERY_PEER => *s,
                    Some(_) => return Err(EncodeError::OutOfRange),
                };
                w.put(u64::from(t), 8);
                w.put(u64::from(*at), 32);
            }
            RoomControl::InjectStall { ms, at } => {
                if *ms > 1_000 {
                    return Err(EncodeError::OutOfRange);
                }
                w.put(u64::from(*ms), 16);
                w.put(u64::from(*at), 32);
            }
            RoomControl::SetPeerProfile { peers, profile } => {
                if *profile == ProfileId::Default {
                    return Err(EncodeError::OutOfRange);
                }
                put_peers(&mut w, peers.as_deref())?;
                w.put(profile_code(*profile), 8);
            }
            RoomControl::SetPeerThrottle { peers, kbps, secs } => {
                if *kbps == 0 || *secs > 600 {
                    return Err(EncodeError::OutOfRange);
                }
                put_peers(&mut w, Some(peers))?;
                w.put(u64::from(*kbps), 16);
                w.put(u64::from(*secs), 16);
            }
            RoomControl::SetPeerBottleneck {
                peers,
                percent,
                secs,
            } => {
                if !(1..=100).contains(percent) || *secs > 600 {
                    return Err(EncodeError::OutOfRange);
                }
                put_peers(&mut w, Some(peers))?;
                w.put(u64::from(*percent), 8);
                w.put(u64::from(*secs), 16);
            }
            RoomControl::PeerBurst { peers, ms } => {
                if *ms > 1_000 {
                    return Err(EncodeError::OutOfRange);
                }
                put_peers(&mut w, Some(peers))?;
                w.put(u64::from(*ms), 16);
            }
            RoomControl::BlockPeerUdp { peers, on } => {
                put_peers(&mut w, peers.as_deref())?;
                w.put(u64::from(*on), 8);
            }
        }
        Ok(())
    }

    /// Decodes one control as its socket delivered it, length prefix included: the size first (at most
    /// [`Limits::ROOM_CONTROL`] bytes), then the prefix, the code and the fields, which must end at the last byte.
    ///
    /// # Errors
    /// [`DecodeError`] for anything that is not one valid control.
    pub fn decode(bytes: &[u8]) -> Result<RoomControl, DecodeError> {
        if bytes.len() > Limits::ROOM_CONTROL {
            return Err(DecodeError::TooLarge);
        }
        let mut r = BitReader::new(bytes);
        let length = usize::from(r.u16()?);
        if length != bytes.len() - 2 {
            return Err(DecodeError::BadLength);
        }
        let code = r.u8()?;
        let c = match code {
            1 => RoomControl::MintToken {
                slot: r.u8()?,
                kind: match r.u8()? {
                    0 => TokenKind::Bot,
                    1 => TokenKind::Browser,
                    2 => TokenKind::Runner,
                    _ => return Err(DecodeError::OutOfRange),
                },
            },
            2 => RoomControl::MintCapability { slot: r.u8()? },
            3 => match r.u16()? {
                0 => return Err(DecodeError::OutOfRange),
                ticks => RoomControl::SetRewindCap { ticks },
            },
            4 => {
                let cap = r.u16()?;
                RoomControl::RaiseAddrCap {
                    cap,
                    addrs: listed(&mut r)?,
                }
            }
            5 => {
                let t = r.u8()?;
                RoomControl::InjectPanic {
                    target: (t != EVERY_PEER).then_some(t),
                    at: r.u32()?,
                }
            }
            6 => {
                let ms = r.u16()?;
                if ms > 1_000 {
                    return Err(DecodeError::OutOfRange);
                }
                RoomControl::InjectStall { ms, at: r.u32()? }
            }
            7 => {
                let peers = get_peers(&mut r, true)?;
                let profile = profile_of(r.u8()?)?;
                if profile == ProfileId::Default {
                    return Err(DecodeError::OutOfRange);
                }
                RoomControl::SetPeerProfile { peers, profile }
            }
            8 => {
                let peers = listed(&mut r)?;
                let (kbps, secs) = (r.u16()?, r.u16()?);
                if kbps == 0 || secs > 600 {
                    return Err(DecodeError::OutOfRange);
                }
                RoomControl::SetPeerThrottle { peers, kbps, secs }
            }
            9 => {
                let peers = listed(&mut r)?;
                let (percent, secs) = (r.u8()?, r.u16()?);
                if !(1..=100).contains(&percent) || secs > 600 {
                    return Err(DecodeError::OutOfRange);
                }
                RoomControl::SetPeerBottleneck {
                    peers,
                    percent,
                    secs,
                }
            }
            10 => {
                let peers = listed(&mut r)?;
                let ms = r.u16()?;
                if ms > 1_000 {
                    return Err(DecodeError::OutOfRange);
                }
                RoomControl::PeerBurst { peers, ms }
            }
            11 => {
                let peers = get_peers(&mut r, true)?;
                RoomControl::BlockPeerUdp {
                    peers,
                    on: bool_of(r.u8()?)?,
                }
            }
            _ => return Err(DecodeError::UnknownType),
        };
        r.finish(true)?;
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_note_with_a_control_or_format_character_is_refused() {
        assert!(note_ok("saw a rubber-band at 0:42, café"));
        for bad in [
            "tab\there",
            "nul\u{0}",
            "del\u{7f}",
            "c1\u{85}",
            "rlo\u{202e}",
            "zwsp\u{200b}",
            "bom\u{feff}",
            "tag\u{e0041}",
        ] {
            assert!(!note_ok(bad), "{bad:?}");
        }
        assert!(note_ok(&"é".repeat(240)));
        assert!(!note_ok(&"a".repeat(481)));
    }
}
