# Advisor: U6b's fix-pass check (A3 of advisor-u6b-r1.md), 2026-11-23

Instructions: main's `.claude/agents/advisor.md` (`git show origin/main:.claude/agents/advisor.md`, 335 lines), read at
the start of this check. My ruling: advisor-u6b-r1.md sha256 dc6973da…35f5 (checked). Snapshot wip/u6b-fix.patch
e29c003c…ac64af1c (checked); the round's wip/u6b-build.patch 49f4488f…0c14c (checked); notes u6b-build.md 74e0d697…
(checked). T = the built tree .claude/worktrees/agent-a2661dc320c9e6d00/spikes/netcode-proof, read only.

## 1. Plan
1. Inputs: hashes (done above); the three reviews' Medium and Low rows, word for word; the brief rev 1.2's section 0.1
   condition 1, section 2 (U6e line), section 3 (bites, container readings), condition 5.
2. Item (1): for each Medium (SEC-U6B-1, -2; NR-U6B-1, -2; T-2..T-9) read the seat's fix words, then the tree at the
   notes' cited lines (base-vs-tree diff of the touched file where needed). Each built Low the same.
3. Item (2): R-PING against rev 1.2's paste-ready text: size bound, queue left out, counts in ws_accept + two
   SERIES rows, trait doc, the three tests, P17.
4. Item (3): grep T for WS_BUDGET_BITE / ws-budget / with_bite on WsServer / BudgetX2 outside cfg(test); ws_gate.rs vs
   base; main.rs bite list.
5. Item (4): every fx-plant-*.log (20) and every plant-*.diff in the worktree's .gate/u6b/: named test FAILED, the
   restore, the diff file's content matches the plant described.
6. Item (5): fx-check1.log and fx-check2.log: 11 steps, counts (754/850 base; 799/898 snapshot; 807/904 now), diff
   hash, test time.
7. Item (6): the size: the patch's own counts (git apply --numstat), the notes' running line, the 5,950 scope line and
   6,000 stop.
8. Item (7): fx-starts.log and fx-release.log: diff hash equal to fx-check2's, the 20 values, p50/max, binary hash.
9. Item (8): carries by number: the fix list section 3, my ruling's section 3 item 9, the builder's not-built Lows.
10. F-TICK-ROUND: the arithmetic and the tree's N test.
11. Findings, verdict, carries, Mustafa, DONE. A cargo test only if a claim needs it (through u6b-run.sh).

## 2. Items
**Provenance (checked again after the cut, 2026-11-24):** T's `git diff HEAD` (repo root) hashes e29c003c… = the
snapshot; `git diff HEAD -- .` in spikes/netcode-proof hashes 219ee92e…0fef = fx-check1/2's diff line; 28 files, +5,952
-267 (`--shortstat`); the worktree's u6b-build.md = the main folder's (74e0d697…). Every plant diff's "before" blob is
the tree's file now (`git hash-object`: session.rs bfe90ca, ws_server.rs 137feec, wt_server.rs c703b8b, pin.rs
496663c, link/lib.rs ee3aee8, server/main.rs 5b9d026, super/main.rs 07886aa). The main session's re-run of the
snapshot script on the builder's worktree (2026-11-24 04:3x UTC, `python3 -I .checkpoints/netcode-proof-test/
worktree_patch.py`: the tracked `git diff HEAD --binary` plus every untracked file, .checkpoints/ left out) hashes
e29c003c…ac64af1c = wip/u6b-fix.patch, base 21765e3, 28 files: the snapshot is the whole tree, no untracked file
outside it (the coordinator's message; not re-run by me). My alternate-index check of each plant's diff hash was
stopped before it ran (it asked for a permission); item (4) uses the logs and diff files instead.

### (1) Each Medium's fix against its seat's words, and each Low built
- **SEC-U6B-1: MET.** Seat's fix (u6b-review-security.md:66-69): "from the link's drop, run the drain and the close
  under one timeout (e.g. CLOSE_WAIT), then `session.close` (pending writes fail) and the permit drops; the same around
  WsLink's drain loop; a test with a peer that never grants control-stream credit (the permit back within the bound)
  and a plant (the bound removed) failing it." Tree: one bound `CLOSE_WAIT` 2 s with its reason (session.rs:103-110);
  `bounded()` runs `work` until done or CLOSE_WAIT after `Flags::dropped` (a `Notify` permit, so a drop before the task
  waits is kept: session.rs:115-126, :135-144); each link's Drop calls it (wt_server.rs:443-446, ws_server.rs:754-757).
  WtLink: the drain and the control stream's finish + `stopped()` under it, then `session.close(0, b"")`, then the
  permit (wt_server.rs:521-547, :317-321). WsLink: the drain, Close, the reunite and shutdown under it, a cut counted a
  close error unless the peer closed, the summary pushed, then the permit (ws_server.rs:833-866); the 10 s SHUTDOWN_WAIT
  is gone (module text :32-36). Test b2_a_peer_that_grants_no_credit_gets_its_permit_back_within_the_bound
  (wt_server_tests.rs:380-420: a 256 B client stream window never read, keep-alives, 2 x 500 B queued, the permit back
  within 4 s). P18 (plant-p18.diff: `bounded` = `Some(work.await)`) fails it at wt_server_tests.rs:418 (item 4). Kick
  tests pass in both counted runs. Note: no WebSocket-side test of the bound (P18 removes it in the shared helper, so
  the WT test catches it; a WsLink that skipped `bounded` would pass): Low, F-1 below.
- **SEC-U6B-2: MET.** Seat's fix (:77-80): "in the server's `transport()` only: `max_concurrent_bidi_streams` small
  (CONNECT + control + slack), `max_concurrent_uni_streams` small (HTTP/3 control, QPACK x2, grease slack; check against
  U7's headless Chromium), `stream_receive_window` 64 KiB, `receive_window` 1-2 MiB; a test that reads the four values
  off the config". Tree: `server_transport(cc)` only from `WtServer::bind` (wt_server.rs:93-100, :128); WtClient keeps
  `transport()` (:615), so my ruling's term (the server's config only, wt_server.rs:588 then) holds. Values with
  reasons (:77-91): bidi 4, uni 6 (3 + 3 grease margin, "U7's headless Chromium run confirms the count"), stream 64 KiB,
  connection 1 MiB. Test b2_the_servers_quic_config_bounds_its_streams_and_windows (wt_server_tests.rs:470-495) reads
  the four off the bound server's `ServerConfig.transport` (quinn's Debug text) and the client's 100.
- **NR-U6B-1: MET, with my ruling's added term.** Seat's fix (u6b-review-netcode.md:161): "End the sessions first: a
  TickLoop step that sets stop, joins the tick thread and awaits the session tasks within FLUSH ... while the writer
  stays open (e.g. `stop_with(f)` ...), then `Net::end`, then the log's end. A binary test: a WsClient still connected
  when stdin closes gives one `ws_summary`, and `ws_split` comes after it." My term (advisor-u6b-r1.md:165-167): the
  summaries and `ws_split` read only after every closing task ended (every permit back), waited for before the
  runtime's shutdown, within item 1's bound; a cut closing still pushes its summary. Tree: `TickLoop::stop_with`
  (tick_loop.rs:1581-1583) → `shutdown`: stop set, the tick thread joined, the session tasks awaited within FLUSH, then
  `at_end(self)` with the runtime taken but alive (a multi-thread runtime, :1326, so the closings run during the wait)
  and the log's sender open, then `shutdown_timeout(FLUSH)`, then `events = None` and the writer's join
  (:1585-1633); the writer no longer stops at the End row (:975-977), and the only other row sender is the joined tick
  thread's. `Net::end` waits until `cap.free() == cap.permits()` or CLOSINGS_WAIT_NS 3 s (main.rs:144-147, :660-664),
  then the summaries, `ws_split`, the two accept events (:665-675); `finish` = `stop_with(|tl| net.end(tl))`
  (main.rs:843-844). WsLink pushes its summary before its permit drops, also when the bound cut it
  (ws_server.rs:855-865), so "every permit back" implies every summary pushed. Binary test (tests/binary.rs:317-321,
  :370-375): the WsClient stays connected through stdin's close; one `ws_summary`, before `ws_split`.
- **NR-U6B-2: MET** (F-TICK-ROUND: section 3). Seat's fix (u6b-review-netcode.md:162): "`Listen` (or `WsServer::bind`)
  takes `tick_ns`; npt-server passes `1_000_000_000 / u64::from(npt_sim::TICK_HZ)`; every `TickWriter::new` takes it;
  `TICK_NS` stays in tests only; one test ... gives N = 10 at srtt 100 ms." Tree: the form is `Listen` (named, my
  ruling's item 5: one form for both caller values), `GateCfg::of(&Listen)` (ws_server.rs:84-98, :462), the gate's
  `TickWriter::new(c.tick_ns)` (:575); `TICK_NS` is `#[cfg(test)]` (:68-70); npt-server passes `GATE_TICK_NS` =
  `1e9.div_ceil(TICK_HZ)` with its reason (server/src/main.rs:148-150, :561); ws_gate.rs equals `<base>` (no diff), so
  `snapshot_budget(srtt, tick_ns)` is unchanged (ws_gate.rs:25-31). Test b4_the_gate_counts_n_in_the_callers_tick_60_hz_
  gives_10_at_100_ms (ws_server_tests.rs:710-729): N = 10 at 60 Hz through `GateCfg::of(&Listen)`, 6 at the tests' 30 Hz.
- **T-2: MET, both halves** (my ruling's carry of the WT half to U6e is not needed: no QUIC-stage hook). Seat's fix
  (u6b-review-test.md:127): "raw TCP, wait for the permit, set 700, finish TLS and the upgrade and assert 500; the same
  on WebTransport between the first packet and the answer." WS: ws_server_tests.rs:215-241 exactly that. WT:
  b2_the_accept_stamp_is_the_first_packets_never_the_answers (wt_server_tests.rs:333-377): the clock moves 500 → 700
  after QUIC's handshake and before HTTP/3's SETTINGS, CONNECT and the answer; the session carries 500.
- **T-3: MET.** Fix (:128): "Wait (bounded) on an arrivals count". Tree: the clock counts its own reads (load, then
  count, SeqCst: ws_server_tests.rs:197-208); the test waits (5 s bound) for the reader's two reads, then moves the
  clock to 9,000 (:242-256). No sleep left in the test. (An unrelated clock read would make it fail, never pass falsely.)
- **T-4: MET as worded; its time alone not measured.** Fix (:129): "Bursts at 0.5 s and 2.5 s (or one burst), or state
  the exception". Tree: bursts at 0.5 s and 2.5 s (ws_server_tests.rs:824-827; the bite's (c) test left with A1). The
  ws_server module's wall time fell from 6.53 s (fx4-test3.log:36, before T-4) to 5.20-5.21 s (fx5-test.log,
  fx6-test3.log:79), with 22-23 tests in parallel: the test is at most about 5.2 s under that load; alone, by the old
  6.54 s less 2 s, about 4.5 s. Not shown under 5 s alone: Low F-2.
- **T-5: MET (the code half).** Fix (:130): "Say so in the report and in R1.md's reading (the identity is a range check
  for R); in the loopback self-test assert each R is the first closing wake after its release." Tree: `reader_split`
  checks R = the first closing wake at or after the release (`partition_point` on `reader_wakes`,
  l2/delay_tests.rs:3097-3127), used by the reader-stamp test (:3142) and the b5 loopback self-test
  (ws_server_tests.rs:934). Report half: U6b's report; R1.md half: U8's next edit (my ruling's section 3 item 9).
- **T-6: MET** (also NR-U6B-12). Fix (:131): "A test on a local `Pool` (not the static): samples, mark, samples; assert
  each window's n and that the parts sum to L; state the in-flight skew bound in the report." Tree: `Pool::new()`
  (l2/delay.rs:229-234); u6b_a_marked_pool_counts_one_window_for_the_parts_and_pooled_l (l2/delay_tests.rs:3151-3190):
  10 releases at 10x, one mark (a second refused), 6 at 1x; each window's n in the parts, the reader parts, the kinds
  and pooled L asserted; the window's own maxima (6,000 and 9,000 µs-ns, not the 10x ones) with each class's parts'
  maxima summing to L's. The sum appears as the maxima of identical releases, not as a sum field; with n asserted it
  fixes the sums. The skew bound is stated in the test's doc (:3155-3157): report text.
- **T-7: MET.** Fix (:132): "Log each listener's serving stamp and OPEN's on one clock and assert serving <= OPEN;
  re-plant P14 between Net::bind and serve." Tree: `Net::serve` returns both stamps on the room clock after each
  `start()` (server/src/main.rs:617-622); OPEN's stamp just before its line, event `open` with three rows
  (:782-791; SERIES.md:883-885); the binary test asserts both <= `open_ns` (tests/binary.rs:366-368); P14b
  (plant-p14b.diff: OPEN and its stamp moved before `serve`) fails it at binary.rs:368 (item 4).
- **T-8: MET.** Fix (:133): "`tokio::time::timeout(5 s, ...)` on each accept with an expect; assert both links Open
  after the wait." Tree: tick_loop/tests.rs:1545-1567: both accepts under 5 s timeouts with expects; both client
  links asserted Open after a 5 s bounded wait.
- **T-9: MET.** Fix (:134): "Send FULL_STATE_MAX bytes and assert them received". Tree: ws_server_tests.rs:503-523
  sends 65,541 B exactly and asserts 65,541 received, malformed 0 (the optional one-more case not built).
- **The Lows built (my ruling's item 7 order; the scope line, item (6)):**
  - NR-U6B-6: MET. The mark task reads `TickLoop::now_ns()` just before `mark_split()` and stores it only when the
    mark is taken (server/src/main.rs:635-643); `ws_split` carries `ws_split_mark_room_ns` (:666-670; SERIES.md:882).
    Tested at 0 only (binary.rs:376-379; a mark needs more than 1 s of run): the non-zero case is read at A2's U6c
    reading, which sets the mark (carry C-6).
  - SEC-U6B-4 (= T-15): MET, in a stronger form than the seat's unit call. pin.rs's
    u6b_the_pin_refuses_the_pinned_certificate_signed_with_another_key: a full TLS 1.3 handshake against a server
    presenting the pinned certificate, taken with its own key, refused with another key; P19 (plant-p19.diff: the
    signature check returns `assertion()`) fails it at pin.rs:141.
  - SEC-U6B-5: MET. The OPEN test lists the run folder and hash/ after the run, scans each file for PKCS#8's P-256
    start (the scan proved on a fresh rcgen key), and asserts exactly hash/room.json, room.jsonl, settings.json,
    tokens (tests/binary.rs:322-345); P15 and P15b fail it at :342. (It lists files, not new subfolders: no finding,
    the room makes none.)
  - NR-U6B-4: MET. `begin_tick(&[])` returns `Sent` with no tick (ws_server.rs:665-668); `send_unreliable` decrements
    `announced` before the `TooLarge` return (:685-698); the trait doc says "an empty `lens` announces nothing"
    (link/src/lib.rs, `begin_tick`'s doc); test b4_an_empty_tick_is_none_and_a_too_large_datagram_takes_its_slot
    (ws_server_tests.rs:696-708).
  - Not built (the scope line): NR-U6B-5, T-12, T-17: carried by number (item 8).
- **Lane:** the diff touches no Cargo.toml or Cargo.lock, marks.json, R1.md, check.mjs, room, client or bots file;
  ws_gate.rs is back to `<base>`; SERIES.md has 82 added lines and none removed; proto's +9 is the round's added item
  1, unchanged in the fix pass (notes :330-331).
**Item (1): MET.**

### (2) R-PING as rev 1.2 words it (advisor-u6b-r1.md:65-81; u6b-brief.md:12-25 binds it by line range): MET
- "never held to the snapshot budget": the outside path never calls the writer's `on_tick` (ws_server.rs:699-713).
- "WsLink writes it only when its length is at most the largest Ping or Pong ... passed in by npt-server beside the
  gate's tick period, as npt-net names no npt-proto item": `bytes.len() <= outside_max` (:703-704), `outside_max` a
  `Listen` field beside `tick_ns` (GateCfg, :84-98), npt-server passes `Limits::PING.max(Limits::PONG)` = 32 B
  (server/src/main.rs:561-562; proto/src/limits.rs:32-33); npt-net's manifest is unchanged (no npt-proto dependency).
- "fix 1's first two conditions hold (nothing owed below WsLink's own send queue, which is bounded and left out of
  this check only, and room in the socket for its model bytes)": `inputs(false)` leaves `queued` out of owed, a tick's
  `inputs(true)` keeps it (:610-645); the check is `owed == 0 && free >= model_bytes(len)` (:704-706).
- "otherwise it is answered BufferFull": :710-712. "None announced, or past its count": `outside` is true exactly when
  `announced == 0` (:685-695).
- "the sent and the refused are counted in `event.ws_accept` (two new SERIES.md rows)": `Counts::outside` on the
  server's shared counts (the gate holds the server's `Arc<Counts>`, :584), rows `outside_tick_sent_count` and
  `outside_tick_refused_count` in `Counts::rows` for WebSocket only (session.rs:182-196), which `WsServer::event_rows`
  writes (ws_server.rs:385-392); SERIES.md:901-902. The round's cfg(test)-only counts (NR-U6B-3 (a)) are now logged.
- "the trait's doc states the rule and the caller's duty (one call per tick before its datagrams, then exactly those
  datagrams, in order, with no other datagram between them)": link/src/lib.rs, `begin_tick`'s doc, both paragraphs
  ("The caller's duty", "Outside a tick").
- Tests (rev 1.2's section 3 line): b4_outside_a_tick_only_a_ping_or_pong_with_nothing_owed_below_the_queue_is_sent
  (ws_server_tests.rs:669-694, current_thread so the writer task cannot drain): 33 B refused, 32 B sent, a 10 B Ping
  after an allowed tick's batch (still in the link's queue, asserted) sent; both counts (2 sent, 1 refused) read from
  `Counts::rows`, the function `ws_accept` writes. The round's outside test now uses 32 B (:657-666). P17
  (plant-p17.diff: the bound made always true) fails the new test at ws_server_tests.rs:678 (item 4).
- The proof that TickLoop announces every snapshot stays u6b_each_tick_is_announced_whole_and_a_refused_one_is_counted
  (as my ruling says; it passes in both counted runs).

### (3) A1's removal complete (item 0's grep): MET
- My grep of T (2026-11-24): `ws-budget`, `ws_budget`, `WS_BUDGET_BITE`, `pm_0001_18_*` in crates/net and
  crates/server: no hit but U5's PM-0001-18 tests and docs (tcp_model/fixes.rs, server/tests/gap.rs: `<base>`'s).
  `with_bite` on WsServer: none; the hits left are TickWriter's (`#[cfg(test)]`, ws_gate.rs:98-99) and its test
  callers, and npt-server's own Schedule, Decoder and loopback bites. `BudgetX2`: ws_gate.rs:63-66 (enum under
  `#[cfg(test)]`), :107-108 (under `#[cfg(test)]`), ws_gate_tests.rs:40, :225, l2/delay_tests.rs:3457 (test files;
  delay_tests.rs's one use is `<base>`'s too). No "bite" left in ws_server.rs.
- ws_gate.rs and ws_gate_tests.rs: `git diff HEAD` empty (= `<base>`). npt-net lib.rs: only the re-export
  `WS_BUDGET_BITE` left against the round (the per-file comparison of the two snapshots). main.rs: the bite list is
  `<base>`'s three (server/src/main.rs:264-266), no WS_BUDGET_BITE, no arming.
- The size fell 5,498 → 5,398 (notes :204), inside my ruling's "about 90-100". The snapshot 49f4488f keeps the hunks
  for U6e (wip/u6b-build.patch, sha256 checked).

### (4) Every plant's log and diff file, the old ones re-run on the fixed tree: MET
Read with Grep over .gate/u6b/fx-plant-*.log (20 logs) and Read of the worktree's .gate/u6b/plant-*.diff (20 files).
Every log: the lane lock held (-x), HEAD 21765e3, untracked = the empty set's hash, one filtered test run ("0 passed;
1 failed"), exit 101; its diff hash differs from the clean 219ee92e…0fef and from the 19 others (20 distinct). Every
diff file names its plant and file, and its "before" blob is the tree's file now (provenance above), so every old
plant was re-applied to the fixed tree.

| Plant | Diff file's edit | Named test, failure (log line) |
|---|---|---|
| P1 | ws_server.rs, Writer::run forgets the permit | b1_one_cap_of_n_plus_8_across_both_transports, ws_server_tests.rs:166 (p1.log:19) |
| P2 | WsLink::poll_recv stamps at the poll | b1_arrival_stamps_are_taken_at_arrival_never_at_poll, :263 (p2.log:19) |
| P3 | wt_server.rs, the deadline over QUIC only | b2_the_deadline_covers_http3_..., wt_server_tests.rs:271 (p3.log:19) |
| P4 | the WebSocket deadline 10 s later | b3_the_deadline_covers_tls_and_the_upgrade, ws_server_tests.rs:299 (p4.log:19) |
| P5 | no closing sequence (immediate close) | b2_a_kick_queued_before_the_drop_is_read_before_the_close, wt_server_tests.rs:330 (p5.log:19) |
| P6 | the Close before the queued messages | b3_a_kick_queued_before_the_drop_..., ws_server_tests.rs:465 (p6.log:19) |
| P7 | tungstenite's limit from the 513 B constant | b3_one_source_for_tungstenite_and_delaystream_limits, :277 (p7.log:19) |
| P8 | the gate's inputs read per datagram | b4_the_inputs_are_read_once_per_tick, :626 (p8.log:19) |
| P9 | each datagram gated alone | b4_a_tick_is_written_all_or_none, :647 (p9.log:19) |
| P10 | owed without rustls's unsent bytes | b4_owed_counts_rustls_unsent_bytes, :1010 (p10.log:20) |
| P11 | a close error counted whatever the peer did | b4_a_close_error_after_the_peers_close_is_a_normal_end, :547 (p11.log:31) |
| P14 | OPEN before Net::bind | u6b_open_..., tests/binary.rs:235 (p14.log:25) |
| P14b | OPEN and its stamp between Net::bind and serve (T-7) | u6b_open_..., binary.rs:368, serving <= open_ns (p14b.log:25) |
| P15 | a stand-in file beside the hash file | u6b_open_..., binary.rs:342, the file list (p15.log:25) |
| P15b | the same beside the log (SEC-U6B-5) | u6b_open_..., binary.rs:342 (p15b.log:25) |
| P16 | link/lib.rs, the Box forward of begin_tick removed | b4_begin_tick_through_a_boxed_link_sees_the_gates_refusal, ws_server_tests.rs:738 (p16.log:19) |
| P17 | R-PING's size bound made always true | b4_outside_a_tick_only_a_ping_or_pong_..., :678, the 33 B one Sent (p17.log:19) |
| P18 | session.rs, `bounded` without its bound | b2_a_peer_that_grants_no_credit_..., wt_server_tests.rs:418, no permit in 4 s (p18.log:20) |
| P19 | pin.rs, the signature check returns `assertion()` | u6b_the_pin_refuses_the_pinned_certificate_signed_with_another_key, pin.rs:141 (p19.log:19) |
| P20 | super/main.rs, LineWriter back to BufWriter (6 sites) | u6b_each_record_reaches_the_file_while_npt_super_runs, super/src/main.rs:624 (p20.log:17) |

- The restores: the last plant ran at 23:01:44 (p20.log:47); the counted check.mjs run 2 started 23:02 with the clean
  tree's diff 219ee92e (item 5), and the tree's blobs are the diff files' "before" blobs now.
- My ruling's new plants are all there: P14b, P15b, P17, P18, P19, P20 (T-19's F28 plant). T-10 is met in substance
  (each plant's edit recorded), but the diff files sit in the worktree's gitignored .gate/u6b/ (F-DIFF-HOME), not
  beside their logs in the main .gate/u6b/: removing the worktree removes them (condition C1).
- Not checked by me: that each log's diff hash is the clean diff with that file's planted blob (it needs an alternate
  git index; the call asked for a permission and was not run). The tie rests on each failure matching its diff's edit.
  If wanted: the main session computes it for one plant before the worktree goes (not a condition).

### (5) check.mjs twice, with its counts: MET
| | fx-check1.log | fx-check2.log (counted) |
|---|---|---|
| Window, lock | 22:29:55-22:47:37 UTC, -x (:1, :159) | 23:02:25-23:18:58 UTC, -x (:1, :159) |
| Tree | diff 219ee92e…0fef, untracked empty, HEAD 21765e3, free 6,698 MiB (:2-3) | the same diff, free 6,672 MiB (:2-3) |
| Both `--no-run` builds | exit 0 (:74, :145) | exit 0 (:74, :145) |
| 11 steps | all PASS, "check: all 11 steps pass" (:147-158) | all PASS (:147-158) |
| test | 863.0 s; measured 807, bites 904, 0 failed, 0 skips (:151) | 848.5 s; 807 and 904 (:151) |
| marks / self / redact | 81 marks; 78/78; 11/11, 0 findings (:155-157) | the same (:155-157) |

- Against `<base>` (754, 850): +53 measured, +54 bites. Against the snapshot 49f4488f (799, 898): +8 and +6.
- Condition 5 as rev 1.2 binds it (advisor-u6b-r1.md:106-107: "at least +45 measured and +45 bites ..., plus each
  test the fix pass adds, by name"): the fix pass adds 8 tests, all in both builds, so at least 807 and 903: met
  (807, 904). The arithmetic holds: 799 + 8 = 807; 898 - 2 (the pm_0001_18_* tests, bites only) + 8 = 904. The 8, by
  name (notes :275-280), each found in T: b2_a_peer_that_grants_no_credit_gets_its_permit_back_within_the_bound,
  b2_the_servers_quic_config_bounds_its_streams_and_windows, b2_the_accept_stamp_is_the_first_packets_never_the_answers
  (wt_server_tests.rs), b4_outside_a_tick_only_a_ping_or_pong_with_nothing_owed_below_the_queue_is_sent,
  b4_the_gate_counts_n_in_the_callers_tick_60_hz_gives_10_at_100_ms, b4_an_empty_tick_is_none_and_a_too_large_datagram_
  takes_its_slot (ws_server_tests.rs), u6b_a_marked_pool_counts_one_window_for_the_parts_and_pooled_l
  (l2/delay_tests.rs), u6b_the_pin_refuses_the_pinned_certificate_signed_with_another_key (pin.rs).
- The test step against 954.8 s: 0.90x and 0.89x (the stop is 1.5x). The counted run's tree is the snapshot's
  spikes/ part (219ee92e = `git diff HEAD -- .` in T, provenance) with every plant restored (item 4).

### (6) The size line: MET
- Final: 28 files, +5,952 -267 (`git diff HEAD --shortstat` at T's root, HEAD = 21765e3; the snapshot's own line).
  Against the round's 5,498 (29 files, -254): +454 net; ws_gate.rs left the diff (back to `<base>`).
- The running line (notes :204-271): item 0 → 5,398 (-100); 1 → 5,476; 2 → 5,522; 3 → 5,569; 4 → 5,686; 5 → 5,874
  (my ruling's "about 5,850-5,900" after the Mediums and R-PING); then the Lows: NR-U6B-6 → 5,883, SEC-U6B-4 → 5,926,
  SEC-U6B-5 → 5,933, NR-U6B-4 → 5,952.
- The scope line (my ruling's item 7: "a Low starts only if the running total plus its estimate stays at or under
  5,950 ... a scope line, not a stop"), read at each start: SEC-U6B-4 5,883 + 25 = 5,908; SEC-U6B-5 5,926 + 20 =
  5,946; NR-U6B-4 5,933 + 15 = 5,948: each allowed to start; NR-U6B-4 took 19, ending 2 past the line, which the line
  allows (it governs a start). NR-U6B-5 at 5,952 + 8 was not started, nor T-12 or T-17: carried by number. The order is
  my ruling's.
- Mustafa's SIZE-CHECK stop (past 6,000) was not reached: 48 under it. No "STOP" line in the notes, none needed.
