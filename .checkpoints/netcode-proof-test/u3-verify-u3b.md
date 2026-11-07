# U3b skeptic (rust-reviewer): ImpairSocket and L2Config at 5a5aba9

Scope: spikes/netcode-proof/crates/net/src/l2.rs and l2/*.rs; build notes u3-build-u3b.md; rules M1-M5, M9-M15, M19
(advisor-u3q.md 230-290) and u3-build-plan.md's U3b bullet (line 29-39). Default verdict "unclear"; refute from
primary sources (quinn 0.11.12 / quinn-udp source in ~/.cargo/registry, crates/l1).

## Plan
1. Read l2.rs, config.rs, impair.rs, shaper.rs, socket.rs, thread.rs, stats.rs, names.rs, wt.rs; then tests.
2. quinn 0.11.12 AsyncUdpSocket trait + its use in endpoint/runtime (poll_recv meta contract, poll_writable,
   try_send semantics, max_transmit_segments/max_receive_segments, may_fragment). Check ImpairSocket.
3. Release thread: heap, try_send error mapping, slack via /proc, Drop, races.
4. Fates vs crates/l1 (ChaCha8 per stream, 16 words, ReorderStep), shaper arithmetic, B1, UDP block.
5. M10 bounds; M1 lateness points and the E[...] computation.
6. Run the tests (and the ignored M13 run), plant defects in a throwaway worktree under .gate/.
7. wt.rs: TLS (M7), accept loops (M8), migration (M9).
8. SERIES.md rows vs names.rs.
9. Build notes' open points.

## Findings (one line each, as found)
- (1) quinn contract CONFIRMED: runtime.rs:42-89 (trait); endpoint.rs:794-850 loops poll_recv until Pending (Ok(0)
  would spin: msgs=0 then loop), so socket.rs:470-477 (Pending when n==0, waker stored) is right; connection.rs:1020-1061
  max_datagrams = max_transmit_segments().min(..) = 1, WouldBlock -> buffered + retry; ImpairSocket never WouldBlocks
  (socket.rs:430 Ok always; AlwaysWritable socket.rs:391-394). Multi-segment split socket.rs:407-413 chunks(seg) as
  quinn-udp's own apple path does (unix.rs:390-397). quinn recv_buf chunk = min(max_udp_payload,64K)*1*32/32 = 65,527 B
  (endpoint.rs:754-758,785) so the Oversize branch at socket.rs:452 never fires in practice.
- (1) may_fragment = UdpSocketState's (quinn-udp unix.rs:125-150, DF via IP_MTU_DISCOVER); allow_mtud = !may_fragment
  (quinn endpoint.rs:140). Nonblocking set by UdpSocketState::new (unix.rs:109), so tokio::UdpSocket::from_std and the
  release thread's try_send never block.
- (2) try_send = raw send() (quinn-udp unix.rs:223-225); send() swallows all but WouldBlock (unix.rs:207-219):
  M5 CONFIRMED. send_fate socket.rs:571-579 maps WouldBlock->Kind::WouldBlock, else SendError with errno.
- (3) Reorder step vs L1 (l1/src/lib.rs:617-645): same condition (now < held.due && succ_due + 1 > held.due), same
  release-on-successor, hold drawn only for delivered, word places 0/1/2 (lib.rs:61-64 vs impair.rs:34-37), same
  below() and delay() formula (lib.rs:418-440 vs impair.rs:107-111). L1's poll-time release (lib.rs:716-723) ==
  L2's token (impair.rs:559-584). CONFIRMED equivalent.
- (3) Shaper arithmetic CONFIRMED (units bit-ns/s consistent; shaper.rs:55-63); -23 5,920 (config.rs:117-119), -24
  max(R/40, 5,920) (config.rs:122-124); before the delay (impair.rs:479-489). Doc nit: shaper.rs:9-10 says "takes it
  1 ns after the first has left, not before"; the test (shaper.rs:119-120) shows it is taken AT the departure ns.
- (6) Tests: 45 pass, 1 ignored (release, all features, 4.87 s). M13 ignored run reproduced: 100,000/dir in 10.000 s,
  delivered up 97,926 / down 98,017 (identical to the build notes: fates deterministic for one peer); up lateness p50
  28.5 us p99 153.4 p99.9 4,374 max 9,136 E 13.5 us; down p50 32.5 p99 105.6 p99.9 2,956 max 8,928 E 7.8 us; release
  CPU 0.393 core; overtaken 507/485, reordered 910/909 permille; slack 1. (Container shared with my own work.)
- (6) Planted defects on a copy (.gate/u3bv-copy, script .checkpoints/netcode-proof-test/u3bv-plant.py; restored and
  diffed equal after):
  - SURVIVED B: both directions on one stream (impair.rs:221 without `| dir`): 45/45 pass. M4's "per direction" has
    no bite (the interleave test compares A-down with itself; separate Rng objects hide the shared key).
  - SURVIVED A: the overtake's `t < h.due_ns` guard removed (impair.rs:498): 45/45 pass. L1 parity of the guard
    (lib.rs:621) untested: every test drains the token before a late successor arrives.
  - SURVIVED D: inbound lateness forced to 0 (socket.rs:466 finish at e.due_ns): 45/45 pass. M1's measurement point
    ("at poll_recv's return") has no test: poll_recv test asserts only count 1 and max < 1 s (socket_tests.rs:176-177).
  - SURVIVED G: shaper offered at arrival (after the delay): 45/45 pass. the_shaper_sits_before_the_delay
    (impair_tests.rs:353-372) uses a fixed delay, so both placements give the same dues; it cannot see placement.
  - SURVIVED H: evict the MOST recently seen unpinned peer (.max()): 45/45 pass. the_peer_table_is_bounded pins
    addr(2) via hold_next (impair_tests.rs:441 -> set_peer pins), leaving one candidate: LRU never tested.
  - SURVIVED K: Drop no longer notifies the reader (socket.rs:383 removed): 45/45 pass. dropping_the_socket_ends_its_
    threads (socket_tests.rs:368-384) checks only the release thread's tid; the reader (and the bound port) can leak.
  - BIT C: send instead of try_send -> send_errors_are_fates_not_deliveries fails (socket_tests.rs:214). M5 bites.
  - BIT I: FIFO by seq -> 3 tests fail (impair_tests.rs:114, 253, 310). M2 bites.
  - BIT J: no per-peer bound -> host_drops test fails (impair_tests.rs:393). M10 per-peer bound bites.
- (6) Clippy -D warnings clean per feature alone (wt-wtq; wt-wtransport; release, all targets).
- (2)/(9.7) Under `chrt -f 10` (this kernel 6.18) a_thread_sets_its_own_slack_to_1_ns FAILS at thread.rs:111 (the
  "default slack > 1" assertion: under RT the default reads 0). the_pass_through... passes. So the RT switch breaks a test.
- (2) Slack header honesty: python probe on this kernel: thread writes 1, reads 1; SCHED_FIFO applied -> reads 0; back
  to SCHED_OTHER -> reads 50,000 (the default, not 1). ImpairSocket reads slack once at thread start (socket.rs:584-588)
  and reports that stored value (socket.rs:262), while policy is read at header time (socket.rs:297). With the
  documented `chrt -p` from outside (thread.rs:8-10), the header pairs SCHED_FIFO with slack 1, and after a round trip
  reports 1 while the kernel runs 50 us. Kernel source of the RT behaviour: ed4fb6d7ef68 (mainline 6.12; 6.6.84
  stable backport); whether Ubuntu 24.04's 6.8 has it: unknown.
- (4) Pool vs byte bounds (config.rs:76-86): pool 8,192 buffers shared by every peer and both directions; total byte
  bound 12 MiB per direction = 12,582,912 B > the pool's whole payload capacity 8,192 x 1,472 = 12,058,624 B, so the
  total bound can never trigger at defaults (dead); per-peer 4 MiB binds before the pool only for datagrams > 512 B.
  One source of small datagrams at about 8,192 / 0.05 s = 164k pps (P2) empties the pool: PoolDrop for every peer and
  for the server's outbound (counted, so visible). M10's letter met, its intent (one peer cannot starve others) not.
- (4) slot_for (impair.rs:417) runs before the oversize/bounds checks (impair.rs:441-449): an oversize or bound-dropped
  datagram from a new address still creates a peer and can evict an unpinned one. Bounded; noted.
- (4) Eviction resets a returning peer's ordinal (new stream), shaper, `highest` and first_seen (impair.rs:340-351);
  unpinned real clients are evicted when new addresses arrive faster than 512 per client gap (about 30k new pps at
  60 Hz). The control plane should pin joined clients with set_peer.
- (7) M7 CONFIRMED: wt.rs:57-79 builder_with_provider + TLS13 + ALPN h3 + AES-128-GCM check; wtransport default ring
  (wtransport Cargo.toml [features] default = self-signed, ring), web-transport-quinn default aws-lc-rs; transport
  TransportConfig::default() in both crates' own builders (wtransport config.rs:396,438; wtq client.rs:46-55), as
  wt.rs uses. M9 CONFIRMED: wtransport build() applies migration (config.rs:556-566) and allow_migration (611-614);
  wtq sc.migration (wt.rs:185). Header l2_migration reports the constant (socket.rs:267), not the config in use.
- (7) M8 accept loop: per-Incoming task (wt.rs:140-155, 195-214) with no cap on concurrent handshakes after QUIC
  completes and no deadline on the H3 stage (wtransport endpoint.rs:621-634 awaits SETTINGS and CONNECT; quinn's idle
  timeout 30 s, transport.rs:376, defeated by client keep-alives). quinn bounds only Incoming (max_incoming 65,536,
  config/mod.rs:251). Fine for loopback tests; must be bounded before U6's WtServer reuses it (.claude/rules/netcode.md
  "No unbounded ... keyed by client data"). `tx.send(conn).await` blocks the task when 64 sessions wait (bounded queue,
  unbounded waiters).
- (9.2) wtransport failure reasons: ConnectionError variants (error.rs:11-41); with_driver_error maps
  DriverError::ApplicationClosed to ApplicationClosed (error.rs:48-51), which wt.rs:128-131 counts as Quic though it
  arises in the H3 stage (no 1-RTT app close before the handshake ends). Suggest ApplicationClosed -> H3.
- (8) SERIES: 51 `second.v.l2_` rows (grep), names test-matched both ways for written names. Text errors: (a) "Each
  name below is also a field of the l2_summary event" (SERIES.md:389-390) but run_summary (socket.rs:333-353) writes
  l2_release_cpu_ns, not l2_release_cpu_permille; (b) `l2_down_queued_bytes`'s meaning says "or waiting for quinn",
  which is up only. M15's TCP-model rows (model name/version, S12 stall and repair counts) are absent from SERIES.md
  (grep "tcp_model|repair": none): carried to U3c, not recorded as carried.
- (9.6) `l2_shaper_rate_bits` is a rate in a header named with the count unit `_bits` (README §1 line 38 has no rate
  unit; second.s.down_bits is bits per one-second window). Acceptable if SERIES's unit column keeps "bits per second".
