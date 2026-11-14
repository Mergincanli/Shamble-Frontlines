# U3b fix pass: ImpairSocket and L2Config per the skeptic (build agent, worktree agent-a9099feea96fafb39)

Base: 975e72b. The worktree could not fast-forward (it sat on main's 69ea88e, which 975e72b does not contain), so it
switched to a new local branch `u3bfix-a9099` at 975e72b (inside the isolated worktree only). Nothing committed.
Inputs: u3-verify-u3b.md (skeptic, REVISE small), u3bv-plant.py (its planted defects), u3-build-u3b.md, advisor-u3q.md
M1-M5, M9-M15, M19, .claude/rules/netcode.md. Edit scope: l2.rs, l2/*.rs, marks/SERIES.md (L2 section) only.

## Plan
1. Bites for the six survivors (all on the new code; planted again with my own script, u3bfix-plant.py, on a copy
   under /home/user/Shamble-Frontlines/.gate/u3bfix-copy):
   - M1 (D): a socket test holds poll_recv off ~90 ms after a 20 ms datagram is due; asserts up lateness >= 50 ms.
   - M4 (B): one peer, P2 jitter, up and down at the same times: per-index delays differ (shared stream => equal).
   - M3 (A): held datagram due 10 ms, successor taken in at 15 ms (after the due, before the token pops): no overtake.
   - M19 (G): same seed with and without the shaper; each due = (a standalone shaper's departure for the intake
     order) + (that index's own delay from the run without shaper); jitter > serialisation, so dues invert.
   - M10 (H): capacity 2, no pins, two rounds: the least recently seen goes each time (slot 1, then slot 0).
   - Drop (K): the reader records its tid (test accessor); after drop both tids vanish and the port binds again.
2. Slack: the release thread answers a header-time request (it alone may read its own timerslack_ns without
   CAP_SYS_NICE: fs/proc/base.c checks the capability when p != current); header gets l2_timer_slack_ns (header
   time) and l2_timer_slack_start_ns (thread start). thread.rs test: default 0 under SCHED_FIFO/RR (ed4fb6d7ef68).
3. Pool: every datagram charged its full 1,500 B buffer against both byte bounds; defaults 2,048 buffers per peer
   per direction, 4,096 per direction in total (pool 8,192 = 2 x total, so the pool cannot empty at defaults and the
   total bound triggers with two flooders). Test: one flooder cannot pool- or queue-drop another peer's datagrams.
4. wt.rs: wtransport ConnectionError::ApplicationClosed -> H3 (cite error.rs); a loopback bite if a client can close
   after QUIC is up; comment + notes: accept-loop handshake cap and H3 deadline carried to U6.
5. intake: lookup only, checks (oversize, bounds, pool) first, slot_for after; a host drop touches no peer state.
   Test: new address with oversize / total-bound / empty-pool drops creates no peer, evicts nobody. U6 note: pin
   joined clients with set_peer.
6. Throttle toggle: each flow keeps its last departure; a new shaper starts with its free time there, no shaper
   departs at max(t, last). Test at -23's settings (fixed delay), on/off/on: release order = intake order.
7. SERIES.md L2 section: l2_summary list, queued_bytes meanings (buffer bytes; down is not "waiting for quinn"),
   slack rows, the TCP-model rows carried to U3c.
8. shaper.rs doc nit.
Then: cargo test (release, all features), fmt, clippy -D warnings, check.mjs (9 steps), planted defects, notes.

## Progress
- Items 1-8 coded (impair.rs, shaper.rs, config.rs, socket.rs, thread.rs, wt.rs, names.rs docs, the three test
  files, SERIES.md). Design as planned; deviations: none. One test fix after the first run (the l2_summary check
  looked for `name` but SERIES rows are `second.v.name`). npt-net release, all features: 104 passed, 0 failed,
  9 ignored (l2: 55 passed, 1 ignored; was 45). Next: fmt, clippy, planted defects, chrt run, M13 run, check.mjs.
- fmt applied (cargo +1.99.1 fmt --all); clippy --release -p npt-net --all-targets -D warnings clean with
  --all-features and with no features. Copy for planted defects: .gate/u3bfix-copy/netcode-proof (taken after fmt);
  script .checkpoints/netcode-proof-test/u3bfix-plant.py (16 defects: the skeptic's 6 survivors B A D G H K re-cut
  for the new code, A2 (<= guard), P S T1 T2 L W for items 2-6, and C I J re-run); running.
- Planted defects, first 9 (release, all features, l2:: tests; each on the restored copy):
  B BIT a_peers_two_directions_draw_apart | A BIT the_reorder_step_holds_until_1_ns_after_the_successor | A2 (<=)
  BIT the same | D BIT inbound_lateness_is_measured_where_quinn_receives_it (+ wtransport_serves_..._at_p2 once:
  to recheck, likely load flake) | G BIT the_shaper_serialises_in_intake_order_before_the_jitter (+ a_throttle_
  toggle_keeps_fifo_order) | H BIT eviction_takes_the_least_recently_seen (+ host_drops_from_new_addresses_touch_
  no_peer) | K BIT dropping_the_socket_ends_its_threads | P BIT one_flooding_peer_cannot_starve_another (+4 that
  read the charge) | S BIT host_drops_from_new_addresses_touch_no_peer.
  Then: T1 (fresh shaper at a change) BIT a_throttle_toggle_keeps_fifo_order | T2 (no floor without a shaper) BIT
  the same | L (header reports the start slack) BIT the_header_reads_the_slack_when_it_is_built | W
  (ApplicationClosed -> Quic) BIT wtransport_counts_a_close_after_quic_as_h3 | C BIT send_errors_are_fates_not_
  deliveries | I BIT one_queue_releases_by_due_time_across_peers (+2) | J BIT host_drops_take_no_index_and_are_
  counted_apart, one_flooding_peer_cannot_starve_another. Restored copy: 55 passed, 0 failed, 1 ignored.
  All 16 bite; none survives. Results file: .gate/u3bfix-copy/results.txt.
- Repeat runs (release, all features, l2::): 18 full-suite runs after the fix, 17 green, 1 red:
  web_transport_quinn_serves_a_session_over_impair_socket_at_p2 at wt_tests.rs:203 (up datagrams below 180 of
  200). Pre-existing tail, not from this fix: up counts at P2 are 185-199 after the fix (24 samples) and 189-197 at
  base 975e72b (16 samples, base built in .gate/u3bfix-base from git archive); the defect-D run's extra wtransport
  P2 failure is the same tail. Likely cause: under CPU contention the client's 5 ms sleeps bunch datagrams into one
  QUIC packet, so one 2 % packet loss takes several. Not changed here (a threshold change without the cause
  measured would hide it): for the main session.
- chrt -f 10 on the test binary: a_thread_sets_its_own_slack_to_1_ns, the_header_reads_the_slack_when_it_is_built
  and the_pass_through_takes_the_same_path_and_the_header_says_so pass (this kernel 6.18 has ed4fb6d7ef68); base
  975e72b under the same chrt fails at thread.rs:111 ("the default slack (Some(0)) is above 1 ns"), as the skeptic
  found.
- M13 (ignored test, release, 10 s, P2, 10,000/s each way): fix run 1: delivered 97,926 up / 98,017 down (same
  fates as base and the build notes), 0 queue/pool/shaper drops at the new defaults, up p50 29.5 us p99 107.3 p99.9
  14.8 ms max 23.1 ms E 32.9 us, down p50 33.5 p99 103.4 p99.9 13.9 ms E 29.9 us, release CPU 0.399 core. Back-to-
  back pair (load 0.4-0.6): base up p99.9 2.37 ms E 7.16 us / down E 6.93 us, 0.368 core; fix up p50 23.5 p99 64.5
  p99.9 0.236 ms max 1.36 ms E 0.314 us, down p99.9 0.214 ms E 0.320 us, 0.369 core. Tails swing with the shared
  container; no regression seen. Slack read back 1.
- check.mjs (node check.mjs from spikes/netcode-proof): all 9 steps PASS: fmt; clippy and clippy-wasm32 (measured
  and bites builds); test (measured build 193 passed, 0 failed; bites build 206 passed, 0 failed); deps; refs (339
  tracked files); marks (81 marks, rows unchanged); self 9/9; redact 10/10 tests, 110 items, 0 findings.
  Cargo.lock and Cargo.toml unchanged (git diff empty). After the planted-defect copy was taken, one cosmetic edit in
  socket.rs (the slack answer's local `now` renamed `read`); tests, clippy and check.mjs ran on the final code.

## For the main session
- Branch note: the worktree sat on main's 69ea88e, which 975e72b does not contain, so `git merge --ff-only 975e72b`
  refused; I created local branch `u3bfix-a9099` at 975e72b inside the worktree. Nothing committed.
- Changed files (uncommitted): l2/config.rs, impair.rs, impair_tests.rs, names.rs (docs only), shaper.rs, socket.rs,
  socket_tests.rs, thread.rs, wt.rs, wt_tests.rs; marks/SERIES.md. l2.rs untouched. Notes and u3bfix-plant.py here.
- Carried (recorded in code comments): U6 bounds the accept loops (concurrent handshakes after QUIC, an H3-stage
  deadline, the `tx.send` wait) before WtServer reuses wt.rs; U6's control plane pins every joined client with
  set_peer; U3c adds the TCP model's SERIES rows (name, version, S12's stall and repair counts); R1 reads whether
  RS's Ubuntu 6.8 kernel carries ed4fb6d7ef68 (thread.rs's test fails under chrt on a kernel without it).
- Behaviour changes worth a reviewer's eye: (a) `l2_*_queued_bytes` and both byte bounds are now buffer bytes
  (1,500 per datagram); defaults 3,072,000 B per peer and 6,144,000 B total per direction (were 4 MiB and 12 MiB of
  payload); (b) a host-dropped datagram touches no peer (no creation, eviction, LRU refresh or first-seen stamp), so
  the Join deadline starts at the first datagram that passes the host checks; (c) a new shaper after a settings
  change inherits the time still owed (it may tail-drop at once if the old queue was deep); (d) header_numbers waits
  up to 1 s for the release thread's answer (microseconds in practice).
- Pre-existing flake, not fixed: the P2 session tests' `up_received >= 180` (wt_tests.rs:203); 1 red in 18 runs here,
  same up-count spread at base. Needs its cause measured (datagram bunching under load) before any threshold change.
DONE

## What changed, per item (for the report)
1. Bites (each with its planted defect; results below):
   - M1 inbound: socket_tests `inbound_lateness_is_measured_where_quinn_receives_it` (20 ms delay, poll held off
     until first-seen + 110 ms; lateness must be in [50 ms, at - first - 20 ms]; also nothing handed over before
     the poll).
   - M4: impair_tests `a_peers_two_directions_draw_apart` (P2, 2,000 indices up and down at the same times; fewer
     than 10 indices may draw the same fate).
   - M3 guard: in `the_reorder_step_holds_until_1_ns_after_the_successor`, successors at 15 ms (after the 10 ms
     due) and at exactly 10 ms: the held one keeps its time, no overtake, no reorder.
   - M19: impair_tests `the_shaper_serialises_in_intake_order_before_the_jitter` (same seed with and without a
     1 Mbps / 25,000 B shaper, ±20 ms jitter, 1,000 B every 5 ms: due = standalone FIFO departure over intake times
     + that index's own delay; and the dues do invert).
   - M10 LRU: impair_tests `eviction_takes_the_least_recently_seen` (capacity 2, no pins, two rounds evicting slot
     1 then slot 0, each the least recently seen).
   - Drop: socket_tests `dropping_the_socket_ends_its_threads` now also checks the reader's tid (recorded by the
     reader at start; cfg(test) accessor `reader_thread_id`) and re-binds the port.
2. Slack: socket.rs `ask_slack(write)` sends a request through the state (slack_ask counter + optional write); the
   release thread answers at its loop top (reads its own /proc/<tid>/timerslack_ns; another thread needs
   CAP_SYS_NICE, fs/proc/base.c), the header waits up to 1 s on a second Condvar. Header: `l2_timer_slack_ns` = read
   when the header is built (with `l2_sched_policy`, also read then), new `l2_timer_slack_start_ns` = the start
   read-back. Test `the_header_reads_the_slack_when_it_is_built` (the thread writes 7 µs on request, standing in for
   an outside change; header shows 7,000 now and 1 at start; 0/0 under RT). thread.rs test: default 0 and read 0
   under SCHED_FIFO/RR, comment cites ed4fb6d7ef68 (6.12; 6.6.84) and that RS's 6.8 is read at R1.
3. Pool: every queued datagram charged its whole buffer (CHARGE_BYTES = 1,500) against both byte bounds; defaults
   pool 8,192, per peer per direction 2,048 buffers (3,072,000 B), total per direction 4,096 (6,144,000 B): both
   totals = the pool, so the pool cannot empty at defaults and the total bound triggers with two flooders. Test
   `one_flooding_peer_cannot_starve_another` (defaults; simulated pool of 8,192; one 50 B flooder both ways:
   7,952 queue drops per direction, 0 pool drops; another peer gets through both ways; a second flooder fills the
   up total; a third peer's up datagram is a total-bound drop, its down gets through; pool never empty).
4. wt.rs: `ConnectionError::ApplicationClosed` -> H3 (wtransport 0.7.2 error.rs:16-18, 48-51, 312-316; RFC 9000
   §12.4/§10.2.3: an application close only travels in 0-RTT/1-RTT, and our TLS has no 0-RTT). Test
   `wtransport_counts_a_close_after_quic_as_h3` (raw quinn client, ALPN h3, QUIC up, closes after 100 ms: H3 1,
   Quic 0). Module doc: accept-loop handshake cap and H3-stage deadline (and the `tx.send` wait) carried to U6.
5. intake: `target()` (no side effects) -> host checks (oversize, bounds, pool) -> `place()` -> touch/stamp; a host
   drop from a new address logs ordinal u64::MAX ("before the table", stats.rs's existing doc). Test
   `host_drops_from_new_addresses_touch_no_peer`; `the_first_raw_packet_is_stamped_at_arrival` updated (an oversize
   first packet from a new address is no longer stamped: the first accepted one is). U6 note (impair.rs and
   socket.rs docs, set_peer's doc): pin every joined client with set_peer.
6. Throttle toggle: Flow.last_depart_ns; `Shaper::busy_until(r, q, last)` on a shaper change; with no shaper a
   datagram departs at max(t, last). Test `a_throttle_toggle_keeps_fifo_order` (-23's 128 kbps / 5,920 B, fixed
   50 ms delay; on, off at 1 ms, on at 600 ms, off at 601 ms, on at 603 ms while a departure is still ahead; 10
   datagrams released in intake order, 0 reordered). Shaper unit test `a_shaper_made_mid_run_starts_busy_until_the_
   last_departure`.
7. SERIES.md L2 section: the l2_summary paragraph lists its fields (every per-second name except
   l2_release_cpu_permille; levels at the run's end; l2_release_cpu_ns; the three reorder shares); queued_bytes
   rows (buffer bytes; down: shaper wait/delay, held, release batch, not "waiting for quinn"); slack rows (header
   time + the new start row); pool/bound header rows (charge, defaults); eviction row (host drops evict nobody);
   "Carried to U3c (M15)" line for the TCP model's name/version and S12's stall and repair counts. socket_tests now
   checks every l2_summary field is named in SERIES.md and that l2_release_cpu_permille is not one of them.
8. shaper.rs doc: the 5th datagram is taken at the very ns the first has left, not 1 ns before.
