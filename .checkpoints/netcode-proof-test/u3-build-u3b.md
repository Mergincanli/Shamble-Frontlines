# U3b build notes: ImpairSocket in npt-net's l2 module (build agent, worktree agent-adf1deee6caf1481f)

(Copied by the main session on 2026-11-07 from the build agent's worktree copy at
.claude/worktrees/agent-adf1deee6caf1481f/.checkpoints/netcode-proof-test/u3-build-u3b.md: the Write tool refuses the
main folder's path from an isolated worktree. Content unchanged below this line.)

Base: a0d7916 (fast-forwarded from d8c485a). Nothing committed: the main session copies these notes and the diff.
Binding rules: advisor-u3q.md M1-M5, M9-M15, M19 (mine), M7 + M8 for the loopback test servers. Mustafa's U3-Q1, Q2,
Q6, Q7, Q8, Q9. Plan §4.7: ImpairSocket and L2Config are npt-net's only new pub items.

## Plan
Layout (all under `spikes/netcode-proof/crates/net/src/l2/`, `mod l2;` in lib.rs):
- `config.rs` L2Config (pub; fields of std + npt-l1 types only): seed, profile (npt-l1 Profile: delay, jitter, loss,
  B1 bursts, ReorderStep), base_rtt_ns (injected one-way = (rtt - base) / 2, PM-0001-3's reading), pass_through
  (M12's mode), shaper Option<(rate_bps, queue_bytes)> + helpers for -23 (128 kbps, 5,920 B) and -24
  (max(200 ms at R, 5,920 B)), udp_block, max_datagram_bytes (1,472), pool_buffers, peer/total queue byte caps,
  peer_capacity, timer_slack_ns (1), fate_log_capacity.
- `fates.rs` the pure impairment core (no I/O, ns stamps): per-peer table (fixed capacity, LRU eviction of unpinned
  peers, overflow flow, counters, IPv4-mapped normalised, first raw packet stamp), per (peer ordinal, direction) ChaCha8
  stream with 16 words per datagram (L1's layout), UDP block, B1, shaper, loss, uniform jitter, L1's held-until-
  successor reorder step (token in the heap), byte bounds per peer and total, outcome per datagram.
- `shaper.rs` FIFO at rate R, drop-tail on payload + 28 B, departure before the profile's delay (M19) + queue math.
- `stats.rs` lateness histograms (1 us buckets to 16.4 ms, log above; count + sum per bucket; p50/p99/p99.9/max and
  E[(L - L0 - 0.25 ms)+] with L0 = the window's p50), counters by fate kind, fate log (fixed capacity, drained).
- `socket.rs` ImpairSocket (pub): AsyncUdpSocket over a real UDP socket (quinn-udp via quinn::udp); reader thread
  (current-thread tokio runtime, epoll) stamps at drain; release thread (one min-heap on (due, seq) across peers and
  both directions, Condvar waits, try_send never send, error kinds as fates, 1 ns slack via /proc/<tid>/timerslack_ns,
  read back); poll_recv hands over due datagrams only (lateness at its return); Box<[u8; 1500]> pool.
- `wt.rs` (cfg features) the loopback test servers: per feature its own rustls ServerConfig (builder_with_provider,
  its crate's provider, TLS 1.3, ALPN h3, AES-128-GCM included), migration off (M9), accept loop driving
  Endpoint::accept with failed handshakes counted per reason (M8). Test-only: a hand-built Ed25519 certificate (no new
  dependency: rcgen is not in wt-wtq's closure) and a pinning client verifier.
- Tests per rule; loopback tests per crate and both in one process; M13's 10,000/s per direction (#[ignore]d, run by
  me in release).
- SERIES.md rows (M15).
Cargo: Cargo.lock must not change, so no new direct dependency at all (a new direct dependency rewrites npt-net's
entry in the lock): quinn-udp through `quinn::udp`, rustls through `quinn::rustls`; wtransport's `quinn` feature only.

## Progress
- (start) fast-forwarded to a0d7916; read the build plan, M rules, scout summary, decisions, plan §4.2/§4.7, E15, E23,
  the probes, npt-l1, check.mjs, redact-check's allow-list (127/8, 192.0.2/24, ::1 ok in tests).
- Item 6 (L2Config) built: `l2/config.rs`, pub fields of std + npt-l1 types; `new`, `clean` (pass-through),
  `with_throttle` (128 kbps, 5,920 B), `with_bottleneck(R)` (max(R/40, 5,920) B). Cargo: wtransport `quinn` feature
  added; Cargo.lock unchanged (checked with git diff).
- Item 3 + M19 built: `l2/shaper.rs` (exact backlog arithmetic as time x R, so no rounding moves a drop; 5 tests: the
  ruling's depths, 4 full datagrams fit and the 5th drops, 1 ns before/after a departure, steady overload).
- Items 3/4 core built: `l2/impair.rs` (pure, generic over the buffer): bounded peer table (LRU eviction of unpinned,
  overflow flow, counters, v4-mapped normalised, first raw packet stamp), ChaCha8 per (slot ordinal, direction),
  16 words at L1's places, host drops (oversize, byte bounds, pool) before any index, block, B1, shaper, loss,
  uniform jitter, L1's held-until-successor step with a token in the one min-heap. 12 tests in `impair_tests.rs`.
- Item 5 stats built: `l2/stats.rs` (fixed histograms: 1 us to 8.192 ms then 8 per octave; p50/p99/p99.9/max and
  E[(L - p50 - 0.25 ms)+] checked against brute force), counters, bounded fate log (56 B canonical entries);
  `l2/names.rs` (the SERIES names, rule-checked); `l2/thread.rs` (tid from /proc/thread-self, own slack written and
  read back = 1 here, policy from stat, CPU from schedstat).
- Socket written (`l2/socket.rs`); 28 unit tests pass (no features, release). Next: socket tests, then `wt.rs`.
- Items 1/2 socket tests: `l2/socket_tests.rs` (9 tests + M13's #[ignore]d run): never WouldBlock / 1 datagram per
  Transmit / may_fragment = the real socket's; poll_recv Pending until due, never Ok(0), first-seen at the drain;
  outbound delayed and sent; M5 send error a fate with its errno (bite: quinn-udp's send returns Ok for the same
  error); oversize inbound before the queue; pass-through on the same path, header mode/slack/switches; windows;
  drop ends the threads. All pass.
- M13 first run (release, this 4-vCPU container, 10 s, P2 with loss, jitter and the step): 10,000 pps each way
  sustained (100,000 per direction in 10.000 s); up lateness p50 24.5 us p99 73.5 p99.9 1,061.8 max 5,189
  E[(L-L0-0.25ms)+] 2.37 us; down p50 29.5 p99 74.4 p99.9 908.3 max 5,224 E 2.17 us; release thread 0.374 core;
  loss 2.07 % / 1.98 %; no host drops; slack read back 1.
- Item 7 WT routes built: `l2/wt.rs` (per-feature rustls config with its own provider, ALPN h3, TLS 1.3, AES-128-GCM
  checked; wtransport through its builder with allow_migration(false); wtq direct with migration(false); accept loops
  over Endpoint::accept counting failures Quic/H3/Accept; sessions via a bounded channel) and `l2/wt_tests.rs`
  (hand-built Ed25519 cert through each provider's key loader, pinning verifier; per crate: session at P2,
  200 datagrams each way, RTT; pass-through RTT; wrong-ALPN handshake counted; both servers in one process).
  First run: wtransport P2 up 196/200 down 198/200, server RTT 97.3 ms, client 95.4 ms, setup 277 ms; wtq P2 up
  195 down 197, RTT 95.0 / 96.9 ms, setup 299 ms; pass-through RTT 0.2-0.5 ms; one process: both 200/200.
- Item 8 (M15) done: SERIES.md section "L2's impairment layer" (51 per-second rows, 19 header rows, the l2_summary
  event, and the L2 probe's writing binary: npt-server's probe mode on RS, proposed; the 7 probe rows' writer cell
  renamed). Two tests read SERIES.md and fail on a missing row (per-second names; header names).
- Fixes after runs: packets can be fewer than datagrams (quinn packs two queued datagrams into one packet: 147 for 197
  once), so the WT check asserts >= half; socket tests wait on the summary instead of fixed sleeps (the release thread
  records a send just after the kernel took it). fmt clean; clippy -D warnings clean with and without features;
  npt-net's 93 tests green 5 runs in a row (release, all features). Next: check.mjs, evidence runs (M13 x3).
- check.mjs run 1 started (fmt, clippy, clippy-wasm32 PASS so far). Queued after it (scratch script
  fix_after_check.py): bind's error path stops the started thread; a test reading migration: false and datagrams on
  from each feature's quinn ServerConfig (Debug output).
- check.mjs run 1: all 9 steps PASS (test: measured 184, bites 195; redact 106 items, 0 findings). Then applied the
  queued fixes; l2 tests 45 pass. Bite run: MIGRATION = true fails the config test and the header test; restored.
- Evidence, M13 (release, 4-vCPU shared container, P2 with 2 % loss, ±10 ms jitter each way, the step on; 10,000/s
  per direction, pacers on dedicated threads with 1 ns slack), 3 x 10 s + 1 x 30 s; fates identical across runs
  (same seed: delivered 97,926 up / 98,017 down every 10 s run):
  - up (inbound, at poll_recv): p50 25.5-26.5 us, p99 69.5-87.6, p99.9 0.89-5.4 ms, max 3.5-12.7 ms,
    E[(L-L0-0.25ms)+] 1.65 / 3.03 / 15.1 us (10 s), 5.53 us (30 s).
  - down (outbound, at try_send's return): p50 29.5-30.5 us, p99 73.5-81.5, p99.9 0.28-1.41 ms, max 3.1-10.6 ms,
    E 1.14 / 1.54 / 1.02 us (10 s), 3.61 us (30 s).
  - release thread CPU 0.380-0.396 core; slack read back 1; no queue, pool, shaper or send drops; loss 1.98-2.07 %.
- Evidence, loopback routes x3 (plus the first run): wtransport at P2 server RTT 93.2-99.3 ms, client 94.0-97.5 ms,
  setup 277 ms, datagrams up 191-197 / down 197-198 of 200; web-transport-quinn server 95.0-100.0 ms, client
  96.9-100.2 ms, setup 299-303 ms, up 191-195 / down 196-198; pass-through RTT 0.16-2.3 ms, 200/200; both servers
  in one process: 200/200 each way on both, no process default provider.
- check.mjs run 2 (after the last fixes): all 9 steps PASS (measured 184, bites 196 tests). clippy -D warnings clean
  on npt-net with and without features (release, all targets); fmt clean; Cargo.lock unchanged (git diff).

## Open points for the main session (not blocking U3b)
1. The stable slot id is the peer's first-seen ordinal (never its address): stable within a run; across runs it
   depends on which peer is seen first (one peer in the probe). If U6 wants room-slot keys, the API needs a slot id
   argument (ImpairSocket::set_peer could take one; not built).
2. wtransport runs the QUIC handshake and the HTTP/3 exchange in one future: only LocalH3Error is counted as H3, every
   other failure as Quic. web-transport-quinn's three stages are counted apart.
3. Inbound lateness is read once per poll_recv call, before copying the batch (at most 32 x 1.5 KB, about 1 us).
4. At 10,000/s, P2's ±10 ms jitter reorders about 91 % of datagrams (independent delay per datagram, as L1); the
   step's own overtakes are about 0.5 % there (a successor overtakes only when it drew a later delay) and 1 % at 30 Hz.
5. Not built here (U6): the L2 probe itself (proposed writer: npt-server's probe mode, SERIES.md), kernel UDP counters,
   U6-1's datagram_send_buffer_size, real certificates (the test certificate is hand-built Ed25519, test-only).
6. `l2_shaper_rate_bits` is bits per second under README's `_bits` unit: test-engineer may want another name.
7. Real-time priority was not exercised under chrt here (the slack test expects 0 under SCHED_FIFO/RR).
DONE
