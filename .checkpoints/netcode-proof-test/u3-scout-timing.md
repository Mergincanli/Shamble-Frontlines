# U3 scout: timers, TCP retransmission, the WebSocket DelayStream (rust-reviewer)

Branch claude/netcode-proof-test-plan-hcky5g, head ad527f3. Research only: no source, manifest or doc edits.
Started 2026-11-06 (fresh; no earlier notes file existed when resumed after the usage limit).

## Plan
1. Q1a: tokio 1.54.1 time driver resolution and rounding (file:line).
2. Q1b: probe crate (src in .checkpoints/netcode-proof-test/u3-timer-probe.rs, Cargo.toml + build under
   .gate/netcode-proof-test/timer-probe/, offline, workspace's pinned tokio). Measure lateness for 0.1/0.5/1/2/10/50 ms,
   >=10,000 waits each: (a) tokio sleep_until multi-thread, (b) std::thread::sleep dedicated thread, (c) sleep to
   due-100us then spin. p50/p99/p99.9/max. Also timerslack_ns and clocksource.
3. Q2: Linux TCP RTO_MIN, sysctl/route rto_min, RACK-TLP, TLP timeout, one lost segment mid-stream at 30 Hz, P2.
4. Q3: tokio-tungstenite 0.30.0 accept_* bounds; WebSocketConfig limits; per-connection delayed-bytes cap.
5. Q4: DelayStream sketch.
6. Final summary, DONE.

## Findings

### F1a. tokio 1.54.1's timer: 1 ms ticks, deadlines rounded UP, then epoll's ms timeout rounded up again
- Tick = 1 ms from the runtime's start: `/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.54.1/src/runtime/time/source.rs:22-33`
  (`instant_to_tick`: `dur.as_millis()`; `tick_to_duration`: `Duration::from_millis(t)`).
- A deadline is rounded up to the end of its millisecond: `source.rs:17-19` ("// Round up to the end of a ms",
  `self.instant_to_tick(t + Duration::from_nanos(999_999))`); applied to every Sleep at
  `/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.54.1/src/time/sleep.rs:341-342` and `:408-409`.
- The driver parks for whole ms: `/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.54.1/src/runtime/time/mod.rs:226-233` ("we effectively round up to
  1ms here"), `tick_to_duration(when.saturating_sub(now))`.
- With the I/O driver on (any runtime that does net I/O), the park is mio's epoll_wait, whose timeout is in ms and
  rounded up again: `/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/mio-1.2.4/src/sys/unix/selector/epoll.rs:46-67` ("`Duration::as_millis` truncates, so round
  up", `to.checked_add(Duration::from_nanos(999_999))...as_millis()`, then `epoll_wait(..., timeout)`). mio 1.2.4 is
  the lockfile's pin (spikes/netcode-proof/Cargo.lock).
- tokio's own docs say so: `/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.54.1/src/time/sleep.rs:14-15` ("`Sleep` operates at millisecond
  granularity and should not be used for tasks that require high-resolution timers").
- The alternative timer (`enable_alt_timer`) is behind `tokio_unstable` (`/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.54.1/src/runtime/builder.rs:444-450`):
  not usable under the spike's stable flags, and it does not change the ms tick (unconfirmed: not read in full).
- Consequence: expected lateness of sleep_until is 0 to about 2 ms (1 ms rounding of the deadline, up to 1 ms more
  from epoll's rounding of the remaining time), so it CANNOT meet 0.25 ms by construction. Measured below.

### F1b. This container's timer facts (NOT RS: a shared 4-vCPU guest, kernel 6.18.44-fc-v242, systemd-detect-virt = docker, cpuinfo flag hypervisor)
- `/proc/self/timerslack_ns` = 50000 (50 us, Linux's default); per-thread `prctl(PR_SET_TIMERSLACK, 1)` takes effect
  (PR_GET_TIMERSLACK read back 1 ns in the probe).
- clocksource = tsc (available: tsc kvm-clock); `clock_getres(CLOCK_MONOTONIC)` = 1 ns.
- cpuidle current_driver = none, governor menu; haltpoll module parameters present (guest_halt_poll_ns 200000).
- Probe: source `.checkpoints/netcode-proof-test/u3-timer-probe.rs`, manifest and build
  `.gate/netcode-proof-test/timer-probe/` (tokio =1.54.1, mio 1.2.4, libc =0.2.191 from the workspace's Cargo.lock
  copy, built --offline --release). Full run (10,000 waits per cell) started in the background; results to
  `.gate/netcode-proof-test/timer-probe/results.txt`. A 200-wait smoke run showed: tokio p50 about 1.0-1.5 ms at every
  delay; thread::sleep p50 70-170 us with p99 up to 570 us; sleep-then-spin(100 us) p50 0.1-75 us but tails above
  0.25 ms at 10 ms (the coarse sleep itself overshoots the 100 us lead in this VM), so a 1 ms-lead variant (c2) was
  added to the full run.

### F2. Linux TCP retransmission vs E15's "one RTO later, max(200 ms, 2 x RTT)" (sources downloaded to the session scratchpad from raw.githubusercontent.com/torvalds/linux at tag v6.8 = Ubuntu 24.04's GA kernel, and master; line numbers are v6.8 unless marked master)
Paths: L68 = https://raw.githubusercontent.com/torvalds/linux/v6.8/<file>, LM = .../master/<file>.
- **TCP_RTO_MIN** = `((unsigned)(HZ/5))`, i.e. 200 ms in jiffies: L68 include/net/tcp.h:145 (master :162).
  TCP_TIMEOUT_INIT 1 s :150; TCP_RTO_MAX 120 s :144; TCP_TIMEOUT_MIN_US 2 ms :148.
- **Who can change it, by version:**
  - per route, all versions in scope: `tcp_rto_min()` takes the route's locked RTAX_RTO_MIN over the socket's
    `icsk_rto_min` (L68 include/net/tcp.h:741-748: `if (dst && dst_metric_locked(dst, RTAX_RTO_MIN)) rto_min =
    dst_metric_rtt(dst, RTAX_RTO_MIN);`). iproute2 sets the lock bit itself (iproute2 main ip/iproute.c:1339-1343,
    `mxlock |= (1<<RTAX_RTO_MIN);`); ip-route(8): "rto_min TIME (Linux 2.6.23+ only) the minimum TCP Retransmission
    TimeOut to use when communicating with this destination" (iproute2 man/man8/ip-route.8.in:544-548).
  - sysctl `net.ipv4.tcp_rto_min_us`: absent at v6.8 and v6.10, present from v6.11 (checked by grepping
    net/ipv4/sysctl_net_ipv4.c at each tag for "tcp_rto_min_us": v6.10 0 hits, v6.11 1 hit). In 6.8 the socket's
    floor is hard-wired: `icsk->icsk_rto_min = TCP_RTO_MIN;` (L68 net/ipv4/tcp.c:424). Master:
    `icsk->icsk_rto_min = usecs_to_jiffies(rto_min_us)` from the sysctl (LM net/ipv4/tcp.c:438-439); doc LM
    Documentation/networking/ip-sysctl.rst:1431-1442 ("rto_min route option has the highest precedence ..., followed by
    the TCP_BPF_RTO_MIN and TCP_RTO_MIN_US socket options, followed by this tcp_rto_min_us sysctl", default 200000).
  - socket option TCP_RTO_MIN_US: from v6.15 (include/uapi/linux/tcp.h:143 at v6.15, absent at v6.14).
  - So on Ubuntu 24.04's 6.8: only the per-route `rto_min` (or a BPF sockops program) changes it. If RS runs an HWE
    kernel (6.11+), the sysctl exists too: read `uname -r` on RS (unconfirmed which kernel Hetzner's image boots).
- **The RTO is NOT max(200 ms, 2 x RTT): it is SRTT + RTTVAR with RTTVAR floored at rto_min**, so at least
  SRTT + 200 ms. L68 include/net/tcp.h:706-709 `__tcp_set_rto`: `usecs_to_jiffies((tp->srtt_us >> 3) +
  tp->rttvar_us)`; tcp_input.c:861-930 `tcp_rtt_estimator`: first sample `tp->rttvar_us = max(tp->mdev_us,
  tcp_rto_min_us(sk))`, and each RTT `tp->mdev_max_us = tcp_rto_min_us(sk)`, with rttvar decaying only toward
  mdev_max_us. At P2 (SRTT 100 ms, mean deviation about 10 ms): RTO about 300 ms; P4: about 400 ms; P1: about 250 ms.
  Backoff doubles it per expiry: L68 net/ipv4/tcp_timer.c:654-655 (`icsk->icsk_backoff++; icsk->icsk_rto =
  min(icsk->icsk_rto << 1, TCP_RTO_MAX);`), unless thin-stream linear timeouts are on (tcp_timer.c:636-645,
  `tcp_thin_linear_timeouts`, off by default; ip-sysctl.rst:1049-1056 at v6.8).
- **But one lost segment mid-stream with steady 30 Hz traffic is not repaired by the RTO. It is repaired by RACK**
  (defaults: `sysctl_tcp_sack = 1`, `sysctl_tcp_early_retrans = 3` (TLP on), `sysctl_tcp_recovery =
  TCP_RACK_LOSS_DETECTION`: L68 net/ipv4/tcp_ipv4.c:3426-3430):
  1. The receiver ACKs out-of-order data at once, with SACK: L68 net/ipv4/tcp_input.c:5691-5707 (with out-of-order
     data queued, `if (tp->dup_ack_counter < TCP_FASTRETRANS_THRESH) { tp->dup_ack_counter++; goto send_now; }`).
  2. RACK marks a segment lost when a segment sent after it is SACKed and `rack.rtt + reo_wnd` has passed since the
     lost one was sent: L68 net/ipv4/tcp_recovery.c:32-36 (`tp->rack.rtt_us + reo_wnd - tcp_stamp_us_delta(now,
     xmit)`), :76-91; otherwise it arms the reorder timer for the remainder plus 2 ms (:106-109).
  3. reo_wnd = min_rtt / 4 (x reo_wnd_steps, initially 1: L68 net/ipv4/tcp.c:451), capped at SRTT, and 0 when no
     reordering has been seen and either the connection is already in recovery or 3 or more segments are SACKed
     (tcp_recovery.c:5-30; tp->reordering = TCP_FASTRETRANS_THRESH = 3: tcp_ipv4.c:3414, include/net/tcp.h:81).
     RFC 8985 says the same: "Return min(RACK.reo_wnd_mult * RACK.min_RTT / 4, SRTT)" (RFC 8985 line 820 of
     https://www.rfc-editor.org/rfc/rfc8985.txt).
  4. TLP: "Probe timeout is 2*rtt" (L68 net/ipv4/tcp_output.c:2851-2865: `timeout_us = tp->srtt_us >> 2`, plus
     rto_min if only one packet is out, else plus 2 ms), capped by the RTO (:2866-2871), scheduled after every
     transmission (:2821-2823, "Send one loss probe per tail loss episode"). With a new segment every 33 ms the next
     snapshot's SACK triggers RACK long before the 2 x SRTT probe; TLP only matters when the stream goes quiet.
     RFC 8985: "PTO = 2 * SRTT" (line 973).
  5. The RTO (about SRTT + 200 ms, from the head segment's send time: include/net/tcp.h:2390-2397,
     tcp_input.c:3214-3225) fires only if no later segment is SACKed (the SACKs lost too, a burst, or the stream
     stops).
- **Expected stall per lost segment at P2 (RTT 100 +- 20 ms, min RTT about 80 ms, snapshots every 33 ms), sender RS
  on Linux 6.8:**
  - lost segment followed by more segments of the same snapshot (back to back): SACK about 1 RTT after the send;
    reo_wnd about 80/4 = 20 ms (0 if 3+ segments get SACKed first); retransmission at about send + RTT + 20 + 2 ms;
    stall about 120 ms (100 to 125 ms with jitter);
  - lost segment that ends a snapshot: the next snapshot's first segment goes 33 ms later, its SACK arrives 33 ms + RTT
    after the loss, by which time rack.rtt + reo_wnd has passed, so the retransmission goes at once: stall about
    RTT + 33 ms = about 133 ms;
  - so RTT + 20 to 35 ms, about 120-135 ms at P2. E15's rule gives max(200, 2 x 100) = 200 ms: **pessimistic by
    65-80 ms for the common single loss at P2** (P1: Linux about 60-85 ms vs E15 200 ms; P4: about 240 ms vs E15
    400 ms; P3: about 185 ms vs 300 ms).
  - The RTO case (SRTT + 200 ms = about 300 ms at P2) is LONGER than E15's 200 ms: E15 is **optimistic whenever the
    RTO is the repair** (the retransmission itself lost and no later SACK, or a burst).
- **B1 is the large gap:** in a 1 s total-loss burst the retransmissions are lost too, the RTO doubles (300 ms,
  600 ms, 1.2 s from the head's send), so recovery comes about 0.9-1.2 s after the burst ends (e.g. burst at 0:
  probes at about 0.2 and 0.3 s lost, retry at 0.9 s lost, next at about 2.1 s). E15 as written delivers every
  burst-lost segment 200 ms after its own loss, i.e. some INSIDE the burst, and the stream back about 0.2 s after it:
  optimistic by about 1 s. Proposed fix for the main session: model the repair, not a fixed delay: (i) a lost
  segment's repair time = max(loss time + RTT + max(reo_wnd, gap to the next segment sent) + 2 ms) if a later segment
  is delivered, else loss time + RTO (SRTT + 200 ms) with doubling per lost retransmission; (ii) the retransmission
  draws its own fate (2 % at P2, 100 % inside B1). Simpler acceptable form: RACK term RTT + max(min_rtt/4, 33 ms)
  for P1-P4 single losses, and an explicit RTO-with-backoff path for any loss whose repair time falls in a burst.
- Not checked here: Windows' and macOS's TCP (they send the upstream inputs from browsers; L2 runs only on RS, so the
  model covers both directions there); Ubuntu 24.04's CONFIG_HZ (jiffy rounding of RTO and reo timers: 1 ms at
  HZ=1000, 4 ms at HZ=250: read /boot/config-$(uname -r) on RS; unconfirmed).

### F3. tokio-tungstenite 0.30.0 / tungstenite 0.30.0: the wrapper fits; four limits must be set, one panic to avoid
(R = /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f)
- **Any stream works:** `accept_async<S>` (R/tokio-tungstenite-0.30.0/src/lib.rs:127-132), `accept_async_with_config`
  (:137-145), `accept_hdr_async` (:153-160) and `accept_hdr_async_with_config` (:164-180) all take
  `S: AsyncRead + AsyncWrite + Unpin`, tokio's traits (`use tokio::io::{AsyncRead, AsyncWrite};` :36), behind the
  `handshake` feature, which is in `default` (R/tokio-tungstenite-0.30.0/Cargo.toml:69-73). So
  `accept_hdr_async_with_config(DelayStream<..>, origin_check, Some(cfg))` compiles with no adapter.
  `Pending` from the wrapper becomes `WouldBlock` inside tungstenite (R/tokio-tungstenite-0.30.0/src/compat.rs:162,
  :178, :189) and back to `Pending` (:197-199).
- **Panic to avoid:** the handshake writer asserts a non-zero write: `assert!(size > 0);`
  (R/tungstenite-0.30.0/src/handshake/machine.rs:75-76). DelayStream's `poll_write` must return `Pending` (never
  `Ready(Ok(0))`) for a non-empty buffer when its 64 KB bound is reached. Also `WebSocketConfig::assert_valid`
  panics if `max_write_buffer_size <= write_buffer_size` (R/tungstenite-0.30.0/src/protocol/mod.rs:146-152, called
  from the constructors documented at :177, :189, :219, :384, :393): config, not network, but set both explicitly.
- **WebSocketConfig** (`#[non_exhaustive]`, so `WebSocketConfig::default().max_frame_size(..)...`:
  R/tungstenite-0.30.0/src/protocol/mod.rs:44-46, setters :108-143), defaults at :95-106:
  - `read_buffer_size` 128 KiB, **eagerly allocated per connection** (:47-57; `BytesMut::with_capacity(in_buf_len)`
    R/tungstenite-0.30.0/src/protocol/frame/mod.rs:126). 1,000 hostile sessions (E14) = 128 MB. Set it to 4-8 KiB.
  - `write_buffer_size` 128 KiB: the size tungstenite collects before writing to the stream; 0 = write each message
    at once (:58-66). For newest-only snapshots set 0 (or small) so a frame reaches DelayStream when sent, not after
    128 KiB, and flush per message.
  - `max_write_buffer_size` default `usize::MAX` (:67-77, :100): tungstenite's own outbound buffer grows only while
    writes fail (WouldBlock from DelayStream's 64 KB bound); beyond it `Error::WriteBufferFull` (:278-279,
    frame/mod.rs:254-255). Set it to about one snapshot plus margin (e.g. 16-32 KiB, must exceed write_buffer_size),
    so the per-connection outbound total is bounded: tungstenite's buffer + DelayStream's 64 KB + the kernel's
    send buffer.
  - `max_frame_size` default 16 MiB (:82-86, :102), checked at the header **before** the payload is read, then the
    whole length is **reserved at once**: `if *len > max_size as u64 { return Err(..MessageTooLong..) }` and
    `self.in_buffer.reserve(len);` (frame/mod.rs:177-191). With the default, one header claiming 16 MiB makes a
    16 MiB reservation per connection with no payload sent: a flood of 100 such headers is 1.6 GB. Set it to the
    largest client message (Join, a few hundred bytes; ADR 0001's input sizes), e.g. 4 KiB.
  - `max_message_size` default 64 MiB (:78-81, :101), checked on every fragment append with overflow care
    (R/tungstenite-0.30.0/src/protocol/message.rs:112-125; mod.rs:686, :699, :707): set equal to max_frame_size.
  - Errors from these limits are `Error::Capacity`, returned, never a panic (mod.rs:784-788).
- **The handshake** has its own fixed caps: 65,536 bytes, 512 reads, and an average read of at least 128 B after 64
  reads, else `Error::AttackAttempt` (R/tungstenite-0.30.0/src/handshake/machine.rs:163-186); 124 headers
  (R/tungstenite-0.30.0/src/handshake/headers.rs:10). With DelayStream under it, a delayed handshake still parses
  (a browser's upgrade request is under one 1,448 B segment), but the wrapper must hand tungstenite whole released
  segments, not 1-byte slices, or the 128 B-average rule can trip.
- **"The message size limit applies before any delay buffering" (plan §4.4) cannot be met by tungstenite's limits if
  DelayStream sits below tungstenite:** tungstenite sees bytes only after release. And if DelayStream sits below TLS
  (the realistic place: ciphertext segments of TCP payload, the TLS handshake delayed too), the wrapper cannot parse
  frames at all. What bounds memory before the delay is the wrapper's own per-connection cap with backpressure:
  - inbound: stop reading the TcpStream once queued-but-unreleased bytes reach the cap (64 KB, the emulated receive
    window); the client's TCP then blocks on the real receive window. Legit inputs are about 30 x 100-200 B/s,
    12 KB even across a 2 s B1 stall, so 64 KB never binds in honest play. Never drop bytes (TCP is reliable) and never
    grow the queue;
  - outbound: the 64 KB write bound returning Pending;
  - the queue is a VecDeque of segments with a fixed-capacity byte budget (segments of at most 1,448 B: at most 46
    per direction at 64 KB), allocated per segment from the bytes read, so the cap is the whole memory bound;
  - tungstenite's `max_frame_size`/`max_message_size` then apply the moment bytes leave the wrapper, before any
    allocation sized by the client (the reserve above).
  So the §4.4 wording needs a reading: "size limits apply before any allocation sized by the client; the delay
  buffer is a fixed per-connection byte budget with backpressure". For the main session / advisor.
- **TLS:** no tokio-rustls in spikes/netcode-proof/Cargo.lock at ad527f3 and none in the local registry
  (ls R | grep rustls: rustls-0.23.46, rustls-native-certs, rustls-pki-types, rustls-platform-verifier(-android),
  rustls-webpki only). `wss` termination needs tokio-rustls (or tokio-tungstenite's `__rustls-tls`, which pulls
  it: R/tokio-tungstenite-0.30.0/Cargo.toml:56-63): a new pin and a fetch at U3 or U6.
- **Correction/precision to F3 after reading the catalogue:** the WebSocket frame and message limits are already
  proposed at 513 B and the write buffer at 128 KB to hold a FullState of 65,541 B
  (/home/user/Shamble-Frontlines/docs/design/netcode-proof-messages.md:73-76), and the catalogue repeats "every size
  limit is applied there at the socket, before any L2 delay buffering" (:53-55). So use max_frame_size =
  max_message_size = 513 B (not my 4 KiB), read_buffer_size small, write_buffer_size 0 and max_write_buffer_size
  128 KB + 1 frame. Two consequences for DelayStream: (1) a 65,541 B FullState is larger than the 64 KB write bound,
  so `poll_write` must accept a PARTIAL write (`Ready(Ok(min(buf.len(), space)))` whenever space > 0; Pending only
  at space == 0), or the FullState can never pass (tungstenite retries partial writes; with rustls below it, records
  are at most about 16 KB anyway); (2) the ":53-55" wording conflicts with a wrapper below tungstenite (and below TLS):
  the limits then apply when bytes LEAVE the delay, and memory before that is bounded by the wrapper's byte budget.

### F4. DelayStream sketch (for the main session; npt-net, `crates/net/src/delay_stream.rs`; types are proposals)
Placement (from F3): `TcpStream -> DelayStream -> tokio-rustls TlsStream -> tokio-tungstenite`, so segments are TCP
payload (ciphertext) of at most 1,448 B, the TLS handshake is delayed like a real path, and the Join deadline starts
at accept (the accept loop stamps `accepted_at` before wrapping: "never from the end of a delay").

```rust
pub struct DelayStream { shared: Arc<Mutex<Shared>>, pump: JoinHandle<()> }   // the stream half tungstenite polls
struct Shared {
    up: Dir,                 // client -> room: filled by the pump from the TcpStream read half
    down: Dir,               // room -> client: filled by poll_write, drained by the pump into the write half
    read_waker: Option<Waker>, write_waker: Option<Waker>, pump_waker: Option<Waker>,
    eof_in: bool, shutdown_out: bool, err: Option<io::ErrorKind>,
}
struct Dir {
    q: VecDeque<Seg>,        // FIFO: order always holds
    budget_used: usize,      // sum of (seg.len + SEG_OVERHEAD); cap DIR_BUDGET = 64 * 1024
    last_delivery: Instant,  // E15: delivery = max(previous delivery, own time)
    rng: ChaCha8Rng,         // rand_chacha =0.10.0 (already pinned); seeded (run seed, conn index, dir)
    stats: Arc<DirStats>,    // AtomicU64s: segments, bytes, lost (= stalls), stall_ns_total, held_segments, max_hold_ns
}
struct Seg { bytes: Bytes, own_due: Instant, delivery: Instant, lost: bool }
const MSS: usize = 1_448; const SEG_OVERHEAD: usize = 64; const DIR_BUDGET: usize = 64 * 1024;
```
- **Scheduling one segment** (both directions, same function, profile read per peer set from the control plane):
  `own_due = t_in + one_way + U(-j/2, +j/2)` where `one_way = (profile_rtt - base_rtt)/2` (E15) and j the profile's
  jitter; `lost = rng < p` (or `t_in` inside a B1 burst); if lost, `own_due += repair(...)` (E15's
  max(200 ms, 2 x RTT), or F2's RACK-like rule; a retransmission falling inside a burst is lost again with the RTO
  doubled); `delivery = max(dir.last_delivery, own_due)`; `dir.last_delivery = delivery`. Draw order per segment fixed
  (jitter, then loss) so fates replay for the same segment sequence. Stats: lost -> `lost += 1`,
  `stall_ns_total += repair`; any segment with `delivery > own_due` and not lost -> `held_segments += 1`.
- **poll_write(cx, buf)** (room -> client): lock; if `err` -> Err; `space = DIR_BUDGET - down.budget_used`;
  if `space <= SEG_OVERHEAD` -> store `write_waker`, return `Pending` (never `Ready(Ok(0))`: tungstenite asserts
  `size > 0`, F3); else take `n = min(buf.len(), space - overhead_for(n))` (partial writes are required: a 65,541 B
  FullState exceeds 64 KB), cut into `MSS` pieces, schedule each with `t_in = now`, push, wake `pump_waker`,
  return `Ready(Ok(n))`.
- **poll_flush**: `Ready(Ok(()))` once bytes are queued (the queue IS the emulated send buffer; a flush that waited
  for delivery would block tungstenite's sender for the one-way delay and break newest-only at 30 Hz).
- **poll_shutdown**: set `shutdown_out`; the pump shuts the TCP write half after the last segment is released;
  return `Ready(Ok(()))`.
- **poll_read(cx, buf)** (client -> room): lock; if the head of `up` has `delivery <= now`: copy
  `min(buf.remaining(), head.len)` (keep the remainder at the head), free its budget, wake `pump_waker` if the pump
  was blocked on budget, `Ready(Ok(()))`; else if `up` is empty and `eof_in` -> `Ready(Ok(()))` with 0 bytes (EOF
  only after every delayed byte); else store `read_waker`, make sure the head's delivery time is armed (below),
  `Pending`.
- **The pump task** (one per connection, spawned by the accept path, owns `OwnedReadHalf` and `OwnedWriteHalf`):
  loop { (1) if `up.budget_used < DIR_BUDGET`: `poll_read` the TCP read half into a 16 KiB scratch, cut into MSS
  pieces with `t_in = now` (RS's clock: `Instant`, monotonic), schedule, push, wake `read_waker` if the new head is
  due; at budget, stop reading, so the client's real TCP window fills (backpressure, no growth, no drops);
  (2) write every `down` segment with `delivery <= now` to the TCP write half (handle partial writes and `Pending`
  from the real socket; budget is freed when the kernel takes the bytes), wake `write_waker` when budget frees;
  (3) sleep until `min(up.head.delivery, down.head.delivery)` or a waker }. A hard cap on the pump's life after
  close (e.g. 5 s) so a dropped connection cannot pin memory.
- **Memory bound per connection:** 2 x 64 KB budgets (SEG_OVERHEAD makes 1-byte segments count 65 B, so at most
  about 1,000 segments per direction) + the 16 KiB scratch + tungstenite's buffers (F3: read buffer set small,
  write buffer max 128 KB + 1 frame). A flood of tiny segments or a client that never reads cannot grow it.
- **Counted per run** (E15: "Stalls are counted per run", "each stream's ... stall share"): per direction `lost`
  (= stalls), `stall_ns_total`, `held_segments / segments` (stall share), `max_hold_ns`, plus the release lateness
  histogram (actual write/readable time minus `delivery`) so the run can prove the timer's precision (below).
- **Waking:** the pump is a normal tokio task; it needs wakes at `delivery` times, from either (a) tokio
  `sleep_until(delivery)`: F1a says it fires 0-2 ms late (ms ticks rounded up, then epoll's ms timeout rounded up),
  or (b) the same release thread as ImpairSocket (F1's recommendation): the pump registers `(delivery, waker)` in
  the thread's heap; the thread wakes it at the due time; the task resumes 30-250 us later (probe cell (d)).
- **Precision it needs:** looser than the datagrams', but not 2 ms. The WebSocket model's own quantities are coarse
  (one-way 25-100 ms, uniform jitter of +-5 to +-20 ms per direction, stalls of 60-400 ms, 33 ms cadence), so the
  release error only has to be small against the RTT marks measured over WebSocket: MP's 100 +- 5 ms at P2 (-3) is
  the tightest. tokio's timer adds about +1.2 ms per direction on average (smoke: p50 1.0-1.3 ms, p99 1.3-2.1 ms;
  full run below), i.e. a +2-4 ms RTT bias, half of -3's 5 ms margin. Proposal: mean bias at most 0.25 ms and p99 at
  most 1 ms per direction (4x looser than the datagrams' 0.25 ms bar at p99), checked per run from the lateness
  histogram. Meeting it: option (b) (shared release thread) meets it by construction; option (a) meets it only with a
  fixed early offset (sleep until `delivery - 1 ms`, then release what is due, re-arming for the rest), which is a
  calibration to record, not a guarantee. Prefer (b) if ImpairSocket's release thread is built anyway.
- **Arrival stamps for the room:** WsServer should stamp `Recv.arrival` with the segment's release time (when
  DelayStream made the bytes readable), not with the decode time, so -47/-50's clock samples see the emulated path.

### F1c. Probe progress notes
- 16:33 UTC: probe gained a `stream <rate> <lead_us> <slack_ns> <seconds>` mode (ImpairSocket's shape: a producer at
  Poisson rate, dues 25-75 ms ahead, one release thread on a heap+condvar, sleep to due-lead with the given timer
  slack, spin to due; lateness per release and the release thread's own CPU from RUSAGE_THREAD). Rebuilt while the
  spinners run was in its (c) 50 ms cell (one ~1 s compile on one core; the running process kept its old binary).
  To run after the main run, alone: rates 3000/s and 7000/s, leads 0 / 100 / 1000 us, slack 1 ns, 60 s each.
- Context for the 0.25 ms bar: quinn's own timers (loss detection, pacing, idle) are tokio sleeps,
  `Box::pin(sleep_until(t.into()))` (R/quinn-0.11.12/src/runtime/tokio.rs:22-23), so they carry F1a's 0-2 ms
  lateness on both the impaired and the unimpaired path; ImpairSocket's precision is about the delay it ADDS, which the
  base-RTT subtraction cannot remove.
- How a release thread wakes quinn (R/quinn-0.11.12/src/runtime.rs:42-68: `poll_recv(&self, cx, bufs, meta)`
  "Receive UDP datagrams, or register to be woken if receiving may succeed in the future"): ImpairSocket's poll_recv
  drains the inner socket into its heap (stamping arrival on RS's clock; splitting GRO batches by `meta.stride`, as
  quinn itself does at R/quinn-0.11.12/src/endpoint.rs:798-800, or reporting `max_receive_segments() == 1`,
  runtime.rs:77-80), copies out what is due, and otherwise stores `cx.waker()` with the head's due time in the
  release thread's slot and returns Pending (the inner socket's own Pending has registered the same waker for new
  arrivals). The release thread calls `waker.wake_by_ref()` at the due time; tokio schedules quinn's endpoint driver
  from outside the runtime (inject queue + unpark of a parked worker). On the send side the release thread can send
  itself at the due time through `quinn_udp::UdpSocketState::send(&self, UdpSockRef, &Transmit)`
  (R/quinn-udp-0.5.16/src/unix.rs:207), splitting GSO transmits by `segment_size` (R/quinn-udp-0.5.16/src/lib.rs:137-144)
  or reporting `max_transmit_segments() == 1` (runtime.rs:73-75), so outbound release needs no task wake at all;
  `try_send` returns Ok after queuing (a bottleneck drops, it never pushes back: R-17's queue depth overflow is a
  counted loss, not WouldBlock).

### F2b. Refinements to F2
- At v6.8 RACK "subsumes and disables RFC6675 recovery for SACK connections" (L68
  Documentation/networking/ip-sysctl.rst:629-643, default 0x1); classic three-duplicate-ACK fast retransmit survives
  only as RACK's dupthresh heuristic (reo_wnd = 0 once 3 segments are SACKed: tcp_recovery.c:16-19). For a 30 Hz
  stream of one-segment snapshots (the 256 kbps cap is about 1,067 B per 33 ms interval: catalogue row `Snapshot`,
  so most WebSocket snapshots and every input fit one 1,448 B segment), three later segments would take 3 x 33 ms, so
  RACK's one-segment path (reo_wnd about 20 ms < 33 ms gap) always wins: stall about RTT + 33 ms (133 ms at P2).
  Curiously, the old dupthresh rule would give RTT + 3 x 33 = 200 ms at P2, which is E15's number; it is not what
  6.8 does.
- min_rtt is a windowed minimum (`tcp_min_rtt_wlen`, seconds: L68 ip-sysctl.rst:570-577), so reo_wnd = min_rtt/4 uses
  the lowest RTT seen, about (RTT - jitter)/4 (P2: about 20 ms).
- **A model DelayStream can compute (lazy repair):** when segment k is drawn lost, leave its delivery open; when the
  next segment k+1 is enqueued at t_next (same direction), set
  `retx_at = max(xmit_k + rtt + reo_wnd + 2 ms, t_next + rtt)` and `delivery_k = retx_at + one_way_draw()`; if no
  next segment comes before `xmit_k + pto` (pto = 2 x srtt + 2 ms, or 2 x srtt + 200 ms with one segment in flight:
  tcp_output.c:2856-2861), use the tail-loss probe: `retx_at = xmit_k + pto`; the retransmission draws its own fate
  (2 % at P2; 100 % inside a B1 burst) and, if lost again, is repaired by the same rule if later data flows, else by
  the RTO = srtt + max(rttvar, 200 ms) doubling per expiry (tcp_timer.c:654-655). Here rtt = the profile's round trip
  (L2 knows it), srtt about the profile RTT, reo_wnd = (profile RTT - jitter)/4. Later segments keep E15's
  `delivery = max(previous delivery, own time)`. This keeps E15's shape (per-segment fates, in-order hold, a stall
  count) and replaces only the constant. If the main session keeps E15's constant instead, the run header should say
  "pessimistic by about 65-120 ms per single loss on P1-P4, optimistic by about 1 s after a B1 burst".

### F1d. Interim reading of the full run (cells done so far; this container, not RS)
- Even a pure spin (cells where the lead covers the whole delay: c 0.1 ms, c2 0.1/0.5/1 ms) shows p99 8-323 us,
  p99.9 0.3-3 ms and maxima of 2-75 ms: the vCPU itself is taken away (host preemption or guest scheduling; /proc/stat
  steal read 0.1-0.6 % over 5-10 s windows, so the guest's steal accounting under-reports it or the pauses are rare
  and long). No user-space mechanism can meet 0.25 ms at p99.9 on this machine; the container can only rank the
  mechanisms, and RS must be measured (dedicated vCPU, no co-tenant on the core: expected far quieter, unconfirmed).
- F3 addendum: after the handshake a zero-length write is not a panic but a dropped connection
  (`if len == 0 { return Err(..ConnectionReset, "Connection reset while sending"..) }`,
  R/tungstenite-0.30.0/src/protocol/frame/mod.rs:277-286). Handshake: panic (machine.rs:76); data phase: reset. Same
  rule for DelayStream either way: Pending, never Ready(Ok(0)), when the budget is full. A unit test for it: fill the
  64 KB budget, then poll_write once more and assert Pending.

### F1e. Which statistic the 0.25 ms bar is about (read from the advisor's U1 ruling)
- The advisor's Q10 ruling: "-3's ... p0.1 and p99.9 replace min and max for the jitter's ends (targets d - a and
  d + a, band 5 % of a ...). U3 measures ImpairSocket's release precision; worse than 0.25 ms (5 % of P1's half
  jitter) goes back to the advisor before R1" (/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-u1.md:89-92;
  the issue at :53-54). So the delivered one-way delay is uniform(d - a, d + a) + lateness L, and its p99.9 must sit
  within d + a +- 0.05a (0.25 ms at P1, a = 5 ms). Since the uniform part never exceeds d + a, the upper end holds if
  P(L - L0 > 0.25 ms) < 0.1 %, where L0 is a constant offset the impairer may subtract (its measured median lateness);
  the lower end (p0.1) is loose (at P1 the bottom 0.15 ms of the uniform already holds 1.5 % of packets). **The bar is
  therefore release lateness p99.9 (minus a recorded constant offset) <= 0.25 ms, measured on RS at the run's packet
  rate; tails beyond p99.9 do not move -3 (min and max are only reported).**
- Cross-check with the sibling scout's note u3-scout-wt.md F3.8 ("late by 0 to 1 ms plus the wake-up"): tokio's
  lateness is 0 to about 2 ms, not 1 (the deadline is rounded up to its ms, and the park adds whole ms that epoll
  rounds up again: F1a); the smoke run's 0.1 ms cell had min 0.93 ms, p99 2.09 ms, max 2.60 ms.
- 16:36 UTC: `ps` shows only this probe running (the sibling scout's notes describe its own timer probe, run 1 done,
  run 2 planned; if both run at once the numbers interfere: each later check records ps).

### R. Recommendation for ImpairSocket's release (draft; numbers filled when the run ends)
- **Mechanism:** one dedicated OS thread per room process ("ReleaseClock"), not tokio's timer. It owns a binary heap
  of (due, kind, index) for both directions, a Condvar for "new earlier entry", `prctl(PR_SET_TIMERSLACK, 1)` on itself
  (per-thread, verified: PR_GET_TIMERSLACK read back 1), and a configurable spin lead L (0 = pure hrtimer wait).
  Loop: wait_timeout until (head.due - L) or a new earlier head; spin on `Instant::now()` to head.due; then release
  every entry with due <= now: outbound entries are sent from this thread (`UdpSocketState::send`), inbound entries
  are moved to a ready queue and quinn's stored waker is woken (`wake_by_ref`), once per batch.
- **Pinned** to the I/O vCPUs, never the tick thread's vCPU (E2: "the tick thread alone on one vCPU").
- **Choice of L on RS by measurement, not here:** a 60 s self-test at each L2 block's start (the probe's `stream` mode
  at the run's packet rate) records p50/p99/p99.9/max lateness and the thread's CPU into the run header; L2 uses the
  smallest L whose p99.9 (minus the recorded median offset) <= 0.25 ms; if even the largest L tested misses it, that
  is the advisor's carried-forward condition (back before R1).
- **CPU:** spin cost is about (releases per second) x (L + spin overshoot), capped at one core; the stream cells
  below give the measured figure at 3,000 and 7,000 releases/s.
- The wake path from a non-runtime thread (cell (d)): `// Otherwise, use the inject queue.` then
  `self.push_remote_task(task); self.notify_parked_remote();` (R/tokio-1.54.1/src/runtime/scheduler/multi_thread/worker.rs:1372-1375);
  a worker parked on the I/O driver is woken through mio's waker (`self.waker.wake().expect("failed to wake I/O
  driver")`, R/tokio-1.54.1/src/runtime/io/driver.rs:277-280), i.e. an eventfd write and an epoll return: one extra
  thread wake per release batch, which cell (d) measures on top of (c).
- Per-connection outbound memory with DelayStream below TLS: tungstenite's write buffer (max 128 KB + 1 frame, F3)
  + rustls's `sendable_tls` (64 KiB default: `DEFAULT_BUFFER_LIMIT: usize = 64 * 1024`,
  R/rustls-0.23.46/src/common_state.rs:1136, used at :88 and conn.rs:846) + DelayStream's 64 KB + the kernel's send
  buffer. All bounded; rustls's limit is already the same 64 KiB as E15's bound.
- Safety cost of the release thread's knobs: std has no timer-slack or per-thread affinity call, and the local registry
  has no nix or rustix (only libc-0.2.191, already in Cargo.lock:690-691 transitively). So `PR_SET_TIMERSLACK` (and a
  per-thread `sched_setaffinity`, if used) is one `unsafe { libc::prctl(..) }` in npt-net, against the workspace's
  `unsafe_code = "deny"` (spikes/netcode-proof/Cargo.toml, [workspace.lints.rust]): it needs an item-level
  `#[allow(unsafe_code)]`, a SAFETY comment (integer arguments only, no pointers, return checked) and a test that reads
  PR_GET_TIMERSLACK back. Process-wide alternatives with no unsafe: systemd's `TimerSlackNSec=` and `CPUAffinity=`
  in the room's unit (systemd.exec(5); unconfirmed here, not read). Whether slack 1 ns is worth it at all: compare
  cells b and b1 below.
- Hetzner's ubuntu-24.04 image kernel: not found by search (web search 2026-11-06 found only other clouds'
  6.8.0-5x-generic); read `uname -r` on RS.
- Checked: systemd's `TimerSlackNSec=` "Sets the timer slack in nanoseconds for the executed processes"
  (https://raw.githubusercontent.com/systemd/systemd/main/man/systemd.exec.xml:1325-1330), and a new thread inherits its
  creator's current slack (measured here with python3 ctypes prctl: main 50000 -> set 1 -> a new thread reads 1). So
  `TimerSlackNSec=1` in the room's unit gives every thread (tokio workers included) 1 ns slack with no unsafe code;
  the per-thread prctl is only needed to confine it to the release thread. Since tokio's waits are whole-ms epoll
  timeouts, a 1 ns slack elsewhere costs little (more precise but not more frequent wakes; unmeasured).

### F2c. MEASURED: Linux repairs a single loss in a 30 Hz stream at the next segment, not at the RTO (this container)
- Method: `.checkpoints/netcode-proof-test/u3-tcp-stall-probe.py` (python3 -I), loopback TCP, client -> server,
  200 B messages every 33.3 ms with TCP_NODELAY, 3,000 messages (100 s); loss from one transient rule
  `iptables -A INPUT -i lo -p tcp --dport 47999 -m statistic --mode random --probability 0.02 -j DROP` (client ->
  server only; removed right after: `iptables -S` back to the three ACCEPT policies). Kernel 6.18.44 here (NOT 6.8;
  same RACK/TLP design, unconfirmed line-for-line), congestion control bbr (/proc/sys/net/ipv4/tcp_congestion_control),
  tcp_recovery 1, tcp_early_retrans 3, tcp_sack 1, tcp_rto_min_us 200000 (sysctl present: kernel >= 6.11).
  Ran 16:40 UTC while the timer probe's c2 50 ms cell ran (light load: one Python thread).
- Result: 3,000 of 3,000 delivered; delay p50 0.204 ms, p99 33.6 ms, max 241.3 ms; 52 stalls (first delayed message
  per stall): 45 in 30-40 ms (p50 33.5 ms, p90 34.9 ms), 3 at 5-15 ms, 3 at 230-241 ms; client tcp_info at the end:
  rto 204.0 ms, rtt 0.201 ms, rttvar 0.075 ms, total_retrans 54.
- Reading: 45/52 stalls = the gap to the next message (33.3 ms) + RTT (0.2 ms): the RACK path (next segment SACKed ->
  lost one retransmitted at once), as F2 predicted. The RTO path (rto 204 ms = srtt 0.2 + the 200 ms floor, rounded to
  4 ms jiffies, so this container's CONFIG_HZ is 250) shows only in the 3 stalls of 230-241 ms (54 retransmissions for
  52 stalls: the retransmission itself lost, or two losses in a row). So on a real path the single-loss stall is
  about gap + RTT (+ reo_wnd when the next segment follows within reo_wnd), i.e. 133 ms at P2, and E15's 200 ms is
  pessimistic for about 9 in 10 losses and optimistic for the rest (RTO = SRTT + 200 ms, about 300 ms at P2, doubling).
- Not measured: the RTT term (no netem/tc in this container: `which tc ip` finds neither), RS's kernel 6.8, and the
  B1 burst. On RS (root, Ubuntu 24.04) the same script plus `tc qdisc ... netem delay 50ms` on a veth pair in a
  network namespace would measure the P2 case directly; that is the "retransmission-timeout floor verified" check U3's
  row asks for, if the main session wants it on 6.8.

### S. Summary so far (16:42 UTC; timer table to follow)
1. Timers: tokio 1.54.1 cannot meet 0.25 ms (1 ms ticks, deadline rounded up, epoll ms timeout rounded up: 0-2 ms
   late by construction). A dedicated release thread is required; this container is too noisy to prove 0.25 ms at
   p99.9 for any mechanism (even pure spin shows ms-scale pauses), so RS must measure it (self-test at each L2 block).
2. TCP: TCP_RTO_MIN = HZ/5 = 200 ms; per route `rto_min` on 6.8; sysctl tcp_rto_min_us from 6.11; TCP_RTO_MIN_US
   socket option from 6.15. Linux's RTO = SRTT + max(RTTVAR, 200 ms), not max(200 ms, 2 x RTT). A single loss in a
   30 Hz stream is repaired by RACK at the next segment's SACK: stall about RTT + 33 ms (133 ms at P2), measured here
   (45 of 52 stalls at 33.5 ms on loopback). E15's 200 ms is pessimistic for single losses, optimistic for RTO repairs
   (about 300 ms at P2, doubling) and about 1 s optimistic after a B1 burst.
3. tokio-tungstenite: accept_* take any `S: AsyncRead + AsyncWrite + Unpin`; set max_frame_size = max_message_size =
   513 B (the default 16 MiB is reserved up front from the header), read_buffer_size small (128 KiB eager default),
   write_buffer_size 0, max_write_buffer_size 128 KB + 1 frame; never return Ready(Ok(0)) from poll_write.
4. DelayStream: below TLS, per-direction FIFO with a 64 KB byte budget (+64 B per segment overhead), partial writes,
   flush = queued, a pump task per connection, stall counters and a release-lateness histogram; precision needed
   about 1 ms p99 and 0.25 ms mean bias, which the shared release thread meets and tokio's timer only with a
   calibrated offset.

### F3b. The ADR's own wording on buffering (an advisor point)
- ADR 0001's R-25 text (in the file today: /home/user/Shamble-Frontlines/docs/adr/0001-multiplayer-netcode.md:1463-1469):
  "... max(200 ms, 2 x the profile's round trip; U3 verifies the floor) ... Bytes buffered per connection are capped
  and counted, and the message size limit applies before buffering (the advisor's reading, 2026-11-06)."
- Conflict: with the wrapper at the TCP level (below tungstenite, and below TLS for realism), no WebSocket message
  exists before buffering; tungstenite's 513 B frame/message limit (catalogue :73-76) can only apply when bytes leave
  the delay. "Capped and counted" holds (F4's 64 KB budget per direction, counted). Two readings for the advisor:
  (a) keep the wrapper at the TCP level and read "the message size limit applies before any allocation the client
  sizes" (the wrapper's buffer is fixed-budget, so a client cannot size it); or (b) put the wrapper above
  tungstenite, delaying whole messages, which applies the limit first but is no longer per-segment and leaves TLS and
  the upgrade undelayed. Recommendation: (a); (b) contradicts the per-segment model in the same sentence.
- Also "U3 verifies the floor": F2/F2c show the floor (200 ms) is real but that the repair for a single loss in a 30 Hz
  stream is not the RTO: the formula itself is the thing to revisit, not only the floor.
- F2 correction (when the RTO timer runs from): Linux re-arms the RTO (or TLP) timer when an ACK acknowledges new
  data (`if (flag & FLAG_ACKED) { flag |= FLAG_SET_XMIT_TIMER; ...`, L68 net/ipv4/tcp_input.c:3412-3413), and on a
  SACK only when "the sack RTT is measured from data sent after when the head was last (re)transmitted" is false
  (:3440-3447, "Otherwise the timeout may continue to extend in loss recovery"); `tcp_set_xmit_timer` tries the TLP
  first, then the RTO (:3230-3234); the head-send-time offset applies when the RTO replaces a pending reorder or probe
  timer (:3214-3225). So for a lost head the RTO runs from about its own (re)transmission time: about SRTT + 200 ms
  after it, as measured in F2c (204 ms + the 33 ms gap = the 230-241 ms stalls).
- 16:43 UTC: stream mode gained a 6th argument, a SCHED_FIFO priority for the release thread (root here: CapEff
  000001fffeffffff), to test whether real-time scheduling removes the tails without a spin. Rebuilt (1 s).

### C. Claims list (for the structured answer)
- C1 high: tokio 1.54.1's Sleep has 1 ms ticks with deadlines rounded up (source.rs:17-33), parks in whole ms
  (time/mod.rs:226-233) and epoll rounds its ms timeout up (mio-1.2.4 epoll.rs:46-67): 0-2 ms late by construction.
- C2 high: Linux TCP_RTO_MIN = HZ/5 (200 ms) at v6.8 include/net/tcp.h:145; on 6.8 only a locked per-route rto_min
  overrides it (tcp.h:741-748; tcp.c:424 hard-wires icsk_rto_min); sysctl tcp_rto_min_us from 6.11; TCP_RTO_MIN_US
  from 6.15.
- C3 high: Linux's RTO = SRTT + RTTVAR with RTTVAR >= rto_min (tcp.h:706-709; tcp_input.c:861-930): about 300 ms at
  P2, not max(200 ms, 2 x RTT) = 200 ms.
- C4 high (source) / medium (measurement on 6.18 not 6.8): a single loss in a 30 Hz stream is repaired by RACK at the
  next segment's SACK (tcp_recovery.c:5-36, 58-112; tcp_input.c:5691-5707); measured 45/52 stalls at 33.5 ms on
  loopback with 2 % loss.
- C5 medium: expected stall at P2 about RTT + 33 ms = 133 ms for one-segment snapshots; E15's 200 ms is pessimistic
  for single losses and optimistic for RTO repairs (about 300 ms, doubling) and B1 (about 1 s).
- C6 high: tokio-tungstenite accept_* take any S: AsyncRead + AsyncWrite + Unpin (lib.rs:127-180).
- C7 high: tungstenite reserves the frame length from the header before the payload arrives (frame/mod.rs:177-191),
  so max_frame_size must be the catalogue's 513 B, not the 16 MiB default.
- C8 high: tungstenite's handshake panics on a zero-byte write (handshake/machine.rs:75-76); DelayStream must return
  Pending, never Ready(Ok(0)).
- C9 high: read_buffer_size (128 KiB) is allocated eagerly per connection (frame/mod.rs:126).
- C10 medium: in this container (shared 4 vCPU, not RS) no mechanism meets 0.25 ms at p99.9; even pure spin shows
  multi-ms pauses; RS must self-measure.
- C11 high: the ADR/plan wording "message size limit applies before buffering" cannot hold with a TCP-level wrapper
  (ADR 0001:1463-1469); the byte budget is the memory bound.
- Line-number corrections (re-checked by sed on the v6.8 files): TLP in net/ipv4/tcp_output.c: function at :2829,
  "Probe timeout is 2*rtt" :2852, `timeout_us = tp->srtt_us >> 2` :2857, `+= tcp_rto_min_us(sk)` when
  packets_out == 1 :2858-2859, else `+= TCP_TIMEOUT_MIN_US` :2860-2861, RTO cap :2867-2872, timer set :2874 (F2 said
  2851-2865 and 2866-2871, F2b said 2856-2861: use these). RTT estimator in tcp_input.c: `tp->mdev_max_us =
  tcp_rto_min_us(sk)` :912, `tp->rttvar_us = max(tp->mdev_us, tcp_rto_min_us(sk))` :920, function :861-927.
- For U3's "stall count checked at P2": 60 s at 30 Hz is 1,800 one-segment writes per direction, so 2 % gives an
  expected 36 stalls, sd sqrt(1800 x 0.02 x 0.98) = 5.9, a 3-sd band of 18-54 per direction; with the seeded fates
  the count must also equal the fate draws' count exactly (both are logged). Bites: loss off (0 stalls -> FAIL), the
  hold off (later bytes overtaking a lost one -> order check FAIL), and the 64 KB bound off (a flood test's queue
  above 64 KB -> FAIL). Stall durations checked against the chosen repair rule (E15's constant, or F2b's lazy rule:
  about 133 ms at P2 for one-segment writes).

### T1. Full-run table, spinners (10,000 waits per cell; lateness in us; cpu = process cores over the cell; this container, not RS)
```
c_sleep_then_spin_100us	delay_ms=0.1	min=0.0	p50=0.1	p99=68.4	p99.9=798.4	max=2423.5	over_250us=52	cpu_cores=0.957
c_sleep_then_spin_100us	delay_ms=0.5	min=0.0	p50=0.1	p99=135.6	p99.9=1550.1	max=4371.9	over_250us=38	cpu_cores=0.069
c_sleep_then_spin_100us	delay_ms=1.0	min=0.0	p50=1.4	p99=168.8	p99.9=1242.9	max=5503.7	over_250us=63	cpu_cores=0.034
c_sleep_then_spin_100us	delay_ms=2.0	min=0.0	p50=17.2	p99=196.2	p99.9=1813.0	max=25441.3	over_250us=91	cpu_cores=0.017
c_sleep_then_spin_100us	delay_ms=10.0	min=0.0	p50=51.9	p99=364.9	p99.9=12492.6	max=74168.4	over_250us=150	cpu_cores=0.004
c_sleep_then_spin_100us	delay_ms=50.0	min=0.0	p50=60.4	p99=1888.9	p99.9=7785.3	max=44600.6	over_250us=245	cpu_cores=0.001
c2_sleep_then_spin_1ms	delay_ms=0.1	min=0.0	p50=0.1	p99=8.6	p99.9=284.2	max=2153.5	over_250us=13	cpu_cores=0.986
c2_sleep_then_spin_1ms	delay_ms=0.5	min=0.0	p50=0.1	p99=323.0	p99.9=2980.8	max=75158.8	over_250us=112	cpu_cores=0.923
c2_sleep_then_spin_1ms	delay_ms=1.0	min=0.0	p50=0.1	p99=267.1	p99.9=2304.5	max=64648.0	over_250us=104	cpu_cores=0.968
c2_sleep_then_spin_1ms	delay_ms=2.0	min=0.0	p50=0.1	p99=358.6	p99.9=3819.9	max=51060.7	over_250us=126	cpu_cores=0.438
c2_sleep_then_spin_1ms	delay_ms=10.0	min=0.0	p50=0.1	p99=426.0	p99.9=5426.2	max=59550.9	over_250us=128	cpu_cores=0.085
c2_sleep_then_spin_1ms	delay_ms=50.0	min=0.0	p50=0.1	p99=942.6	p99.9=7818.2	max=45909.3	over_250us=175	cpu_cores=0.017
d_release_thread_spin_then_notify_task	delay_ms=0.1	min=8.5	p50=32.8	p99=178.3	p99.9=1732.4	max=10042.4	over_250us=61	cpu_cores=0.835
d_release_thread_spin_then_notify_task	delay_ms=0.5	min=6.9	p50=64.2	p99=308.5	p99.9=1837.0	max=5795.6	over_250us=143	cpu_cores=0.179
```
- Reading T1: c2's pure-spin cells (lead >= delay: 0.5 and 1 ms) still have about 1.1 % of waits more than 250 us
  late and maxima of 65-75 ms: a spinning thread here loses the CPU for over 250 us roughly ten to twenty times a
  second. That is the platform, not the timer. 16:45 UTC: added a `hiccup <seconds>` mode (one thread spins on
  Instant::now() and logs every gap above 20 us: the noise floor; the same 10 s check on RS separates "the timer is
  imprecise" from "the vCPU pauses"). Runs after the main run, alone.
- Reading (d) so far: the cross-thread wake into a parked tokio worker adds about 30-120 us at p50 and 180-500 us at
  p99 on top of (c), growing with the delay (a worker idle longer wakes slower). For OUTBOUND datagrams this does not
  apply (the release thread sends itself). For INBOUND datagrams the release-to-quinn wake is an extra wake the
  unimpaired path does not have (a real arrival already costs one epoll wake, which the base RTT includes), so the
  inbound release precision is (d)'s, not (c)'s. The subtractable part is the median (the "recorded constant offset"
  of F1e: release inbound entries early by the measured median wake, e.g. 50-100 us here); the tail is not. So RS's
  self-test must measure both: release-thread lateness (outbound) and lateness seen inside the endpoint driver task
  (inbound), i.e. cells (c) and (d) in `stream` form.
- 16:46 UTC: stream mode gained a 7th argument (1 = hand each release batch to a task on a 4-worker tokio runtime via
  Notify and measure lateness inside the task: the inbound path). Planned extra runs, alone after the main run:
  hiccup 10; stream 3000 {0,100,1000} us lead, slack 1; stream 3000 0 us slack 50000 (default); stream 3000 0 1 60 50
  (FIFO); stream 7000 100 1; stream 3000 100 1 60 0 1 (inbound/notify).
- How DelayStream's Pending reaches WsServer's newest-only logic: tokio-tungstenite's `start_send` treats WouldBlock
  as "the message was accepted and queued" and sets `ready = false`; `poll_ready` then flushes and stays Pending
  until the stream takes the bytes (R/tokio-tungstenite-0.30.0/src/lib.rs:339-376, Sink impl). So WsServer
  should call `poll_ready` before each snapshot and, while it is Pending, keep only the newest snapshot waiting
  (ADR 0001's newest-only), never `send().await` in the tick path. One queued message sits in tungstenite's buffer
  on top of DelayStream's 64 KB, bounded by max_write_buffer_size.

### F2d. Against the advisor's U2 reading (advisor-u2.md, U2-R2 and carried C1)
- The advisor wrote: "The first timeout max(200 ms, 2 x RTT) is conservative against QUIC's first PTO (about 155-160 ms
  at P2) and equal to Linux's 200 ms floor at P2" and carried C1: retries back off (twice the last timeout), loss per
  1,448 B segment (/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-u2.md, U2-R2).
- Evidence says two things differ: (1) Linux's 200 ms floor is on RTTVAR, not on the RTO: RTO = SRTT + RTTVAR with
  RTTVAR >= 200 ms (F2: tcp.h:706-709, tcp_input.c:912, :920), so Linux's first RTO at P2 is about 300 ms, not
  200 ms; (2) for a single loss in a 30 Hz stream the RTO is not what repairs it: RACK does, at the next segment's
  SACK, about RTT + 33 ms = 133 ms at P2 (F2, measured on loopback in F2c: 45 of 52 stalls at 33.5 ms = gap + RTT).
  So "max(200 ms, 2 x RTT)" sits between Linux's two paths (133 ms usual, about 300 ms with backoff for a lost
  retransmission or a burst). C1's backoff is right and matches Linux (tcp_timer.c:654-655) and F2b's model; the
  constant it backs off from should be SRTT + 200 ms for the RTO path if the model is meant to be Linux's.
- For the advisor to rule: keep E15's constant (simple, documented as "between RACK and RTO"), or adopt F2b's lazy
  RACK-TLP-RTO model in DelayStream (and in L1's stream model, for C1).

### D. Design for U3 (timers, TCP model, WebSocket wrapper), as it will go in the answer
1. `ReleaseClock` (npt-net, private module `l2::clock`): one std thread per room process, named "npt-l2-release",
   pinned off the tick vCPU; state `Mutex<BinaryHeap<Reverse<(Instant, u64 seq, Entry)>>>` + `Condvar`; per-thread
   `prctl(PR_SET_TIMERSLACK, 1)` (one `#[allow(unsafe_code)]` fn with a SAFETY note and a read-back test) or
   systemd `TimerSlackNSec=1` (no unsafe); config `lead: Duration` (0, 100 us, 250 us, 1 ms). Loop:
   `wait_timeout(head.due - lead)`; spin `while Instant::now() < head.due { spin_loop() }`; pop every due entry;
   outbound -> `UdpSocketState::send(sock_ref, &transmit)` from this thread (quinn-udp unix.rs:207); inbound -> push
   to the socket's ready queue, then `waker.wake_by_ref()` once per batch (quinn runtime.rs:62-68 contract).
   Records each release's lateness (release instant - due) in a fixed-bucket histogram (no allocation per packet),
   per direction, into the per-second records.
2. `ImpairSocket` uses it: `poll_recv` drains the inner socket (split GRO by stride or max_receive_segments 1),
   stamps arrival with `Instant::now()`, draws fates per peer profile, pushes inbound entries, returns due ones, else
   stores `cx.waker()` and returns Pending; `try_send` splits GSO by segment_size (or max_transmit_segments 1),
   draws fates, pushes outbound entries, returns Ok (bottleneck overflow = counted drop).
3. RS self-test before each L2 block: the probe's `stream` (release-thread lateness, outbound) and `stream ... notify`
   (task-side lateness, inbound) at the block's packet rate for 60 s, plus `hiccup 10`; header gets p50/p99/p99.9/max
   per direction and the thread's CPU; L2 uses the smallest lead whose p99.9 - p50 <= 0.25 ms (F1e's statistic);
   failing every lead -> back to the advisor (the carried condition) before R1.
4. `DelayStream` (F4): `TcpStream -> DelayStream -> TlsStream (tokio-rustls, new pin) -> tokio-tungstenite`; per
   direction FIFO `VecDeque<Seg>` with a 64 KB budget (+64 B per segment), MSS 1,448, partial writes, flush = queued,
   Pending (never Ready(Ok(0))) at a full budget, a pump task per connection owning both TCP halves, woken by the
   ReleaseClock at segment delivery times; per-direction counters (segments, lost = stalls, stall_ns, held, max hold)
   and the release lateness histogram. Repair rule: a function `repair(profile, xmit, next_xmit, fate)` so the run
   header names the model: E15's `max(200 ms, 2 x RTT)` or F2b's lazy RACK/TLP/RTO rule (recommended), with
   retransmission fates drawn again and RTO doubling (C1).
5. WebSocketConfig: `WebSocketConfig::default().read_buffer_size(4096).write_buffer_size(0)
   .max_write_buffer_size(128 * 1024 + 1024).max_frame_size(Some(513)).max_message_size(Some(513))`, passed to
   `accept_hdr_async_with_config(stream, origin_callback, Some(cfg))` (lib.rs:164-180).
6. Tests (U3): poll_write at full budget is Pending; a 65,541 B write passes in pieces; order holds across a loss;
   stall count at P2 within 18-54 per 1,800 segments and equal to the fate draws; inbound budget stops reading
   (flood test, RSS flat); bites as in the stall-count note.
- (d) at 10 ms: p50 176 us, p99 834 us, 11 % above 250 us: the inbound wake gets slower as the worker idles longer.
  If RS shows the same shape, the inbound path cannot meet 0.25 ms at p99.9 by waking a parked worker; the fallback is
  to wake the task early (due - W) and let ImpairSocket's poll_recv spin to the due time when it is less than W away
  (a bounded spin on a tokio worker: CPU about rate x W; at 3,000/s and W = 300 us about 0.9 core: costly). Decide on
  RS's numbers, not these.
- The release thread can send without tokio: `UdpSockRef` is "constructible via `From<T: AsFd>`"
  (R/quinn-udp-0.5.16/src/lib.rs:185-197), so `state.send((&std_socket).into(), &transmit)` works from a plain
  thread on a `std::net::UdpSocket` (or a dup of the tokio socket's fd); on EAGAIN (send buffer full) the thread
  retries at its next pass and counts it.
- RFC 6298 for comparison (https://www.rfc-editor.org/rfc/rfc6298.txt): "RTO <- SRTT + max (G, K*RTTVAR)" (line 155),
  "(2.4) Whenever RTO is computed, if it is less than 1 second, then the RTO SHOULD be rounded up to 1 second"
  (157-158), "(5.5) The host MUST set RTO <- RTO * 2" (261). Linux keeps the SRTT + variance shape but replaces the
  1 s floor by a 200 ms floor on the variance term (F2), so its minimum RTO is SRTT + 200 ms.
- 6.8 vs master RACK: lines 1-112 of net/ipv4/tcp_recovery.c (reo_wnd, skb timeout, detect_loss, mark_lost) are
  identical except one comment (draft-ietf-tcpm-rack-01 -> RFC8985); master moved tcp_rack_advance and
  tcp_rack_update_reo_wnd out of this file (diff of the two downloads). So the container's 6.18 measurement (F2c)
  exercises the same RACK detection code as 6.8 (medium confidence: callers elsewhere not diffed).
- 16:49 UTC: sleepers mode gained a selector (`sleepers <n> a|b|b1`). Plan to save session time: when the
  wrapper starts its sequential `sleepers 10000` (cell (a) first), start `sleepers 10000 b` and `sleepers 10000 b1`
  as two more background processes at once, and stop the wrapper's process after its (a) 50 ms line. So (a), (b) and
  (b1) run CONCURRENTLY in three processes (each one sleeping thread; measured CPU of these cells 0.001-0.08 core):
  disclosed with the numbers. F4's "task resumes 30-250 us later" is superseded by T1's (d) rows (p50 33-176 us,
  p99 178-834 us).
- Why the wake path's latency grows with idle time on a VM, and what to read on RS: a halted vCPU must be woken by
  the host (IPI, VM entry); guest halt polling ("The cpuidle_haltpoll driver, with the haltpoll governor, allows the
  guest vcpus to poll for a specified amount of time before halting", which avoids the IPI and "The VM-exit cost";
  guest_halt_poll_ns default 200000: https://raw.githubusercontent.com/torvalds/linux/master/Documentation/virt/guest-halt-polling.rst:5-15, :33-38)
  hides it for idles under 200 us. This container reports cpuidle driver "none" (so no guest polling). On RS read
  `/sys/devices/system/cpu/cpuidle/current_driver` (haltpoll or not), `uname -r`, the clocksource, `/proc/self/timerslack_ns`,
  and CONFIG_HZ, and run `hiccup 10` and the `stream` cells: those decide the lead and whether inbound meets 0.25 ms.
- The Origin check plugs into the same call: `Callback` is implemented for any
  `FnOnce(&Request, Response) -> Result<Response, ErrorResponse>` (R/tungstenite-0.30.0/src/handshake/server.rs:173-189),
  so `accept_hdr_async_with_config(delay_tls_stream, |req, resp| origin_ok(req).then_some(resp).ok_or(forbidden()),
  Some(cfg))` (shape only), with the refusal counted by reason.
- RS's CPU, per Hetzner's FAQ (https://docs.hetzner.com/cloud/servers/faq, fetched 2026-11-06): "Plans (CCX) with
  dedicated resources have their CPU resources exclusively. Here one vCPU equals one thread of a physical CPU core."
  So RS's 4 vCPU are 4 hyper-threads held exclusively: no co-tenant steals the thread, but the sibling thread of each
  core may be busy (unconfirmed whose), and host interrupts still land. Better than this container's shared vCPU;
  not a guarantee of sub-0.25 ms tails, hence the self-test.
- Permission for the slack on RS (answers the sibling scout's open point "Whether RS's kernel lets a thread lower its
  own slack without CAP_SYS_NICE"): yes. At v6.8, `case PR_SET_TIMERSLACK:` sets `current->timer_slack_ns = arg2`
  with no capability check (https://raw.githubusercontent.com/torvalds/linux/v6.8/kernel/sys.c:2547-2552); only
  writing ANOTHER task's /proc/<pid>/timerslack_ns needs CAP_SYS_NICE (`if (p != current) { ... ns_capable(...,
  CAP_SYS_NICE)`, fs/proc/base.c:2539, :2555-2559 at v6.8). /proc/thread-self/timerslack_ns does not exist (the sibling
  saw it absent; my probe read it empty), so per-thread slack is prctl only.
- Statistic: the sibling proposes p99 <= 0.25 ms (u3-scout-wt.md, Open questions 1); F1e derives p99.9 (minus a
  recorded constant offset) from -3's p99.9 band as the advisor ruled it. Both go to the advisor; F1e's is the one the
  mark actually tests.
- Newer kernels treat real-time threads differently: master's `case PR_SET_TIMERSLACK: if (rt_or_dl_task_policy(current))
  break;` (https://raw.githubusercontent.com/torvalds/linux/master/kernel/sys.c:2649-2651): an RT thread's slack is
  managed by the kernel (zero for RT, per the upstream change; not traced here) and prctl is ignored. 6.8 has no such
  check (v6.8 kernel/sys.c:2547-2552). So on RS, for a SCHED_FIFO release thread, set the slack explicitly on 6.8.
- Release rate on RS for the CPU estimate (my arithmetic, not measured): per client about 30 input datagrams up and
  30 snapshot datagrams down per second (catalogue rows Input and Snapshot: 1 per tick at 30 Hz), plus QUIC ACK-only
  packets (DATAGRAM frames are ack-eliciting; quinn's ACK frequency not read here: say 10-20/s each way) and pings:
  roughly 80-100 packets/s per client, about 4,000-5,000 releases/s for 48 clients. Hence the stream cells at 3,000
  and 7,000/s.

### Q. Open questions (for the main session and the advisor)
1. The 0.25 ms statistic: p99.9 of release lateness minus a recorded constant offset (F1e, from -3's p99.9 band),
   or p99 (the sibling scout's proposal)? And separately for outbound (release thread) and inbound (task-side)?
2. E15/R-25's repair constant max(200 ms, 2 x RTT): keep it (documented as between Linux's RACK repair, about
   RTT + 33 ms, and its RTO, SRTT + 200 ms doubling), or adopt the lazy RACK/TLP/RTO model (F2b) in DelayStream and
   L1? B1 needs the retransmission-in-burst rule either way (with C1's backoff).
3. ADR 0001:1463-1469 / plan §4.4 / catalogue :53-55, "the message size limit applies before buffering": with a
   TCP-level wrapper below TLS, read it as "before any allocation the client sizes; the wrapper's buffer is a fixed,
   counted byte budget"? Or move the wrapper above tungstenite (loses per-segment realism)?
4. tokio-rustls is not pinned and not in the local registry: which version, fetched when (U3 or U6)?
5. RS facts to read before R1: kernel (6.8 GA or HWE), CONFIG_HZ, cpuidle driver (haltpoll?), clocksource, and the
   probe's `hiccup`/`stream` results: they decide the spin lead and whether inbound meets the bar.
6. If a spin lead is needed on RS, the release thread costs up to about rate x lead of one I/O vCPU (measured below
   for 3,000 and 7,000/s); is that acceptable against the CPU marks, recorded in the header?
- The header already has `clock_res_ns` (spikes/netcode-proof/crates/log/src/lib.rs:41; plan E18 "clock resolution per
  machine", plan.md:422). clock_getres gives 1 ns here and will on RS; it says nothing about wake precision. The RS
  self-test's lateness summary (p50/p99/p99.9/max per direction, the lead used, the release thread's CPU, the hiccup
  count) is what E18 needs beside it: new header or per-second fields for the main session (npt-log is U1's).
- docs.rs agrees (https://docs.rs/tokio-tungstenite/0.30.0/tokio_tungstenite/fn.accept_hdr_async_with_config.html):
  "pub async fn accept_hdr_async_with_config<S, C>( stream: S, callback: C, config: Option<WebSocketConfig>, ) ->
  Result<WebSocketStream<S>, Error> where S: AsyncRead + AsyncWrite + Unpin, C: Callback + Unpin".
- docs.rs agrees for tokio (https://docs.rs/tokio/1.54.1/tokio/time/fn.sleep_until.html): "`Sleep` operates at
  millisecond granularity and should not be used for tasks that require high-resolution timers."

### T1b. (d) rows complete; sleepers started 16:55:36 (a, in results.txt), 16:55:40 (b, results-b.txt) and 16:55:41 (b1, results-b1.txt): CONCURRENT, three processes
```
d_release_thread_spin_then_notify_task	delay_ms=0.1	min=8.5	p50=32.8	p99=178.3	p99.9=1732.4	max=10042.4	over_250us=61	cpu_cores=0.835
d_release_thread_spin_then_notify_task	delay_ms=0.5	min=6.9	p50=64.2	p99=308.5	p99.9=1837.0	max=5795.6	over_250us=143	cpu_cores=0.179
d_release_thread_spin_then_notify_task	delay_ms=1.0	min=3.5	p50=96.5	p99=382.4	p99.9=2593.5	max=22076.7	over_250us=252	cpu_cores=0.104
d_release_thread_spin_then_notify_task	delay_ms=2.0	min=4.4	p50=118.0	p99=499.7	p99.9=3237.8	max=10937.5	over_250us=347	cpu_cores=0.059
d_release_thread_spin_then_notify_task	delay_ms=10.0	min=11.6	p50=176.0	p99=834.3	p99.9=9669.4	max=71671.9	over_250us=1116	cpu_cores=0.015
d_release_thread_spin_then_notify_task	delay_ms=50.0	min=27.5	p50=233.6	p99=1725.7	p99.9=13690.2	max=67294.0	over_250us=3932	cpu_cores=0.004
```

### A. Answer draft (16:56 UTC; the timer numbers to be completed from T2/T3)
tokio 1.54.1's timer cannot meet the advisor's 0.25 ms bar: deadlines round up to a 1 ms tick and the driver parks in
whole ms through epoll's ms timeout, so sleep_until is 0-2 ms late by construction (measured: p50 about 1.0 ms,
p99 about 2.1 ms, 9,999 of 10,000 waits above 0.25 ms at 0.1 ms). ImpairSocket needs a dedicated release thread
(heap + condvar, timer slack 1 ns, a configurable spin lead) that sends outbound datagrams itself and wakes quinn's
endpoint driver through the waker poll_recv stored. In this shared container no mechanism reaches p99.9 <= 0.25 ms
because the vCPU itself pauses (even a pure spin), so the condition is decided by a self-test on RS before R1; the
statistic -3 actually tests is the release lateness p99.9 (minus a recorded offset). Linux's TCP_RTO_MIN is HZ/5 =
200 ms; on 6.8 only a per-route rto_min changes it (sysctl from 6.11, socket option from 6.15); the RTO is
SRTT + max(RTTVAR, 200 ms), about 300 ms at P2; a single loss in a 30 Hz stream is repaired by RACK at the next
segment's SACK, about RTT + 33 ms = 133 ms at P2 (measured on loopback: 45 of 52 stalls at 33.5 ms). So E15's 200 ms
is pessimistic for most single losses, optimistic for RTO repairs and about 1 s optimistic after B1. tokio-tungstenite
takes any AsyncRead + AsyncWrite + Unpin stream; set max_frame_size and max_message_size to 513 B (the 16 MiB default
is reserved from the header alone), read_buffer_size small, and never return Ready(Ok(0)).
- The BPF route on 6.8 exists: `TCP_BPF_RTO_MIN = 1004` in include/uapi/linux/bpf.h:6916 at v6.8 (bpf_setsockopt from
  a sockops program), so 6.8 offers two ways to change the floor: the per-route `rto_min` and a BPF sockops program.
  Neither is needed for the spike (L2 emulates TCP's repair; it does not change RS's TCP).
- Thin streams (a 30 Hz game stream qualifies: "less than 4 packets in flight"): Linux can skip exponential backoff
  for them, but "Both modifications are turned off by default" (https://raw.githubusercontent.com/torvalds/linux/master/Documentation/networking/tcp-thin.rst;
  6.8 code: tcp_timer.c:636-645, `tcp_stream_is_thin`: `tp->packets_out < 4 && !tcp_in_initial_slowstart(tp)`,
  include/net/tcp.h:2119-2122). Not for L2's model (defaults), but a real game server's WebSocket path could set
  TCP_THIN_LINEAR_TIMEOUTS to shorten B1-type recoveries: a production question for ADR 0001, not the spike.
- Reuse: npt-l1 already has `Profile` (rtt, jitter as the round trip's +-, loss_ppm, B1 bursts:
  spikes/netcode-proof/crates/l1/src/lib.rs:86-141) and the pinned ChaCha8 draws; DelayStream and ImpairSocket should
  take the same `Profile` type so L1 and L2 cannot drift (npt-net may depend on npt-l1: it is a leaf with no tokio,
  §4.2 allows it; or the type moves to npt-link). For the main session.
- L1 cross-check: npt-l1's stream model already retries a message sent inside a burst "every timeout until a retry
  falls outside it", with backoff due at U4 (C1) and "about 450 ms with backoff" after a 1 s burst quoted
  (spikes/netcode-proof/crates/l1/src/lib.rs:22-29). That 450 ms assumes a 200 ms first timeout doubling (200, 400,
  800). Linux's first RTO at P2 is SRTT + 200 ms, about 300 ms, so retries at about 0.3, 0.9 and 2.1 s after the
  burst's start: about 1.1 s after a 1 s burst ends (F2), not 450 ms. Whichever constant L2 adopts, L1 (C1) should use
  the same, or -18's 500 ms heal limit is judged against a kinder model on L1 than on L2.
- 16:58 UTC: 10 ms sleeper cells in: b p50 148.7 / p99 408.5 / p99.9 2,765 us; b1 p50 99.6 / p99 367.3 / p99.9
  3,168 us. The 50 ms cells end about 17:06-17:07; then the wrapper's sequential b/b1 get stopped and the extras run.
- RFC 8985 matches Linux's reo_wnd = 0 rule: "The reordering window SHOULD be set to zero if no reordering has been
  observed on the connection so far, and either (a) three segments have been SACKed since the last recovery or (b) the
  sender is already in fast or RTO recovery. Otherwise, the reordering window SHOULD start from a small fraction of the
  round-trip time" (https://www.rfc-editor.org/rfc/rfc8985.txt lines 350-356). Note for L2: L2's own 1 % reorder step
  is on datagrams only; on a real path with reordering, Linux would widen reo_wnd (DSACK-driven), lengthening the
  single-loss stall toward RTT + SRTT/4 x steps; not modelled (fine at P1-P4 where TCP sees no reordering).
- Extras command (to run alone after the sleepers; B = the probe binary; output .gate/netcode-proof-test/timer-probe/results-stream.txt):
  `B hiccup 10; B stream 3000 0 1 60; B stream 3000 0 50000 60; B stream 3000 100 1 60; B stream 3000 1000 1 60;
  B stream 7000 100 1 60; B stream 3000 0 1 60 50; B stream 3000 100 1 60 0 1; B stream 3000 0 1 60 0 1`.
- docs.rs agrees for WebSocketConfig (https://docs.rs/tungstenite/0.30.0/tungstenite/protocol/struct.WebSocketConfig.html):
  read_buffer_size "This buffer is eagerly allocated ... a smaller buffer, e.g. 4 KiB, would be appropriate to lower
  total memory usage. The default value is 128 KiB"; max_frame_size "The default value is 16 MiB ... small enough to
  prevent memory eating by a malicious user" (per connection; F3 shows it is reserved up front, so not per server).
- For the release thread's sends use `UdpSocketState::try_send` (R/quinn-udp-0.5.16/src/unix.rs:222-223, "without any
  additional error handling"), not `send` (:200-207: errors other than WouldBlock are "logged" and become Ok), so
  every failed release is counted; both take `&self` (UdpSocketState is Send + Sync per docs.rs quinn-udp 0.5.13's
  auto-trait list; the 0.5.16 source has the same signatures).
- Source for the inheritance measured above: `p->default_timer_slack_ns = current->timer_slack_ns;` in copy_process
  (https://raw.githubusercontent.com/torvalds/linux/v6.8/kernel/fork.c:2409); timer_slack_ns itself is copied with the
  task struct (dup_task_struct; not traced line by line). Slack "used to round up poll() and select() etc timeout
  values" (include/linux/sched.h, master, task_struct comment).
- Where slack applies to std::thread::sleep (nanosleep) at 6.8: `slack = current->timer_slack_ns; if
  (rt_task(current)) slack = 0; ... hrtimer_set_expires_range_ns(&t.timer, rqtp, slack);`
  (https://raw.githubusercontent.com/torvalds/linux/v6.8/kernel/time/hrtimer.c:2095-2100). So a SCHED_FIFO release
  thread gets zero slack automatically on 6.8 for nanosleep; for a Condvar::wait_timeout (futex) the same
  rt_task rule applies at :2303 (the futex/hrtimer sleeper path; read the context before relying on it). Default
  slack (50 us) is the expected extra lateness of a normal thread's wake: matches b vs b1's p50 gap here (about
  50 us at 0.1-2 ms delays: 67 vs 17, 87 vs 37, 103 vs 54, 117 vs 64 us).
  (Checked: :2303 is in `schedule_hrtimeout_range_clock` (:2277), "Override any slack passed by the user if under rt
  contraints", the path epoll_wait/poll timeouts use; futex waits (Condvar) take their slack elsewhere: unconfirmed.)
  Futex (Condvar::wait_timeout, the release thread's wait) at v6.8: `to = futex_setup_timer(abs_time, &timeout, flags,
  current->timer_slack_ns);` (https://raw.githubusercontent.com/torvalds/linux/v6.8/kernel/futex/waitwake.c:694-695),
  then `hrtimer_set_expires_range_ns(&timeout->timer, *time, range_ns)` (kernel/futex/core.c:149). So the thread's
  slack applies to the Condvar wait too (no RT override on this path at 6.8), and prctl(1 ns) is what tightens it.
- strace confirms both mechanisms (17:03 UTC, 3-wait runs, while the 50 ms sleeper cells ran): std::thread::sleep is
  `clock_nanosleep(CLOCK_MONOTONIC, 0, {tv_sec=0, tv_nsec=99716}, ...)` (relative, ns precision); tokio's runtime parks
  in `epoll_wait(3, [...], 1024, 1)` and other whole-ms timeouts (1, 2, 3, 6, 7, 11 seen): the integer-ms epoll
  timeout of F1a, observed directly.
- The release thread's Condvar::wait_timeout is `futex(.., FUTEX_WAIT_BITSET_PRIVATE, .., {tv_sec=12550, ..})`: an
  ABSOLUTE CLOCK_MONOTONIC deadline (tv_sec equals the container's uptime, 3 h 29 min), so the wait does not drift
  with the time spent before the syscall (strace of `stream 200 100 1 1`, 17:03 UTC).
- Probe validity for (d) and the notify stream: `notify_one` with no waiter "a permit is stored in this `Notify`
  value" (https://docs.rs/tokio/1.54.1/tokio/sync/struct.Notify.html), so a release that beats the task's await is
  not lost; the lateness then reads near 0 (no wake needed), which can only flatter (d), never inflate it.
- rustls's docs (docs.rs rustls 0.23.31 ConnectionCommon::set_buffer_limit; the pinned 0.23.46 source sets the same
  64 * 1024 default, common_state.rs:1136): "By default the limit is 64KB"; it covers pre-handshake plaintext and
  outgoing TLS records. Keep the default (never None, which "will mean that written data is buffered without bound").
- F4's pump owns both halves through `TcpStream::into_split` ("Splits a `TcpStream` into a read half and a write half,
  which can be used to read and write the stream concurrently": docs.rs tokio 1.54.1), and the accept path sets
  `set_nodelay(true)` on the real socket so a released segment leaves at once (Nagle would merge releases and add
  up to an ACK-delay; the emulated path, not the kernel, should decide segment timing).
- The trait contract agrees with F3's rule: tokio's AsyncWrite::poll_write: "A return value of `0` typically means
  that the underlying object is no longer able to accept bytes ... or that the buffer provided is empty"; "If the
  object is not ready for writing, the method returns `Poll::Pending` and arranges for the current task (via
  `cx.waker()`) to receive a notification" (https://docs.rs/tokio/1.54.1/tokio/io/trait.AsyncWrite.html). A full
  64 KB budget is "not ready", never 0.
- And AsyncRead::poll_read: "If the difference is 0, either EOF has been reached, or the output buffer had zero
  capacity" (https://docs.rs/tokio/1.54.1/tokio/io/trait.AsyncRead.html): so DelayStream returns Ready with no bytes
  only after the peer's FIN AND the last delayed byte; while segments wait for their delivery time it must return
  Pending with the waker armed, or tungstenite reads EOF early.
- The Sink contract behind the newest-only rule: poll_ready "must be called and return `Poll::Ready(Ok(()))` prior to
  each call to `start_send`" (https://docs.rs/futures-sink/0.3.31/futures_sink/trait.Sink.html; futures-sink's pinned
  version in Cargo.lock not checked). So WsServer polls readiness first and, while Pending, replaces its one waiting
  snapshot.
  (futures-sink pinned at 0.3.34 in spikes/netcode-proof/Cargo.lock; the quoted contract is unchanged across 0.3.x: unconfirmed for 0.3.34 specifically.)
  Confirmed in the pinned source: R/futures-sink-0.3.34/src/lib.rs:58 ("This method must be called and return `Poll::Ready(Ok(()))` prior to").
  Pinned-source lines for the two contracts: R/tokio-1.54.1/src/io/async_write.rs:43-48 ("A return value of `0` typically means that the underlying object is no longer able to accept"; "If the object is not ready for writing, the method returns" Pending); R/tokio-1.54.1/src/io/async_read.rs:21 and :49 (EOF).

### T2. Sleepers (10,000 waits per cell; lateness in us; a, b and b1 ran concurrently as three processes, 16:55:36-17:07; this container, not RS)
```
b_thread_sleep_slack_default	delay_ms=0.1	min=11.7	p50=67.1	p99=177.6	p99.9=451.9	max=2199.8	over_250us=28	cpu_cores=0.084
b_thread_sleep_slack_default	delay_ms=0.5	min=18.5	p50=86.8	p99=226.2	p99.9=1467.1	max=7213.3	over_250us=75	cpu_cores=0.037
b_thread_sleep_slack_default	delay_ms=1.0	min=14.6	p50=103.0	p99=283.3	p99.9=2124.0	max=32642.1	over_250us=137	cpu_cores=0.023
b_thread_sleep_slack_default	delay_ms=2.0	min=15.8	p50=117.0	p99=316.1	p99.9=1998.7	max=19789.5	over_250us=162	cpu_cores=0.013
b_thread_sleep_slack_default	delay_ms=10.0	min=41.1	p50=148.7	p99=408.5	p99.9=2765.4	max=13606.1	over_250us=282	cpu_cores=0.003
b_thread_sleep_slack_default	delay_ms=50.0	min=35.0	p50=164.7	p99=620.9	p99.9=9190.2	max=43680.3	over_250us=420	cpu_cores=0.001
b1_thread_sleep_slack_1ns	delay_ms=0.1	min=8.7	p50=16.7	p99=120.8	p99.9=685.5	max=2765.9	over_250us=18	cpu_cores=0.116
b1_thread_sleep_slack_1ns	delay_ms=0.5	min=10.2	p50=37.1	p99=198.6	p99.9=1345.7	max=2497.3	over_250us=61	cpu_cores=0.040
b1_thread_sleep_slack_1ns	delay_ms=1.0	min=11.0	p50=54.1	p99=232.9	p99.9=2107.3	max=28006.3	over_250us=85	cpu_cores=0.024
b1_thread_sleep_slack_1ns	delay_ms=2.0	min=15.0	p50=64.3	p99=287.8	p99.9=2333.2	max=10496.7	over_250us=116	cpu_cores=0.013
b1_thread_sleep_slack_1ns	delay_ms=10.0	min=17.6	p50=99.6	p99=367.3	p99.9=3168.4	max=15087.8	over_250us=155	cpu_cores=0.004
b1_thread_sleep_slack_1ns	delay_ms=50.0	min=22.3	p50=118.3	p99=553.0	p99.9=11853.2	max=76665.9	over_250us=245	cpu_cores=0.001
```
```
a_tokio_sleep_until	delay_ms=0.1	min=81.5	p50=1023.8	p99=2097.0	p99.9=2348.0	max=4580.8	over_250us=9999	cpu_cores=0.031
a_tokio_sleep_until	delay_ms=0.5	min=515.3	p50=700.1	p99=1787.2	p99.9=3355.5	max=12743.3	over_250us=10000	cpu_cores=0.024
a_tokio_sleep_until	delay_ms=1.0	min=81.5	p50=1132.0	p99=1384.9	p99.9=3700.2	max=19553.5	over_250us=9911	cpu_cores=0.020
a_tokio_sleep_until	delay_ms=2.0	min=85.3	p50=1144.5	p99=1362.5	p99.9=3560.3	max=7089.3	over_250us=9894	cpu_cores=0.015
a_tokio_sleep_until	delay_ms=10.0	min=104.3	p50=1171.8	p99=1495.2	p99.9=6110.7	max=75374.0	over_250us=9868	cpu_cores=0.005
a_tokio_sleep_until	delay_ms=50.0	min=106.5	p50=1313.8	p99=1771.9	p99.9=9672.7	max=35421.3	over_250us=9915	cpu_cores=0.002
# check on concurrency: the wrapper's own (b) 0.1 ms cell, run ALONE at 17:07:22 before it was stopped:
b_thread_sleep_slack_default	delay_ms=0.1	min=9.4	p50=60.8	p99=89.1	p99.9=244.7	max=2365.3	over_250us=10	cpu_cores=0.086
```
- The wrapper's sequential (b)/(b1) were stopped after that one cell (kill at 17:07:2x; '# end 2026-11-06T17:07:28Z').
- Concurrency caveat for T2: the same (b) 0.1 ms cell run alone gave p99 89 us / p99.9 245 us against 178 / 452 us
  when (a), (b), (b1) ran together, so the concurrent cells' tails may be inflated up to about 2x (or the minute
  differed: one sample each). Medians agree (60.8 vs 67.1 us). The ranking does not change.
- 17:07:40 UTC: extras started alone (results-stream.txt): hiccup 10; stream 3000/s with lead 0 (slack 1 and slack
  50 us), 100 us, 1 ms; 7000/s lead 100 us; 3000/s lead 0 with SCHED_FIFO 50; 3000/s inbound (notify) with lead
  100 us and lead 0. 30 s each (90,000 releases at 3,000/s).

### T3. Noise floor and the ImpairSocket-shaped release thread (alone, 17:07:46 on; this container, not RS)
- hiccup 10 (one thread spinning on Instant::now(), gaps > 20 us logged): 2,816 gaps in 10.0 s (281.6/s), 3.535 % of
  the time lost; gaps > 50 us 701, > 100 us 402, > 250 us 170 (17/s), > 1 ms 60 (6/s), > 10 ms 1; gap p50 27.8 us,
  p99 2,050 us, max 62.3 ms. So here a thread loses the CPU for more than 0.25 ms about 17 times a second whatever it
  does: the 0.25 ms bar at p99.9 is out of reach on this machine for every mechanism, which is why T1/T2 tails sit at
  ms. RS's `hiccup 10` is the first number to read: if it shows gaps > 250 us at more than about 1/s, no timer
  design can meet the bar there either (it would go back to the advisor).
- stream 3000/s, lead 0, slack 1 ns: n 89,943, p50 27.5 us, p99 313.4 us, p99.9 2,710 us, max 6,539 us, 1.16 % above 250 us, release thread CPU 0.056 core.
- stream 3000/s, lead 0, slack 50 us (default): p50 78.5 us, p99 329.5 us, p99.9 13,449 us, max 29,403 us, 1.33 % above 250 us, CPU 0.050 core. (Slack 1 ns cuts the median by about the 50 us slack; tails are the platform's.)
- stream 3000/s, lead 100 us, slack 1 ns: p50 0.1 us, p99 457.6 us, p99.9 2,321 us, max 7,977 us, 1.40 % above 250 us, CPU 0.214 core (the spin costs about 0.16 core at 3,000/s; the median drops to 0 but the platform's tail stays).
- stream 3000/s, lead 1 ms, slack 1 ns: p50 0.1 us, p99 963.3 us, p99.9 12,187 us, max 31,702 us, 1.96 % above 250 us, CPU 0.915 core: a full core of spin buys nothing here (the tail is the platform's pauses, which hit a spinning thread as often).
- stream 7000/s, lead 100 us, slack 1 ns: n 209,789, p50 0.1 us, p99 549.3 us, p99.9 11,150 us, max 27,057 us, 1.37 % above 250 us, CPU 0.479 core (spin cost scales with rate: about 0.07 core per 1,000 releases/s at a 100 us lead).
- stream 3000/s, lead 0, slack 1 ns, SCHED_FIFO 50 (sched_setscheduler = 0): p50 26.6 us, p99 200.5 us, p99.9 1,956 us, max 8,472 us, 0.83 % above 250 us, CPU 0.054 core: the best p99 for the least CPU here (p99 under 0.25 ms), but p99.9 is still the platform's ms pauses.
- stream 3000/s INBOUND (release thread lead 100 us, slack 1 ns, then Notify to a task on a 4-worker runtime; lateness inside the task): all 89,943 releases seen; p50 45.3 us, p99 802.2 us, p99.9 4,478 us, max 13,593 us, 2.24 % above 250 us; release thread CPU 0.238 core. The task-side wake roughly doubles the share above 250 us against the thread-side figure (1.40 %).
- stream 3000/s INBOUND, lead 0, slack 1 ns: p50 77.6 us, p99 520.5 us, p99.9 2,316 us, max 7,258 us, 2.98 % above
  250 us; release thread CPU 0.077 core.

### T3 summary (release thread at 3,000 releases/s unless noted; us; CPU = the release thread's own cores)
| variant | p50 | p99 | p99.9 | >250 us | CPU |
|---|---|---|---|---|---|
| lead 0, slack 50 us (default) | 78.5 | 329.5 | 13,449 | 1.33 % | 0.050 |
| lead 0, slack 1 ns | 27.5 | 313.4 | 2,710 | 1.16 % | 0.056 |
| lead 0, slack 1 ns, SCHED_FIFO 50 | 26.6 | 200.5 | 1,956 | 0.83 % | 0.054 |
| lead 100 us, slack 1 ns | 0.1 | 457.6 | 2,321 | 1.40 % | 0.214 |
| lead 1 ms, slack 1 ns | 0.1 | 963.3 | 12,187 | 1.96 % | 0.915 |
| 7,000/s, lead 100 us | 0.1 | 549.3 | 11,150 | 1.37 % | 0.479 |
| inbound (task side), lead 100 us | 45.3 | 802.2 | 4,478 | 2.24 % | 0.238 |
| inbound (task side), lead 0 | 77.6 | 520.5 | 2,316 | 2.98 % | 0.077 |
| noise floor (hiccup): gaps > 250 us | | | | 17/s | 1 core spin |

### R (final). Release mechanism recommendation
- tokio's timer is out (p50 0.7-1.3 ms, 99 %+ of waits above 0.25 ms at every delay: T2 (a)).
- The cheapest mechanism that is best here: a dedicated release thread, lead 0 (no spin), timer slack 1 ns, and, if
  RS's unit may grant it, SCHED_FIFO: 0.054 core at 3,000/s, p50 27 us, p99 200 us (the only variant with p99 under
  0.25 ms here). A spin lead lowers the median to 0 but does not move p99/p99.9 on this platform and costs
  0.21 core (100 us) to 0.92 core (1 ms) at 3,000/s: not worth it unless RS's hiccup floor is clean and the RS
  self-test shows the lead helping.
- p99.9 <= 0.25 ms (the statistic -3 tests, F1e) is NOT met by any variant here, because the platform pauses a
  thread for over 0.25 ms about 17 times a second (hiccup). It must be measured on RS (`hiccup 10`, then the
  `stream` cells for outbound and inbound) before R1; if RS misses it, the carried condition goes to the advisor.
- Inbound adds the cross-thread wake: about +20-50 us at p50 and 0.5-0.8 ms at p99 here; on RS measure it as the
  `stream ... notify` cell; release inbound entries early by the measured median (the recorded offset) if the
  advisor accepts the offset reading.

### Close (17:12 UTC)
- State left: no probe process running; the transient iptables rule removed (`iptables -S`: three ACCEPT policies);
  no source, manifest or doc edited (git status shows only this notes file modified; the probe sources
  u3-timer-probe.rs and u3-tcp-stall-probe.py sit in this folder; builds and raw results under
  .gate/netcode-proof-test/timer-probe/: results.txt (c, c2, d, a), results-b.txt, results-b1.txt, results-stream.txt).
- Final answer: sections S, R (final), T1-T3, F2-F4, D (design) and Q (open questions) above; the extra open point
  from T3: SCHED_FIFO for the release thread needs CAP_SYS_NICE or an RLIMIT_RTPRIO grant in the room's unit
  (security-auditor's call), and RT throttling (sched_rt_runtime_us 950000 here) bounds a runaway.

DONE
