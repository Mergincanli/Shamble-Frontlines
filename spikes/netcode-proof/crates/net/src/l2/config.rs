//! `L2Config`, the one small L2 configuration type (plan §4.7; Mustafa's U3-Q7, 2026-11-06; the advisor's M6). Its
//! fields are std and npt-l1 types only, so it adds no other public item: the profile, its reorder step and its B1
//! bursts are npt-l1's `Profile` and `ReorderStep`, read the same way on both layers (M3).

use npt_l1::{Profile, ReorderStep};

/// One L2 configuration: the run's settings for an [`ImpairSocket`](super::ImpairSocket), and the per-peer settings
/// a peer gets (its profile with its reorder step and bursts, the shaper, the UDP block).
///
/// The per-peer part is `profile`, `base_rtt_ns`, `pass_through`, `shaper` and `udp_block`: what
/// [`ImpairSocket::set_peer`](super::ImpairSocket::set_peer) and
/// [`ImpairSocket::set_default`](super::ImpairSocket::set_default) read. The rest is the socket's and is read once,
/// at [`ImpairSocket::bind`](super::ImpairSocket::bind).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L2Config {
    /// Every fate draw comes from it (the host passes it in; never the clock: plan §4.5).
    pub seed: u64,
    /// The peer's network profile: per direction half its round trip net of `base_rtt_ns`, a uniform jitter of
    /// ± half its jitter, its loss, its B1 bursts, and its reorder step (npt-l1's, as built: M3). The bursts' start
    /// and length are on the clock of the layer that applies them: [`ImpairSocket::now_ns`](super::ImpairSocket::now_ns)
    /// for datagrams, [`DelayStream::now_ns`](super::DelayStream::now_ns) on the WebSocket path (u3c-plan.md C5).
    /// DelayStream uses no reorder step: its TCP stream never reorders (S1).
    pub profile: Profile,
    /// The path's measured base round trip: the injected one-way delay is (profile round trip − base) ÷ 2
    /// (PM-0001-3's reading, the advisor, 2026-11-06). 0 on loopback.
    pub base_rtt_ns: u64,
    /// The pass-through: every impairment off (no delay, jitter, loss, reorder step, burst, shaper or block), through
    /// the same socket, release thread and timer slack, so clean references pay the same CPU (M12). The header records
    /// the mode.
    pub pass_through: bool,
    /// The shaper (M19): `(rate in bits per second, drop-tail queue in bytes)`, each datagram counted as its payload
    /// plus 28 B, a FIFO before the profile's delay, in each direction. See [`L2Config::with_throttle`] (PM-0001-23)
    /// and [`L2Config::with_bottleneck`] (PM-0001-24).
    pub shaper: Option<(u64, u64)>,
    /// PM-0001-29's UDP block: every datagram dropped, both ways, until lifted.
    pub udp_block: bool,
    /// The largest datagram (UDP payload) accepted; a larger one is dropped and counted before it is queued (M10).
    /// At most [`L2Config::BUFFER_BYTES`].
    pub max_datagram_bytes: usize,
    /// The fixed pool of [`L2Config::BUFFER_BYTES`] buffers every queued datagram is copied into (M11), shared by every
    /// peer and both directions; an empty pool is a counted drop. At the defaults it holds both directions' total
    /// bounds exactly, so it cannot run empty and the byte bounds are what bind.
    pub pool_buffers: usize,
    /// The bytes one peer may hold queued in one direction (its own bound, M10), every queued datagram charged its
    /// whole pool buffer ([`L2Config::BUFFER_BYTES`]) whatever its payload, so this is a number of buffers × 1,500 B;
    /// beyond it a tail drop, counted apart from the profile's loss.
    pub peer_queue_bytes: u64,
    /// The bytes all peers may hold queued in one direction together (M10), charged the same way.
    pub total_queue_bytes: u64,
    /// The per-peer table's fixed capacity (M10): beyond it the least recently seen peer not set by
    /// [`ImpairSocket::set_peer`](super::ImpairSocket::set_peer) is evicted, counted; with none, the datagram takes
    /// the shared overflow flow, counted.
    pub peer_capacity: usize,
    /// The release thread's timer slack, written by the thread itself to `/proc/<tid>/timerslack_ns` and read back
    /// (M14; Mustafa's U3-Q2 (b)): 1 ns. 0 leaves the kernel's default (50 µs) in place. The header records the value
    /// read back at the thread's start and the value the thread reads when the header is built.
    pub timer_slack_ns: u64,
    /// The fate log's fixed capacity in entries (0: no log); `ImpairSocket::drain_fate_log` empties it. Entries
    /// beyond it are counted, not kept.
    pub fate_log_capacity: usize,
    /// The WebSocket frame and message limit in payload bytes (the catalogue's 513 B: `SessionControl`'s 512 B plus
    /// the channel tag; docs/design/netcode-proof-messages.md:61-78). One source: DelayStream applies it to the
    /// client's frames before any delay buffering (plan §4.4), and tungstenite's `max_frame_size` and
    /// `max_message_size` take the same value.
    pub ws_max_message_bytes: usize,
    /// The cap on the client's WebSocket upgrade request on DelayStream's path (8,192 B, nginx's
    /// `large_client_header_buffers` default); a longer request closes the connection before it is buffered further.
    /// The plain path keeps tungstenite's own 64 KiB handshake cap (u3c-plan.md 1.2).
    pub ws_max_request_bytes: usize,
}

impl L2Config {
    /// Each pool buffer's size (M11).
    pub const BUFFER_BYTES: usize = 1_500;
    /// A full datagram on the shaper's count: quinn's largest packet after MTU discovery, 1,452 B, plus 28 B (the
    /// advisor's ruling on M19).
    pub const FULL_DATAGRAM_BYTES: u64 = 1_480;
    /// PM-0001-23's throttle: 128 kbps.
    pub const THROTTLE_BITS_PER_S: u64 = 128_000;
    /// The largest UDP payload on a 1,500 B path (1,500 − 28): quinn's receive buffer per datagram.
    pub const DEFAULT_MAX_DATAGRAM_BYTES: usize = 1_472;
    /// The catalogue's WebSocket frame and message limit (513 B). pub(crate): the fields are the public surface (C5).
    pub(crate) const WS_MAX_MESSAGE_BYTES: usize = 513;
    /// The WebSocket upgrade request's cap on DelayStream's path (8,192 B).
    pub(crate) const WS_MAX_REQUEST_BYTES: usize = 8_192;

    /// The settings for `profile` with this crate's defaults: the profile's own reorder step, loopback (base 0), no
    /// shaper, no block, a 1,472 B size limit, 8,192 pool buffers (12.3 MB), per direction 2,048 buffers' worth per
    /// peer (3,072,000 B) and 4,096 in total (6,144,000 B), 512 peers, 1 ns timer slack, no fate log.
    ///
    /// The bounds' relation is the point (M10's intent: one peer cannot starve the others): the two directions' totals
    /// together are the whole pool, so the pool never runs empty; one peer holds at most half a direction's total, so a
    /// single flooder leaves every other peer room, and it takes two to fill a direction's total. M13's one peer at
    /// 10,000 datagrams a second each way at P2 holds about 600 a direction.
    pub fn new(seed: u64, profile: Profile) -> L2Config {
        L2Config {
            seed,
            profile,
            base_rtt_ns: 0,
            pass_through: false,
            shaper: None,
            udp_block: false,
            max_datagram_bytes: Self::DEFAULT_MAX_DATAGRAM_BYTES,
            pool_buffers: 8_192,
            peer_queue_bytes: 2_048 * Self::BUFFER_BYTES as u64,
            total_queue_bytes: 4_096 * Self::BUFFER_BYTES as u64,
            peer_capacity: 512,
            timer_slack_ns: 1,
            fate_log_capacity: 0,
            ws_max_message_bytes: Self::WS_MAX_MESSAGE_BYTES,
            ws_max_request_bytes: Self::WS_MAX_REQUEST_BYTES,
        }
    }

    /// The pass-through (M12): P0 with every impairment off, on the same path.
    pub fn clean(seed: u64) -> L2Config {
        let mut p = Profile::p0();
        p.reorder = ReorderStep::OFF;
        L2Config {
            pass_through: true,
            ..L2Config::new(seed, p)
        }
    }

    /// PM-0001-23's throttle: 128 kbps with a drop-tail queue of 4 full datagrams, 5,920 B (M19).
    pub fn with_throttle(mut self) -> L2Config {
        self.shaper = Some((Self::THROTTLE_BITS_PER_S, throttle_queue_bytes()));
        self
    }

    /// PM-0001-24's bottleneck at `bits_per_s`: a drop-tail queue of max(200 ms at that rate, 4 full datagrams)
    /// (M19; R-17's deep queue, intended: NC2-F3).
    pub fn with_bottleneck(mut self, bits_per_s: u64) -> L2Config {
        self.shaper = Some((bits_per_s, bottleneck_queue_bytes(bits_per_s)));
        self
    }
}

/// The injected one-way delay: (the profile's round trip − the path's measured base round trip) ÷ 2 (PM-0001-3's
/// reading, the advisor, 2026-11-06). One helper for both layers: ImpairSocket's datagrams and DelayStream's TCP model
/// (u3c-plan.md C5).
pub(crate) fn one_way_ns(cfg: &L2Config) -> u64 {
    cfg.profile.rtt_ns.saturating_sub(cfg.base_rtt_ns) / 2
}

/// -23's queue: 4 full datagrams of 1,480 B.
pub(crate) fn throttle_queue_bytes() -> u64 {
    4 * L2Config::FULL_DATAGRAM_BYTES
}

/// -24's queue: the larger of 200 ms at the bottleneck rate (bits/s ÷ 8 × 0.2) and 4 full datagrams.
pub(crate) fn bottleneck_queue_bytes(bits_per_s: u64) -> u64 {
    (bits_per_s / 40).max(4 * L2Config::FULL_DATAGRAM_BYTES)
}

/// A queue's depth as time at its rate, in ns: what the header's `*_queue_ns` records.
pub(crate) fn queue_ns(queue_bytes: u64, bits_per_s: u64) -> u64 {
    if bits_per_s == 0 {
        return u64::MAX;
    }
    (u128::from(queue_bytes) * 8 * 1_000_000_000 / u128::from(bits_per_s)) as u64
}
