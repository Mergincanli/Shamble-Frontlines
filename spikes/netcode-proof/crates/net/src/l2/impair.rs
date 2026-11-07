//! The impairment core: pure logic on ns stamps (no I/O, no clock), generic over the buffer it carries, so the tests
//! drive it on simulated time and ImpairSocket drives it from its two threads.
//!
//! - **Peers** (M10): a table of fixed capacity keyed by the peer's address with IPv4-mapped IPv6 normalised; a new
//!   address takes a free slot, else evicts the least recently seen peer not pinned by `set_peer` (counted), else
//!   takes the shared overflow flow (counted). Nothing grows with client data. Each peer keeps its first raw packet's
//!   stamp (the Join deadline's start, read by npt-server at U6: U3-Q8). A datagram the host checks drop (size, byte
//!   bounds, pool) touches no peer: the table is consulted, not changed, until a datagram passes them, so a flood from
//!   new (possibly spoofed) addresses creates no peer and evicts nobody. Carried to U6: the control plane pins every
//!   joined client with `set_peer`, so unpinned addresses can never evict a real client.
//! - **Fates** (M4): one ChaCha8 stream per (peer, direction), keyed by the run's seed and the peer's stable slot id
//!   (its first-seen ordinal, never its ephemeral address), 16 words per datagram used or not, at L1's word places
//!   (loss 0, jitter 1, reorder 2). So no other peer's traffic, and no interleaving of the two directions, moves a
//!   datagram's fate.
//! - **Each datagram, in order:** the size limit (dropped before it is queued: no index), the byte bounds per peer
//!   and in total (a tail drop counted apart from the profile's loss: no index), a pool buffer (an empty pool is a
//!   counted drop: no index); then its peer's slot, its index and 16 words; the UDP block; a B1 burst; the shaper
//!   (M19); the profile's loss; its delay, the profile's one-way delay net of the base RTT plus its own uniform jitter
//!   (M3's "jitter per datagram"); and L1's reorder step (M3).
//! - **The byte bounds count buffers:** every queued datagram is charged its whole pool buffer
//!   ([`L2Config::BUFFER_BYTES`]), whatever its payload, since that is the memory it holds. So a peer's bound is a
//!   number of pool buffers, and one peer sending small datagrams cannot hold more of the shared pool than its own
//!   bound; at the defaults both directions' totals together are the pool, so the totals, not the pool, are what bind
//!   (L2Config's defaults).
//! - **The shaper across a settings change** (a throttle's start or end): each flow keeps its last departure; a new
//!   shaper starts busy until then, and with no shaper a datagram departs at max(intake, last departure). So the
//!   datagrams the old shaper accepted keep their departures and nothing taken in later leaves before them.
//! - **The reorder step** is npt-l1's as built: a datagram it draws is held; if the next datagram of its flow is
//!   taken in before the held one is due and is not dropped, the held one becomes due 1 ns after it (overtaken);
//!   either way it is released into the queue then. A held datagram whose own due time comes first is released at it
//!   (a token in the queue marks that time).
//! - **One release queue** (M2): a min-heap ordered by (due, sequence) across all peers and both directions, so one
//!   peer's jitter never holds back another's datagrams.

use super::config::L2Config;
use super::shaper::Shaper;
use super::stats::{Counters, ERR_CLASSES, Fate, FateLog, Kind, Lateness, err_class};
use rand_chacha::ChaCha8Rng;
use rand_core::{Rng, SeedableRng};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::net::{IpAddr, SocketAddr};

/// The 64-bit draws every datagram takes from its flow's stream, used or not (npt-l1's count).
pub(crate) const WORDS: usize = 16;
const W_LOSS: usize = 0;
const W_JITTER: usize = 1;
const W_REORDER: usize = 2;
/// The fixed part of the 32-byte ChaCha8 key; the run's seed, little-endian, fills the first 8 bytes.
const KEY_TAG: &[u8; 24] = b"npt-l2 fate draws, v1\0\0\0";
/// What one queued datagram counts against the byte bounds: its whole pool buffer, whatever its payload.
pub(crate) const CHARGE_BYTES: u64 = L2Config::BUFFER_BYTES as u64;

/// Inbound: client to server, on RS. Outbound: server to client.
pub(crate) const UP: usize = 0;
pub(crate) const DOWN: usize = 1;
/// The overflow flow's slot.
pub(crate) const OVERFLOW: u32 = u32::MAX;
const NO_INDEX: u64 = u64::MAX;

/// IPv4-mapped IPv6 addresses as IPv4 (M10), so one peer is one key on a dual-stack socket.
pub(crate) fn normalise(a: SocketAddr) -> SocketAddr {
    match a {
        SocketAddr::V6(v6) => match v6.ip().to_ipv4_mapped() {
            Some(v4) => SocketAddr::new(IpAddr::V4(v4), v6.port()),
            None => a,
        },
        SocketAddr::V4(_) => a,
    }
}

/// A uniform integer in `[0, n)` from one 64-bit draw (npt-l1's mapping).
fn below(word: u64, n: u64) -> u64 {
    ((u128::from(word) * u128::from(n)) >> 64) as u64
}

/// A peer's impairment: L2Config's per-peer part, as the draws read it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Settings {
    pub(crate) one_way_ns: u64,
    pub(crate) half_jitter_ns: u64,
    pub(crate) loss_ppm: u32,
    pub(crate) reorder_permille: u32,
    pub(crate) bursts_ns: Vec<(u64, u64)>,
    pub(crate) shaper: Option<(u64, u64)>,
    pub(crate) udp_block: bool,
}

impl Settings {
    pub(crate) fn of(cfg: &L2Config) -> Settings {
        if cfg.pass_through {
            return Settings {
                one_way_ns: 0,
                half_jitter_ns: 0,
                loss_ppm: 0,
                reorder_permille: 0,
                bursts_ns: Vec::new(),
                shaper: None,
                udp_block: false,
            };
        }
        let p = &cfg.profile;
        Settings {
            one_way_ns: p.rtt_ns.saturating_sub(cfg.base_rtt_ns) / 2,
            half_jitter_ns: p.half_jitter_ns(),
            loss_ppm: p.loss_ppm,
            reorder_permille: p.reorder.permille,
            bursts_ns: p.bursts_ns.clone(),
            shaper: cfg.shaper,
            udp_block: cfg.udp_block,
        }
    }

    fn in_burst(&self, t: u64) -> bool {
        self.bursts_ns
            .iter()
            .any(|&(start, len)| t >= start && t - start < len)
    }

    /// One direction's delay: d − a + uniform [0, 2a] (npt-l1's formula).
    fn delay(&self, word: u64) -> u64 {
        let a = self.half_jitter_ns;
        (self.one_way_ns + below(word, 2 * a + 1)).saturating_sub(a)
    }
}

/// A datagram the reorder step holds.
struct Held<B> {
    index: u64,
    due_ns: u64,
    t_in_ns: u64,
    len: u32,
    item: B,
}

struct Flow<B> {
    rng: ChaCha8Rng,
    next_index: u64,
    shaper: Option<Shaper>,
    /// The latest departure given (the shaper's, or the intake time without one): a new shaper starts busy until
    /// then and no shaper departs no earlier, so a settings change never lets a datagram overtake a queued one.
    last_depart_ns: u64,
    held: Option<Held<B>>,
    /// Buffer bytes queued: [`CHARGE_BYTES`] per datagram.
    queued_bytes: u64,
    /// The highest index released (RFC 4737's reorder count).
    highest: Option<u64>,
}

struct Peer<B> {
    key: SocketAddr,
    ordinal: u64,
    pinned: bool,
    settings: Settings,
    flows: [Flow<B>; 2],
    first_seen_ns: Option<u64>,
    last_seen_ns: u64,
}

/// One queued datagram, or a held datagram's token (`item` None).
pub(crate) struct Entry<B> {
    pub(crate) due_ns: u64,
    seq: u64,
    pub(crate) dir: usize,
    slot: u32,
    ordinal: u64,
    index: u64,
    pub(crate) t_in_ns: u64,
    len: u32,
    overtaken: bool,
    item: Option<B>,
}

impl<B> Entry<B> {
    /// The datagram this entry carries (never None for an entry `pop_due` returns).
    pub(crate) fn take_item(&mut self) -> Option<B> {
        self.item.take()
    }
    pub(crate) fn item(&self) -> Option<&B> {
        self.item.as_ref()
    }
}

impl<B> PartialEq for Entry<B> {
    fn eq(&self, other: &Self) -> bool {
        (self.due_ns, self.seq) == (other.due_ns, other.seq)
    }
}
impl<B> Eq for Entry<B> {}
impl<B> PartialOrd for Entry<B> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<B> Ord for Entry<B> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.due_ns, self.seq).cmp(&(other.due_ns, other.seq))
    }
}

/// What became of a datagram at intake.
pub(crate) enum Intake<B> {
    /// Queued (or held by the reorder step) and due then.
    Queued { due_ns: u64 },
    /// Dropped; the buffer comes back if one was taken. (The kind is counted and logged here; the tests read it.)
    Dropped {
        #[cfg_attr(not(test), allow(dead_code))]
        kind: Kind,
        item: Option<B>,
    },
}

/// Where a datagram from an address goes, found without changing the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    /// Its peer's slot.
    Known(u32),
    /// A new peer in a free slot.
    Free(usize),
    /// A new peer in this slot, whose unpinned peer (the least recently seen) is evicted.
    Evict(usize),
    /// The shared overflow flow: every slot pinned.
    Overflow,
}

/// The impairment core of one socket.
pub(crate) struct Impairer<B> {
    key: [u8; 32],
    max_datagram: usize,
    /// Buffer bytes ([`CHARGE_BYTES`] per datagram) one peer may hold queued in one direction.
    peer_queue_bytes: u64,
    /// Buffer bytes all peers may hold queued in one direction.
    total_queue_bytes: u64,
    default: Settings,
    slots: Vec<Option<Peer<B>>>,
    /// (normalised address, slot), sorted by address; at most `slots.len()` entries.
    index: Vec<(SocketAddr, u32)>,
    overflow: Peer<B>,
    next_ordinal: u64,
    heap: BinaryHeap<Reverse<Entry<B>>>,
    seq: u64,
    /// Buffer bytes queued per direction ([`CHARGE_BYTES`] per datagram).
    queued: [u64; 2],
    pub(crate) run: Counters,
    pub(crate) second: Counters,
    pub(crate) late_run: [Lateness; 2],
    pub(crate) late_second: [Lateness; 2],
    pub(crate) fates: FateLog,
}

fn flows<B>(key: &[u8; 32], ordinal: u64, s: &Settings) -> [Flow<B>; 2] {
    [UP, DOWN].map(|dir| {
        let mut rng = ChaCha8Rng::from_seed(*key);
        rng.set_stream(ordinal.wrapping_mul(2) | dir as u64);
        Flow {
            rng,
            next_index: 0,
            shaper: s.shaper.map(|(r, q)| Shaper::new(r, q)),
            last_depart_ns: 0,
            held: None,
            queued_bytes: 0,
            highest: None,
        }
    })
}

impl<B> Impairer<B> {
    pub(crate) fn new(cfg: &L2Config) -> Impairer<B> {
        let mut key = [0u8; 32];
        key[..8].copy_from_slice(&cfg.seed.to_le_bytes());
        key[8..].copy_from_slice(KEY_TAG);
        let default = Settings::of(cfg);
        let cap = cfg.peer_capacity.max(1);
        let overflow = Peer {
            key: SocketAddr::from(([0, 0, 0, 0], 0)),
            ordinal: u64::MAX,
            pinned: false,
            flows: flows(&key, u64::MAX, &default),
            settings: default.clone(),
            first_seen_ns: None,
            last_seen_ns: 0,
        };
        // Every entry owns a buffer or is the token of a held datagram, which owns one: at most twice the pool.
        let heap_cap = 2 * cfg.pool_buffers.max(1) + 16;
        Impairer {
            key,
            max_datagram: cfg.max_datagram_bytes.min(L2Config::BUFFER_BYTES),
            peer_queue_bytes: cfg.peer_queue_bytes,
            total_queue_bytes: cfg.total_queue_bytes,
            default,
            slots: (0..cap).map(|_| None).collect(),
            index: Vec::with_capacity(cap),
            overflow,
            next_ordinal: 0,
            heap: BinaryHeap::with_capacity(heap_cap),
            seq: 0,
            queued: [0; 2],
            run: Counters::default(),
            second: Counters::default(),
            late_run: [Lateness::new(), Lateness::new()],
            late_second: [Lateness::new(), Lateness::new()],
            fates: FateLog::new(cfg.fate_log_capacity),
        }
    }

    fn lookup(&self, key: SocketAddr) -> Option<u32> {
        self.index
            .binary_search_by(|(a, _)| a.cmp(&key))
            .ok()
            .map(|i| self.index[i].1)
    }

    fn peer_mut(&mut self, slot: u32) -> &mut Peer<B> {
        if slot == OVERFLOW {
            return &mut self.overflow;
        }
        self.slots[slot as usize]
            .as_mut()
            .expect("an indexed slot holds a peer")
    }

    /// The peer at `slot` if it is still the one with `ordinal` (an evicted peer's entries outlive it).
    fn live_peer_mut(&mut self, slot: u32, ordinal: u64) -> Option<&mut Peer<B>> {
        if slot == OVERFLOW {
            return Some(&mut self.overflow);
        }
        self.slots
            .get_mut(slot as usize)?
            .as_mut()
            .filter(|p| p.ordinal == ordinal)
    }

    fn count(&mut self, dir: usize, kind: Kind) {
        self.run.fates[dir][kind as usize] += 1;
        self.second.fates[dir][kind as usize] += 1;
    }

    fn log(&mut self, f: Fate) {
        if !self.fates.push(f) {
            self.run.fate_log_full += 1;
            self.second.fate_log_full += 1;
        }
    }

    /// Where a datagram from `key` (normalised) would go, changing nothing (M10's bounded table).
    fn target(&self, key: SocketAddr) -> Target {
        if let Some(s) = self.lookup(key) {
            return Target::Known(s);
        }
        if let Some(s) = self.slots.iter().position(Option::is_none) {
            return Target::Free(s);
        }
        let victim = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                p.as_ref()
                    .filter(|p| !p.pinned)
                    .map(|p| (p.last_seen_ns, i))
            })
            .min();
        match victim {
            Some((_, v)) => Target::Evict(v),
            None => Target::Overflow,
        }
    }

    /// The slot for `key` (normalised) at `t` per `target`: creates the new peer (evicting the least recently seen
    /// unpinned one when the table is full), or counts a use of the overflow flow. Called only for a datagram that
    /// passed the host checks, and by `set_peer`.
    fn place(&mut self, key: SocketAddr, target: Target, t: u64) -> u32 {
        let slot = match target {
            Target::Known(s) => return s,
            Target::Overflow => {
                self.run.peer_overflow += 1;
                self.second.peer_overflow += 1;
                return OVERFLOW;
            }
            Target::Free(s) => s,
            Target::Evict(v) => {
                self.evict(v);
                v
            }
        };
        let ordinal = self.next_ordinal;
        self.next_ordinal += 1;
        let settings = self.default.clone();
        self.slots[slot] = Some(Peer {
            key,
            ordinal,
            pinned: false,
            flows: flows(&self.key, ordinal, &settings),
            settings,
            first_seen_ns: None,
            last_seen_ns: t,
        });
        let at = self.index.partition_point(|(a, _)| *a < key);
        self.index.insert(at, (key, slot as u32));
        slot as u32
    }

    /// Evicts the peer in `slot`: its held datagrams go into the queue at their own due time (their buffers are never
    /// lost), and its index entry goes.
    fn evict(&mut self, slot: usize) {
        let Some(mut p) = self.slots[slot].take() else {
            return;
        };
        for dir in [UP, DOWN] {
            if let Some(h) = p.flows[dir].held.take() {
                self.push_datagram(dir, slot as u32, p.ordinal, h, false);
            }
        }
        if let Ok(i) = self.index.binary_search_by(|(a, _)| a.cmp(&p.key)) {
            self.index.remove(i);
        }
        self.run.peers_evicted += 1;
        self.second.peers_evicted += 1;
    }

    fn push_datagram(&mut self, dir: usize, slot: u32, ordinal: u64, h: Held<B>, overtaken: bool) {
        self.seq += 1;
        self.heap.push(Reverse(Entry {
            due_ns: h.due_ns,
            seq: self.seq,
            dir,
            slot,
            ordinal,
            index: h.index,
            t_in_ns: h.t_in_ns,
            len: h.len,
            overtaken,
            item: Some(h.item),
        }));
    }

    fn push_token(&mut self, dir: usize, slot: u32, ordinal: u64, index: u64, due_ns: u64) {
        self.seq += 1;
        self.heap.push(Reverse(Entry {
            due_ns,
            seq: self.seq,
            dir,
            slot,
            ordinal,
            index,
            t_in_ns: 0,
            len: 0,
            overtaken: false,
            item: None,
        }));
    }

    /// Takes in one datagram of `len` bytes from (inbound) or to (outbound) `addr` at `t`. `take` gives the buffer
    /// with the datagram copied in, or None for an empty pool.
    pub(crate) fn intake(
        &mut self,
        dir: usize,
        addr: SocketAddr,
        t: u64,
        len: usize,
        take: impl FnOnce() -> Option<B>,
    ) -> Intake<B> {
        // The host checks read the table but change nothing in it: a dropped datagram creates no peer, evicts nobody
        // and stamps nothing. A new address's own flow is empty (or it is the overflow flow's, if it would go there).
        let key = normalise(addr);
        let target = self.target(key);
        let (known, flow_bytes) = match target {
            Target::Known(s) => {
                let p = self.peer_mut(s);
                (p.ordinal, p.flows[dir].queued_bytes)
            }
            Target::Overflow => (self.overflow.ordinal, self.overflow.flows[dir].queued_bytes),
            // No peer yet: the fate log's ordinal for a drop before the table.
            Target::Free(_) | Target::Evict(_) => (u64::MAX, 0),
        };
        let len32 = u32::try_from(len).unwrap_or(u32::MAX);
        let early = |ordinal, kind| Fate {
            ordinal,
            dir: dir as u8,
            kind,
            overtaken: false,
            index: NO_INDEX,
            t_in_ns: t,
            due_ns: 0,
            out_ns: 0,
            len: len32,
            errno: 0,
        };
        // Host-protection drops, before any fate draw: they take no index. The byte bounds count the whole buffer.
        let pre = if len > self.max_datagram {
            Some(Kind::Oversize)
        } else if flow_bytes + CHARGE_BYTES > self.peer_queue_bytes
            || self.queued[dir] + CHARGE_BYTES > self.total_queue_bytes
        {
            Some(Kind::QueueDrop)
        } else {
            None
        };
        if let Some(kind) = pre {
            self.count(dir, kind);
            self.log(early(known, kind));
            return Intake::Dropped { kind, item: None };
        }
        let Some(item) = take() else {
            self.count(dir, Kind::PoolDrop);
            self.log(early(known, Kind::PoolDrop));
            return Intake::Dropped {
                kind: Kind::PoolDrop,
                item: None,
            };
        };
        // Past the host checks: only now is the datagram's peer placed, seen and stamped.
        let slot = self.place(key, target, t);
        let ordinal = {
            let p = self.peer_mut(slot);
            p.last_seen_ns = p.last_seen_ns.max(t);
            if dir == UP && p.first_seen_ns.is_none() {
                p.first_seen_ns = Some(t);
            }
            p.ordinal
        };
        let bytes = len as u64;
        // The fates: an index and 16 words, used or not.
        let (index, w, outcome, held) = {
            let p = self.peer_mut(slot);
            let s = &p.settings;
            let f = &mut p.flows[dir];
            let index = f.next_index;
            f.next_index += 1;
            let mut w = [0u64; WORDS];
            for x in &mut w {
                *x = f.rng.next_u64();
            }
            let outcome = if s.udp_block {
                Err(Kind::Blocked)
            } else if s.in_burst(t) {
                Err(Kind::Burst)
            } else {
                // The shaper (M19) serialises in intake order, before the delay. Without one the datagram departs at
                // once, but never before the flow's last departure (a throttle that just ended).
                let depart = match f.shaper.as_mut() {
                    Some(sh) => sh.offer(t, bytes),
                    None => Some(t.max(f.last_depart_ns)),
                };
                if let Some(d) = depart {
                    f.last_depart_ns = d;
                }
                match depart {
                    None => Err(Kind::ShaperDrop),
                    Some(_) if below(w[W_LOSS], 1_000_000) < u64::from(s.loss_ppm) => {
                        Err(Kind::Lost)
                    }
                    Some(d) => Ok(d + s.delay(w[W_JITTER])),
                }
            };
            (index, w, outcome, f.held.take())
        };
        // The reorder step: the held datagram is overtaken by this one if this one is taken in before the held one is
        // due and is not dropped; either way it is released into the queue now.
        if let Some(mut h) = held {
            let mut overtaken = false;
            if let Ok(due) = outcome
                && t < h.due_ns
                && due + 1 > h.due_ns
            {
                h.due_ns = due + 1;
                overtaken = true;
                self.run.overtaken[dir] += 1;
                self.second.overtaken[dir] += 1;
            }
            self.push_datagram(dir, slot, ordinal, h, overtaken);
        }
        let due = match outcome {
            Err(kind) => {
                self.count(dir, kind);
                self.log(Fate {
                    index,
                    ..early(ordinal, kind)
                });
                return Intake::Dropped {
                    kind,
                    item: Some(item),
                };
            }
            Ok(due) => due,
        };
        let hold = {
            let p = self.peer_mut(slot);
            below(w[W_REORDER], 1000) < u64::from(p.settings.reorder_permille)
        };
        let h = Held {
            index,
            due_ns: due,
            t_in_ns: t,
            len: len32,
            item,
        };
        if hold {
            self.peer_mut(slot).flows[dir].held = Some(h);
            self.push_token(dir, slot, ordinal, index, due);
        } else {
            self.push_datagram(dir, slot, ordinal, h, false);
        }
        self.peer_mut(slot).flows[dir].queued_bytes += CHARGE_BYTES;
        self.queued[dir] += CHARGE_BYTES;
        Intake::Queued { due_ns: due }
    }

    /// The earliest due time in the queue (a token's included).
    pub(crate) fn next_due(&self) -> Option<u64> {
        self.heap.peek().map(|e| e.0.due_ns)
    }

    /// The next datagram due by `now`, in (due, sequence) order across all peers and both directions.
    pub(crate) fn pop_due(&mut self, now: u64) -> Option<Entry<B>> {
        loop {
            if self.heap.peek()?.0.due_ns > now {
                return None;
            }
            let Reverse(e) = self.heap.pop()?;
            if e.item.is_some() {
                return Some(e);
            }
            // A token: release the held datagram at its own due time if it is still held.
            let held = match self.live_peer_mut(e.slot, e.ordinal) {
                Some(p)
                    if p.flows[e.dir]
                        .held
                        .as_ref()
                        .is_some_and(|h| h.index == e.index) =>
                {
                    p.flows[e.dir].held.take()
                }
                _ => None,
            };
            if let Some(h) = held {
                return Some(Entry {
                    due_ns: h.due_ns,
                    seq: e.seq,
                    dir: e.dir,
                    slot: e.slot,
                    ordinal: e.ordinal,
                    index: h.index,
                    t_in_ns: h.t_in_ns,
                    len: h.len,
                    overtaken: false,
                    item: Some(h.item),
                });
            }
        }
    }

    /// Records a popped datagram's end: released (`Delivered`), or refused by the kernel (`SendError` with its
    /// errno, `WouldBlock`), at `out_ns`; or dropped at hand-over (`Oversize`).
    pub(crate) fn finish(&mut self, e: &Entry<B>, kind: Kind, out_ns: u64, errno: i32) {
        let bytes = CHARGE_BYTES;
        self.queued[e.dir] = self.queued[e.dir].saturating_sub(bytes);
        let mut reordered = false;
        if let Some(p) = self.live_peer_mut(e.slot, e.ordinal) {
            let f = &mut p.flows[e.dir];
            f.queued_bytes = f.queued_bytes.saturating_sub(bytes);
            if kind == Kind::Delivered {
                match f.highest {
                    Some(h) if e.index < h => reordered = true,
                    _ => f.highest = Some(e.index),
                }
            }
        }
        self.count(e.dir, kind);
        if reordered {
            self.run.reordered[e.dir] += 1;
            self.second.reordered[e.dir] += 1;
        }
        if kind == Kind::SendError {
            let c = err_class(errno);
            debug_assert!(c < ERR_CLASSES.len());
            self.run.send_err[c] += 1;
            self.second.send_err[c] += 1;
        }
        if kind != Kind::Oversize {
            let late = out_ns.saturating_sub(e.due_ns);
            self.late_run[e.dir].record(late);
            self.late_second[e.dir].record(late);
        }
        self.log(Fate {
            ordinal: e.ordinal,
            dir: e.dir as u8,
            kind,
            overtaken: e.overtaken,
            index: e.index,
            t_in_ns: e.t_in_ns,
            due_ns: e.due_ns,
            out_ns,
            len: e.len,
            errno: u32::try_from(errno).unwrap_or(0),
        });
    }

    /// Sets one peer's settings and pins it (never evicted). False if every slot is pinned already.
    pub(crate) fn set_peer(&mut self, addr: SocketAddr, s: Settings, t: u64) -> bool {
        let key = normalise(addr);
        let target = self.target(key);
        let slot = self.place(key, target, t);
        if slot == OVERFLOW {
            return false;
        }
        let p = self.peer_mut(slot);
        p.pinned = true;
        apply(p, s);
        true
    }

    /// Sets the settings every unpinned peer has, now and later (the overflow flow's too).
    pub(crate) fn set_default(&mut self, s: Settings) {
        for p in self.slots.iter_mut().flatten().filter(|p| !p.pinned) {
            apply(p, s.clone());
        }
        apply(&mut self.overflow, s.clone());
        self.default = s;
    }

    /// The first raw packet's stamp from `addr` (M10, U3-Q8).
    pub(crate) fn first_seen(&self, addr: SocketAddr) -> Option<u64> {
        let slot = self.lookup(normalise(addr))?;
        self.slots[slot as usize].as_ref()?.first_seen_ns
    }

    pub(crate) fn peers(&self) -> usize {
        self.index.len()
    }

    pub(crate) fn queued_bytes(&self) -> [u64; 2] {
        self.queued
    }

    /// Starts a new per-second window.
    pub(crate) fn take_second(&mut self) -> Counters {
        for l in &mut self.late_second {
            l.clear();
        }
        std::mem::take(&mut self.second)
    }

    #[cfg(test)]
    fn ordinal_of(&self, addr: SocketAddr) -> Option<u64> {
        let slot = self.lookup(normalise(addr))?;
        Some(self.slots[slot as usize].as_ref()?.ordinal)
    }
}

/// New settings for a peer. A changed shaper starts busy until the flow's last departure (its first departure is never
/// before the old shaper's last), and the datagrams the old one accepted keep their departures, so a throttle's start
/// or end never lets a datagram overtake a queued one.
fn apply<B>(p: &mut Peer<B>, s: Settings) {
    if p.settings.shaper != s.shaper {
        for f in &mut p.flows {
            f.shaper = s
                .shaper
                .map(|(r, q)| Shaper::busy_until(r, q, f.last_depart_ns));
        }
    }
    p.settings = s;
}

#[cfg(test)]
#[path = "impair_tests.rs"]
mod tests;
