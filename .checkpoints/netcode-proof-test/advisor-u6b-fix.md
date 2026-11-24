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
496663c, link/lib.rs ee3aee8, server/main.rs 5b9d026, super/main.rs 07886aa).

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
