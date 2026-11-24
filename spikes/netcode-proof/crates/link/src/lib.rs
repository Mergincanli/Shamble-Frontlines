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
//!
//! **`Send`** (T6, U6a): `Link` itself has no `Send` bound, because npt-l1's links share one simulator in one thread
//! (`Rc<RefCell<..>>`) and stay there. A link that may move to another thread is a `Box<dyn Link + Send>`: npt-server's
//! I/O runtime holds its sessions' links that way. [`Loopback`] is `Send` (its two ends share one `Arc<Mutex<..>>`), so
//! a test can run the room's end on the runtime and the client's end on the test's thread.

use std::collections::VecDeque;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

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

impl Channel {
    /// The channel's 1 B tag before each binary message on the WebSocket fallback (catalogue :73-76): `Datagram` 0,
    /// `Control` 1, `FullState` 2. One home for the server, the native clients and U7's page (advisor-u7-brief.md C2).
    pub fn ws_tag(self) -> u8 {
        match self {
            Channel::Datagram => 0,
            Channel::Control => 1,
            Channel::FullState => 2,
        }
    }

    /// The channel a WebSocket message's tag names; `None` for any other tag (the session closes as malformed).
    pub fn from_ws_tag(tag: u8) -> Option<Channel> {
        match tag {
            0 => Some(Channel::Datagram),
            1 => Some(Channel::Control),
            2 => Some(Channel::FullState),
            _ => None,
        }
    }
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

    /// The start of one tick's datagrams (W18; u6b-brief.md Q-U6B-1 (a)): `lens` holds each datagram's length, in the
    /// order they follow through [`Link::send_unreliable`].
    ///
    /// **The caller's duty:** call it once per tick, before that tick's datagram sends, and then send exactly those
    /// datagrams, in that order, with no other datagram between them (an empty `lens` announces nothing). A backend
    /// that gates a tick's whole snapshot (the WebSocket fallback's fix 1) answers `BufferFull` for a tick it skips,
    /// and then answers each of that tick's datagrams `BufferFull` too; every other backend keeps this default, `Sent`,
    /// and changes nothing. Reliable sends are never gated by it.
    ///
    /// **Outside a tick** (none announced, or past its count: the room's own Pings and Pongs, R-PING), that backend
    /// never holds a datagram to the snapshot budget and never writes one ungated: only one at most the largest Ping or
    /// Pong, with nothing owed below its own send queue and room in the socket; else `BufferFull`, both counted.
    fn begin_tick(&mut self, lens: &[usize], now_ns: u64) -> SendResult {
        let _ = (lens, now_ns);
        SendResult::Sent
    }
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
    fn begin_tick(&mut self, lens: &[usize], now_ns: u64) -> SendResult {
        (**self).begin_tick(lens, now_ns)
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
/// stamped with its send time and can be polled once the caller's clock has reached that time. The ends are `Send`:
/// each may live on its own thread, each call taking the pair's one lock for its own length only.
pub struct Loopback {
    pipe: Arc<Mutex<Pipe>>,
    side: usize,
}

impl Loopback {
    /// Two connected ends; datagrams above `max_datagram_size` bytes are refused as `TooLarge`.
    pub fn pair(max_datagram_size: usize) -> (Loopback, Loopback) {
        let pipe = Arc::new(Mutex::new(Pipe {
            waiting: [VecDeque::new(), VecDeque::new()],
            open: [true, true],
            max_datagram_size,
        }));
        (
            Loopback {
                pipe: Arc::clone(&pipe),
                side: 0,
            },
            Loopback { pipe, side: 1 },
        )
    }

    /// The pair's shared state. No call panics while it holds the lock, so a poisoned lock (a panic on another
    /// thread between two calls) still holds a whole `Pipe`, which is taken as it is.
    fn lock(&self) -> MutexGuard<'_, Pipe> {
        self.pipe.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn send(&mut self, channel: Channel, bytes: &[u8], now_ns: u64) -> SendResult {
        let mut pipe = self.lock();
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
        if bytes.len() > self.lock().max_datagram_size {
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
        let mut pipe = self.lock();
        let queue = &mut pipe.waiting[self.side];
        if queue.front().is_some_and(|r| r.arrival_ns <= now_ns) {
            queue.pop_front()
        } else {
            None
        }
    }

    fn state(&self) -> LinkState {
        let pipe = self.lock();
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
        self.lock().max_datagram_size
    }

    fn unsent_unreliable(&self) -> usize {
        0
    }
}

impl Drop for Loopback {
    fn drop(&mut self) {
        let side = self.side;
        let mut pipe = self.lock();
        pipe.open[side] = false;
        pipe.waiting[side].clear();
    }
}

/// T6's compile-time asserts: a Loopback end and a boxed `Send` link may move to another thread.
const _: () = {
    const fn is_send<T: Send + ?Sized>() {}
    is_send::<Loopback>();
    is_send::<Box<dyn Link + Send>>();
};

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
            a.lock().waiting[1].len(),
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

    /// T6: a boxed `Send` link moves to another thread, which answers every message on the other end's channel and
    /// time; both ends keep one order, and dropping the moved end closes the pair for the end left behind.
    #[test]
    fn t6_a_loopback_end_runs_on_another_thread() {
        let (a, mut b) = Loopback::pair(1200);
        let a: Box<dyn Link + Send> = Box::new(a);
        let echo = std::thread::spawn(move || {
            let mut a = a;
            let mut seen = Vec::new();
            while seen.len() < 3 {
                if let Some(r) = a.poll_recv(u64::MAX) {
                    let reply = [r.bytes.as_slice(), b"!"].concat();
                    let sent = match r.channel {
                        Channel::Datagram => a.send_unreliable(&reply, r.arrival_ns + 1),
                        ch => a.send_reliable(ch, &reply, r.arrival_ns + 1),
                    };
                    assert_eq!(sent, SendResult::Sent);
                    seen.push(r.channel);
                } else {
                    std::thread::yield_now();
                }
            }
            seen
        });
        b.send_unreliable(b"d", 10);
        b.send_reliable(Channel::Control, b"c", 20);
        b.send_reliable(Channel::FullState, b"f", 30);
        let seen = echo.join().expect("the echo thread");
        assert_eq!(
            seen,
            [Channel::Datagram, Channel::Control, Channel::FullState]
        );
        let back: Vec<(Channel, Vec<u8>, u64)> = drain(&mut b, 31)
            .into_iter()
            .map(|r| (r.channel, r.bytes, r.arrival_ns))
            .collect();
        assert_eq!(
            back,
            vec![
                (Channel::Datagram, b"d!".to_vec(), 11),
                (Channel::Control, b"c!".to_vec(), 21),
                (Channel::FullState, b"f!".to_vec(), 31),
            ]
        );
        assert_eq!(
            b.state(),
            LinkState::Closed,
            "the moved end was dropped with its thread"
        );
    }

    /// Added item 1 (advisor-u7-brief.md C2): each channel's WebSocket tag reads back as that channel, the three tags
    /// are 0, 1 and 2 (the catalogue's line), and every other tag names none.
    #[test]
    fn u6b_ws_tags_round_trip_and_an_unknown_tag_names_no_channel() {
        let all = [Channel::Datagram, Channel::Control, Channel::FullState];
        assert_eq!(all.map(Channel::ws_tag), [0, 1, 2]);
        for c in all {
            assert_eq!(Channel::from_ws_tag(c.ws_tag()), Some(c));
        }
        for tag in 3..=u8::MAX {
            assert_eq!(Channel::from_ws_tag(tag), None, "tag {tag}");
        }
    }

    /// W18's default: a backend that does not gate (Loopback here) answers `begin_tick` `Sent` and sends the tick's
    /// datagrams as before, boxed or not.
    #[test]
    fn u6b_begin_tick_defaults_to_sent_and_changes_nothing() {
        let (a, mut b) = Loopback::pair(64);
        let mut a: Box<dyn Link + Send> = Box::new(a);
        assert_eq!(a.begin_tick(&[2, 3], 5), SendResult::Sent);
        assert_eq!(a.send_unreliable(b"d1", 5), SendResult::Sent);
        assert_eq!(a.send_unreliable(b"d22", 5), SendResult::Sent);
        assert_eq!(drain(&mut b, 5).len(), 2);
    }

    /// T6's `Send` asserts as a test: a Loopback end, a boxed `Send` link and a generic `Send` function all take a
    /// Loopback; the const block above the tests holds the same at compile time.
    #[test]
    fn t6_a_loopback_end_and_a_boxed_send_link_are_send() {
        fn takes_send<T: Send>(t: T) -> T {
            t
        }
        let (a, b) = Loopback::pair(64);
        let mut a = takes_send(a);
        let mut b: Box<dyn Link + Send> = takes_send(Box::new(b));
        assert_eq!(a.send_unreliable(b"s", 1), SendResult::Sent);
        assert_eq!(b.poll_recv(1).map(|r| r.bytes), Some(b"s".to_vec()));
        let moved = std::thread::spawn(move || b.state()).join().expect("moved");
        assert_eq!(moved, LinkState::Open);
        assert_eq!(
            a.state(),
            LinkState::Closed,
            "the other end ended with its thread"
        );
    }
}
