//! npt-link: the thin transport interface of the netcode proof test (plan §4.7; ADR 0001:431-435; ADR 0002:188).
//!
//! Poll-based, and every call that can move time takes `now_ns`, nanoseconds on the caller's clock: npt-lab passes its
//! simulated clock (L1), npt-server its monotonic clock (L2). Nothing here reads a clock, spawns a task or opens a
//! socket, so the crate builds for wasm32. A session has three channels (ADR 0001:425-427): datagrams, the
//! bidirectional control stream, and the unidirectional stream that carries the first full state. The backends are
//! [`Loopback`] here (the same encoded bytes between a client and a local server in one thread, SLICE-ORDER
//! condition 1), L1 in npt-l1, and WebTransport and WebSocket in npt-net.
//!
//! Dropping a link closes it: the other end's [`Link::state`] becomes [`LinkState::Closed`]. A send on a closed link is
//! dropped and reports [`SendResult::Sent`], as a datagram to a vanished peer would be; `state` is how a caller learns
//! that the session is gone.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::rc::Rc;

/// A session's channels (ADR 0001:425-427).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Channel {
    /// Unreliable and unordered: inputs, snapshots, pings (binary messages on the WebSocket fallback).
    Datagram,
    /// The bidirectional control stream: reliable and ordered (room control, session controls).
    Control,
    /// The unidirectional stream of the first full state (and each Resync's): reliable and ordered.
    FullState,
}

/// What one send did. The caller counts each result: rate adaptation reads them, and so do PM-0001-14's and -24's
/// counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendResult {
    /// Handed to the transport.
    Sent,
    /// The send buffer (unreliable) or the stream's write side (reliable) is at its bound: nothing was queued.
    BufferFull,
    /// Larger than [`Link::max_datagram_size`] (unreliable) or the backend's message limit (reliable): nothing was
    /// sent.
    TooLarge,
}

/// Where a link is in its life.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkState {
    /// A client link whose handshake has not finished; sends are refused as `BufferFull` until it opens.
    Connecting,
    Open,
    /// Either end was dropped or the transport failed. Messages that had already arrived can still be polled.
    Closed,
}

/// One received message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recv {
    pub channel: Channel,
    pub bytes: Vec<u8>,
    /// When it arrived, on the caller's clock (L1: the simulated arrival instant; L2: the backend's stamp at the
    /// datagram's own event, ADR 0001 part 5), never the time it was polled.
    pub arrival_ns: u64,
}

/// One end of a session, seen from the room or from a client.
pub trait Link {
    /// Sends one datagram: unreliable, unordered, never retransmitted.
    fn send_unreliable(&mut self, bytes: &[u8], now_ns: u64) -> SendResult;

    /// Sends one message on a reliable, ordered stream: [`Channel::Control`] or [`Channel::FullState`].
    ///
    /// # Panics
    /// On [`Channel::Datagram`], which is a caller's bug: datagrams go through [`Link::send_unreliable`].
    fn send_reliable(&mut self, channel: Channel, bytes: &[u8], now_ns: u64) -> SendResult;

    /// The next message that has arrived by `now_ns`, on any channel, in arrival order (each stream in its own order).
    fn poll_recv(&mut self, now_ns: u64) -> Option<Recv>;

    fn state(&self) -> LinkState;

    /// The backend's smoothed round-trip estimate as of the last call that took `now_ns`; `None` before its first
    /// sample.
    fn rtt_ns(&self) -> Option<u64>;

    /// The largest datagram [`Link::send_unreliable`] takes now. It may change during a session (PM-0001-36).
    fn max_datagram_size(&self) -> usize;

    /// Datagrams handed to the backend and not yet on the wire.
    fn unsent_unreliable(&self) -> usize;
}

/// The server side of a transport: yields each new session with its peer's address, which the per-address limits key
/// on (plan §4.4).
pub trait Listener {
    fn poll_accept(&mut self, now_ns: u64) -> Option<(Box<dyn Link>, SocketAddr)>;
}

/// A boxed link is a link, so a room can hold `Box<dyn Link>` from any backend.
impl<L: Link + ?Sized> Link for Box<L> {
    fn send_unreliable(&mut self, bytes: &[u8], now_ns: u64) -> SendResult {
        (**self).send_unreliable(bytes, now_ns)
    }
    fn send_reliable(&mut self, channel: Channel, bytes: &[u8], now_ns: u64) -> SendResult {
        (**self).send_reliable(channel, bytes, now_ns)
    }
    fn poll_recv(&mut self, now_ns: u64) -> Option<Recv> {
        (**self).poll_recv(now_ns)
    }
    fn state(&self) -> LinkState {
        (**self).state()
    }
    fn rtt_ns(&self) -> Option<u64> {
        (**self).rtt_ns()
    }
    fn max_datagram_size(&self) -> usize {
        (**self).max_datagram_size()
    }
    fn unsent_unreliable(&self) -> usize {
        (**self).unsent_unreliable()
    }
}

/// The message both ends share: what waits for each side, and which sides are still open.
struct Pipe {
    /// `waiting[i]`: messages sent to side `i`, in send order.
    waiting: [VecDeque<Recv>; 2],
    open: [bool; 2],
    max_datagram_size: usize,
}

/// One end of an in-memory pair: no delay, no loss, one order for all channels. Each message reaches the other end
/// stamped with its send time and can be polled once the caller's clock has reached that time. Both ends live in one
/// thread.
pub struct Loopback {
    pipe: Rc<RefCell<Pipe>>,
    side: usize,
}

impl Loopback {
    /// Two connected ends; datagrams above `max_datagram_size` bytes are refused as `TooLarge`.
    pub fn pair(max_datagram_size: usize) -> (Loopback, Loopback) {
        let pipe = Rc::new(RefCell::new(Pipe {
            waiting: [VecDeque::new(), VecDeque::new()],
            open: [true, true],
            max_datagram_size,
        }));
        (
            Loopback {
                pipe: Rc::clone(&pipe),
                side: 0,
            },
            Loopback { pipe, side: 1 },
        )
    }

    fn send(&mut self, channel: Channel, bytes: &[u8], now_ns: u64) -> SendResult {
        let mut pipe = self.pipe.borrow_mut();
        let other = 1 - self.side;
        if pipe.open[other] {
            pipe.waiting[other].push_back(Recv {
                channel,
                bytes: bytes.to_vec(),
                arrival_ns: now_ns,
            });
        }
        SendResult::Sent
    }
}

impl Link for Loopback {
    fn send_unreliable(&mut self, bytes: &[u8], now_ns: u64) -> SendResult {
        if bytes.len() > self.pipe.borrow().max_datagram_size {
            return SendResult::TooLarge;
        }
        self.send(Channel::Datagram, bytes, now_ns)
    }

    fn send_reliable(&mut self, channel: Channel, bytes: &[u8], now_ns: u64) -> SendResult {
        assert!(
            channel != Channel::Datagram,
            "send_reliable on Channel::Datagram: datagrams go through send_unreliable"
        );
        self.send(channel, bytes, now_ns)
    }

    fn poll_recv(&mut self, now_ns: u64) -> Option<Recv> {
        let mut pipe = self.pipe.borrow_mut();
        let queue = &mut pipe.waiting[self.side];
        if queue.front().is_some_and(|r| r.arrival_ns <= now_ns) {
            queue.pop_front()
        } else {
            None
        }
    }

    fn state(&self) -> LinkState {
        let pipe = self.pipe.borrow();
        if pipe.open[0] && pipe.open[1] {
            LinkState::Open
        } else {
            LinkState::Closed
        }
    }

    fn rtt_ns(&self) -> Option<u64> {
        Some(0)
    }

    fn max_datagram_size(&self) -> usize {
        self.pipe.borrow().max_datagram_size
    }

    fn unsent_unreliable(&self) -> usize {
        0
    }
}

impl Drop for Loopback {
    fn drop(&mut self) {
        let mut pipe = self.pipe.borrow_mut();
        pipe.open[self.side] = false;
        pipe.waiting[self.side].clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drain(l: &mut dyn Link, now: u64) -> Vec<Recv> {
        let mut out = Vec::new();
        while let Some(r) = l.poll_recv(now) {
            out.push(r);
        }
        out
    }

    #[test]
    fn loopback_keeps_one_order_across_channels_and_stamps_send_times() {
        let (mut a, mut b) = Loopback::pair(1200);
        assert_eq!(a.send_unreliable(b"d1", 10), SendResult::Sent);
        assert_eq!(
            a.send_reliable(Channel::Control, b"c1", 20),
            SendResult::Sent
        );
        assert_eq!(
            a.send_reliable(Channel::FullState, b"f1", 30),
            SendResult::Sent
        );
        assert_eq!(a.send_unreliable(b"d2", 40), SendResult::Sent);
        let got = drain(&mut b, 40);
        let seen: Vec<(Channel, &[u8], u64)> = got
            .iter()
            .map(|r| (r.channel, r.bytes.as_slice(), r.arrival_ns))
            .collect();
        assert_eq!(
            seen,
            vec![
                (Channel::Datagram, &b"d1"[..], 10),
                (Channel::Control, &b"c1"[..], 20),
                (Channel::FullState, &b"f1"[..], 30),
                (Channel::Datagram, &b"d2"[..], 40),
            ]
        );
        assert!(drain(&mut a, 40).is_empty(), "nothing was sent to a");
    }

    #[test]
    fn loopback_holds_a_message_until_the_callers_clock_reaches_its_time() {
        let (mut a, mut b) = Loopback::pair(1200);
        a.send_unreliable(b"x", 100);
        assert!(b.poll_recv(99).is_none());
        assert_eq!(b.poll_recv(100).map(|r| r.bytes), Some(b"x".to_vec()));
    }

    #[test]
    fn loopback_refuses_a_datagram_above_its_size_and_sends_one_at_it() {
        let (mut a, mut b) = Loopback::pair(4);
        assert_eq!(a.send_unreliable(b"12345", 0), SendResult::TooLarge);
        assert_eq!(a.send_unreliable(b"1234", 0), SendResult::Sent);
        assert_eq!(drain(&mut b, 0).len(), 1, "the refused datagram never went");
        assert_eq!(a.max_datagram_size(), 4);
        assert_eq!(a.unsent_unreliable(), 0);
        assert_eq!(a.rtt_ns(), Some(0));
    }

    #[test]
    fn dropping_an_end_closes_the_link_and_later_sends_go_nowhere() {
        let (mut a, b) = Loopback::pair(1200);
        assert_eq!(a.state(), LinkState::Open);
        drop(b);
        assert_eq!(a.state(), LinkState::Closed);
        assert_eq!(a.send_unreliable(b"late", 0), SendResult::Sent);
        assert_eq!(
            a.pipe.borrow().waiting[1].len(),
            0,
            "no queue grows for a dropped end"
        );
    }

    #[test]
    fn messages_that_arrived_before_the_peer_closed_can_still_be_polled() {
        let (mut a, mut b) = Loopback::pair(1200);
        a.send_reliable(Channel::Control, b"bye", 5);
        drop(a);
        assert_eq!(b.state(), LinkState::Closed);
        assert_eq!(b.poll_recv(5).map(|r| r.bytes), Some(b"bye".to_vec()));
    }

    #[test]
    #[should_panic(expected = "send_reliable on Channel::Datagram")]
    fn a_reliable_send_on_the_datagram_channel_is_a_bug() {
        let (mut a, _b) = Loopback::pair(1200);
        a.send_reliable(Channel::Datagram, b"x", 0);
    }

    #[test]
    fn a_boxed_link_is_a_link() {
        let (a, b) = Loopback::pair(1200);
        let mut a: Box<dyn Link> = Box::new(a);
        let mut b: Box<dyn Link> = Box::new(b);
        fn send_one<L: Link>(l: &mut L) -> SendResult {
            l.send_unreliable(b"boxed", 7)
        }
        assert_eq!(send_one(&mut a), SendResult::Sent);
        assert_eq!(b.poll_recv(7).map(|r| r.arrival_ns), Some(7));
        assert_eq!(b.state(), LinkState::Open);
    }
}
