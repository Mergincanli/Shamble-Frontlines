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
- Smoke (3 s, 20,000 releases/s with real loopback sends, lead 0, slack 1): -3's P1 p999 31.12 / 31.35 ms (FAIL, band
  <= 30.25), E[(L-0.25ms)+] 24-25 us (> 10 us): consistent with my criterion (P(U+L > 30.25) ~ 24 us / 10 ms = 0.24 %).
- Battery started 17:22:00 UTC (alone; ps showed no other probe). hiccup 10 #1: 392 gaps > 250 us (39.2/s, the scout
  saw 17/s), 77 > 1 ms, lost 4.93 %, max 5.3 ms: the noise floor itself varies about 2x between runs.
  m3 1500/s per dir (3,000 releases/s, the scout's rate), lead 0, slack 1, sends: p99 lateness 356/342 us (scout
  313), p99.9 16.0/12.7 ms (scout 2.7 ms), -3 P1 p999 40.7/38.8 ms FAIL, E excess 38/34 us. So the scout's
  single-run p99.9 figures move by about 5x run to run here; rankings between variants rest on one run each.
- KEY: m3 1500/s per dir run #2 (same settings as #1): -3's P1 bands all PASS both ways (up p999 30.132 ms, down
  30.158 ms; p001 20.04; p90 29.03/29.08; mean 25.04/25.05) WITH release-lateness p99.9 = 2,111 / 2,339 us and
  1.07-1.13 % of releases > 250 us; E[(L-0.25ms)+] 9.0 / 10.6 us. So (a) -3 can pass with lateness p99.9 ~ 2 ms: the
  scout's bar (lateness p99.9 <= 0.25 ms) is not what -3 tests (F1e/C15 refuted as the criterion); (b) C3's "no
  mechanism can meet it in this container" is refuted for the mark itself: here it passes in one run and fails in
  the next (#1: p999 40.7 ms) with the same settings. The deciding quantity is the expected excess lateness
  (~10 us at P1), which a few long pauses (max 33 ms in #1) dominate.
- m3 1500/s per dir, lead 0, slack 1, SCHED_FIFO 50 (sched_setscheduler 0), sends, two runs: #1 -3 p999 30.541 /
  30.555 ms FAIL (E excess 12.6/12.0 us, lateness p99 287/320 us, p99.9 3.4/3.2 ms); #2 30.143 / 30.111 ms PASS
  (E 6.8/7.7 us, p99 232/251 us, p99.9 2.0/2.2 ms). Release thread 0.073-0.074 core with or without FIFO.
  FIFO vs normal over 2+2 runs: p99 {232, 251, 287, 320} vs {266, 280, 342, 356} us: perhaps a little better, ranges
  touch; pass/fail 1 of 2 each. C4's "FIFO gives the best p99 (200.5 vs 313.4)" is one run per variant: UNCLEAR.
- m3 10000/s per dir (-3's real rate, 20,000 releases/s, real loopback sends), lead 0, slack 1, run #1: release
  thread 0.269 core (3.6x the 3,000/s figure: C4's "0.054 core" is for a rate 7x below -3's); lateness p50 13 us,
  p99 271/274 us, p99.9 3.0/3.0 ms; -3 P1 p999 30.457 / 30.475 ms FAIL (just), E excess 11.9 / 11.6 us.
- Derived go/no-go from the hiccup data (my arithmetic): if the release thread stalls for the same pauses G_i as a
  spinning thread, packets falling due in a pause are late by 0..G, so E[(L-0.25ms)+] ~ sum_i (G_i - 0.25 ms)^2 / (2T),
  independent of the packet rate. -3 at P1 needs that <= ~10 us, i.e. sum (G_i - 0.25 ms)^2 <= 2e-5 s x T: over a
  100 s run (1,000,000 packets at 10,000/s) one 45 ms pause alone fails it, while 17/s pauses of 0.3 ms cost 0.02 us.
  So the scout's RS rule ("gaps > 250 us at more than about 1/s -> no design can meet it") is wrong both ways: the
  long pauses decide, not the count above 250 us.
- m3 10000/s per dir run #2 (lead 0, slack 1, normal sched): 0.272 core; p99 358/382 us; p99.9 4.3/4.2 ms; -3 P1
  p999 31.128 / 31.130 ms FAIL; E 16.8 / 17.3 us.
- m3 10000/s per dir, SCHED_FIFO 50, run #1: 0.271 core; p99 144/141 us; p99.9 2.2/2.1 ms; over 250 us 0.67 %;
  -3 P1 p999 30.018 / 30.028 ms PASS (E 6.8 / 6.7 us). Better than both normal-sched runs at this rate (p99 271-382).
- m3 10000/s per dir, SCHED_FIFO 50, run #2: p99 310/312 us, p99.9 14.0/13.5 ms, max 30 ms; -3 P1 p999 38.85 /
  38.96 ms FAIL (E 34.9 / 35.2 us). So FIFO does not shield the thread from the long (hypervisor-scale) pauses that
  decide -3. Tally of -3 P1 at lead 0, slack 1: normal 1 of 2 PASS at 3,000/s, 0 of 2 at 20,000/s; FIFO 1 of 2 at
  each rate. In this machine the outcome is a coin toss decided by whether a 10-30 ms pause lands in the 30 s window.
- m3 10000/s per dir, lead 100 us, slack 1, normal sched (one run): release thread 0.856 core (at -3's rate the
  thread spins almost all the time: mean gap 50 us < lead); p50 1.3 us, p99 241/255 us, p99.9 2.25/2.35 ms; -3 P1
  p999 30.162 / 30.156 ms PASS (E 9.2 / 9.4 us). So a spin lead at -3's rate costs most of one of RS's 4 vCPUs; the
  scout's "0.07 core per 1,000 releases/s" extrapolates to the cap; one run cannot show it helps -3.
- m3 10000/s per dir, lead 0, slack 1, INBOUND hand-off (up entries woken into a tokio task by Waker; lateness taken
  in the task): a 61 ms pause hit this run (both directions FAIL: p999 57.6 / 57.3 ms). Same run, task side vs
  release-thread side: p50 35.6 vs 13.4 us, p99 1,204 vs 293 us, > 250 us 2.2 % vs 1.1 %, E excess 94.7 vs 75.9 us.
  The hand-off alone adds about 19 us of E excess here, twice -3's whole P1 budget (~10 us): the inbound path, as
  stamped where quinn receives it, is the binding one (the scout measured it, C4, but its design records lateness in
  the release thread only).
- m3 1500/s per dir INBOUND (one run, a 40 ms pause hit it): task side p50 89.6 vs thread 32.3 us, p99 539 vs 210 us,
  > 250 us 3.3 % vs 0.86 %, E 42.6 vs 32.9 us (hand-off adds ~10 us of E), both FAIL. Battery ended 17:27:51 UTC.
- hiccup 10 #2 (17:27): 155 gaps > 250 us (15.5/s; #1 had 39.2/s), 20 > 1 ms, none > 10 ms, max 5.1 ms. But the m3
  runs in between met pauses of 30-61 ms (max lateness): a 10 s hiccup does not see the rare long pauses that decide
  -3, so the scout's "hiccup 10" RS go/no-go is too short. Adding the predicted-E metric and running 30 s hiccups.
- Predictor check (results-2.txt, 17:28-17:29 UTC): hiccup 30 s predicted E 12.4 us; m3 1500/s measured E 11.3 /
  9.9 us (-3 P1 p999 30.400 / 30.407 ms FAIL); hiccup 30 s again predicted 13.5 us. The hiccup-integral predicts the
  measured excess within about 20-30 %. Across all runs the -3 boundary sits at E about 9.5-10.5 us (PASS at 6.7-9.4
  and one at 10.6; FAIL at 9.9-35): fuzzy because above d + a only late packets remain, so p999 jumps fast.
  Summary of 12 runs x 2 directions: -3 P1 PASS in 4 of 12 runs (#2 normal 3k, #2 FIFO 3k, #1 FIFO 20k, lead-100 20k),
  every PASS with lateness p99.9 of 2.0-2.35 ms. Nothing here can meet "lateness p99.9 <= 0.25 ms"; the mark itself
  is met about one run in three.

## Verdicts
C1 confirmed (nuance: the second ms comes from the park computed from a truncated now; mio's round-up of a whole-ms
Duration adds nothing). C2 confirmed. C3 confirmed as stated (lateness p99.9 here 2-16 ms in all my runs; the pause rate
varies 13-39/s > 250 us). C4 unclear (transcribed right, but one run per variant; FIFO's edge not beyond noise in my
2+2 runs; CPU is at 1/7 of -3's rate). C5 confirmed. C6 confirmed. C7 confirmed. C8 confirmed (source; measurement not
re-run). C9 confirmed (derivation; "about 1 s" is vs E15's constant, about 0.7 s vs L1's planned backoff). C10
confirmed. C11 confirmed (16 MiB reserve is virtual until filled). C12 unclear (true directly under tungstenite; in
the recommended TLS stack tokio-rustls sees DelayStream's writes, unconfirmed; partial writes needed only if
poll_write_vectored is implemented, rustls offers up to 64 records of <= 16 KiB + overhead: rustls vecbuf.rs:200-207,
msgs/fragmenter.rs:4). C13 confirmed (reading conflict). C14 confirmed. C15 refuted (-3 tests p99.9 of the delivered
delay, which passed here with lateness p99.9 2.0-2.35 ms; the operative quantity is E[(L-0.25ms)+] <= about 10 us).

## Design problems
1. Acceptance statistic wrong: "smallest lead whose lateness p99.9 - p50 <= 0.25 ms" fails every variant here while -3
   itself passes about 1 run in 3; use -3's own bands (p001, p90, p999, mean) at 10,000/s per direction over the
   1,000,000-packet probe, with E[(L-0.25ms)+] and the hiccup integral as diagnostics.
2. Rate: the scout's cells run 3,000-7,000/s; -3's probe runs 10,000/s per direction (ADR 0001:1538), 20,000 releases/s
   on one ImpairSocket: lead 0 costs 0.27 core, a 100 us lead 0.86 core.
3. One 30-60 s self-test per block flips pass/fail with identical settings here; a lead picked from one run is noise.
4. Inbound lateness recorded in the release thread misses the hand-off to quinn (adds about 10-19 us of E, more than
   -3's budget); stamp the up direction where poll_recv returns the datagram; arrival is stamped at drain (no kernel
   timestamp in quinn-udp 0.5.16 RecvMeta: lib.rs:96-118).
5. `#[allow(unsafe_code)]` in npt-net contradicts the workspace rule ("nothing else may", Cargo.toml lints comment);
   libc is not a workspace pin; SCHED_FIFO per thread needs unsafe or a new crate.
6. New self-test modes and header fields are unplanned surface (plan §4.7 lists npt-net's symbols and npt-lab probe).
7. wss needs tokio-rustls: not pinned, not in the registry, E6 says wss only: U3's WebSocket leg cannot build offline.
8. Lazy repair rule: t_next must be the next delivered segment; TLP retransmits the tail; reo_wnd 0 after 3 SACKs;
   stall count vs fate draws must define retransmission draws and adjacent losses (one stall, two losses).

## Missed
- -3's rate (10,000/s per direction) and its delivered-delay criterion (E-excess); -3 can pass here.
- run-to-run variance (p99.9 moved 2.1 to 16 ms between identical runs; hiccup 13-39/s).
- hiccup 10 s is too short to see the 10-60 ms pauses that decide -3; integrate sum (G-0.25ms)^2 / 2T over >= 30 s.
- the unsafe rule's "nothing else may"; the no-unsafe leader-slack route (untested).
- a third DelayStream placement (above TLS, below tungstenite: frames visible, the size limit literally before the
  delay buffer, TLS handshake undelayed).
- §4.4: npt-server hands npt-room each client's send-buffer space: DelayStream must expose its free budget.
- the -18 consequence: a Linux-faithful stream model recovers about 1.1 s after a B1 burst, above -18's 500 ms
  limit (advisor-u2.md:72-77): the model choice decides a mark.
- ImpairSocket copies each Transmit's borrowed contents into its queue: at 20,000/s that wants a buffer pool.

DONE
