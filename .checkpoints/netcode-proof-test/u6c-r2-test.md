# U6c round 2, test seat (test-engineer, xhigh), on the fix pass's diff only. Read only.
Inputs, sha256 checked equal (2026-11-24): advisor-u6c-r1.md ad72c0b8…faed179, u6c-review-netcode.md f29e1ac1…,
u6c-review-test.md 79197b42…, u6c-review-security.md c29540fa…, u6c-build.md a9d45dd4…, wip/u6c-fix.patch 0549389e….
W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3a3e3fdf559de095/spikes/netcode-proof. Diff aad21031 →
6c3ff00b (10 files, +356/-94). File:line below are in W unless named otherwise.

## 1. Plan
1. Read the diff file by file: net l2/probe.rs and probe_tests.rs; server probe.rs and probe_tests.rs; bots
   tests/load.rs, src/load.rs, load_tests.rs; host serve.rs, serve_tests.rs; SERIES.md.
2. FP-1: the new `leg_series`, FP-1's test, plant-p22.log; the C-U6-5 and leg_series tests' new calls; the e2e bound.
3. FP-2: `edge_events`' signature and calls; every read after `drop(sock)`; plant-p8.log, plant-p8c.log.
4. FP-3 / F-P2-HEADER: the new assert, plant-p2b.log; whether the assert bites; a variant plant and its size.
5. FP-4: (b) and (c), plant-p19b.log; timing dependence.
6. FP-5: fp5-list.log, read-a2-ws.log, read-a2-wt.log.
7. FP-6's tests: F-NO-PEER, P18 (plant-p18b.log), NC-U6C-8, F-NAMES, F-5S, SEC-U6C-3's 404 entry, "room sign".
8. Counts: fix-check.log against the snapshot's hashes.
Run nothing unless a claim needs it; name each run.

## 2. Notes (one line per finding or check)
- Run r2-test-1 (.gate/u6c/r2-test-1.log, 18:01:32 UTC; runner, shared lock, command `true`): only to print W's
  hashes. diff 0e3199ae…, untracked 22bda910…: equal to fix-check.log (13:27), read-a2-ws/-wt (14:08, 14:10) and
  read-probe.log (14:29). So fix-check's tree is W's at 18:01, the tree the 17:5x patch came from. No other run.
- Counts: the diff adds three `#[test]` and removes none: FP-1's test, F-NO-PEER's, and the ignored A2 WebTransport
  one. So 861 = 859 + 2 and 958 = 956 + 2 against end-check2.log. They tie. The test step's +59.5 s on round 1 (1,176.3
  against 1,116.8 s) is not the two tests (0.26 s and about 0.2 s): machine load, not root-caused.
- FP-1's test (net probe_tests.rs:288-357): up t_in strictly rising (100 µs + i/2 ns), down strictly rising (last three
  +60 µs, no reorder); per direction the log order is the t_in order; no two fates share a t_in. It pins: exactly N
  outside the burst a direction (:331), the burst's fates apart (:336), both spans (:341-348), the first and last
  second (:349-355), with and without the B1 burst. It does not pin the t_in order against log order or the tie clause.
- P22 (plant-p22.log): break at t_in >= start + packets × interval; fails :331 with (119,999, 119,999), exactly the one
  record each arm (5 ppm up, late pacer down) puts past the clock span. The right reason. The plant shows in the
  untracked hash (79241c8a), as net probe.rs is a new file. OK.
- TE-U6C-R2-1 (Low): t_in order and the tie clause unpinned. Both are live at R1: Delivered fates are logged at release
  (impair.rs:685-696), drops at intake (impair.rs:576-579), so the log is not in t_in order; up t_in is one clock read
  per receive batch (socket.rs:633-639), so ties occur. Dropping the sort (net probe.rs:721) or the tie clause
  (`last[d] != Some(t)`, :725) passes all three leg_series tests. No mark flips (a direction still keeps >= packets).
- C-U6-5 test (packets 1,002, net probe_tests.rs:208): the same 1,002 up fates as [10 s, 20 s) (1,000, the Oversize at
  11 s, the Lost at 12 s); span 2 s + 1 ms checked. leg_series test (packets 4, :266): all 5 down (4 outside, 1 in the
  burst) and the 4 peer ups, one second. Same fates. Both pass at any larger packets, so they pin "the same fates", not
  the cut; FP-1's test pins the cut. OK.
- e2e bound (server probe_tests.rs:363-366): `n >= s.packets` a direction; P1's leg runs about 2 s at 2,000 a second
  each way (window 1 s plus a tail of about 1 s), so about 4,000 candidates for 2,000 records. Settles F-TIMING (a).
- FP-2: `edge_events(_bound: &ImpairSocket, …)` (server probe.rs:179); calls :465 and :553 pass `&sock`. After
  `drop(sock)` (:571) nothing at :572-657 reads the kernel (header, params, seconds and summary are values taken
  before). plant-p8c.log: E0382 at :571, "value borrowed here after move". u6c-build.md:278-279 words it as the ruling
  does, never as a test failure. The hold is on the call's place: `_bound` is unused, so any live ImpairSocket would
  satisfy it; no realistic plant passes another socket. Closed.
- FP-3: the assert (server probe_tests.rs:367-375) compares P1's whole-life `l2_down_delivered_count` (run_summary,
  cumulative: net probe_tests.rs:157-163) with P1's pacer count (server probe.rs:547, :646). It cannot fire in a sound
  run (only the pacer sends down). plant-p2b.log fails at :358 (`base_rtt_ns` 0, not 525,313) before :372.
- TE-U6C-R2-2 (Low): FP-3's assert is not shown to bite. A variant P2c (P2b plus one line writing the leg's `base` as
  P1's header `base_rtt_ns`, after server probe.rs:578) passes :358 and :363-366 (P1's records are counted from P1's
  start) and fails first at :372 (P1 delivers P0's downs plus its own). Size: P2b's edit plus one line, one runner call.
- FP-4 (b) (tests/load.rs:279-284): the pacer catches up by schedule up to `count` (net probe.rs:324-357, :527-528), so
  the count is short only if LG is stopped while behind; >= 5,000 of 10,000 tolerates about 0.5 s. Closed.
- TE-U6C-R2-6 (Low): no upper bound. With PROBE_RATE_PPS 10,000 (load.rs:51) and 1 s, `count` caps the sends at
  10,000; a doubled rate passes, a halved one sits on the bound.
- FP-4 (c) (net probe_tests.rs:510-522): one sender on loopback keeps order, so after echo 9 no echo can follow; the
  ten reads are deterministic. The bound left: a 1 s stall of LG (200 times its 5 ms spacing); each `up()` also panics
  after 1 s (:473, :477). plant-p19b.log fails at :506, an assert from before FP-4 (:437 in round 1); the ruling asked
  only that P19 still fail. Closed.
- FP-5: fp5-list.log (13:15, before FP-6, diff d91df182): "1 test, 0 benchmarks" for each full name. read-a2-ws.log
  and read-a2-wt.log ran at the final tree (hashes equal), each "1 passed" (70.8 s, 100.1 s). Closed.
- TE-U6C-R2-7 (Low): the A2 WebTransport reading's prints mislead. read-a2-wt.log:22 prints `ws_split` as Some(all
  zeros, `ws_split_mark_set` 1), not "None" as u6c-build.md:236 says. Its "min 0" (:23) spans every measured second
  (tests/load.rs:371-376): LG runs 30 s past the room's close. LG's log (/tmp/npt-u6c-load-7849-a2-wt/load.jsonl):
  slot 23 gave 0 in second 0 (READY's part second); from second 71, 47 of 48 clients give 0; slot 41 gave 28 in second
  70 (the others about 10), 30 in 71 and 3 in 72, so it commanded about 1 s longer than the rest.
- F-NO-PEER (server probe_tests.rs:145-183): wait_peer's Err (server probe.rs:430-435) becomes (1, "leg P0: …") at
  :396-397 before any write, so code 1 and "no leg log" are what the code does; a plant that wrote a log or returned Ok
  fails it. 200 ms wait: fast. Closed.
- P18 rerun (plant-p18b.log): the source check dropped, SEC-U6C-4's decode kept; the stranger (127.0.0.2) now sends
  probe datagrams (server probe_tests.rs:258-267), is pinned, gets the downs, and P0 ends "no round trip in its
  window's first 10 s" (fails :274). The right reason. Closed.
- TE-U6C-R2-3 (Low): SEC-U6C-4's own clause (`ProbeDatagram::decode(b, None).is_some()`, server probe.rs:283) has no
  test that fails without it: P18 tests the source check, and no test sends junk from the peer's IP first.
- NC-U6C-8 (tests/load.rs:285-293): every `probe_peer` field and LG header param needs "`name`" in SERIES.md; each new
  name's only backticked use is its row (SERIES.md:934, :946-948), so removing a row fails it. Closed.
- F-NAMES, steal (server probe_tests.rs:314-321): `!log.seconds.is_empty() &&` closes the vacuous `all`. Closed.
- F-NAMES, last second's index (tests/load.rs:185-193): WT `load_s − 1`; WS `k + 1 < 30`. Closed.
- TE-U6C-R2-4 (Medium): hosts_four's WebTransport case now needs inputs from every client in its last second, which
  lies after the room's close. The room ends 3,000 ms after OPEN (tests/load.rs:139; server main.rs:918-961); LG
  starts after OPEN (load.rs:508) and runs to 4 s (tests/load.rs:234), so second 3 begins at OPEN + 3 s + LG's start
  lag. A client gives commands only up to MAX_LEAD 9 ticks (0.3 s, load.rs:45, :601) past the last tick it heard; A2's
  own log shows the cut (47 of 48 at 0 one second after the room's close). So `whole.iter().all(|v| v[i] > 0)`
  (tests/load.rs:177-184) passes only while LG starts within about 0.3 s (plus P2's 50 ms one way) of OPEN, and a
  change to MAX_LEAD breaks it for an unrelated reason. Introduced by F-NAMES on the WT arm (plus F-5S's 4 s).
- TE-U6C-R2-5 (Low): the WebSocket arm's slice (tests/load.rs:177) needs 3 measured seconds; the room's 3 s gives LG
  seconds 0, 1 and a part second 2, so READY must come inside LG's first second (phase set per record at its write,
  load.rs:679-683). The guard at :171 still asks for 2. The budget for spawn plus four joins went from about 2 s to
  about 1 s, unstated.
- TE-U6C-R2-8 (Low): FP-1's run_leg parts are unpinned: the Measured phase run on to the last record's second (server
  probe.rs:604-609) and `probe_window_ns`'s value (:640, moved from leg_series; the C-U6-5 test's old assert on it went).
  No assert reads a record second's phase or that value, and in the e2e test LG and RS run the same rate, so records
  rarely pass the nominal end. No code reads `probe_window_ns` (grep: only SERIES.md).
- SEC-U6C-3's 404 entry (host serve_tests.rs:210-212): the template is `<dir>/room-NN.json` (:49) and Rust's u8 parse
  takes a leading `+`, so without the digit clause (serve.rs:204) `+5` would serve the written `room-+5.json`; nothing
  before :199 refuses `+`. Bites by reading (no plant run). Closed.
- "room sign" (bots load_tests.rs:45): no other check in load.rs:259-297 refuses "+5", so only the digit clause
  (:266) can; the table asserts is_err (:68-70). Bites by reading. Closed.

## 3. Verdict
APPROVE WITH FINDINGS (one Medium, seven Low; no Critical or High; each becomes a follow-up per the ruling, section 4).

## 4. Findings
| Id | Severity | Where (W) | What goes wrong | Smallest fix | Test or plant that shows it |
|---|---|---|---|---|---|
| TE-U6C-R2-4 | Medium | crates/bots/tests/load.rs:177-184 (with :139, :234; bots load.rs:45, :508) | WT arm: second 3 starts after the room's close (OPEN + 3 s); inputs there come only from the 9-tick lead (0.3 s). If LG starts more than about 0.3 s after OPEN (a loaded gate), or MAX_LEAD drops, "client i gave commands every whole measured second" fails with a last entry of zeros: a flake that blames the clients | Drop the last measured second for both transports: `&measured[1..measured.len() - 1]` (keep the last-second index asserts at :185-193); for WT that leaves seconds 1 and 2, both with the room alive | Test-body plant: sleep 500 ms before spawning npt-load in hosts_four (WT): today's form fails at :180, the fixed form passes |
| TE-U6C-R2-5 | Low | crates/bots/tests/load.rs:171, :177 (room 3,000 ms at :139) | WS arm: `whole` is empty unless READY lands in LG's first second, while :171 still accepts 2 measured seconds; a READY after 1 s fails with "2 measured seconds" | Give the WS room 4,000 to 5,000 ms (WS ends at the close, so +1 to 2 s), or assert `measured.len() >= 3` for WS with that reason | A test-body delay of 1.2 s before the spawn (WS): fails today; passes with the longer room |
| TE-U6C-R2-1 | Low | crates/net/src/l2/probe.rs:721, :725; test probe_tests.rs:288-357 | The t_in sort and the tie clause are pinned by no test (log order = t_in order, no shared t_in). Dropping either passes; at R1 the log is release-ordered (impair.rs:685-696) and up t_in is per receive batch (socket.rs:633-639), so a few boundary records would shift (no mark flips) | One more case in FP-1's test: a Lost down fate logged before an earlier-t_in Delivered one at the boundary, and two up fates sharing the last kept t_in; assert the kept sets | Plants: sort removed; `last[d] != Some(t)` removed: each must fail the new case |
| TE-U6C-R2-2 | Low | crates/server/src/probe_tests.rs:367-375 | FP-3's assert is not shown to bite: plant P2b fails at :358 (header) first. The ruling's "it must fail that assert" is not met; F-P2's defect is caught (by :358) | Run variant P2c: P2b plus one line writing `base` into P1's header `base_rtt_ns` (after server probe.rs:578); restore and hash-check; else the report says the assert is unshown | P2c must fail at :372 ("P1 delivered X down, sent Y", X about twice Y) |
| TE-U6C-R2-3 | Low | crates/server/src/probe.rs:283 | SEC-U6C-4's decode clause: removing it passes every test (P18 covers the source check only) | In the e2e test, a socket on 127.0.0.1 (the peer's IP) sends 40 zero bytes before LG starts (about 6 lines) | Plant: the decode clause dropped: the junk socket is pinned, fails at :274 |
| TE-U6C-R2-6 | Low | crates/bots/tests/load.rs:279-284 | `>= 5_000` has no upper bound: a doubled rate passes, a halved one sits on the bound | `(5_000..=10_000).contains(&n)` (10,000 = PROBE_RATE_PPS × 1 s, the pacer's cap) | Plant: PROBE_RATE_PPS 20,000 must fail |
| TE-U6C-R2-7 | Low | crates/bots/tests/load.rs:364, :371-386; u6c-build.md:236; read-a2-wt.log:22-23 | The A2 WT reading prints `ws_split` Some(all zeros), not None as the notes say, and a "min 0" that spans the 30 s after the room's close; the per-second log shows the real picture (slot 23 at 0 in second 0, slot 41 about 1 s longer than the rest at the end) | Correct the notes line; in the reading, take the min over measured seconds before the room's close, and have U6c's report quote the per-second facts, not the min | Re-read of the same log (no run): seconds 71-99 hold the zeros |
| TE-U6C-R2-8 | Low | crates/server/src/probe.rs:604-609, :640 | FP-1's Measured extension and `probe_window_ns`'s value have no test: setting `measured_end = nominal_end` or writing 0 passes | In the e2e test: LG at 1,900 a second (records pass 1 s), assert every second with probe rows is Measured, and `probe_window_ns == packets × (SEC / rate_pps)` | Plants: `measured_end = nominal_end`; `probe_window_ns` 0: each must fail |

## 5. Each fix-pass item in my scope, by its round-1 id
- NC-U6C-1 (High) / FP-1: closed. The test pins exactly `packets` outside the burst a direction, the burst's fates
  apart, both spans and the rows' last second, with and without a burst; P22 fails it for the right reason (one record
  lost each way). The C-U6-5 and leg_series tests keep the same fates; the e2e bound is `>= packets`. Residuals:
  TE-U6C-R2-1 (Low) and TE-U6C-R2-8 (Low).
- F-P8 = NC-U6C-11 / FP-2: closed. The compiler holds every read after the drop (E0382); it is reported as the ruling
  words it.
- F-P2 / FP-3: closed in substance (P2 in shipped code fails the e2e test, at the header); FP-3's own assert is open as
  unshown (TE-U6C-R2-2, Low; F-P2-HEADER).
- F-TIMING (a): closed (FP-1's bound). (b): closed (catch-up pacer, 5,000 floor), residual TE-U6C-R2-6 (Low). (c):
  closed; P19b still fails the test.
- NC-U6C-2 = F-A2-EXACT / FP-5: closed (one test per name; both read at the final tree). F-READINGS' transport half:
  taken (read-a2-wt.log); its prints: TE-U6C-R2-7 (Low).
- F-NO-PEER: closed. SEC-U6C-4's P18 rerun: closed (fails :274 for the right reason); the clause itself: TE-U6C-R2-3.
- NC-U6C-8 = F-SERIES-LOAD (the cross-check): closed.
- F-NAMES: the steal check closed; the last second's index closed; "every whole measured second" open: TE-U6C-R2-4
  (Medium, WT) and TE-U6C-R2-5 (Low, WS).
- F-5S: built (4 s); it makes WT's last second the post-close one, part of TE-U6C-R2-4.
- SEC-U6C-3 = F-HASH-PLUS: the 404 entry and "room sign" closed (both bite by reading; no plant run).
- The counts: tie (861 = 859 + 2, 958 = 956 + 2; the hashes equal from 13:27 to 18:01, run r2-test-1).

## 6. Outside the diff
- Low, server probe.rs:430-435: wait_peer's message prints whole seconds, so F-NO-PEER's 200 ms reads "within 0 s".
  Taker: after R1 (U6d), wording only (R1's waits are whole seconds).
- Low, tests/load.rs:21-28 and server probe_tests.rs:149-153, :226-229: the free-port pattern (bind, read, drop, rebind)
  can race another process for the port; tiny odds, same pattern as round 1's. Taker: after R1 (U6d or U6e).

## 7. What I did not check
- I ran no test and no plant; one runner call (`true`) only printed W's hashes. Every plant result is the builder's log.
- TE-U6C-R2-4's margin (about 0.3 s plus P2's one-way delay) is reasoned from MAX_LEAD, the room's end and A2's log;
  no log here holds LG's start lag against the room's OPEN (their clocks are separate), so it is an estimate.
- SEC-U6C-3's 404 entry and "room sign" bite by reading only (no plant was asked for or run).
- I did not read serve.rs's accept sleep (SEC-U6C-1), NC-U6C-3, NC-U6C-5, NC-U6C-6 or FP-1's logic beyond what the tests
  touch: the netcode and security seats'. I did not read U8's snapshot or marks.json.
- The test step's +59.5 s against round 1 is not root-caused (not the two new tests).
- Slot 41's longer run in the A2 WT log is described, not explained.

DONE
