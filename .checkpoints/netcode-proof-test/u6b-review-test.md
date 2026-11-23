# U6b review, the test seat (test-engineer, xhigh), 2026-11-23

Inputs checked by sha256 at start: brief 32baaebd... OK; advisor check 3c0f8121... OK; snapshot patch 49f4488f... OK;
builder notes be6807f4... OK. W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a2661dc320c9e6d00/spikes/netcode-proof.
No test-engineer memory folder exists (.claude/agent-memory-local/test-engineer absent). This agent has no Edit tool:
the notes file is rewritten whole with Write at each save.

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
- N0 (provenance, holds): base 21765e3's 29 files plus the patch, rebuilt in scratch (snapcheck.sh), equal W's byte for
  byte (0 of 29 differ). The four plant files' sha256 in W equal the builder's restore hashes (ws_server 346af9fe..ce26,
  wt_server a9872ed6..8139, link 5f9fcbd8..b6, main 8215ef58..63fd). check5.log's `diff` hash d4ad43fd.. equals sha256
  of the snapshot's spikes/ part (patch lines 17-end) exactly, and its untracked hash is the empty set's: the counted
  run ran on the snapshot. clippy4.log has the same hash. My four runs (rev-test-1..4) show the same hash.
- N1 (counts, hold): check5 PASS all 11 steps; measured 799 (+45 on 754), bites 898 (+48 on 850), 0 failed, 0 skips;
  test step 891.5 s vs base 954.8 s = 0.93x (stop is 1.5x). Rev 1.1 asks at least +45 and +47: met. By the source: 46
  new non-bite tests, of which `b2_both_features_on_give_web_transport_quinn_on_ring` is all-features only (+45
  measured); bites adds it and the two pm_0001_18 tests (cfg(feature = "bites")): +48.
- N2 (plants, hold as far as their logs go): all 14 logs (P1-P11, P14-P16) run one filtered test under -x, exit 101,
  the named test FAILED at an assertion whose message is the planted reason (P1 ws_server_tests.rs:153 "the permit came
  back"; P2 :218 [9000,9000] vs [1000,1000]; P3 wt_server_tests.rs:269 Elapsed; P4 ws_server_tests.rs:254 Elapsed; P5
  wt :328 16 of 20; P6 :420 1 of 20; P7 :232 one_source (513 vs 400); P8 :564 20 vs 5; P9 :585 BufferFull vs Sent; P10
  :927 120,004 vs 143,909; P11 :497 close_failed_count 1; P14 binary.rs:235 hash file NotFound; P15 binary.rs:325
  room.key; P16 :616 Sent vs BufferFull). Each plant log's diff hash differs from every clean run's, and the four files
  are back at their hashes.
- N3 (plants, Low): the planted edits are recorded nowhere (no diff, no plant script; earlier units kept .py plant
  scripts): each plant's content rests on the builder's one-line description and the failure message. The plants ran
  16:42-16:48 on the tree before proc_read.rs's two later test edits, which touch no plant file.
- N4 (bite, High): neither bite test goes through an armed WsServer's accepted connection.
  pm_..._is_armed_by_id_per_instance (ws_server_tests.rs:825-841) checks two instances' `bite` field and a hand-made
  `TickWriter`; pm_..._fails_test_c_bound (:847-859) sets the writer on a hand-built WsLink (writer_run_through_ws_link
  :634-637), never `with_bite`. The code carrying the instance's bite into each accepted link (ws_server.rs:372,
  :494-501) and the binary's arming (main.rs:573-579) run in no test: deleting :494-501 passes both. Measured: rev-test-4
  (both pass; "2 of 2 returns above N x size", 6.15 s).
- N5 (stamp, Medium): b1_arrival_stamps (ws_server_tests.rs:196-219) holds the injected clock at 500 through the whole
  handshake, so `accept_ns == 500` cannot tell an accept stamp at the TCP accept (ws_server.rs:432) from one taken after
  TLS or the upgrade; no test covers WtServer's stamp at the first QUIC packet.
- N6 (flake, Medium): the same test syncs on a real-clock `sleep(300 ms)` (:212) before moving the clock to 9,000:
  slower loopback delivery under check.mjs's parallel load fails it (stamps [] or 9,000).
- N7 (5 s cap, Medium): b4_the_writer_keeps_outstanding_within_n_snapshots_through_ws_link took 6.54 s alone
  (rev-test-3) and pm_0001_18 (c) about 6 s (rev-test-4: 6.15 s for both bite tests): two 1 s bursts at 1.5 s and 4.5 s
  (:703, :849) put both past section 3's "real-clock tests at most 5 s each" and the module's own text (:2-3).
- N8 (kick tests, hold with a note): WT kick 2.50 s alone (rev-test-1), WS kick 0.14 s (rev-test-2): both under 5 s.
  The WT test's setup is sequential (20 sends then 20 receives, each limited at 5 s: wt_server_tests.rs:301-315), so its
  limits sum far past 5 s; only the reads run concurrently. Low.
- N9 (reader stamp, Medium): the identity on the reader parts telescopes: pass + reader = (R - P) + ((3) - R) = (3) - P
  for any R in [P, (3)], both in the code's own check (delay.rs, the "RC-12, up" block in Conn's release: `ok &=
  parts[0] + pass + reader + hop == total && step <= r && r <= at`) and in the test's `reader_split`
  (delay_tests.rs, `(r - l.step) + (l.at - r)`). So `ws_late_up_split_violations_count` cannot catch a misplaced R inside
  the window, and R1's "violations 0" says nothing about R. Only the test-only `reader_wakes` membership check bites.
- N10 (bite variant, Low): `SplitBite::ReaderAtRelease` (delay.rs stamp_reader) also skips the `reader_wakes` push, so
  `reader_split(bitten)` fails on an empty wake list, not only on R's value: the bite differs from the faithful path in
  two ways.
- N11 (mark, Medium): no test calls `Pool::mark` or `DelayStream::mark_split` (only main.rs:640): condition 3's third
  test ("the identity exact across the mark" for the split's parts and pooled L together) is missing; stats.rs's
  u6b_the_mark_... tests one `Pooled` only. Pool::mark (delay.rs:277-300) marks each histogram in turn, so a release
  during the mark can land on different sides for parts and L: "exact" holds only with no traffic.
- N12 (OPEN, Medium): P14 failed only because the test read the hash file before Net::bind wrote it (binary.rs:235), a
  race the test won (F-P14). The test cannot tell "bound" from "accepting": TCP is listening from Net::bind
  (main.rs:563-572), QUIC packets queue, so OPEN moved between Net::bind (:715) and net.serve (:767) passes it, and the
  clients' 4 s poll loop (binary.rs, `for _ in 0..4_000`) absorbs any delay.
- N13 (e2e hang, Medium): u6b_clients_over_both_transports (tick_loop/tests.rs, `rt.block_on(async {
  (wt.accept().await, ws.accept().await) })`) has no timeout: a WebTransport handshake that misses the 2 s deadline
  under P2's 2 % loss is dropped by the server and accept() never returns, so the test step hangs. The Open wait after
  it has no assert, and the Join deadline runs from accept_ns, so a slow handshake also fails "the session stays".
