//! The connection cap (u6b-brief.md Block 1; replan-units.md:17-18; F7): one per process, N + 8 permits (N the
//! settings' client count), shared by both listeners. A permit is taken at the first QUIC packet (quinn's `Incoming`)
//! or at the TCP accept, before any TLS, and returned when the session's closing task ends (or when its handshake
//! fails or misses its deadline). With none free a QUIC `Incoming` is refused and a TCP connection dropped at once,
//! each counted per transport. At about 0.31 MB per DelayStream connection (`ws_conn_fixed_bytes`) it bounds the
//! WebSocket path's memory at (N + 8) x 0.31 MB plus the process's fixed part; the header records it.

use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;

use tokio::sync::{OwnedSemaphorePermit, Semaphore};

/// The transports the cap counts apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Via {
    Wt = 0,
    Ws = 1,
}

/// The process's connection cap (the module text).
#[derive(Debug)]
pub struct ConnCap {
    sem: Arc<Semaphore>,
    permits: usize,
    refused: [AtomicU64; 2],
}

/// One connection's permit: back to the cap when dropped (the session's closing task holds it to its end).
#[derive(Debug)]
pub(crate) struct Permit(#[allow(dead_code, reason = "held for its drop")] OwnedSemaphorePermit);

impl ConnCap {
    /// The permits beyond the client count.
    pub const SPARE: usize = 8;

    /// The cap for `clients` clients: `clients + 8` permits.
    pub fn new(clients: usize) -> Arc<ConnCap> {
        let permits = clients.saturating_add(Self::SPARE);
        Arc::new(ConnCap {
            sem: Arc::new(Semaphore::new(permits)),
            permits,
            refused: Default::default(),
        })
    }

    /// Its permits in all (the header's `conn_cap_count`).
    pub fn permits(&self) -> usize {
        self.permits
    }

    /// Permits free now.
    pub fn free(&self) -> usize {
        self.sem.available_permits()
    }

    /// Connections refused with no permit free, on WebTransport (`wt` true) or WebSocket.
    pub fn refused(&self, wt: bool) -> u64 {
        self.refused[usize::from(!wt)].load(Relaxed)
    }

    /// A permit for a new connection on `via`, or `None` (counted) when none is free.
    pub(crate) fn take(&self, via: Via) -> Option<Permit> {
        match Arc::clone(&self.sem).try_acquire_owned() {
            Ok(p) => Some(Permit(p)),
            Err(_) => {
                self.refused[via as usize].fetch_add(1, Relaxed);
                None
            }
        }
    }
}
