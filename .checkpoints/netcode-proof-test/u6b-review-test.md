# U6b review, the test seat (test-engineer, xhigh), 2026-11-23

Inputs checked by sha256 at start: brief 32baaebd... OK; advisor check 3c0f8121... OK; snapshot patch 49f4488f... OK;
builder notes be6807f4... OK. W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a2661dc320c9e6d00/spikes/netcode-proof.
No test-engineer memory folder exists (.claude/agent-memory-local/test-engineer absent). This agent has no Edit tool:
the notes file was rewritten whole with Write at each save. No edit in the worktree, no plant, nothing cleaned.

## 1. Plan
1. Read the advisor's check (conditions bearing on tests: 1, 3, 5, 7, 8) and the builder's notes (Plan, items 1-6,
   plant table, counts).
2. Section 3's named tests, block by block: list each test the brief names, find it in W (grep the patch's test names),
   read its body, judge whether it can fail for the claim it is named for (negative cases, injected clock, no timing bar).
3. Plants: for each of P1-P11, P14-P16, read .gate/u6b/plant-p*.log: the named test failing, the failure's reason
   matching the plant, the restore and its sha256 against the tree.
4. The bite PM-0001-18/ws-budget-x2: `with_bite`, `Bite::BudgetX2`, the two bites tests; per-instance arming by id, the
   other instance at the formula, the armed instance failing test (c)'s bound; compiled out of run binaries (SEC-U6BD-2).
5. Counts: section 3's line as rev 1.1 condition 5 restates it (+45 measured, +47 bites) against check5.log and base
   (754, 850).
6. Test step time 891.5 s vs base 954.8 s (1.5x stop).
7. End-to-end tests over WtClient and WsClient at P2 (tokens, snapshots, no kind-10).
8. Timing: the two 20-iteration kick tests under the 5 s cap; F-P14's note; the proc_read wait; flake risk.
9. F28's test (npt-super log flushed per record).
10. Container readings: what their record must show.
11. Run a test only where a claim needs it, through u6b-run.sh, logs .gate/u6b/rev-test-<n>.log.

## 2. Notes (one line per finding, saved as I go)
- N0 (provenance, holds): base 21765e3's 29 files plus the patch, rebuilt in scratch (snapcheck.sh in my scratchpad),
  equal W's byte for byte (0 of 29 differ). The four plant files' sha256 in W equal the builder's restore hashes
  (ws_server 346af9fe..ce26, wt_server a9872ed6..8139, link 5f9fcbd8..b6, main 8215ef58..63fd). check5.log's `diff`
  hash d4ad43fd.. equals sha256 of the snapshot's spikes/ part (patch lines 17 to the end) exactly, and its untracked
  hash is the empty set's: the counted run ran on the snapshot. clippy4.log and my seven runs (rev-test-1..7) show the
  same hash.
- N1 (counts, hold): check5 PASS all 11 steps; measured 799 (+45 on 754), bites 898 (+48 on 850), 0 failed, 0 skips;
  test step 891.5 s vs base 954.8 s = 0.93x (stop at 1.5x). Rev 1.1 asks at least +45 and +47: met. By the source: 46
  new non-bite tests, of which `b2_both_features_on_give_web_transport_quinn_on_ring` is all-features only (+45
  measured); the bites build adds it and the two pm_0001_18 tests (cfg(feature = "bites")): +48.
- N2 (plants, hold as far as their logs go): all 14 logs (P1-P11, P14-P16) run one filtered test under -x, exit 101,
  and the named test FAILED at an assertion whose message is the planted reason (P1 ws_server_tests.rs:153 "the permit
  came back"; P2 :218 [9000,9000] vs [1000,1000]; P3 wt_server_tests.rs:269 Elapsed; P4 ws_server_tests.rs:254
  Elapsed; P5 wt :328 16 of 20; P6 :420 1 of 20; P7 :232 one_source (513 vs 400); P8 :564 20 vs 5; P9 :585 BufferFull
  vs Sent; P10 :927 120,004 vs 143,909; P11 :497 close_failed_count 1; P14 binary.rs:235 (inside hash_of) NotFound;
  P15 binary.rs:325 room.key; P16 :616 Sent vs BufferFull). Each plant log's diff hash differs from every clean run's,
  and the four files are back at their hashes.
- N3 (plants, Low): the planted edits are recorded nowhere (no diff, no plant script; earlier units kept .py plant
  scripts): each plant's content rests on the builder's one-line description and the failure message. The plants ran
  16:42-16:48 on the tree before proc_read.rs's two later test edits, which touch no plant file.
- N4 (bite, High): neither bite test goes through an armed WsServer's accepted connection.
  pm_..._is_armed_by_id_per_instance (ws_server_tests.rs:825-841) checks two instances' `bite` field and a hand-made
  `TickWriter`; pm_..._fails_test_c_bound (:847-859) sets the writer on a hand-built WsLink (writer_run_through_ws_link
  :634-637), never `with_bite`. The code carrying the instance's bite into each accepted link (ws_server.rs:372,
  :494-501) and the binary's arming (main.rs:573-579) run in no test: deleting :494-501 passes both. rev-test-4: both
  pass, "2 of 2 returns above N x size", 6.15 s.
- N5 (stamp, Medium): b1_arrival_stamps (ws_server_tests.rs:196-219) holds the injected clock at 500 through the whole
  handshake, so `accept_ns == 500` (:207) cannot tell an accept stamp at the TCP accept (ws_server.rs:432) from one
  taken after TLS or the upgrade; no test covers WtServer's stamp at the first QUIC packet (wt_server.rs:202).
- N6 (flake, Medium): the same test syncs on a real-clock `sleep(300 ms)` (:212) before moving the clock to 9,000:
  slower loopback delivery under check.mjs's parallel load fails it (stamps [] or 9,000).
- N7 (5 s cap, Medium): b4_the_writer_keeps_outstanding_within_n_snapshots_through_ws_link took 6.54 s alone
  (rev-test-3) and the two bite tests 6.15 s (rev-test-4): two 1 s bursts at 1.5 s and 4.5 s (:703, :849) put both past
  section 3's "real-clock tests at most 5 s each" and the module's own text (ws_server_tests.rs:2-3).
- N8 (kick tests, hold, with a Low note): WT kick 2.50 s alone (rev-test-1), WS kick 0.14 s (rev-test-2): both under
  5 s. The WT test's setup is sequential (20 sends then 20 receives, each limited at 5 s: wt_server_tests.rs:301-315),
  so its limits sum far past 5 s; only the reads run concurrently. Its planted run took 17.73 s (plant-p5.log).
- N9 (reader stamp, Medium): the identity on the reader parts telescopes: pass + reader = (R - P) + ((3) - R) = (3) - P
  for any R in [P, (3)], both in the code's own check (delay.rs:731-735) and in the test's `reader_split`
  (delay_tests.rs:3114). So `ws_late_up_split_violations_count` cannot catch a misplaced R inside the window, and R1's
  "violations 0" says nothing about R. Only the test-only `reader_wakes` membership check bites.
- N10 (bite variant, Low): `SplitBite::ReaderAtRelease` (delay.rs:775) also skips the `reader_wakes` push (:784), so
  `reader_split(bitten)` fails on an empty wake list, not only on R's value: the bite differs from the faithful path in
  two ways.
- N11 (mark, Medium): no test calls `Pool::mark` or `DelayStream::mark_split` (only main.rs:640): condition 3's third
  test ("the identity exact across the mark" for the split's parts and pooled L together) is missing; stats.rs's
  u6b_the_mark_... tests one `Pooled` only. Pool::mark (delay.rs:277-300) marks each histogram in turn, so a release
  during the mark can land on different sides for parts and L: "exact" holds only with no traffic.
- N12 (OPEN, Medium): P14 failed only because the test read the hash file before Net::bind wrote it (binary.rs:282 into
  hash_of :235), a race the test won (F-P14). The test cannot tell "bound" from "accepting": TCP listens from Net::bind
  (main.rs:563-572), QUIC packets queue, so OPEN moved between Net::bind (main.rs:715) and net.serve (:767) passes it,
  and the clients' 4 s poll loop (binary.rs:298) absorbs any delay.
- N13 (e2e hang, Medium): u6b_clients_over_both_transports (tick_loop/tests.rs:1543) awaits both accepts with no
  timeout: a WebTransport handshake that misses the 2 s deadline under P2's 2 % loss is counted and dropped
  (wt_server.rs:227-231) and accept() never returns, so the test step hangs. The Open wait after it (:1551) has no
  assert, and the Join deadline runs from accept_ns, so a slow handshake also fails "the session stays". rev-test-5:
  passed in 3.81 s, 96 and 99 snapshots, 0 kind-10 (room.rs:167-169: `buffer_full` is kind 10).
- N14 (condition 7, Medium): b3_a_largest_full_state_reaches_the_client (ws_server_tests.rs:457-472) sends 4 + 65,536 =
  65,540 B, but the largest FullState is 65,536 + 5 B = 65,541 B (catalogue :76; FULL_STATE_MAX 65,541 at
  wt_server.rs:46). One byte short: a client limit one too small (65,541 for the tagged 65,542) passes it.
- N15 (WT permit, Low): no test returns a permit after an established WebTransport session's closing task (b1_one_cap
  drops a WebSocket session only, :151); b2_one_failure covers the failed-handshake return only (wt_server_tests.rs:243).
- N16 (QUIC deadline, Low): the deadline test covers the HTTP/3 stage only; a client stalled inside QUIC's handshake is
  not tested (the brief: "the deadline covers QUIC and HTTP/3").
- N17 (pin, Low): no negative for "the handshake's signature still checked" (pin.rs:58-70): a server holding the pinned
  certificate with another key is not tried. Security's area too.
- N18 (C1c, Low): b4_a_close_error (ws_server_tests.rs:478-498) runs on the plain path (`L2Config::clean(12)`, :480);
  RC-11 is DelayStream's shutdown. The classification (ws_server.rs:852) is path-independent, so the risk is small; the
  positive case (a close failure with the peer still there counted) is not asserted.
- N19 (0644, Low): cert.rs:141 `.mode(0o644)` is masked by the process umask; the test's `mode == 0o644` (:238) passes
  only under umask 022 or looser. Under U8's service unit with a stricter UMask the file is 0600 or 0640.
- N20 (early class, Low): the early-reader class is counted (pool.early) but asserted nowhere: reader_split counts n[2]
  and the tests only print it (delay_tests.rs:3131-3145, ws_server_tests.rs:808-818).
- N21 (F28, Low): the test (super/src/main.rs, u6b_each_record_...) reads like a real regression test (npt-log's Writer
  flushes only in finish: log/src/lib.rs:587-593, :602-605, so a BufWriter would keep room_start off disk), but no
  plant shows it failing on the old BufWriter.
- N22 (proc_read, holds): check3 failed once on it (measured build only); the builder's root cause (join returns at the
  tid futex's clear in exit_mm, before release_task removes /proc/self/task/<tid>) matches Linux's exit path; the fixed
  wait is a bounded count (5,000 x 1 ms), no clock read. Passed since in clippy4.log and in both check5 builds (3
  runs). Not retried into green: root-caused. Low residual risk (a traced thread is not reaped at once).
- N23 (count composition, Low): the +45 includes unnamed tests (link default, pin's SHA-256 vector, config's
  `named`, added "for the +45" per the notes, proc_read, F28, the outside-a-tick datagram) while condition 3's third
  named test is missing (N11).
- N24 (binary test ports, Low): free_port (binary.rs, `fn free_port`) binds and releases a port the room binds next: a
  parallel test can take it between (small chance).
- N25 (other runs): connector stamp test 3.14 s (rev-test-6); b5 loopback self-test 4.59 s alone, 723 woken up
  releases with all four parts over 8 connections (rev-test-7): under the cap, near it.

## 3. Verdict: REVISE
One High (the bite's evidence) and eight Mediums. All are test changes, each small, and they fit the planned fix pass
before the merge (beside the container readings). The High closes with a test that arms the bite through
`with_bite` on a WsServer and runs (c) on a link that instance accepted, plus a plant that removes ws_server.rs:494-501
and fails it. What holds: provenance (the counted run is the snapshot), the counts (+45 and +48 against +45 and +47),
the time (0.93x), the fourteen plants as their logs show them, both kick tests under 5 s, the end-to-end test over
both transports at P2, and F28's fix.

## 4. Findings table
| ID | Sev | Where (W) | Finding | Smallest fix |
|---|---|---|---|---|
| T-1 | High | crates/net/src/ws_server_tests.rs:825-859; ws_server.rs:372, :494-501; server/src/main.rs:573-579 | The bite PM-0001-18/ws-budget-x2 is never armed through an instance's accepted connection: one test reads the `bite` field and a hand-made TickWriter, the other hand-sets the writer on a hand-built WsLink. Deleting :494-501 passes both, so "armed by id per instance, the armed instance fails (c)" is not proven. | In the (c) bite test take the link from `WsServer::bind(..P2..).with_bite(WS_BUDGET_BITE)` after start, connect and accept; in the per-instance test accept one connection on each instance and assert 2x formula vs formula on the accepted links; a plant removing :494-501 fails both. |
| T-2 | Medium | crates/net/src/ws_server_tests.rs:196-219 (:207); ws_server.rs:432; wt_server.rs:202 | The accept stamp at the TCP accept (or first QUIC packet) cannot fail its test: the injected clock stays 500 through the handshake. | Move the injected clock between the TCP accept and the upgrade (raw TCP, wait for the permit, set 700, finish TLS and the upgrade) and assert 500; the same on WebTransport between the first packet and the answer. |
| T-3 | Medium | crates/net/src/ws_server_tests.rs:212 | Timing-dependent: a 300 ms real sleep stands in for "both messages arrived" before the clock moves to 9,000. | Wait (bounded) on an arrivals count, or until the inbox holds two messages, then move the clock. |
| T-4 | Medium | crates/net/src/ws_server_tests.rs:2-3, :703, :849 | Two tests break section 3's 5 s real-clock cap: (c) through WsLink 6.54 s, the bite's (c) about 6 s (bursts at 1.5 s and 4.5 s). | Bursts at 0.5 s and 2.5 s (or one burst), or state the exception and its reason in the module text and the report. |
| T-5 | Medium | crates/net/src/l2/delay.rs:731-735; delay_tests.rs:3114 | The reader-stamp identity telescopes ((R - P) + ((3) - R) = (3) - P): the counted violations and the "identity exact" check cannot see a misplaced R inside [P, (3)]; only the test-only `reader_wakes` membership bites. | Say so in the report and in R1.md's reading (the identity is a range check for R); in the loopback self-test assert each R is the first closing wake after its release. |
| T-6 | Medium | crates/net/src/l2/delay.rs:277-300, :1316; stats.rs (u6b_the_mark_...) | Condition 3's third test is missing: nothing marks the whole pool and checks that the parts and pooled L share one window; the per-histogram mark is not one cut under traffic. | A test on a local `Pool` (not the static): samples, mark, samples; assert each window's n and that the parts sum to L; state the in-flight skew bound in the report. |
| T-7 | Medium | crates/server/tests/binary.rs:282, :298; server/src/main.rs:715, :767, :772 | "OPEN only after both listeners accept" is not proven: P14 failed on the hash file's absence, a race the test won (F-P14); OPEN moved between Net::bind and serve (bound, not accepting) would pass. | Log each listener's serving stamp and OPEN's on one clock and assert serving <= OPEN; re-plant P14 between Net::bind and serve. |
| T-8 | Medium | crates/server/src/tick_loop/tests.rs:1543, :1551 | The e2e test awaits both accepts with no timeout: a handshake dropped at its 2 s deadline under 2 % loss hangs the test step; the Open wait has no assert. | `tokio::time::timeout(5 s, ...)` on each accept with an expect; assert both links Open after the wait. |
| T-9 | Medium | crates/net/src/ws_server_tests.rs:457-472; wt_server.rs:46 | Condition 7's test sends 65,540 B, 1 B under the largest FullState (65,541 B): an off-by-one client limit passes. | Send FULL_STATE_MAX bytes and assert them received; optionally one byte more refused. |
| T-10 | Low | .gate/u6b/plant-p*.log | The planted edits are not recorded (only a diff hash). | Save each plant's `git diff` beside its log in the fix pass. |
| T-11 | Low | crates/net/src/wt_server_tests.rs:301-315 | The WT kick test's setup is sequential with 5 s limits per step (2.50 s alone; 17.73 s planted). | Run the per-client send and receive steps concurrently, as the reads are. |
| T-12 | Low | crates/net/src/l2/delay.rs:775, :784 | The ReaderAtRelease bite also skips the `reader_wakes` push, so part of its catch is self-made. | Push `r` in the bite path too. |
| T-13 | Low | crates/net/src/ws_server_tests.rs:151 | No test returns a permit after an established WebTransport session's closing task. | Drop a held WT session in b1_one_cap too and wait for the permit. |
| T-14 | Low | crates/net/src/wt_server_tests.rs:250-275 | The deadline is tested on the HTTP/3 stage only, not inside QUIC's handshake. | A client that sends an Initial and then nothing, closed and counted at the deadline. |
| T-15 | Low | crates/net/src/pin.rs:58-70 | No negative for the signature check behind the pin. | A server presenting the pinned certificate with another key is refused (security's read). |
| T-16 | Low | crates/net/src/ws_server_tests.rs:478-498 | C1c is tested on the plain path only, and the counted case (peer still there) is not asserted. | Run it on DelayStream too, and assert a close failure with the peer present counts 1. |
| T-17 | Low | crates/server/src/cert.rs:141, :238 | 0644 depends on the umask; under a stricter UMask the hash file is not 0644 and the test fails. | Name the umask in U8's unit (UMask=0022) and in the report, or assert mode = 0644 & !umask. |
| T-18 | Low | crates/net/src/l2/delay_tests.rs:3131-3145 | The early-reader class is counted but asserted nowhere. | Assert n[2] > 0 on a session that has app-step releases, and that the pool's early equals it. |
| T-19 | Low | crates/super/src/main.rs (u6b_each_record_...) | F28's test has no plant showing it fails on the old BufWriter. | One plant: BufWriter back, the test fails. |
| T-20 | Low | (count line) | The +45 is reached partly with unnamed tests (one added "for the +45") while a named one (T-6) is missing. | Count T-6's test in; report the named-vs-extra split. |
| T-21 | Low | crates/server/tests/binary.rs (`fn free_port`) | Bind-and-release ports can be taken by a parallel test before the room binds them. | Bind port 0 in the room and print the bound ports before OPEN, or retry on EADDRINUSE with the reason. |

Counts: Critical 0, High 1, Medium 8, Low 12.

## 5. What the container readings' record must show (fix pass, before the merge, before Stop B's read)
- **Provenance for all three:** the tree's diff hash (as u6b-run.sh prints it) equal to the merged head's, the build
  profile (release), the features (default only, no `bites`: SEC-U6BD-2), the binary's sha256, the machine (vCPUs,
  model), the free disk and the time, and each command line, in a log under .gate/u6b/.
- **20 starts to OPEN:** npt-super `time-starts` with count 20 (or 20 single starts), `transport` both, both listeners
  bound, the dev certificate made each time; each start with a fresh token file, log and hash path (create_new refuses
  an existing one: exit 2); every `room_start` (spawn_to_open_ns) and `room_exit` (nonzero 0); p50 and max of the 20;
  any start that failed, named; the line that these replace U6a's lower bound; and, for at least one start, a WebSocket
  and a WebTransport client opened at once after OPEN (until T-7's ordering check exists).
- **The 48-connection WsServer example at P2 for 60 s:** the example's path under crates/server/examples/ and no
  manifest change; P2 and the seed; 48 WsClient connections, each with its reader on split halves, a 1,023 B datagram
  down a tick through fix 1 and an up message a tick; ws_accept's counts (accepted 48; failed, deadline and refused 0);
  the cap (N + 8 = 56); per direction n (each at least 10,000), the four parts (woken: clock, hop, pass, reader; other:
  wait, pass, reader) with p50, p99, p99.9, max and E; the early-reader count; the violations per direction, with T-5's
  note that they cannot see a misplaced R; pooled L in both forms (L0 = 0 and the median form) with their over counts;
  whether the mark was taken and when; steal from /proc/stat before and after, and the load; "reported, never judged".
- **The memory bound at N = 48:** a measurement, not the builder's arithmetic: VmRSS or VmHWM (/proc/<pid>/status)
  before any connection and at the peak with 56 DelayStream connections (the cap full, the 57th refused and counted),
  per-connection bytes from the difference, against (N + 8) x about 0.31 MB plus the fixed part (ImpairSocket's pool
  12.3 MB, pooled L 270 KB, the mark 1.62 MB once taken, ws_split_fixed_bytes), with the method.

## 6. What I did not verify
- The plants' contents: they are not recorded (T-10); I judged them from their failure messages and the notes.
- Flake rates: each test I ran ran once, alone (rev-test-1..7), not under check.mjs's parallel load; per-test times
  under load are unknown. T-3 and T-8 are judged by reading, not seen failing.
- That F28's test fails on the old BufWriter (no plant; read only) and that T-1's propagation code works (read only:
  :494-501 reads right).
- SERIES.md's rows, the header's full field list, the pub items, fix 1's model bytes: the netcode seat's. The accept
  paths' security, the dev key, the token file: the security seat's.
- No browser, no RS, no netem, no release build, no container reading (section 5 above).

DONE
