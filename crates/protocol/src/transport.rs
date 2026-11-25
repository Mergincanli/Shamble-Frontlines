//! The thin transport interface (ADR 0001 part 9): four operations, the same for every backend. The loopback is the
//! first backend; the link simulator and, online, WebTransport and the WebSocket fallback are others. Whoever holds
//! an end sends and receives whole messages as bytes; the simulation never sees them.

/// Which end of a link.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Client,
    Server,
}

/// A named reliable, ordered channel. The control channel carries the join exchange; others join with the messages
/// that need them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Control,
}

/// How a received message travelled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    /// Unreliable and unordered: it may be lost, repeated or overtaken.
    Datagram,
    /// Reliable and in order on its channel.
    Reliable(Channel),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkState {
    Open,
    Closed,
}

/// Why a message was not sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendError {
    /// The link holds as much as it can; a later send may work.
    Full,
    /// The message is longer than the link ever takes.
    TooLarge,
    /// The link is closed.
    Closed,
}

/// One end of a link.
pub trait Transport {
    /// Sends a datagram.
    ///
    /// # Errors
    /// When the link does not take it.
    fn send_unreliable(&mut self, bytes: &[u8]) -> Result<(), SendError>;

    /// Sends a message on a reliable channel.
    ///
    /// # Errors
    /// When the link does not take it.
    fn send_reliable(&mut self, channel: Channel, bytes: &[u8]) -> Result<(), SendError>;

    /// Takes the next message that has arrived into `out`, replacing what it held, and says how it travelled; none
    /// when nothing waits, and `out` is then left as it was.
    fn receive(&mut self, out: &mut Vec<u8>) -> Option<Delivery>;

    fn link_state(&self) -> LinkState;

    /// The link's round-trip time in whole microseconds, when the backend measures one.
    fn rtt_us(&self) -> Option<u32>;
}

/// Both ends of a link in one object, for a single process that drives each side in turn: local play's one step,
/// and the link simulator's tests.
pub trait TransportPair {
    type End<'a>: Transport
    where
        Self: 'a;

    /// The end that `side` holds.
    fn end(&mut self, side: Side) -> Self::End<'_>;
}
