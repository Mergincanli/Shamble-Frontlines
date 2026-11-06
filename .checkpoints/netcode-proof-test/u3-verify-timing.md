# U3 verifier (skeptic) — timer, TCP and WebSocket parts

Role: refute the scout's notes `.checkpoints/netcode-proof-test/u3-scout-timing.md` (claims C1-C15 and the
recommended design). Branch claude/netcode-proof-test-plan-hcky5g. The task names head ad527f3; the branch head at my
start is 9b9c186 (autosave checkpoints on top; to check that no spike source changed since ad527f3).
Started 2026-11-06 (after a usage-limit restart: no earlier notes of mine existed).

## Plan
1. Check ad527f3..HEAD touches only .checkpoints (so the sources the scout cites are the same).
2. C1: open tokio source.rs, time/mod.rs, sleep.rs, mio epoll.rs lines.
3. C2/C3/C4: read the probe source and its results files; re-run a smaller tokio sleep and hiccup cell with my own
   probe build (offline) if time allows.
4. C5-C9: fetch the cited Linux v6.8 lines (raw.githubusercontent.com) and RFC 8985; check tcp_rto_min_us / TCP_RTO_MIN_US
   tag claims; check the RTO formula.
5. C10-C14: open tokio-tungstenite, tungstenite, rustls, tokio lines; check Cargo.lock for tokio-rustls.
6. C15: open advisor-u1.md lines.
7. Design problems and misses.
8. DONE.

## Findings (one line per finding, appended as I go)
- 0. ad527f3..9b9c186 outside .checkpoints: one commit 04e40bc (U2 close: check.mjs, l1/src/lib.rs doc comment,
  marks/README.md, SERIES.md). Only l1's crate doc moved (the "450 ms with backoff" sentence is now at
  crates/l1/src/lib.rs:27-30, inside the scout's :22-29 range shifted). No net/link/tokio source changed.
- C1 CONFIRMED (with a mechanism nuance). tokio-1.54.1/src/runtime/time/source.rs:17-19 deadline_to_tick adds 999_999 ns;
  :22-30 instant_to_tick truncates (as_millis); time/mod.rs:226-233 park duration = tick_to_duration(when - now) where
  now is the TRUNCATED tick (source.rs:36-37), so the park lasts whole ms from a truncated now: that is where the second
  0-1 ms comes from. mio-1.2.4 epoll.rs:46-60 rounds up a Duration, but a whole-ms Duration stays the same, so epoll's
  round-up adds nothing to the time driver's own parks (the scout's "rounded up again" is true code but not the source
  of the extra ms). Net bound 0-2 ms (+ kernel slack on the epoll timeout) stands. sleep.rs:14-15 quote exact; sleep.rs
  :341-342 and :408-409 call deadline_to_tick. mio 1.2.4 pinned (Cargo.lock:755-756). enable_alt_timer behind
  tokio_unstable (builder.rs:445-446).
- C2 source of numbers CONFIRMED as transcribed: results.txt / results-b*.txt rows match T2 (a_tokio p50 1023.8/700.1/
  1132.0/1144.5/1171.8/1313.8 us; over_250us 9999/10000/9911/9894/9868/9915). Probe u3-timer-probe.rs:88-105 is a
  correct sleep_until lateness loop (due taken before the sleep). Caveat the scout disclosed: a, b, b1 ran as three
  concurrent processes; the tokio result is structural (C1) so concurrency cannot rescue it.
- Probe review (u3-timer-probe.rs): stream mode :202-307 is a fair ImpairSocket-shaped release loop (heap + condvar,
  absolute futex wait, spin to due, pop all due). Lateness for the outbound cells is taken at pop (:265), BEFORE any
  send syscall: a real release adds a sendmsg per datagram (not measured). Rates tested 3,000 and 7,000/s, Poisson,
  dues 25-75 ms. Each variant ran ONCE for 30 s (results-stream.txt): single runs on a platform with 17 pauses/s
  >250 us, so p99/p99.9 rankings between variants (e.g. FIFO 200.5 vs non-FIFO 313.4 us p99) are not shown to be
  outside run-to-run noise. To re-run.
- C15 / F1e: the marks are in spikes/netcode-proof/marks/marks.json:814-857 (P1 up: p001 in [19.75, 20.25] ms,
  p999 in [29.75, 30.25] ms, p90 in [28.75, 29.25] ms, series second.s.probe_up_delay_ns) and the advisor's ruling
  advisor-u1.md:89-92 (quoted correctly by the scout). So "-3 tests p99.9" is CONFIRMED, but the scout's derived bar
  "release-lateness p99.9 (minus offset) <= 0.25 ms" is WRONG as the criterion: it is sufficient, not necessary. With
  U ~ uniform(d-a, d+a) and lateness L' = L - L0 independent, P(U + L' > d + 1.05a) = E[min(1, (L' - 0.05a)+ / 2a)],
  so the p99.9 upper check holds iff E[(L' - 0.25 ms)+] <= about 0.001 x 10 ms = 10 us at P1 (plus the 0.01 ms the
  uniform's own p99.9 sits below d + a). A lateness with 1 % of releases 1 ms late passes. The bar is an expected
  excess, not a quantile of L; the container's data may already pass it (to measure directly).
- MISSED (rate): ADR 0001 PM-0001-3 row (docs/adr/0001-multiplayer-netcode.md:1538): "On L2 the count runs on a
  dedicated probe: a free slot port with its own ImpairSocket, 10,000 packets per second per direction, one-way
  delays stamped on RS's clock ... each direction's delays logged at the impairment layer". -3 is judged at 10,000/s
  per direction (20,000 releases/s if both directions share one release thread), not the scout's 3,000-7,000/s.
- C5 CONFIRMED. v6.8 kernel/sys.c:2547-2552 (case PR_SET_TIMERSLACK, no capability check; arg2 0 resets to default);
  fs/proc/base.c:2555 `if (p != current)` then ns_capable(.., CAP_SYS_NICE) (write path); kernel/fork.c:2409
  `p->default_timer_slack_ns = current->timer_slack_ns;` (that line sets the DEFAULT; the current slack itself is
  copied with the task struct: inheritance rests on the scout's ctypes test, not this line). systemd line not re-read.
- C6 CONFIRMED. v6.8 include/net/tcp.h:145 `#define TCP_RTO_MIN ((unsigned)(HZ/5))`; :741-748 tcp_rto_min() takes a
  locked RTAX_RTO_MIN; net/ipv4/tcp.c:424 `icsk->icsk_rto_min = TCP_RTO_MIN;`; include/uapi/linux/bpf.h:6916
  TCP_BPF_RTO_MIN = 1004. sysctl_net_ipv4.c "tcp_rto_min_us": v6.10 0 hits, v6.11 2 hits (scout said 1: trivial).
  uapi tcp.h TCP_RTO_MIN_US: v6.15:143, absent at v6.14. master ip-sysctl.rst:1431-1442 quoted correctly.
- C7 CONFIRMED. v6.8 tcp.h:706-709 `usecs_to_jiffies((tp->srtt_us >> 3) + tp->rttvar_us)`; tcp_input.c:920 first
  sample rttvar = max(mdev, rto_min); :912 mdev_max reset to rto_min once per RTT and rttvar only decays toward
  mdev_max (:907-913), so rttvar >= rto_min always; note mdev_us is 4x the mean deviation (:900-901 update), so the
  formula is SRTT + max(4 x mdev, 200 ms). tcp_timer.c:654-655 backoff doubling. Linux minimum RTO = SRTT + 200 ms.
- C8 CONFIRMED on source (measurement not re-run: it needs a transient iptables rule; the scout's numbers are
  consistent with the code). v6.8 tcp_input.c:5703-5706 immediate dup-ACK for out-of-order data (first 3, then SACK
  compression); tcp_recovery.c:5-30 reo_wnd (0 in recovery or with sacked_out >= reordering, else
  min(min_rtt/4 x steps, srtt)); :32-36 timeout = rack.rtt + reo_wnd - elapsed; :106-109 reorder timer + 2 ms;
  tcp_output.c:2852-2874 TLP = srtt/4 (i.e. 2 x SRTT since srtt_us is 8x) + rto_min if packets_out == 1, capped by the
  RTO delta; tcp_ipv4.c:3414/3426/3429/3430 defaults reordering 3, sack 1, early_retrans 3, recovery RACK.
- C9 CONFIRMED as a derivation (medium). The RTO runs from the head's send (tcp.h:2390-2397 tcp_rto_delta_us; tcp_input
  .c:3214-3224 tcp_rearm_rto), so in a 1 s burst Linux retries near 0.3, 0.9, 2.1 s at P2. One wording correction: E15
  as written says nothing about bursts on the stream; "optimistic by about 1 s" is against E15's constant applied
  per segment, against L1's planned backoff (0.2, 0.6, 1.4 s) the gap is about 0.7 s.
- C10 CONFIRMED. tokio-tungstenite-0.30.0/src/lib.rs:127-180 (all four accept_* take S: AsyncRead + AsyncWrite +
  Unpin, behind cfg(feature = "handshake")); compat.rs:160-162 Ready(Ok) -> filled len, Pending -> WouldBlock; :178,
  :189 same for write/flush; :195-199 cvt maps WouldBlock back to Pending.
- C11 CONFIRMED (one nuance). tungstenite-0.30.0/src/protocol/frame/mod.rs:177-191: size check against max_size,
  then `self.in_buffer.reserve(len)` before any payload; :126 `BytesMut::with_capacity(in_buf_len)`;
  protocol/mod.rs:47-57 "This buffer is eagerly allocated", :95-106 defaults 128 KiB / 128 KiB / usize::MAX /
  64 MiB / 16 MiB. Nuance: a 16 MiB reserve is virtual until written (malloc of that size is an untouched mmap), so
  "100 headers = 1.6 GB" is address space, not RSS, until payload arrives; the fix (513 B) is right either way.
- C12 CONFIRMED for a stream directly under tungstenite: handshake/machine.rs:75-76 `assert!(size > 0);`;
  frame/mod.rs:277-286 len == 0 -> ConnectionReset. But the scout's own design puts TLS between them (TcpStream ->
  DelayStream -> TlsStream -> tungstenite), so DelayStream's Ok(0) would reach tokio-rustls, not tungstenite: what
  tokio-rustls does with it is UNCONFIRMED (not in the registry). The rule (Pending, never Ready(Ok(0))) stands.
  "Partial writes are required because FullState 65,541 B > 64 KB" holds only without TLS: under TLS, rustls hands
  DelayStream records of at most about 16 KiB + overhead, so Pending-until-space would also pass; partial writes are
  still the simpler correct choice.
- C13 CONFIRMED as a reading conflict: docs/adr/0001-multiplayer-netcode.md R-25 text and the catalogue say the size
  limit applies before buffering; a wrapper below TLS sees only ciphertext (scout's F3b). Not a source-line claim.
- C14 CONFIRMED. spikes/netcode-proof/Cargo.lock has rustls 0.23.46 (:1242-1243), tokio-tungstenite 0.30.0
  (:1670-1671), no tokio-rustls; registry has no tokio-rustls dir; rustls-0.23.46/src/common_state.rs:1136
  DEFAULT_BUFFER_LIMIT = 64 * 1024, used at :88; tokio-tungstenite Cargo.toml `__rustls-tls` pulls tokio-rustls.
  Plan E6 (plan.md:410) says "`wss` only" and the workspace pins (Cargo.toml:48-54) list no TLS stream crate: so U3's
  WebSocket leg cannot be built offline today. Plan grep finds no tokio-rustls anywhere.
- DESIGN PROBLEM (unsafe): spikes/netcode-proof/Cargo.toml [workspace.lints.rust] says `unsafe_code = "deny"` with the
  comment "deny, not forbid: npt-client-wasm relaxes it for wasm-bindgen's generated code, and nothing else may." The
  scout's `#[allow(unsafe_code)]` prctl fn in npt-net breaks that written rule; libc is also only transitive
  (Cargo.lock:690-691), not a workspace pin. Alternatives the scout did not list: /proc/<pid>/timerslack_ns is
  per-process only (v6.8 fs/proc/base.c:3345 sits in tgid_base_stuff, :3248; tid_base_stuff starts :3601), and a write
  by the leader thread itself needs no capability (:2555), so with std::fs only: leader writes 1, spawns the release
  thread (inherits), writes the old value back, then builds tokio (UNTESTED here). SCHED_FIFO per thread has no
  no-unsafe route (systemd CPUSchedulingPolicy= is process-wide, which would make tokio workers RT too).
- My probe: source .checkpoints/netcode-proof-test/u3-verify-timing-probe.rs, build .gate/netcode-proof-test/
  u3-verify-timing-probe/ (tokio =1.54.1, libc =0.2.191, offline, the scout's lockfile copy). Mode m3 judges -3's
  P1 bands directly (delivered delay = release - arrival, delay 25 ms + U(-5,+5) ms, 10,000/s per direction, both
  directions on one release thread = 20,000 releases/s; optional real loopback UDP send per release; optional inbound
  hand-off to a tokio task by Waker, like poll_recv). Also a hiccup mode. Results to
  .gate/netcode-proof-test/u3-verify-timing-probe/results.txt.
