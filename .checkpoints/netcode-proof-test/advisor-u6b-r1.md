# Advisor: U6b's fix list after its one round (R-PING, A1, A2, A3), 2026-11-23

Instructions: main's `.claude/agents/advisor.md` (`git show origin/main:.claude/agents/advisor.md`, 335 lines), read
21:4x UTC. Rules in force: NPT-TRIM (history.md:7561-7593: "One review round per unit ... A second round only for an
open Critical or High."); SIZE-CHECK (history.md:2048-2051, u6b-brief.md section 6); his SIZE-STOP (a) of 2026-11-23
(history.md:7703-7712: "The new stop for U8 is 6,200 added lines": U8's alone). My limits: no pass mark, judge rule,
marks.json text, money or stop of his changes on this ruling; none does.

## 1. My reading
**Inputs, hashes checked:** u6b-brief.md rev 1.1 32baaebd…b99b and wip/u6b-build.patch 49f4488f…c14c (both as the task
gives them); advisor-u6b-brief.md 3c0f8121…; u6b-build.md be6807f4…; u6b-fix-pass.md 031742e4…; the seats' notes,
netcode c75ec1d7…, security e63ad804…, test a9397a0c…, each ending DONE. W = the built tree
.claude/worktrees/agent-a2661dc320c9e6d00/spikes/netcode-proof, read only (no cargo run by me).

**Checked myself (file:line in W unless named):**
- **F-PING / NR-U6B-3 hold.** Every gate reading adds WsLink's own queue to owed (`inp.owed += queued`,
  crates/net/src/ws_server.rs:646); the outside path asks `owed == 0` (:699-708), so a Ping or Pong the pump sends in
  the same poll after a tick's batch is refused; the outside counts are cfg(test) only (`gate_counts`, :650-660); there
  is no size bound. The only datagrams sent outside a tick are the pump's (server/src/pump.rs:351-356): Ping (10 B
  encoded) and Pong (24 B), npt-proto's limits 32 B each (proto/src/limits.rs:32-33; catalogue :91-92). The smallest
  Snapshot is 17 B (1 B type, 12 B header, 4 B counts: proto/src/snapshot.rs:23-26), so a size bound cannot tell a
  near-empty snapshot from a Pong: it is a backstop, and the proof that TickLoop announces every snapshot stays
  `u6b_each_tick_is_announced_whole_and_a_refused_one_is_counted` (every outside datagram decoded as Ping or Pong).
  npt-net has no npt-proto dependency (crates/net/Cargo.toml), so the bound comes from the caller, as NR-U6B-2's tick
  period does. The WebSocket session's RTT is its own Pings' (`RttSource::Pings`, server/src/main.rs:634).
- **T-1 holds.** The per-instance test reads `armed.bite` and a hand-made `TickWriter` (ws_server_tests.rs:825-841); the
  (c) test runs `writer_run_through_ws_link`, which sets the writer by hand (:627-637, :849); the block that carries an
  instance's bite into each accepted link (ws_server.rs:494-501) and the binary's arming (main.rs:573-579) run in no
  test.
- **R1 reads no part of the bite; R3 does.** R1's bites are U6a's three room bites (wip/u8-build.patch:1805-1806:
  PM-0001-1/shared-vcpu, PM-0006-11/no-smt-map, PM-0001-16/double-load); W's marks.json:4472-4473 judges
  PM-0001-18/ws-budget-x2 "on the L2 B1 ws part", R3's (replan-marks.md:37); U6e comes before R2 and R3 (the card's
  Next steps, item 2). The reader stamp is R1's (plan.md:1256: "the split built in U3c (b5c9009) plus U6's reader
  stamp"), so it stays (NR-U6B-9).
- **The bite part's size in the snapshot:** npt-net lib.rs 1 (patch :1397), ws_gate.rs +10/-6 (:1675-1740),
  ws_server.rs about 34 (the field, the id, `with_bite`, the accept-path block, the acceptor's field: :1977-1998,
  :2232-2240), the two tests 38 (:3553-3590), main.rs 11 (:5110-5113, :5192, :5288-5295): about 90-100 added lines.
  `Bite::BudgetX2` itself stays: U3c's own tests use it under cfg(test) (ws_gate_tests.rs:40, :225).
- **NR-U6B-1 holds, and its fix needs one more term.** main.rs:777-779 runs `net.end(&tl)` before `finish(tl)`.
  TickLoop's stop awaits the session tasks within FLUSH (500 ms, tick_loop.rs:63) and then calls
  `runtime.shutdown_timeout(FLUSH)` (:1603-1612). The links' closing tasks run on that I/O runtime, and WsLink's close
  waits up to SHUTDOWN_WAIT, 10 s (ws_server.rs:840-850): a fix that only moves `Net::end` after the session tasks
  still loses every closing not ended by the runtime's shutdown, and with it its `ws_summary`.
- **SEC-U6B-1 holds.** WtLink's writer drains with `write_all`/`open_uni` and no deadline (wt_server.rs:496-509); only
  `stopped()` is under CLOSE_WAIT (:515-519); the permit drops when the task ends (:292-296). WsLink's drain loop
  (ws_server.rs:828-839) has the same shape before its 10 s closing timeout.
- **SEC-U6B-2 holds, with a scope note.** `transport()` (wt_server.rs:57-75) sets cc and the datagram buffer only, and it
  is shared by the server (:103) and WtClient (:588): the limits belong in the server's config only.
- **A2's tool exists in U6c's plan.** U6c's container readings already hold "npt-load with 48 clients for 60 s on each
  transport against a room at P2", "before Stop B's read" (u6c-brief.md:353-357), and U8's `ds` room is the same shape
  (transport ws, P2, 48 clients, a mark: wip/u8-build.patch:2076). The card's dates: U6c merged about Sat 28, Stop B
  about Tue 1 (netcode-proof-test.md:181).

**Not checked by me:** the seats' other findings (taken from their files, each with file:line); every line estimate
(judgements); the netcode seat's "about 1 in 33" refused pings (its arithmetic, not measured).

## 2. Verdicts
### R-PING: ruled. Condition 1's letter is replaced by NR-U6B-3's rule, with a size bound from the caller
The letter ("a datagram send no current `begin_tick` announced ... is answered BufferFull and counted, never written
ungated") would refuse every Ping and Pong on WebSocket: no RTT for the room (its own Pings) and no clock sync for the
client (the room's Pongs). The built rule keeps its purpose; NR-U6B-3's four parts make it whole: a size bound, the
link's own bounded queue out of the outside check, the counts in the log, the rule in the trait's doc. A test and one
plant prove the new parts. Severity unchanged (NR-U6B-3 Low); the brief's text changes because the condition is mine.

**Paste-ready for u6b-brief.md rev 1.2 (section 0.1, condition 1):** in condition 1's quoted text, replace "a datagram
send no current `begin_tick` announced, or one past its count, is answered BufferFull and counted, never written
ungated;" with:
> "a datagram send outside an announced tick (none announced, or past its count: the room's own Pings and Pongs, which
> its I/O step sends between ticks) is never held to the snapshot budget and never written ungated: WsLink writes it
> only when its length is at most the largest Ping or Pong (npt-proto's `limits::PING` and `limits::PONG`, 32 B, passed
> in by npt-server beside the gate's tick period, as npt-net names no npt-proto item) and fix 1's first two conditions
> hold (nothing owed below WsLink's own send queue, which is bounded and left out of this check only, and room in the
> socket for its model bytes); otherwise it is answered BufferFull; the sent and the refused are counted in
> `event.ws_accept` (two new SERIES.md rows); the trait's doc states the rule and the caller's duty (one call per tick
> before its datagrams, then exactly those datagrams, in order, with no other datagram between them);"

and add after condition 1: "(Rev 1.2, R-PING, advisor-u6b-r1.md: NR-U6B-3 replaces the letter, which would refuse every
WebSocket Ping and Pong. The size bound is a backstop, since a Snapshot can be 17 B; that TickLoop announces every
snapshot stays proven by `u6b_each_tick_is_announced_whole_and_a_refused_one_is_counted`.) Section 3 adds: a datagram of
33 B outside a tick refused and counted, one of 32 B with nothing owed sent; a Ping sent in the same poll after an
allowed tick's batch is sent; both counts read from `ws_accept`; plant P17 (the size bound removed) fails the first."

The builder's text is item 4 of section 4.

### A1: ruled (a). The bite part moves to U6e, with T-1 word for word as U6e's condition; T-1 is closed for U6b
- **Why (a):** (1) no R1 reader: R1 runs U6a's three room bites, and ws-budget-x2 is judged at R3's L2 B1 ws part,
  after U6e; (2) the valve's own rule, read per item ("past 5,400 they wait for U6e if R1 reads neither"; the reader
  stamp is R1's, the bite is not), and the unit stands at 5,498; (3) (b) costs a second round (in my U6b check, "a
  second round on any lane unit" is one of the lost days that slip R1), and by the list's own figure the Mediums with
  T-1's fix reach about 5,950-6,000 before R-PING and any Low, at his stop; (a) takes about 90-100 lines out and
  leaves about 5,850-5,900 after the Mediums and R-PING; (4) nothing is loosened: no mark, judge rule or marks.json text moves, the bite still lands before
  the run that judges it, and its evidence must meet T-1 there.
- **What it means for T-1:** closed for U6b by removing its subject, not by a fix: U6b's merged tree claims no
  ws-budget-x2 behaviour. It stays High as U6e's condition: U6e's round sees it met, never a STATUS follow-up. U6b's
  report says exactly that.
- **Cost:** U6e grows by about 150-200 likely lines after R1 (the hunks plus T-1's tests and plant): at about 1,175
  merged lines a day, about 0.15 of a day, inside the close calendar's spare day (Tue 15, my U6b check section 3). No
  run date moves.

**Paste-ready for rev 1.2:**
- Section 0, "What R1 asks of U6b": "Bite built here: PM-0001-18/ws-budget-x2 (judged at R3's L2 B1 ws part)." reads
  "The bite PM-0001-18/ws-budget-x2 is U6e's (rev 1.2, A1; judged at R3's L2 B1 ws part)."
- Block 5's bites bullet reads: "moved to U6e with SP-4 (rev 1.2, A1); `Bite::BudgetX2` and the queueing bite stay
  test-only, as at `<base>`."
- Section 3's Bites paragraph reads: "None in U6b (rev 1.2, A1: moved to U6e)."
- Condition 5's counts read: "at least +45 measured and +45 bites (the two bite tests moved: rev 1.2), plus each test
  the fix pass adds, by name; plants P1-P11, P14, P15, P16 and the fix pass's".
- Section 2's U6e line adds: "the bite PM-0001-18/ws-budget-x2 (SP-4 and SEC-U6BD-2 by number): snapshot 49f4488f's
  hunks (wip/u6b-build.patch:1397, :1675-1740, :1977-1998, :2232-2240, :3553-3590, :5110-5113, :5192, :5288-5295)
  rebuilt on U6e's base, with T-1 (u6b-review-test.md section 4) as U6e's condition, word for word: 'The bite
  PM-0001-18/ws-budget-x2 is never armed through an instance's accepted connection: one test reads the `bite` field and
  a hand-made TickWriter, the other hand-sets the writer on a hand-built WsLink. Deleting :494-501 passes both, so
  "armed by id per instance, the armed instance fails (c)" is not proven.' Its fix, word for word: 'In the (c) bite
  test take the link from `WsServer::bind(..P2..).with_bite(WS_BUDGET_BITE)` after start, connect and accept; in the
  per-instance test accept one connection on each instance and assert 2x formula vs formula on the accepted links; a
  plant removing :494-501 fails both.' (Line numbers: W at 49f4488f: ws_server_tests.rs:825-859, ws_server.rs:372,
  :494-501, server/src/main.rs:573-579.) By number with it: the test seat's N4 (the binary's arming runs in no test)
  and T-4's bite half (the (c) bite test inside the 5 s real-clock cap)."

### A2: ruled as leaned. The 48-connection reading and the memory bound are read at U6c's close, with U6c's npt-load
- **Why:** U6c's plan already runs the very load R1 uses (48 npt-load clients at P2 for 60 s) "before Stop B's read";
  its WebSocket leg against the release room, with the mark set, gives every number the example would, from the room's
  own `ws_split`, `ws_summary` and /proc, through R1's real path, at about no added code. A bespoke example costs
  150-250 lines here, near his stop, for a tool thrown away after one reading. Condition 8 asks for the reading "before
  Stop B's read of R1's run plan": U6c's close (about Sat 28) comes before it (about Tue 1). The same run also shows
  NR-U6B-1's fix at work (the room ends with the 48 still connected: 48 `ws_summary`, `ws_split` after them).
- **Terms:** (1) the record follows u6b-review-test.md section 5's provenance, 48-connection and memory bullets (the
  memory: VmHWM and VmRSS from /proc/<pid>/status before the first connection and with the 48 held, per-connection
  bytes from the difference, against (N + 8) x that plus the fixed part with NR-U6B-5's recount, the method stated);
  the mark set after the 48 are up so that each direction's n is at least 10,000; "reported, never judged";
  (2) if U6c's close would come after Stop B's read, the main session runs it on U6c's built tree before that read,
  with its provenance; if npt-load is not built by then, it asks the advisor before that read; (3) U6b's report gives
  the 20 starts and names this reading by number (it reverses the card's lean of 2026-11-23 "U6b's container readings
  ... built in its fix pass": the main session records the change in the day's message, one line).

**Paste-ready for rev 1.2 (section 3, "Container readings"):** "20 starts' spawn to `OPEN` with both listeners and the
certificate (p50, max; U6a's were a lower bound), in U6b's fix pass on the head that merges. DelayStream's self-test
reading (48 WsClient connections at P2 for 60 s against the release room, the mark set after the 48 are up: the four
parts' p50, p99, p99.9, max and E per direction, the early-reader count, the violations with T-5's note, pooled L in
both forms with their over counts, the mark, steal before and after) and the cap's memory bound at N = 48 (measured, the
method stated) are read at U6c's close, in the WebSocket leg of U6c's own reading 'npt-load with 48 clients for 60 s
... against a room at P2' (u6c-brief.md:356), from the room's own log and /proc/<pid>/status, before Stop B's read of
R1's run plan (condition 8; rev 1.2, A2). No example under crates/server/examples/."

### A3: no second round. The advisor's check of the fix diff suffices
- With A1, no Critical or High is open in U6b; NPT-TRIM allows a second round only for one. The security seat says of
  its two Mediums: "the advisor's check of the fix diff is enough". The test seat's REVISE rested on T-1 (moved) and
  eight Mediums (fixed, or carried by number where the list says).
- **The check** (an advisor agent from main's advisor.md, xhigh, read only, a test through the runner only where a claim
  needs one), on the fix pass's snapshot against 49f4488f, before the merge's M-steps: (1) each Medium's fix against its
  seat's words (SEC-U6B-1, -2; NR-U6B-1, -2; T-2 to T-9) and each Low built; (2) R-PING as rev 1.2 words it; (3) A1's
  removal complete (item 0's grep); (4) every plant's log and diff file, the old ones re-run on the fixed tree; (5)
  check.mjs twice, its counts against `<base>` (754, 850) and the snapshot (799, 898); (6) the size line; (7) the 20
  starts' record; (8) the report's carries by number.
- If that check finds a Critical or High (a fix that opens one, as U6a's fix pass once did), the seat that owns the item
  reads that item only, as round 2. Nothing else opens a round.

## 3. Changes to the fix list (u6b-fix-pass.md)
1. **New item 0, first: A1's removal** (the size falls first, and no later fix edits code that leaves).
2. **Item 1, SEC-U6B-1:** the bound runs from the link's drop over the drain and the close (WtLink: CLOSE_WAIT; WsLink:
   one stated bound, SHUTDOWN_WAIT or shorter, with its reason), then the close, then the permit; the kick tests (P5,
   P6) still pass inside it; plant P18 (the bound removed) fails the new test.
3. **Item 2, SEC-U6B-2:** the server's config only (`transport()` is shared with WtClient at wt_server.rs:588); each
   value with its reason; a test that reads the four values off the server's config.
4. **Item 3, NR-U6B-1, one added term:** `Net::end` reads the summaries and `ws_split` only after every link's closing
   task has ended (every permit back in the cap), waited for on the I/O runtime before its shutdown and within item 1's
   bound (FLUSH alone cuts the closings); a closing that misses the bound still pushes its summary.
5. **Items 4 and 6 together:** NR-U6B-2's tick period and R-PING's size bound are both caller values through one form
   (`Listen` or `WsServer::bind`: the builder's choice, named), so one signature change and one set of test edits.
   R-PING is built as rev 1.2 words it, with its tests and P17.
6. **Item 5, T-2 to T-9 as worded,** with: T-2's WebTransport half carried by number to U6e with T-14 if it needs a QUIC
   stage hook (named in the notes); T-4 on the remaining test only (the bite's test moves); T-5's R1.md half goes to U8's
   next edit; T-7's re-plant is P14b (OPEN between `Net::bind` and `serve`).
7. **Item 7, the Lows, in this order:** NR-U6B-6 (the row only; R1.md's INVALID rule reading it is U8's next edit),
   SEC-U6B-4 with its plant P19 (the signature check skipped), SEC-U6B-5 with P15b (P15's file written beside the log),
   NR-U6B-4, NR-U6B-5, T-12, T-17. **The Lows' line:** a Low starts only if the running total plus its estimate stays
   at or under 5,950. It is a scope line (an unbuilt Low is carried by number), not a stop.
8. **Items 8 and 9 swap:** the end checks first (fmt, check.mjs twice, every plant old and new with its diff file), then
   the 20 starts on that same tree (the runner's diff hash equal to the counted check.mjs's), so no fix lands after the
   reading. The 48-connection reading and the memory bound leave (A2). The test seat's "a WebSocket and a WebTransport
   client opened at once after OPEN" is answered by T-7's check instead.
9. **Carries:** T-16 moves from U6d to U6e (U6e is the next unit in ws_server_tests.rs, with T-1's tests, and T-16 runs
   on DelayStream). Added: **U8's next edit** (R1.md): NR-U6B-6's room-clock mark read by the INVALID rule, NR-U6B-9's
   reader rows, T-5's note (the identity is a range check for R); **U7:** SEC-U6B-2's unidirectional count confirmed in
   its headless Chromium run; **U6c's close:** A2's reading; **PR #40's closing list:** SEC-U6B-3, carried to the
   security baseline, is a security finding left open (advisor.md step 4), so it is named there (PR #40 is
   needs-mustafa already). Every other taker in the list's section 3 stands.
10. **Lane:** TickLoop's stop step (NR-U6B-1) joins U6b's lane beside the accept seam and the I/O send site, named in
    the report; crates/super for T-19's plant (restored).
11. **The size line:** 5,498, less about 90-100 (item 0), plus about 400-440 (the Mediums) and 40-60 (R-PING): about
    5,850-5,900, room for most Lows under 5,950. His stop stays past 6,000; the builder's part is in section 4.

## 4. Paste-ready block for the builder's message (to a2661dc320c9e6d00)
```text
U6b's fix pass, by the advisor's ruling advisor-u6b-r1.md (R-PING, A1, A2, A3). Your notes: u6b-build.md, a new
"## Fix pass" heading, one line per item with the running size. Every cargo call through u6b-run.sh (logs under
.gate/u6b/), the lane lock, the disk floors; on the 4 GB floor (exit 3) you stop, note it and report; you never clean.

Start: your tree must be the snapshot 49f4488f: no plant in place, and the runner's diff hash equal to check5.log's
(d4ad43fd...). If not, stop and report.

The rulings: R-PING, condition 1's sentence is replaced (item 4). A1, the ws-budget-x2 bite leaves U6b for U6e
(item 0). A2, the 48-connection reading and the memory bound are read at U6c's close, not by you. A3, no second
round: the advisor checks your fix diff.

Plants: one -x -k runner call each, restored, sha256-checked; each fails its named test; each plant's edit saved beside
its log as plant-<id>.diff with Write (copied from git diff's output, never a redirect).

The items, in this order:
0. A1, the bite part leaves: WS_BUDGET_BITE (ws_server.rs and npt-net lib.rs's re-export), WsServer::with_bite, the
   `bite` field of WsServer and of its acceptor, the accept-path block that sets the writer's bite (ws_server.rs:494-501),
   main.rs's WS_BUDGET_BITE constant, its entry in the settings check's bite list and the arming at :573-579, and the two
   pm_0001_18_* tests; ws_gate.rs's Bite and TickWriter.bite back to <base>'s cfg(test) text. Then grep crates/net and
   crates/server for ws-budget, WS_BUDGET_BITE and WsServer's with_bite (none left), and for BudgetX2 (cfg(test) only),
   and put the output in your notes. The snapshot keeps the hunks for U6e.
1. SEC-U6B-1: from a link's drop, its drain and its close run under one bound (WtLink: CLOSE_WAIT; WsLink: one bound,
   SHUTDOWN_WAIT or shorter, stated with its reason), then the close, then the permit drops. A test with a peer that
   never grants control-stream credit gets the permit back within the bound (this covers T-13); the kick tests still
   pass; plant P18 (the bound removed) fails the new test.
2. SEC-U6B-2: the server's QUIC config only (transport() is shared with WtClient at wt_server.rs:588): small
   max_concurrent_bidi_streams and max_concurrent_uni_streams (HTTP/3's control, QPACK encoder and decoder, plus a
   stated margin), stream_receive_window 64 KiB, receive_window 1-2 MiB, each with its reason; a test reads the four
   values off the server's config.
3. NR-U6B-1: the sessions end first (stop set, the tick thread joined, the session tasks awaited), then every link's
   closing task is awaited on the I/O runtime before its shutdown, within item 1's bound (every permit back in the cap;
   FLUSH alone cuts the closings), with the log writer still open; then Net::end (summaries, ws_split, the accept
   events); then the log's end. A binary test: a WsClient still connected when stdin closes gives one ws_summary, and
   ws_split comes after it.
4. NR-U6B-2 with R-PING, both caller values through one form (Listen or WsServer::bind, your choice, named):
   - the gate's tick period: npt-server passes 1_000_000_000 / u64::from(npt_sim::TICK_HZ); TICK_NS stays in tests
     only; a 60 Hz test gives N = 10 at srtt 100 ms;
   - R-PING, condition 1 as rev 1.2 words it: "a datagram send outside an announced tick (none announced, or past its
     count: the room's own Pings and Pongs, which its I/O step sends between ticks) is never held to the snapshot budget
     and never written ungated: WsLink writes it only when its length is at most the largest Ping or Pong (npt-proto's
     limits::PING and limits::PONG, 32 B, passed in by npt-server beside the gate's tick period, as npt-net names no
     npt-proto item) and fix 1's first two conditions hold (nothing owed below WsLink's own send queue, which is bounded
     and left out of this check only, and room in the socket for its model bytes); otherwise it is answered BufferFull;
     the sent and the refused are counted in event.ws_accept (two new SERIES.md rows); the trait's doc states the rule
     and the caller's duty (one call per tick before its datagrams, then exactly those datagrams, in order, with no
     other datagram between them)". Tests: 33 B outside a tick refused and counted, 32 B with nothing owed sent; a Ping
     sent in the same poll after an allowed tick's batch is sent; both counts read from ws_accept. Plant P17 (the size
     bound removed) fails the first.
5. T-2 to T-9 as u6b-review-test.md section 4 words each fix: T-2 the injected clock moved during the handshake (its
   WebTransport half, if it needs a QUIC-stage hook, is named in your notes and carried to U6e by number); T-3 a bounded
   wait on the arrivals for the 300 ms sleep; T-4 the 5 s cap on b4_the_writer_keeps_outstanding_within_n_snapshots_
   through_ws_link (the bite's test has left); T-5 the loopback self-test asserts each R is the first closing wake after
   its release; T-6 the pool's mark test on a local Pool (it also covers NR-U6B-12); T-7 the serving stamps against
   OPEN on one clock, and P14b (OPEN between Net::bind and serve) fails it; T-8 timeouts on both accepts and the Open
   assert in the end-to-end test; T-9 FULL_STATE_MAX exactly.
6. The Lows, in this order, each started only if the running total plus its estimate stays at or under 5,950 (a scope
   line: an unbuilt Low is carried by number, not a stop): NR-U6B-6 (the mark on the room's clock, one row; R1.md is
   U8's, not yours); SEC-U6B-4 (a garbled signature over the pinned certificate refused) with P19 (the signature check
   skipped); SEC-U6B-5 (the whole run folder listed and scanned for the PKCS#8 P-256 prefix) with P15b (P15's file
   written beside the log); NR-U6B-4 (an empty tick sends no tick; announced decremented before the size check);
   NR-U6B-5 (the pool's added heap in a header row); T-12 (the bite path pushes r); T-17 (the hash file's mode against
   the umask).
7. End checks: rustfmt's diff applied by Edit; check.mjs twice (-x; the second is the counted one); its counts against
   <base> (754 measured, 850 bites) and the snapshot (799, 898): at least +45 measured and +45 bites plus each new test,
   by name; the test step's time against 954.8 s (past 1.5 times it stops); every plant, old (P1-P11, P14, P15, P16) and
   new (P14b, P15b, P17, P18, P19, and T-19's F28 plant, P20: BufWriter back), failing its named test on this tree.
8. The 20 starts, on the same tree (the runner's diff hash equal to the counted check.mjs's): the release build, default
   features, no bites; 20 starts to OPEN by a scratch script beside your notes, written with Write, each start with a
   fresh token file, log and hash path (npt-super time-starts only if it gives each start fresh paths: create_new
   refuses a used hash path); its record as u6b-review-test.md section 5's provenance and starts bullets (the binary's
   sha256, the machine, free disk, the time, each command line; each start's spawn_to_open_ns and exit; p50 and max;
   any failure named; the line that they replace U6a's lower bound).

The size: after each item, git diff --shortstat 21765e3 -- spikes/netcode-proof docs (new files intent-to-add, as
your 5,498 was measured), one notes line with the total.
The stop is Mustafa's SIZE-CHECK (history.md:2048-2051; the brief's section 6): past 6,000 added lines the build stops.
If an item's end shows more than 6,000: start no further item; write "STOP: past 6,000 after item <n> (<total>)" and
each unbuilt item by number under "Not built (past 6,000)"; run item 7's end checks on the tree as it stands; leave
item 8 (the starts run on the head that merges); return. You never ask him, choose another stop, or cut a built item
to get under it; the main session gives him the three lines. His 6,200 of 2026-11-23 is U8's stop, not yours.

Your lane: crates/net, crates/server (main.rs, cert.rs, the settings, tests; TickLoop's accept seam, its I/O send site
and, for item 3, its stop step), crates/link (begin_tick's doc), crates/super (P20 only, restored), marks/SERIES.md new
rows only. Not yours: every Cargo.toml and Cargo.lock, proto, room, client, bots, R1.md and U8's files, marks.json, any
existing SERIES.md row, check.mjs. A part that needs one stops, is reported, and the rest is built. Tests on 127.0.0.1
only, an injected clock wherever a time is asserted, real-clock tests at most 5 s each, no timing bar.
Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing
files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write.

Return (in this order):
1. Each item 0-8: done, or not built and why, with its tests by name, its plants and its size line.
2. The final shortstat (added, removed, files), against 5,498.
3. check.mjs x2: each of the 11 steps, both counts against <base> and the snapshot, the test step's time, the logs.
4. Every plant: id, its diff file, the named test's failure line, the restore hash.
5. The 20 starts' record (item 8).
6. Item 0's grep output.
7. The new pub items with reasons (the netcode seat's NR-U6B-8 list less the bite's two), the SERIES.md rows added,
   the TickLoop changes, each named.
8. F-items found; anything carried, by number with its taker.
9. Your notes file's sha256.
```

## 5. What goes to Mustafa
**Decisions needed:** none. Each ruling is inside the plan (no pass mark, judge rule, marks.json text, money, run day or
stop of his changes): the main session names them in the day's message, one line each, as choices taken without him
(NPT-TRIM): R-PING (condition 1's letter replaced by the built rule with a size bound), A1 (the ws-budget-x2 bite to
U6e with T-1 as its condition), A2 (the 48-connection reading at U6c's close), A3 (no second round).
**For Mustafa:** nothing. Only if the builder stops past 6,000 does he get the three lines, by his own rule. With no
second round and about 90-100 lines out, U6b's merge by end Thu 26 stays in reach; the card's three-line triggers
stand unchanged.

DONE
