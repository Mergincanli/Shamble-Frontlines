//! `DelayStream`: the WebSocket fallback's delay wrapper in the server's accept path (plan E15, §4.7; Mustafa's
//! U3-Q3 (b) and U3-Q4 (b); the advisor's S0-S14 and u3c-plan.md rev 2, C1-C10). It sits above TLS and below
//! tokio-tungstenite and runs every byte of one WebSocket connection through the Linux-faithful TCP model (U3a's
//! `TcpModel`), both ways, released at the model's delivery stamps.
//!
//! **Shape.** One `TcpModel` per connection (end 0 is RS, end 1 the client; Linux 6.8's defaults, the 64 KB budget),
//! under L2Config's profile with its one-way delay netted of `base_rtt_ns` (`config::one_way_ns`, ImpairSocket's
//! helper) and its fates from (seed, connection index, direction) (S10). The model runs on its own axis, 0 at the wrap
//! (the kernel's Cubic and HyStart start at the connection's establishment); every stamp DelayStream takes or reports
//! is on [`DelayStream::now_ns`], the model's plus the wrap's stamp. The real TCP and TLS handshakes run undelayed
//! below it (recorded in the header).
//!
//! **Records (S0, C1).** Down: each `poll_write` takes at most one TLS record, n = min(len, 16,384, free − 22), where
//! free is the model's free budget less the bytes released and not yet taken by S; with free <= 22 it returns Pending
//! and is woken when the budget frees, never Ready(Ok(0)). Each record counts as its plaintext + 22 B, is released only
//! once the model delivers the whole of it in order, and goes into S in one write call (a call S takes in part is
//! counted, `ws_split_writes_count`). `poll_flush` is Ready once the bytes are in the model, as a kernel socket's flush.
//! Up (C2): the client's plaintext, read from S, is cut at its WebSocket boundaries (the request one record, then each
//! frame, + 22 B each) and a record enters the model at end 1 once its whole frame is read and the model has room for
//! it; it is released to tungstenite once delivered in order, one record per `poll_read` (so each record's lateness
//! is taken when its own last byte is handed over, and the poll in which tungstenite completes a message is the one
//! that took that message's last byte). S is read only while the up room (the free budget less the bytes released and
//! not yet read) is at least one largest record (513 + 14 + 22 = 549 B at the defaults).
//!
//! **Limits before any delay buffering (plan §4.4, C6).** The client's request and frames go through
//! [`ws_frame::Scanner`](super::ws_frame::Scanner): a frame or message above `ws_max_message_bytes`, an unmasked
//! frame, a bad control frame, a 64-bit length with its top bit set, or a request above `ws_max_request_bytes` closes
//! the connection, counted by reason, before any of that payload enters the model.
//!
//! **The pump (C3).** [`DelayStream::wrap`] spawns one tokio task per connection that owns S. It advances the model
//! event by event (next event, then advance to it), so every release carries its model stamp, writes released down
//! records into S at once, reads S into the scanner, and hands released up records to tungstenite. The release clock
//! (`delay_clock`, one dedicated thread with 1 ns slack) wakes it at the model's next event. Dropping the DelayStream
//! ends the pump and drops S at once; `poll_shutdown` waits at most 10 s in all, for delivery, for S to take the bytes
//! and for S's own shutdown, then drops S and counts it (the pump's first run at or after the deadline does it, and
//! before it the pump sleeps until the deadline or a wake from S); an EOF from S ends the up direction after its last
//! delivered byte. The connection's lock is never held across S's own calls (F3): the pump stages the front down record
//! under it (once per record, F4), releases it for S's write, flush, read or shutdown, then takes it again to commit
//! what S took or read; S's reads land in the pump's own buffer and enter the model at a fresh now. Wakes always fire
//! after the lock is released (F2).
//!
//! **Held bytes (NC-U3c-2).** Down at most the 64 KB budget (it covers released-but-untaken bytes) plus the
//! acknowledged prefix of the front record not yet released: a loss inside a record lets the acknowledgement of its
//! first segments free that prefix in the budget while the record waits to be delivered whole, so below 64 KB plus one
//! record (16,406 B). Up likewise at most 64 KB plus the acknowledged prefix of its front unreleased record (below one
//! record: 549 B, or the request's 8,214 B), plus the scanner's buffer (one frame, 527 B, or the request cap). Counted,
//! current and largest, per direction.
//!
//! **Memory per connection (F7).** Fixed at wrap, before any WebSocket byte: one lateness histogram per direction, its
//! per-second window kept as a difference against a sparse mark of the buckets the second touched (`stats::Running`:
//! 135,040 B of histogram, a 1,056 B bitmap and a 6,144 B reserve for the mark), the scanner's buffer (8,192 B at the
//! defaults), the down record's staging buffer (16,384 B) and the pump's read buffer (527 B): 309,583 B at the defaults
//! (`ws_conn_fixed_bytes`; it was about 565 KB with two whole histograms per direction). On top: the held bytes above
//! (below 81,942 B each way), and a dense mark of 135,040 B per direction once a second's lateness touches more than
//! 256 buckets. U6 sizes WsServer's accept cap from these and tungstenite's own buffers (48 players: about 15 MB here).
//!
//! **Lateness (M1, C10).** Down at the return of S's `poll_write` that takes a record's last byte; up at the return of
//! `poll_read` that hands its last byte to tungstenite; each against the record's model delivery stamp, in the same
//! histograms and E[(L − L0 − 0.25 ms)+] as `l2::stats`.

use super::config::{L2Config, one_way_ns};
use super::delay_clock::{self, Clock};
use super::delay_names as names;
use super::stats::{Running, Summary};
use super::thread;
use super::ws_frame::{Boundary, MAX_HEADER, REASONS, Reason, Scanner};
use crate::tcp_model::{MAX_BURSTS, MODEL, NOT_MODELLED, TcpConfig, TcpModel};
use crate::ws_gate::Inputs;
use std::collections::VecDeque;
use std::fmt;
use std::future::Future;
use std::io;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::task::{Context, Poll, Waker};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// The largest TLS 1.3 record's plaintext (RFC 8446 §5.1).
pub(crate) const RECORD_MAX: usize = 16_384;
/// A TLS 1.3 record's overhead in the model: 5 B header, 1 B inner content type, 16 B AEAD tag (S0).
pub(crate) const OVERHEAD: u64 = 22;
/// How long `poll_shutdown` waits for delivery and for S to take the bytes.
pub(crate) const SHUTDOWN_WAIT_NS: u64 = 10_000_000_000;
/// The largest `ws_max_message_bytes` wrap takes: a frame with its 14 B header fits one TLS record.
const MAX_MESSAGE: usize = RECORD_MAX - MAX_HEADER;
/// The largest `ws_max_request_bytes` wrap takes: half the 64 KB budget.
const MAX_REQUEST: usize = 32_768;
/// The per-return records a connection keeps (beyond: counted, `ws_returns_dropped_count`).
pub(crate) const MAX_RETURNS: usize = 1_024;
/// The model's ends: end 0 (RS) sends down, end 1 (the client) sends up.
pub(crate) const DOWN: usize = 0;
pub(crate) const UP: usize = 1;

const CONN_INDEX_RULE: &str = "the accept path's order: the first connection the listener accepts is 0, the next 1; \
     fates come from (seed, index, direction) (S10)";
const RECORDS: &str = "down: one TLS record per poll_write (at most 16,384 B, + 22 B in the model), one write call into \
     TLS per record; up: one record per client frame and one for the request, + 22 B each, cut at frame boundaries. \
     Real TLS records can differ (rustls cuts its own; invisible on loopback)";
const HANDSHAKE: &str = "the TCP and TLS handshakes run undelayed below the wrapper; the model starts established with \
     no RTT sample: RTO 1 s and fix 1's 100 ms srtt until the first data acknowledgement, where Linux has the \
     handshake's sample (tcp_synack_rtt_meas); the upgrade exchange gives one within the first round trip";

/// One record: its plaintext length, its end in the model's byte count, its model write and delivery stamps, and
/// (up) whether it ends a data message.
#[derive(Debug, Clone, Copy)]
struct Rec {
    len: usize,
    model_end: u64,
    released: Option<u64>,
    message_end: bool,
}

/// The per-return record (C8): at each burst's end, as the model passes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Return {
    /// The burst's end on DelayStream's clock.
    pub(crate) at_ns: u64,
    /// The model's outstanding(0) (SIOCOUTQ) and srtt_ns(0) (tcpi_rtt) then.
    pub(crate) outstanding: u64,
    pub(crate) srtt_ns: Option<u64>,
    /// Down bytes (model bytes) released and not yet taken by S then.
    pub(crate) untaken: u64,
    /// Bytes tungstenite held above DelayStream then (offered in its latest write and not taken: `owed`).
    /// advisor-ws-docs-r2.md:379 (3)'s outstanding_bytes is `outstanding` plus this, plus what WsServer holds itself
    /// (NC-U3c-4; U6 sums them for the event).
    pub(crate) owed: u64,
    /// The receiving ends' in-order counts then, `[down, up]`.
    delivered: [u64; 2],
    /// S12's per-burst times, `[down, up]`: from the return to the first in-order byte, and to the first whole data
    /// message released.
    pub(crate) first_byte_ns: [Option<u64>; 2],
    pub(crate) first_message_ns: [Option<u64>; 2],
}

impl Return {
    fn complete(&self) -> bool {
        self.first_byte_ns.iter().all(Option::is_some)
            && self.first_message_ns.iter().all(Option::is_some)
    }

    /// The `burst_return` event's DelayStream fields (`delay_names::RETURN`'s order; u64::MAX: not seen yet). U6's
    /// npt-server writes them; until then the rows test reads them.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn row(&self) -> Vec<(&'static str, u64)> {
        let t = |v: Option<u64>| v.unwrap_or(u64::MAX);
        let vals = [
            self.at_ns,
            self.outstanding,
            self.srtt_ns.unwrap_or(0),
            u64::from(self.srtt_ns.is_some()),
            self.untaken,
            self.owed,
            t(self.first_byte_ns[DOWN]),
            t(self.first_message_ns[DOWN]),
            t(self.first_byte_ns[UP]),
            t(self.first_message_ns[UP]),
        ];
        names::RETURN.iter().copied().zip(vals).collect()
    }
}

/// The connection's own counts (cumulative): closes by reason, shutdown timeouts, split writes, dropped returns.
#[derive(Debug, Clone, Copy, Default)]
struct Own {
    closes: [u64; REASONS],
    shutdown_timeouts: u64,
    split_writes: u64,
    returns_dropped: u64,
}

impl Own {
    fn values(&self) -> [u64; REASONS + 3] {
        let mut v = [0; REASONS + 3];
        v[..REASONS].copy_from_slice(&self.closes);
        v[REASONS] = self.shutdown_timeouts;
        v[REASONS + 1] = self.split_writes;
        v[REASONS + 2] = self.returns_dropped;
        v
    }
}

/// What the tests read back to replay the run on a pure model (S10) and to tie messages to their releases.
#[cfg(test)]
#[derive(Debug, Default, Clone)]
pub(crate) struct Logs {
    /// Every write into the model and every burst added after the wrap, in order, with its model stamp.
    pub(crate) steps: Vec<(u64, Step)>,
    /// (end that wrote it, its model end, model delivery stamp) of every release.
    pub(crate) releases: Vec<(usize, u64, u64)>,
    /// `[down, up]`: each data message's release on DelayStream's clock, in order.
    pub(crate) messages: [Vec<u64>; 2],
    /// Up: each data message's hand-over to tungstenite on DelayStream's clock (the `poll_read` return that handed its
    /// last byte) and the lateness recorded for it then, in order.
    pub(crate) handed_up: Vec<(u64, u64)>,
}

/// One atomic reading of a connection for the tests (W2): every field taken under one lock, after one step to now.
#[cfg(test)]
#[derive(Debug, Clone)]
pub(crate) struct Snapshot {
    /// The model's origin on DelayStream's clock, and its now (model axis).
    pub(crate) origin: u64,
    pub(crate) now: u64,
    /// The model's counters `[down, up]` and in-order counts `[down, up]`.
    pub(crate) counters: [crate::tcp_model::Counters; 2],
    pub(crate) delivered: [u64; 2],
    pub(crate) logs: Logs,
    /// The model's data transmissions: (end, segment start, attempt, model time, lost).
    pub(crate) sent_log: Vec<(usize, u64, u32, u64, bool)>,
    /// Down plaintext S has taken.
    pub(crate) taken_down: u64,
}

/// The pump's lifetime for the shutdown tests (W3, M1): its polls, those at or after the shutdown deadline, the
/// deadline and the stamp it ended at (DelayStream's clock).
#[cfg(test)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Life {
    pub(crate) polls: u64,
    pub(crate) polls_after_deadline: u64,
    pub(crate) deadline: Option<u64>,
    pub(crate) ended_ns: Option<u64>,
}

/// One logged step (model axis).
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Step {
    /// `bytes` model bytes written at `end`.
    Write { end: usize, bytes: u64 },
    /// A burst added: its start and length.
    Burst { start: u64, len: u64 },
}

struct Conn {
    model: TcpModel,
    /// The wrap's stamp: the model's 0 on DelayStream's clock.
    origin: u64,
    conn: u32,
    accepted_ns: u64,
    /// `[down, up]`: plaintext not yet taken by S (down) or read by tungstenite (up), records in the model or released.
    bytes: [VecDeque<u8>; 2],
    recs: [VecDeque<Rec>; 2],
    /// Released records (a prefix of `recs`), the front one's bytes already taken or read, and their model bytes.
    released: [usize; 2],
    front_done: [usize; 2],
    released_model: [u64; 2],
    /// Model bytes written, plaintext written, plaintext released.
    model_end: [u64; 2],
    written: [u64; 2],
    plain_released: [u64; 2],
    /// Down: the server's stream scanned for data message ends (plaintext offsets not yet released).
    down_scan: Scanner,
    down_msg_ends: VecDeque<u64>,
    /// Bytes tungstenite offered in its latest poll_write that DelayStream did not take (tungstenite 0.30 offers its
    /// whole out buffer each call: protocol/frame/mod.rs:273-287).
    owed: u64,
    /// Up: the client's bytes read from S and not yet in the model, the scanned part, and the boundaries found.
    parse: Box<[u8]>,
    parse_len: usize,
    scanned: usize,
    bounds: VecDeque<(usize, bool)>,
    up_scan: Scanner,
    up_eof: bool,
    max_request: usize,
    max_frame: usize,
    read_floor: u64,
    /// Burst ends not yet passed (model axis), and the per-return records.
    burst_ends: Vec<u64>,
    returns: Vec<Return>,
    open_from: usize,
    pump_waker: Option<Waker>,
    reader: Option<Waker>,
    writer: Option<Waker>,
    shutter: Option<Waker>,
    dropped: bool,
    pump_done: bool,
    /// The shutdown wait (SHUTDOWN_WAIT_NS; the tests shorten it) and the deadline once a shutdown began.
    shutdown_wait: u64,
    shutdown: Option<u64>,
    down_done: bool,
    failed: Option<io::ErrorKind>,
    closed: Option<Reason>,
    /// The release lateness per direction: the connection's whole and its per-second window (F7).
    late: [Running; 2],
    held: [u64; 2],
    held_max: [u64; 2],
    held_max_run: [u64; 2],
    own: Own,
    mark_own: Own,
    mark_tcp: [[u64; 20]; 2],
    /// The heap bytes the connection allocated at wrap (F7; `ws_conn_fixed_bytes`).
    fixed_bytes: u64,
    #[cfg(test)]
    logs: Logs,
    #[cfg(test)]
    life: Life,
}

impl Conn {
    fn model_t(&self, clock_ns: u64) -> u64 {
        clock_ns.saturating_sub(self.origin)
    }

    /// The down room: the model's free budget less the bytes released and not yet taken by S.
    fn down_free(&self) -> u64 {
        self.model
            .free_budget(DOWN)
            .saturating_sub(self.released_model[DOWN])
    }

    /// The up room: the model's free budget at the client's end less the bytes released and not yet read.
    fn up_room(&self) -> u64 {
        self.model
            .free_budget(UP)
            .saturating_sub(self.released_model[UP])
    }

    fn held_now(&mut self) {
        self.held = [
            self.bytes[DOWN].len() as u64,
            (self.bytes[UP].len() + self.parse_len) as u64,
        ];
        for d in 0..2 {
            self.held_max[d] = self.held_max[d].max(self.held[d]);
            self.held_max_run[d] = self.held_max_run[d].max(self.held[d]);
        }
    }

    fn late(&mut self, dir: usize, ns: u64) {
        self.late[dir].record(ns);
    }

    /// One record of `len` plaintext bytes into the model at `t` from `dir`'s end. The caller checked the room, so the
    /// model takes all of it.
    fn model_write(&mut self, dir: usize, len: usize, t: u64, message_end: bool) {
        let bytes = len as u64 + OVERHEAD;
        let took = self.model.write(dir, bytes, t);
        debug_assert_eq!(took, bytes, "the room was checked");
        self.model_end[dir] += took;
        self.written[dir] += len as u64;
        self.recs[dir].push_back(Rec {
            len,
            model_end: self.model_end[dir],
            released: None,
            message_end,
        });
        #[cfg(test)]
        self.logs.steps.push((t, Step::Write { end: dir, bytes }));
    }

    /// Advances the model to `t` event by event (and through each burst's end), releasing what it delivers with each
    /// event's stamp. Returns, per direction, whether anything was released.
    fn step_to(&mut self, t: u64) -> [bool; 2] {
        let mut new = [false; 2];
        loop {
            let ev = self.model.next_event();
            let be = self.burst_ends.iter().copied().min();
            let next = match (ev, be) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (a, b) => a.or(b),
            };
            let Some(x) = next.filter(|&x| x <= t) else {
                break;
            };
            self.model.advance(x);
            let got = self.harvest(x);
            new[DOWN] |= got[DOWN];
            new[UP] |= got[UP];
            if be == Some(x) {
                self.burst_ends.retain(|&e| e != x);
                self.record_return(x);
            }
        }
        self.model.advance(t);
        new
    }

    /// Releases every record the model has delivered whole, stamped `t`, and fills the open per-return times.
    fn harvest(&mut self, t: u64) -> [bool; 2] {
        let mut new = [false; 2];
        let mut message = [false; 2];
        for dir in [DOWN, UP] {
            let delivered = self.model.delivered(1 - dir);
            while self.released[dir] < self.recs[dir].len() {
                let i = self.released[dir];
                let r = &mut self.recs[dir][i];
                if r.model_end > delivered {
                    break;
                }
                r.released = Some(t);
                let (len, model_end, message_end) = (r.len, r.model_end, r.message_end);
                self.released[dir] += 1;
                self.released_model[dir] += len as u64 + OVERHEAD;
                self.plain_released[dir] += len as u64;
                new[dir] = true;
                #[cfg(test)]
                self.logs.releases.push((dir, model_end, t));
                let _ = model_end;
                if dir == UP && message_end {
                    message[UP] = true;
                    #[cfg(test)]
                    self.logs.messages[UP].push(t.saturating_add(self.origin));
                }
            }
        }
        while let Some(&e) = self.down_msg_ends.front() {
            if e > self.plain_released[DOWN] {
                break;
            }
            self.down_msg_ends.pop_front();
            message[DOWN] = true;
            #[cfg(test)]
            self.logs.messages[DOWN].push(t.saturating_add(self.origin));
        }
        let delivered = [self.model.delivered(1), self.model.delivered(0)];
        for i in self.open_from..self.returns.len() {
            let r = &mut self.returns[i];
            let since = t.saturating_add(self.origin).saturating_sub(r.at_ns);
            for d in 0..2 {
                if r.first_byte_ns[d].is_none() && delivered[d] > r.delivered[d] {
                    r.first_byte_ns[d] = Some(since);
                }
                if r.first_message_ns[d].is_none() && message[d] {
                    r.first_message_ns[d] = Some(since);
                }
            }
        }
        while self.open_from < self.returns.len() && self.returns[self.open_from].complete() {
            self.open_from += 1;
        }
        new
    }

    /// The per-return record at a burst's end `e` (model axis), as the model passes it (C8).
    fn record_return(&mut self, e: u64) {
        if self.returns.len() >= MAX_RETURNS {
            self.own.returns_dropped += 1;
            return;
        }
        self.returns.push(Return {
            at_ns: e.saturating_add(self.origin),
            outstanding: self.model.outstanding(DOWN),
            srtt_ns: self.model.srtt_ns(DOWN),
            untaken: self.released_model[DOWN],
            owed: self.owed,
            delivered: [self.model.delivered(1), self.model.delivered(0)],
            first_byte_ns: [None; 2],
            first_message_ns: [None; 2],
        });
    }

    /// When the pump must next run, on DelayStream's clock: the model's next event, the next burst end, the shutdown
    /// deadline. Saturating (F1): a time past the clock's range is never due, never wrapped into the past.
    fn due(&self) -> Option<u64> {
        let ev = self
            .model
            .next_event()
            .map(|x| x.saturating_add(self.origin));
        let be = self
            .burst_ends
            .iter()
            .min()
            .map(|x| x.saturating_add(self.origin));
        let sd = self.shutdown.filter(|_| !self.down_done);
        [ev, be, sd].into_iter().flatten().min()
    }

    /// Arms the release clock at `due` for the pump (once the pump has run once and left its waker).
    fn reschedule(&self, clock: &Clock, key: u64) {
        if self.pump_done || self.dropped {
            return;
        }
        match (self.due(), &self.pump_waker) {
            (Some(d), Some(w)) => clock.schedule(key, d, w),
            (None, _) => clock.cancel(key),
            _ => {}
        }
    }

    /// Scans the client's new bytes for boundaries, then moves each whole record into the model in order (so the
    /// records before a bad frame enter, as far as the room holds them); a limit closes before the bad frame's payload
    /// is buffered. Records that entered and are not yet released when a limit closes the connection are never handed
    /// to tungstenite: the close ends the pump, so the model stops (released ones are still read first; F-F).
    fn scan_up(&mut self, t: u64) -> Result<(), Reason> {
        let mut result = Ok(());
        while self.scanned < self.parse_len {
            let (n, ev) = self.up_scan.scan(&self.parse[self.scanned..self.parse_len]);
            self.scanned += n;
            match ev {
                Some(Ok(Boundary::Head)) => self.bounds.push_back((self.scanned, false)),
                Some(Ok(Boundary::Frame { message_end })) => {
                    self.bounds.push_back((self.scanned, message_end));
                }
                Some(Err(r)) => {
                    result = Err(r);
                    break;
                }
                None => break,
            }
        }
        self.move_up(t);
        result
    }

    /// Moves every whole record found in the scanner's buffer into the model at `t` while the up room holds it, then
    /// moves what is left to the buffer's start once (F10).
    fn move_up(&mut self, t: u64) {
        let mut start = 0;
        while let Some(&(end, message_end)) = self.bounds.front() {
            let len = end - start;
            if self.up_room() < len as u64 + OVERHEAD {
                break;
            }
            self.bytes[UP].extend(&self.parse[start..end]);
            self.model_write(UP, len, t, message_end);
            self.bounds.pop_front();
            start = end;
        }
        if start > 0 {
            self.parse.copy_within(start..self.parse_len, 0);
            self.parse_len -= start;
            self.scanned -= start;
            for b in &mut self.bounds {
                b.0 -= start;
            }
        }
    }

    /// The scanner's read limit: the request cap while in the request, else one largest frame.
    fn read_limit(&self) -> usize {
        if self.up_scan.in_head() {
            self.max_request
        } else {
            self.max_frame
        }
    }

    /// Closes on a limit, before the frame's payload enters the model (C6).
    fn close(&mut self, r: Reason) {
        self.closed = Some(r);
        self.own.closes[r as usize] += 1;
        self.parse_len = 0;
        self.scanned = 0;
        self.bounds.clear();
    }

    /// The wakers to call after the lock is released.
    fn wakers(&mut self, reader: bool, writer: bool) -> [Option<Waker>; 3] {
        let writer = writer || (self.writer.is_some() && self.down_free() > OVERHEAD);
        [
            if reader { self.reader.take() } else { None },
            if writer { self.writer.take() } else { None },
            if self.down_done || self.pump_done {
                self.shutter.take()
            } else {
                None
            },
        ]
    }
}

fn wake_all(w: [Option<Waker>; 3]) {
    for w in w.into_iter().flatten() {
        w.wake();
    }
}

/// Wakes owed by a step, fired when dropped.
struct Wakes([Option<Waker>; 4]);

impl Drop for Wakes {
    fn drop(&mut self) {
        for w in self.0.iter_mut().filter_map(Option::take) {
            w.wake();
        }
    }
}

/// The connection's lock after a step to now (`DelayStream::current`), with the wakes that step owes. Fields drop in
/// their declaration order, so the lock is released first and the wakes fire after it (F2): a woken task never meets
/// the lock still held by its waker.
struct Stepped<'a> {
    c: MutexGuard<'a, Conn>,
    _wakes: Wakes,
}

impl Deref for Stepped<'_> {
    type Target = Conn;

    fn deref(&self) -> &Conn {
        &self.c
    }
}

impl DerefMut for Stepped<'_> {
    fn deref_mut(&mut self) -> &mut Conn {
        &mut self.c
    }
}

struct Shared {
    key: u64,
    clock: &'static Clock,
    conn: Mutex<Conn>,
}

impl Shared {
    fn lock(&self) -> MutexGuard<'_, Conn> {
        self.conn.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// The WebSocket fallback's delay wrapper (plan §4.7): wrap the server's TLS stream, then run tokio-tungstenite over
/// it. See the module documentation.
pub struct DelayStream<S> {
    sh: Arc<Shared>,
    _s: PhantomData<fn(S) -> S>,
}

impl<S> fmt::Debug for DelayStream<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = self.sh.lock();
        f.debug_struct("DelayStream")
            .field("conn", &c.conn)
            .field("accepted_ns", &c.accepted_ns)
            .finish_non_exhaustive()
    }
}

impl<S> DelayStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    /// Wraps the server's TLS stream `stream` of connection `conn` (the accept path's index: the first accepted is
    /// 0), accepted at `accepted_ns` on [`DelayStream::now_ns`]'s clock (the TCP accept, never the end of a delay:
    /// npt-server's Join deadline reads it at U6). Spawns the connection's pump on the current tokio runtime.
    ///
    /// Refused: a pass-through `cfg` (R20: clean WebSocket runs go without the wrapper), limits out of range, more
    /// than 64 bursts still to come, no tokio runtime, or no release clock thread.
    pub fn wrap(
        stream: S,
        cfg: &L2Config,
        conn: u32,
        accepted_ns: u64,
    ) -> io::Result<DelayStream<S>> {
        let handle = tokio::runtime::Handle::try_current().map_err(io::Error::other)?;
        let (ds, pump) = Self::build(stream, cfg, conn, accepted_ns)?;
        handle.spawn(pump);
        Ok(ds)
    }

    /// `wrap`'s connection and its pump, not yet spawned (the tests poll a pump by hand).
    fn build(
        stream: S,
        cfg: &L2Config,
        conn: u32,
        accepted_ns: u64,
    ) -> io::Result<(DelayStream<S>, Pump<S>)> {
        let bad = |why: &str| io::Error::new(io::ErrorKind::InvalidInput, why.to_owned());
        if cfg.pass_through {
            return Err(bad(
                "a pass-through L2Config: clean WebSocket runs go without DelayStream (R20)",
            ));
        }
        // A largest client frame (with its 14 B header) fits one TLS record, so the up read floor (one largest record
        // + 22 B) stays far inside the 64 KB budget; the request cap stays below the budget too.
        if !(1..=MAX_MESSAGE).contains(&cfg.ws_max_message_bytes)
            || !(16..=MAX_REQUEST).contains(&cfg.ws_max_request_bytes)
        {
            return Err(bad(
                "ws_max_message_bytes must be 1 to 16,370 and ws_max_request_bytes 16 to 32,768",
            ));
        }
        let clock = delay_clock::clock();
        let origin = clock.now_ns();
        let mut profile = cfg.profile.clone();
        let mut burst_ends = Vec::new();
        profile.bursts_ns = cfg
            .profile
            .bursts_ns
            .iter()
            .filter_map(|&(s, l)| {
                let end = s.saturating_add(l);
                (end > origin).then(|| {
                    let start = s.saturating_sub(origin);
                    burst_ends.push(end - origin);
                    (start, end - origin - start)
                })
            })
            .collect();
        if profile.bursts_ns.len() > MAX_BURSTS {
            return Err(bad("more than 64 bursts still to come"));
        }
        let key = clock.start(cfg.timer_slack_ns)?;
        let model = TcpModel::new(&profile, cfg.seed, conn).with_one_way_ns(one_way_ns(cfg));
        let max_frame = cfg.ws_max_message_bytes + MAX_HEADER;
        let mut c = Conn {
            model,
            origin,
            conn,
            accepted_ns,
            bytes: [VecDeque::new(), VecDeque::new()],
            recs: [VecDeque::new(), VecDeque::new()],
            released: [0; 2],
            front_done: [0; 2],
            released_model: [0; 2],
            model_end: [0; 2],
            written: [0; 2],
            plain_released: [0; 2],
            down_scan: Scanner::server(),
            down_msg_ends: VecDeque::new(),
            owed: 0,
            parse: vec![0u8; cfg.ws_max_request_bytes.max(max_frame)].into_boxed_slice(),
            parse_len: 0,
            scanned: 0,
            bounds: VecDeque::new(),
            up_scan: Scanner::client(
                cfg.ws_max_message_bytes as u64,
                cfg.ws_max_request_bytes as u64,
            ),
            up_eof: false,
            max_request: cfg.ws_max_request_bytes,
            max_frame,
            read_floor: max_frame as u64 + OVERHEAD,
            burst_ends,
            returns: Vec::new(),
            open_from: 0,
            pump_waker: None,
            reader: None,
            writer: None,
            shutter: None,
            dropped: false,
            pump_done: false,
            shutdown_wait: SHUTDOWN_WAIT_NS,
            shutdown: None,
            down_done: false,
            failed: None,
            closed: None,
            late: [Running::new(), Running::new()],
            held: [0; 2],
            held_max: [0; 2],
            held_max_run: [0; 2],
            own: Own::default(),
            mark_own: Own::default(),
            mark_tcp: [[0; 20]; 2],
            fixed_bytes: 0,
            #[cfg(test)]
            logs: Logs::default(),
            #[cfg(test)]
            life: Life::default(),
        };
        let scratch = vec![0u8; RECORD_MAX].into_boxed_slice();
        let rbuf = vec![0u8; max_frame].into_boxed_slice();
        c.fixed_bytes = c.late.iter().map(Running::heap_bytes).sum::<u64>()
            + (c.parse.len() + scratch.len() + rbuf.len()) as u64;
        let sh = Arc::new(Shared {
            key,
            clock,
            conn: Mutex::new(c),
        });
        let pump = Pump {
            sh: Arc::clone(&sh),
            s: Some(stream),
            scratch,
            staged: 0,
            sent: 0,
            rbuf,
            unflushed: false,
            front_calls: 0,
        };
        Ok((
            DelayStream {
                sh,
                _s: PhantomData,
            },
            pump,
        ))
    }
}

impl DelayStream<()> {
    /// DelayStream's clock: ns since its origin (the first use in the process). Every stamp it takes or reports is on
    /// this clock, and so are bursts on the WebSocket path (L2Config's profile and [`DelayStream::add_burst`]). One
    /// clock for every stream type, so it is called as `DelayStream::now_ns()`.
    pub fn now_ns() -> u64 {
        delay_clock::clock().now_ns()
    }
}

impl<S> DelayStream<S> {
    /// Adds a burst of total loss, both ways, from `start_ns` (DelayStream's clock) for `len_ns`: per-session B1 on
    /// the fallback (plan E23; ADR 0001's check item 11). Refused (InvalidInput) when:
    /// - it starts at or before the model's now (a transmission stamped at the model's now was already sent: NC-U3c-8);
    /// - it has no length, or ends past DelayStream's clock's range (start + length above u64::MAX; the longest burst
    ///   from `start_ns` is u64::MAX − start_ns: F1);
    /// - 64 bursts are already to come (C4).
    ///
    /// U6 carry (S7): the per-session control that triggers it is a client's, so npt-server range-checks the start and
    /// the length before this call (a stated maximum length, a start within the session).
    pub fn add_burst(&self, start_ns: u64, len_ns: u64) -> io::Result<()> {
        let sh = &self.sh;
        let mut c = sh.lock();
        let t = c.model_t(sh.clock.now_ns());
        let got = c.step_to(t);
        let now = c.model.now_ns();
        let start = start_ns
            .checked_sub(c.origin)
            .filter(|&s| s > now && start_ns.checked_add(len_ns).is_some());
        let added = start.is_some_and(|s| c.model.add_burst(s, len_ns));
        if added {
            let s = start.unwrap_or_default();
            c.burst_ends.push(s.saturating_add(len_ns));
            #[cfg(test)]
            c.logs.steps.push((
                t,
                Step::Burst {
                    start: s,
                    len: len_ns,
                },
            ));
        }
        c.reschedule(sh.clock, sh.key);
        let w = c.wakers(got[UP], false);
        let pump = if got[DOWN] {
            c.pump_waker.clone()
        } else {
            None
        };
        drop(c);
        wake_all(w);
        if let Some(p) = pump {
            p.wake();
        }
        if added {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "a burst at or before the model's now, of no length, ending past the clock's range, or past the 64 \
                 still to come",
            ))
        }
    }

    /// Steps the model to now under the lock, owing wakes to what the step released or freed: the pump for released
    /// down records, the reader for released up records, a waiting writer when acknowledgements freed the budget (the
    /// pump's own clock entry may have moved past the events stepped here, so the wakes cannot wait for it). The wakes
    /// fire when the returned guard drops, after the lock is released (F2).
    fn current(&self) -> Stepped<'_> {
        let sh = &self.sh;
        let mut c = sh.lock();
        let mut wakes = [None, None, None, None];
        if !c.pump_done {
            let t = c.model_t(sh.clock.now_ns());
            let got = c.step_to(t);
            c.reschedule(sh.clock, sh.key);
            let [r, w, s] = c.wakers(got[UP], false);
            let pump = if got[DOWN] {
                c.pump_waker.clone()
            } else {
                None
            };
            wakes = [r, w, s, pump];
        }
        Stepped {
            c,
            _wakes: Wakes(wakes),
        }
    }
}

/// Fix 1's inputs and the per-return records (u3c-plan.md C5, C8): U6's WsServer reads them through Link; until then
/// the mechanism tests do.
#[cfg_attr(not(test), allow(dead_code))]
impl<S> DelayStream<S> {
    /// The down direction's smoothed RTT (the model's end 0: what tcpi_rtt means, tcp.c:3842), None before a sample.
    pub(crate) fn srtt_ns(&self) -> Option<u64> {
        self.current().model.srtt_ns(DOWN)
    }

    /// write_seq − snd_una at end 0 (what SIOCOUTQ means, tcp.c:630), in the model's bytes.
    pub(crate) fn outstanding_bytes(&self) -> u64 {
        self.current().model.outstanding(DOWN)
    }

    /// The room in the 64 KB budget for new bytes (less what is released and not yet taken by S): the send-buffer
    /// space npt-room gets (plan §4.4).
    pub(crate) fn free_bytes(&self) -> u64 {
        self.current().down_free()
    }

    /// Bytes tungstenite offered in its latest write and DelayStream did not take: what it holds above this socket.
    pub(crate) fn owed_bytes(&self) -> u64 {
        self.sh.lock().owed
    }

    /// Fix 1's gate inputs in one reading (F11, NC-U3c-5): srtt, outstanding, owed and free taken under one lock after
    /// one step of the model to now, so N's srtt and the outstanding bytes describe the same model instant. WsServer
    /// reads this once per tick (U6); the four accessors above stay for the tests' single readings.
    pub(crate) fn gate_inputs(&self) -> Inputs {
        let c = self.current();
        Inputs {
            srtt_ns: c.model.srtt_ns(DOWN),
            outstanding: c.model.outstanding(DOWN),
            owed: c.owed,
            free: c.down_free(),
        }
    }

    /// The TCP accept stamp passed to `wrap` (npt-server's Join deadline, U6).
    pub(crate) fn accepted_ns(&self) -> u64 {
        self.sh.lock().accepted_ns
    }

    /// The per-return records so far (C8).
    pub(crate) fn returns(&self) -> Vec<Return> {
        self.current().returns.clone()
    }
}

impl<S> DelayStream<S> {
    /// The bite's input for mechanism test (c): unsent bytes only (SIOCOUTQNSD), not outstanding.
    #[cfg(test)]
    pub(crate) fn unsent_bytes(&self) -> u64 {
        self.current().model.unsent(DOWN)
    }

    #[cfg(test)]
    pub(crate) fn logs(&self) -> Logs {
        self.current().logs.clone()
    }

    /// One atomic reading (W2): one step to now and every field read under that one lock, so the logs, the counters,
    /// the transmissions and the model's now all describe the same instant (the pump keeps stepping the model between
    /// separate calls).
    #[cfg(test)]
    pub(crate) fn snapshot(&self) -> Snapshot {
        let c = self.current();
        Snapshot {
            origin: c.origin,
            now: c.model.now_ns(),
            counters: [c.model.counters(DOWN), c.model.counters(UP)],
            delivered: [c.model.delivered(1), c.model.delivered(0)],
            logs: c.logs.clone(),
            sent_log: c.model.sent_log().to_vec(),
            taken_down: c.written[DOWN] - c.bytes[DOWN].len() as u64,
        }
    }

    /// The pump's lifetime so far (W3, M1).
    #[cfg(test)]
    pub(crate) fn life(&self) -> Life {
        self.sh.lock().life
    }

    /// The tests' bite (M2): the client direction's checks for `off` turned off. For the request cap the read limit,
    /// the same limit applied a second time, is lifted too (to the largest request wrap takes), else the connection
    /// would only stall at the cap. Called before any client byte arrives.
    #[cfg(test)]
    pub(crate) fn bite_check_off(&self, off: &[Reason]) {
        let mut c = self.sh.lock();
        for &r in off {
            c.up_scan.set_off(r);
            if r == Reason::Request {
                c.parse = vec![0u8; MAX_REQUEST].into_boxed_slice();
                c.max_request = MAX_REQUEST;
            }
        }
    }

    /// (pump ended, closed by a limit, held bytes now, largest held) for the lifetime and bound tests.
    #[cfg(test)]
    pub(crate) fn probe(&self) -> (bool, Option<Reason>, [u64; 2], [u64; 2]) {
        let c = self.sh.lock();
        (c.pump_done, c.closed, c.held, c.held_max_run)
    }

    #[cfg(test)]
    pub(crate) fn clock_key(&self) -> u64 {
        self.sh.key
    }

    /// Whether the model has nothing left to do: every byte acknowledged both ways and no event pending.
    #[cfg(test)]
    pub(crate) fn model_idle(&self) -> bool {
        let c = self.current();
        c.model.next_event().is_none()
            && c.model.outstanding(DOWN) == 0
            && c.model.outstanding(UP) == 0
    }

    #[cfg(test)]
    pub(crate) fn set_shutdown_wait(&self, ns: u64) {
        self.sh.lock().shutdown_wait = ns;
    }

    /// The header's numbers (`header.params.<name>`; SERIES.md): the wrapper on, the injected delay, the model's TCP
    /// settings, the release clock's slack (read now by the clock thread itself, and as set at its start), the limits,
    /// the connection's fixed memory.
    ///
    /// It blocks (F6): the clock thread answers the slack reading, and the caller waits for it (one wake and one /proc
    /// read, at most 1 s if that thread is stuck). From async code call it through `tokio::task::spawn_blocking` (U6's
    /// header writer), never on a runtime worker.
    pub fn header_numbers(&self) -> Vec<(&'static str, u64)> {
        let (one_way, half_jitter, mss, max_message, max_request, read_floor, wait, fixed) = {
            let c = self.sh.lock();
            (
                c.model.one_way_ns(),
                c.model.half_jitter_ns(),
                c.model.mss(),
                (c.max_frame - MAX_HEADER) as u64,
                c.max_request as u64,
                c.read_floor,
                c.shutdown_wait,
                c.fixed_bytes,
            )
        };
        let tcp = TcpConfig::default();
        let info = self.sh.clock.info();
        let slack_now = self.sh.clock.slack_now();
        let vals = [
            1,
            one_way,
            half_jitter,
            tcp.write_budget,
            mss,
            tcp.rto_min_us * 1_000,
            u64::from(tcp.thin_linear_timeouts),
            u64::from(tcp.timestamps),
            tcp.syn_rto_us * 1_000,
            u64::from(tcp.syn_linear_timeouts),
            slack_now.unwrap_or(u64::MAX),
            info.slack.read_ns.unwrap_or(u64::MAX),
            u64::from(info.slack.write_error.is_none()),
            max_message,
            max_request,
            RECORD_MAX as u64,
            OVERHEAD,
            read_floor,
            wait,
            MAX_BURSTS as u64,
            MAX_RETURNS as u64,
            fixed,
        ];
        names::HEADER_NUMBERS.iter().copied().zip(vals).collect()
    }

    /// The header's strings: the model's name and version, what it leaves out (S11), the release clock's scheduling
    /// policy (read now), the connection index rule, the record boundaries' approximation and the handshake's note.
    pub fn header_texts(&self) -> Vec<(&'static str, String)> {
        let info = self.sh.clock.info();
        let sched = match info.tid.and_then(thread::sched) {
            Some((policy, prio)) => format!("{} {prio}", thread::policy_name(policy)),
            None => "unknown".to_owned(),
        };
        let mut v: Vec<(&'static str, String)> = names::HEADER_TEXTS
            .iter()
            .copied()
            .zip([
                MODEL.to_owned(),
                NOT_MODELLED.to_owned(),
                sched,
                CONN_INDEX_RULE.to_owned(),
                RECORDS.to_owned(),
                HANDSHAKE.to_owned(),
            ])
            .collect();
        if let Some(e) = info.slack.write_error {
            v.push((names::HEADER_TEXTS[6], e));
        }
        v
    }

    /// The per-second window since the last call (`second.v.<name>`), which it closes: per direction the model's
    /// counts (its maxima as they stand), the held bytes now and the window's largest, the lateness; then the
    /// connection's own counts. Per connection: npt-server's writer merges its connections (U6).
    pub fn take_second(&self) -> Vec<(&'static str, u64)> {
        let mut c = self.current();
        let cur = [
            names::tcp_counts(&c.model.counters(DOWN)),
            names::tcp_counts(&c.model.counters(UP)),
        ];
        let mut tcp = [[0u64; 20]; 2];
        for d in 0..2 {
            for i in 0..20 {
                tcp[d][i] = if names::TCP_IS_MAX[i] {
                    cur[d][i]
                } else {
                    cur[d][i] - c.mark_tcp[d][i]
                };
            }
        }
        c.held_now();
        let held = [
            [c.held[DOWN], c.held_max[DOWN]],
            [c.held[UP], c.held_max[UP]],
        ];
        let late: [Summary; 2] = [c.late[DOWN].window(), c.late[UP].window()];
        let now = c.own.values();
        let mark = c.mark_own.values();
        let mut own = [0u64; REASONS + 3];
        for i in 0..own.len() {
            own[i] = now[i] - mark[i];
        }
        let mut out = Vec::with_capacity(80);
        names::window(tcp, held, late, &own, &mut out);
        c.mark_tcp = cur;
        c.mark_own = c.own;
        c.late[DOWN].close_window();
        c.late[UP].close_window();
        c.held_max = c.held;
        out
    }

    /// The connection's whole life (the `ws_summary` event's fields): the per-second names over it (counts as totals,
    /// the held bytes now and the largest, the lateness over every sample), then its index, accept stamp and the
    /// plaintext bytes written each way.
    pub fn run_summary(&self) -> Vec<(&'static str, u64)> {
        let mut c = self.current();
        c.held_now();
        let tcp = [
            names::tcp_counts(&c.model.counters(DOWN)),
            names::tcp_counts(&c.model.counters(UP)),
        ];
        let held = [
            [c.held[DOWN], c.held_max_run[DOWN]],
            [c.held[UP], c.held_max_run[UP]],
        ];
        let late = [c.late[DOWN].whole(), c.late[UP].whole()];
        let mut out = Vec::with_capacity(84);
        names::window(tcp, held, late, &c.own.values(), &mut out);
        out.push((names::CONNS_BEFORE, u64::from(c.conn)));
        out.push((names::ACCEPTED, c.accepted_ns));
        out.push((names::WRITTEN[DOWN], c.written[DOWN]));
        out.push((names::WRITTEN[UP], c.written[UP]));
        out
    }
}

impl<S> Drop for DelayStream<S> {
    /// Ends the pump (it drops S on its next poll) and removes the connection's clock entry at once.
    fn drop(&mut self) {
        let w = {
            let mut c = self.sh.lock();
            c.dropped = true;
            c.pump_waker.take()
        };
        self.sh.clock.cancel(self.sh.key);
        if let Some(w) = w {
            w.wake();
        }
    }
}

impl<S> AsyncRead for DelayStream<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let sh = &self.sh;
        let mut c = self.current();
        // A zero-capacity read is answered at once while a record waits (F5): the caller passed no room, and no later
        // event would wake it again for that record (tokio's own streams answer such a read at once too).
        if buf.remaining() == 0 && c.released[UP] > 0 {
            return Poll::Ready(Ok(()));
        }
        // One record per call (or the rest of the front one): see the module documentation.
        if buf.remaining() > 0 && c.released[UP] > 0 {
            let len = c.recs[UP][0].len;
            let take = (len - c.front_done[UP]).min(buf.remaining());
            {
                let (a, b) = c.bytes[UP].as_slices();
                let k = take.min(a.len());
                buf.put_slice(&a[..k]);
                buf.put_slice(&b[..take - k]);
            }
            c.bytes[UP].drain(..take);
            c.front_done[UP] += take;
            let mut freed = false;
            if c.front_done[UP] == len
                && let Some(rec) = c.recs[UP].pop_front()
            {
                c.released[UP] -= 1;
                c.released_model[UP] -= len as u64 + OVERHEAD;
                c.front_done[UP] = 0;
                freed = true;
                let at = sh.clock.now_ns();
                let due = c.origin + rec.released.unwrap_or_default();
                let late = at.saturating_sub(due);
                c.late(UP, late);
                #[cfg(test)]
                if rec.message_end {
                    c.logs.handed_up.push((at, late));
                }
            }
            c.held_now();
            let pump = if freed { c.pump_waker.clone() } else { None };
            drop(c);
            if let Some(p) = pump {
                p.wake();
            }
            return Poll::Ready(Ok(()));
        }
        if let Some(r) = c.closed {
            return Poll::Ready(Err(io::Error::new(io::ErrorKind::InvalidData, r.text())));
        }
        if let Some(k) = c.failed {
            return Poll::Ready(Err(k.into()));
        }
        if c.pump_done || (c.up_eof && c.bounds.is_empty() && c.recs[UP].is_empty()) {
            return Poll::Ready(Ok(()));
        }
        match &c.reader {
            Some(w) if w.will_wake(cx.waker()) => {}
            _ => c.reader = Some(cx.waker().clone()),
        }
        Poll::Pending
    }
}

impl<S> AsyncWrite for DelayStream<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        if buf.is_empty() {
            return Poll::Ready(Ok(0));
        }
        let sh = &self.sh;
        let mut c = self.current();
        if c.closed.is_some() || c.pump_done || c.shutdown.is_some() {
            return Poll::Ready(Err(c.failed.unwrap_or(io::ErrorKind::BrokenPipe).into()));
        }
        let free = c.down_free();
        if free <= OVERHEAD {
            c.owed = buf.len() as u64;
            match &c.writer {
                Some(w) if w.will_wake(cx.waker()) => {}
                _ => c.writer = Some(cx.waker().clone()),
            }
            return Poll::Pending;
        }
        // `current` stepped the model to now.
        let t = c.model.now_ns();
        let n = buf.len().min(RECORD_MAX).min((free - OVERHEAD) as usize);
        let data = &buf[..n];
        c.bytes[DOWN].extend(data);
        // The server's message ends, for the per-return record's first whole message.
        let base = c.written[DOWN];
        let mut i = 0;
        while i < n {
            let (k, ev) = c.down_scan.scan(&data[i..]);
            i += k;
            match ev {
                Some(Ok(Boundary::Frame { message_end: true })) => {
                    c.down_msg_ends.push_back(base + i as u64);
                }
                Some(_) => {}
                None => break,
            }
            if k == 0 {
                break;
            }
        }
        c.model_write(DOWN, n, t, false);
        c.owed = (buf.len() - n) as u64;
        c.held_now();
        c.reschedule(sh.clock, sh.key);
        Poll::Ready(Ok(n))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let c = self.sh.lock();
        match c.failed {
            Some(k) => Poll::Ready(Err(k.into())),
            None => Poll::Ready(Ok(())),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let sh = &self.sh;
        let mut c = sh.lock();
        if c.down_done || c.pump_done {
            return Poll::Ready(Ok(()));
        }
        let mut pump = None;
        if c.shutdown.is_none() {
            c.shutdown = Some(sh.clock.now_ns().saturating_add(c.shutdown_wait));
            c.reschedule(sh.clock, sh.key);
            pump = c.pump_waker.clone();
        }
        match &c.shutter {
            Some(w) if w.will_wake(cx.waker()) => {}
            _ => c.shutter = Some(cx.waker().clone()),
        }
        // The pump is woken after the lock is released (F2).
        drop(c);
        if let Some(p) = pump {
            p.wake();
        }
        Poll::Pending
    }
}

/// The connection's pump: owns S (C3).
struct Pump<S> {
    sh: Arc<Shared>,
    s: Option<S>,
    /// The front down record's bytes, staged once under the lock (F4): `staged` bytes, of which S has taken `sent`.
    /// Kept across a Pending or a partial write, so S is offered the rest without a copy.
    scratch: Box<[u8]>,
    staged: usize,
    sent: usize,
    /// S's reads land here, outside the lock, before they go into the scanner's buffer under it (F3): one largest
    /// frame, the most the scanner takes after the request.
    rbuf: Box<[u8]>,
    unflushed: bool,
    /// Write calls the front down record has taken so far without finishing (a split record).
    front_calls: u32,
}

impl<S> Drop for Pump<S> {
    /// Ends the connection's part of the clock and wakes whoever waits, whatever ended the pump.
    fn drop(&mut self) {
        let w = {
            let mut c = self.sh.lock();
            c.pump_done = true;
            #[cfg(test)]
            {
                c.life.ended_ns = Some(self.sh.clock.now_ns());
            }
            [c.reader.take(), c.writer.take(), c.shutter.take()]
        };
        self.sh.clock.release(self.sh.key);
        wake_all(w);
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> Future for Pump<S> {
    type Output = ();

    /// One run: each block takes the connection's lock for its own state and releases it before S is polled (F3), so
    /// S's calls (rustls's encryption, the syscalls) never run under it and DelayStream's side never waits on them.
    /// The pump alone owns S, the front down record (nothing else removes it) and the scanner's buffer, so what it
    /// staged stays valid while the lock is free. A dropped DelayStream ends it at the next block.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();
        let sh = Arc::clone(&this.sh);
        let Some(s) = this.s.as_mut() else {
            return Poll::Ready(());
        };
        let mut wake_reader;
        let mut wake_writer = false;
        {
            let mut c = sh.lock();
            if c.dropped {
                return Poll::Ready(());
            }
            match &c.pump_waker {
                Some(w) if w.will_wake(cx.waker()) => {}
                _ => c.pump_waker = Some(cx.waker().clone()),
            }
            #[cfg(test)]
            {
                c.life.polls += 1;
                c.life.deadline = c.shutdown;
                if c.shutdown.is_some_and(|d| sh.clock.now_ns() >= d) {
                    c.life.polls_after_deadline += 1;
                }
            }
            let t = c.model_t(sh.clock.now_ns());
            wake_reader = c.step_to(t)[UP];
        }

        // Down: each released record into S in one write call (C1), staged once (F4), S polled outside the lock.
        loop {
            if this.staged == 0 {
                let c = sh.lock();
                if c.dropped {
                    return Poll::Ready(());
                }
                if c.released[DOWN] == 0 {
                    break;
                }
                let rem = c.recs[DOWN][0].len - c.front_done[DOWN];
                let (a, b) = c.bytes[DOWN].as_slices();
                let k = rem.min(a.len());
                this.scratch[..k].copy_from_slice(&a[..k]);
                this.scratch[k..rem].copy_from_slice(&b[..rem - k]);
                this.staged = rem;
                this.sent = 0;
            }
            let r = Pin::new(&mut *s).poll_write(cx, &this.scratch[this.sent..this.staged]);
            // The release's lateness is taken at poll_write's return (M1), before the lock.
            let at = sh.clock.now_ns();
            match r {
                Poll::Ready(Ok(n)) if n > 0 => {
                    // AsyncWrite's contract bounds n by what was offered; kept so even a faulty S cannot overrun it.
                    let n = n.min(this.staged - this.sent);
                    this.sent += n;
                    this.unflushed = true;
                    let mut c = sh.lock();
                    c.bytes[DOWN].drain(..n);
                    c.front_done[DOWN] += n;
                    if c.dropped {
                        return Poll::Ready(());
                    }
                    if this.sent < this.staged {
                        this.front_calls += 1;
                        continue;
                    }
                    this.staged = 0;
                    this.sent = 0;
                    if let Some(rec) = c.recs[DOWN].pop_front() {
                        c.released[DOWN] -= 1;
                        c.released_model[DOWN] -= rec.len as u64 + OVERHEAD;
                        c.front_done[DOWN] = 0;
                        if this.front_calls > 0 {
                            c.own.split_writes += 1;
                        }
                        this.front_calls = 0;
                        let due = c.origin.saturating_add(rec.released.unwrap_or_default());
                        c.late(DOWN, at.saturating_sub(due));
                        wake_writer = true;
                    }
                }
                Poll::Ready(Ok(_)) => {
                    sh.lock().failed = Some(io::ErrorKind::WriteZero);
                    return Poll::Ready(());
                }
                Poll::Ready(Err(e)) => {
                    sh.lock().failed = Some(e.kind());
                    return Poll::Ready(());
                }
                Poll::Pending => break,
            }
        }
        if this.unflushed {
            match Pin::new(&mut *s).poll_flush(cx) {
                Poll::Ready(Ok(())) => this.unflushed = false,
                Poll::Ready(Err(e)) => {
                    sh.lock().failed = Some(e.kind());
                    return Poll::Ready(());
                }
                Poll::Pending => {}
            }
        }

        // Up: S read outside the lock into the pump's buffer while the up room holds one largest record (C2); under the
        // lock the bytes join the scanner's buffer, the model steps to a fresh now (it never goes back: another task
        // may have stepped it while the lock was free; and a record enters the model no earlier than its bytes were
        // read, NC-U3c-3), then the limits apply before anything is buffered (C6) and whole records enter the model.
        let mut fresh = 0;
        loop {
            let room = {
                let mut c = sh.lock();
                if c.dropped {
                    return Poll::Ready(());
                }
                // The previous read's bytes (none on the first pass); the room was computed for them under the lock,
                // and only the pump moves the scanner's buffer.
                let at = c.parse_len;
                c.parse[at..at + fresh].copy_from_slice(&this.rbuf[..fresh]);
                c.parse_len += fresh;
                let t = c.model_t(sh.clock.now_ns());
                wake_reader |= c.step_to(t)[UP];
                if let Err(r) = c.scan_up(t) {
                    c.close(r);
                    return Poll::Ready(());
                }
                let limit = c.read_limit();
                if c.up_eof || c.parse_len >= limit || c.up_room() < c.read_floor {
                    break;
                }
                (limit - c.parse_len).min(this.rbuf.len())
            };
            let mut rb = ReadBuf::new(&mut this.rbuf[..room]);
            match Pin::new(&mut *s).poll_read(cx, &mut rb) {
                Poll::Ready(Ok(())) => {
                    fresh = rb.filled().len();
                    if fresh == 0 {
                        sh.lock().up_eof = true;
                        wake_reader = true;
                        break;
                    }
                }
                Poll::Ready(Err(e)) => {
                    sh.lock().failed = Some(e.kind());
                    return Poll::Ready(());
                }
                Poll::Pending => break,
            }
        }

        // Shutdown (C3, W3): once every down byte is delivered and taken, S's own shutdown (polled outside the lock).
        // The deadline bounds all of it, S's shutdown included: at or past it, S is dropped and counted, whatever it
        // still waits for. Before it, the pump sleeps until the deadline (`due`) or a wake from S, so a shutdown that S
        // never completes costs one wake at the deadline, never a spin. Nothing new enters the down direction once a
        // shutdown began (poll_write refuses), so "taken" read under the lock stays true while it is free.
        let shutdown = {
            let c = sh.lock();
            if c.dropped {
                return Poll::Ready(());
            }
            c.shutdown
                .filter(|_| !c.down_done)
                .map(|d| (d, c.recs[DOWN].is_empty() && !this.unflushed))
        };
        if let Some((deadline, taken)) = shutdown {
            let done = taken && Pin::new(&mut *s).poll_shutdown(cx).is_ready();
            let mut c = sh.lock();
            if done {
                c.down_done = true;
            } else if sh.clock.now_ns() >= deadline {
                c.own.shutdown_timeouts += 1;
                c.down_done = true;
                return Poll::Ready(());
            }
        }

        let w = {
            let mut c = sh.lock();
            if c.dropped {
                return Poll::Ready(());
            }
            c.held_now();
            c.reschedule(sh.clock, sh.key);
            c.wakers(wake_reader, wake_writer)
        };
        wake_all(w);
        Poll::Pending
    }
}

#[cfg(test)]
#[path = "delay_tests.rs"]
pub(crate) mod tests;
