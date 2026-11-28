//! Fix 1's send gate on the WebSocket fallback (Mustafa, 2026-11-06: newest snapshot only with a small send buffer;
//! advisor-ws-docs-r2.md U3c (2), (3); the verifier's W22; u3c-plan.md 1.3 and C9). Shipped code: U6's WsServer calls
//! these on every tick (no copy), and `tcp_model::fixes`'s adaptive limit calls [`snapshot_budget`], so the model's
//! harness and the build cannot drift (mechanism test (e)).
//!
//! - [`snapshot_budget`]: N, the snapshots the budget holds: ceil(3 x srtt / (2 x tick)) + 1, at least 3, with srtt
//!   100 ms before the first sample (fixes.rs's rule since harness v1).
//! - [`may_write`]: a tick's snapshot is written only when all three of fixes.rs's conditions hold (fixes.rs's room
//!   step, W22): nothing is owed above the kernel socket; the socket has room for the whole snapshot; and what is
//!   outstanding (unsent plus unacknowledged: SIOCOUTQ, tcp.c:630) plus the snapshot fits in N snapshots.
//! - [`TickWriter`]: the per-tick decision. Each tick it is offered that tick's snapshot, the newest, and either
//!   writes it now or skips it, counted; it holds nothing, so a skipped snapshot can never be written later and at
//!   most one snapshot (the one owed to the socket) is ever above the socket (PM-0001-31's reading, R13).
//!
//! `size` is the whole tick's snapshot in the model's bytes: every message of the tick, written all or none, each
//! counted as its payload plus its WebSocket frame header plus TLS 1.3's 22 B per record (S0), as fixes.rs's
//! SNAP = 1,050 counts a 1,024 B snapshot.

/// srtt before the first sample (fixes.rs:166's 100 ms).
pub(crate) const NO_SAMPLE_SRTT_NS: u64 = 100_000_000;
/// The budget's floor in snapshots.
pub(crate) const MIN_SNAPSHOTS: u64 = 3;

/// N: snapshots for 1.5 smoothed round trips plus one, at least 3 (srtt 100 ms before the first sample).
pub(crate) fn snapshot_budget(srtt_ns: Option<u64>, tick_ns: u64) -> u64 {
    let srtt = srtt_ns.unwrap_or(NO_SAMPLE_SRTT_NS);
    let n = srtt
        .saturating_mul(3)
        .div_ceil(tick_ns.saturating_mul(2).max(1))
        .saturating_add(1);
    n.max(MIN_SNAPSHOTS)
}

/// Whether a snapshot of `size` bytes may be written now (W22: fixes.rs's three conditions): nothing `owed` above the
/// kernel socket; `free` (the socket's room) holds the whole snapshot; `outstanding` plus it fits in `n` snapshots.
pub(crate) fn may_write(outstanding: u64, owed: u64, free: u64, size: u64, n: u64) -> bool {
    owed == 0 && free >= size && outstanding.saturating_add(size) <= n.saturating_mul(size)
}

/// What the gate reads each tick, from the connection's socket (DelayStream's model on L2; TCP_INFO and SIOCOUTQ on
/// the plain path, U6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Inputs {
    /// The down direction's smoothed RTT (tcpi_rtt, tcp.c:3842), None before the first sample.
    pub(crate) srtt_ns: Option<u64>,
    /// Unsent plus unacknowledged bytes (SIOCOUTQ: write_seq − snd_una, tcp.c:630).
    pub(crate) outstanding: u64,
    /// Bytes held above the kernel socket (tungstenite's buffer; on the plain path rustls's unsent TLS too).
    pub(crate) owed: u64,
    /// The socket's room for new bytes (the send-buffer space npt-room gets: plan §4.4).
    pub(crate) free: u64,
}

/// One tick's snapshot: its id (the tick it was made on) and its size in the model's bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Snap {
    pub(crate) id: u64,
    pub(crate) size: u64,
}

/// The planted defects of the mechanism tests (u3c-plan.md 1.4 (d)); never in a build that runs.
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Bite {
    /// A budget twice the formula's (budget_scale 2): fails (a) and (c).
    BudgetX2,
    /// A queueing writer: a skipped snapshot is kept and written later, oldest first: fails (b) and PM-0001-31's "at
    /// most one snapshot above the socket".
    Queueing,
}

/// The per-tick writer decision (fix 1): newest only, a skip counted, never written later.
#[derive(Debug)]
pub(crate) struct TickWriter {
    tick_ns: u64,
    written: u64,
    skipped: u64,
    #[cfg(test)]
    bite: Option<Bite>,
    /// The queueing bite's held snapshots (always empty otherwise).
    #[cfg(test)]
    queue: std::collections::VecDeque<Snap>,
}

impl TickWriter {
    pub(crate) fn new(tick_ns: u64) -> TickWriter {
        TickWriter {
            tick_ns,
            written: 0,
            skipped: 0,
            #[cfg(test)]
            bite: None,
            #[cfg(test)]
            queue: std::collections::VecDeque::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn with_bite(mut self, bite: Bite) -> TickWriter {
        self.bite = Some(bite);
        self
    }

    /// N at this srtt (the bite doubles it).
    pub(crate) fn budget(&self, srtt_ns: Option<u64>) -> u64 {
        let n = snapshot_budget(srtt_ns, self.tick_ns);
        #[cfg(test)]
        if self.bite == Some(Bite::BudgetX2) {
            return 2 * n;
        }
        n
    }

    /// This tick's snapshot `snap`, the newest: `Some` of the snapshot to write now, or `None` to skip it (counted;
    /// the caller drops it, so it is never written later). The caller calls this once per tick and nowhere else.
    pub(crate) fn on_tick(&mut self, snap: Snap, inp: Inputs) -> Option<Snap> {
        let n = self.budget(inp.srtt_ns);
        #[cfg(test)]
        if self.bite == Some(Bite::Queueing) {
            // Everything goes through a queue; the oldest leaves when the gate lets it.
            self.queue.push_back(snap);
            let front = self.queue.front().copied()?;
            if may_write(inp.outstanding, inp.owed, inp.free, front.size, n) {
                self.written += 1;
                return self.queue.pop_front();
            }
            self.skipped += 1;
            return None;
        }
        if may_write(inp.outstanding, inp.owed, inp.free, snap.size, n) {
            self.written += 1;
            Some(snap)
        } else {
            self.skipped += 1;
            None
        }
    }

    /// Snapshots written so far.
    pub(crate) fn written(&self) -> u64 {
        self.written
    }

    /// Snapshots skipped so far (WsServer reports them to npt-room as BufferFull: plan §4.4).
    pub(crate) fn skipped(&self) -> u64 {
        self.skipped
    }

    /// Snapshots the writer itself holds above the socket: none (the queueing bite holds its queue).
    pub(crate) fn held(&self) -> usize {
        #[cfg(test)]
        return self.queue.len();
        #[cfg(not(test))]
        0
    }
}

#[cfg(test)]
#[path = "ws_gate_tests.rs"]
mod tests;
