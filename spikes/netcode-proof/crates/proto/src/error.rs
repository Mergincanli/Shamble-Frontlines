//! Why a buffer is not a message, and why a value cannot be sent.

/// Why [`crate::decode`] refused a buffer. Every decoder returns one of these and never panics (ADR 0001:446); the
/// size is checked before anything else is read (catalogue item 3, the size half), and nothing is allocated before the
/// size check passes.
///
/// npt-server's pump counts each refusal by its kind (PM-0001-21's counters): `TrailingBytes`, `UnknownField` and
/// `WrongChannel` are the socket checks of plan §4.4's server column, implemented here (u4-build-plan.md §4.4, A26).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// Larger than what the receiving end takes on that channel, or than the type's own receive limit
    /// ([`crate::Limits`]; `Snapshot`'s is [`crate::Limits::SNAPSHOT_RECEIVE`]), whatever the bytes hold.
    TooLarge,
    /// The bytes end inside the framing, the header or a field.
    Truncated,
    /// A stream message whose length prefix does not count its bytes.
    BadLength,
    /// A type byte no message has.
    UnknownType,
    /// A type the receiving end never receives: a room → client type from a client, or the reverse (refused and
    /// counted as forged: the catalogue's `Snapshot` row).
    WrongDirection,
    /// A known type on a channel it does not travel on (catalogue item 2; plan E7).
    WrongChannel,
    /// Bytes after the message's last field (ADR 0001:238: the datagram is dropped whole).
    TrailingBytes,
    /// A field the format does not have: a reserved or pad bit set (ADR 0001:238), or a value whose flag says it is
    /// absent (a snapshot's command sequence, an `Input`'s acknowledged tick with no group) other than 0.
    UnknownField,
    /// A field's code outside its range: an integer its grid field does not hold (pitch's 10-bit code -512), an enum
    /// code no variant has, a count above its cap, a step outside 1 to 14.
    OutOfRange,
    /// Well framed, but not a message: a group index not below its count, an entity update that changes nothing, a
    /// note that is not UTF-8 or holds a control or format character, a delta whose baseline entity is missing.
    Malformed,
}

/// Why [`crate::encode`] could not code a message: a value the wire does not carry. The sender's bug, never the
/// network's; nothing is written when it is returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncodeError {
    /// A value outside what its field carries: an integer outside its grid field's range, a view time more than
    /// 65,535 ticks before its command, a repeat more than 255 ticks before the command beside it, a group index not
    /// below the group count, a note above 480 B, a step outside 1 to 14.
    OutOfRange,
    /// More items than the format holds: commands above the redundancy cap, a group count above 8, more entities or
    /// events than their counts carry, more than 16 peers.
    TooMany,
    /// The encoding is above its type's send limit ([`crate::Limits::TYPES`]; `Snapshot`'s is the 1,000 B send cap
    /// [`crate::Limits::SNAPSHOT`], below the 1,150 B a client takes).
    TooLarge,
    /// A value the format codes in one form only, given in another: a repeat whose sequence is not the one before the
    /// command beside it, an entity update that changes nothing or sets an entry it does not mark changed, an
    /// `Input`'s acknowledged tick without a group (no snapshot decoded is a tick of 0).
    NotCanonical,
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for DecodeError {}
impl std::error::Error for EncodeError {}
