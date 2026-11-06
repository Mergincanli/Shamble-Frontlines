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
