# U6c round 2, test seat (test-engineer, xhigh), on the fix pass's diff only. Read only.
Inputs, sha256 checked equal (2026-11-24): advisor-u6c-r1.md ad72c0b8…faed179, u6c-review-netcode.md f29e1ac1…,
u6c-review-test.md 79197b42…, u6c-review-security.md c29540fa…, u6c-build.md a9d45dd4…, wip/u6c-fix.patch 0549389e….
W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3a3e3fdf559de095/spikes/netcode-proof.

## 1. Plan
1. Read the diff (aad21031 → 6c3ff00b) file by file: net l2/probe.rs and probe_tests.rs; server probe.rs and
   probe_tests.rs; bots tests/load.rs, src/load.rs, load_tests.rs; host serve.rs, serve_tests.rs; SERIES.md.
2. FP-1: read the new `leg_series` and the FP-1 test; check it pins exactly `packets` outside bursts, burst fates apart,
   ties at the last kept t_in, both spans; with and without a burst. Read plant-p22.log for the failure's reason. Check
   C-U6-5 and leg_series tests' new calls and the e2e bound.
3. FP-2: read `edge_events`' new signature and both calls; check every read after `drop(sock)`; plant-p8/p8c logs.
4. FP-3 / F-P2-HEADER: read the new assert and plant-p2b.log; judge whether the assert is shown to bite; size a variant.
5. FP-4: (b) and (c), plant-p19b.log; timing dependence.
6. FP-5: fp5-list.log, read-a2-ws.log, read-a2-wt.log.
7. FP-6 tests: F-NO-PEER, P18 (plant-p18b.log), NC-U6C-8 cross-check, F-NAMES, F-5S, SEC-U6C-3 404 entry, "room sign".
8. Counts: fix-check.log vs the snapshot hashes.
Run nothing unless a claim needs it (name each run here).

## 2. Notes (one line per finding)
- Run r2-test-1 (.gate/u6c/r2-test-1.log, 18:01:32 UTC): runner, shared lock, command `true`, only to print W's
  hashes: diff 0e3199ae…, untracked 22bda910…, equal to fix-check.log's (13:27), read-a2-ws/-wt (14:08, 14:10) and
  read-probe (14:29). So fix-check's tree is W's at 18:01, the tree the 17:5x patch was made from. Counts tie.
- FP-1 test read (net probe_tests.rs:288-357): up t_in strictly increasing (100 µs + i/2 ns), down strictly increasing
  (last three +60 µs, no reorder); log order per direction = t_in order; no two fates share a t_in. It pins: exactly N
  outside the burst a direction (:331), the burst's fates apart (:336), both spans (:341-348), first and last second
  (:349-355), with and without the B1 burst. It does not pin: t_in order against log order, or the tie clause.
- P22 (plant-p22.log): break at t_in >= start + packets × interval; fails :331 with (119,999, 119,999), exactly the
  one record each arm (5 ppm up, late pacer down) puts past the clock span: the right reason. Untracked hash 79241c8a
  (net probe.rs is a new file, so the plant shows there). OK.
- TE-U6C-R2-1 (Low): t_in order and the tie clause unpinned. Live at R1: Delivered fates are logged at release
  (impair.rs:685-696), drops at intake (impair.rs:576-579), so the log is not in t_in order; up t_in is one clock read
  per receive batch (socket.rs:633-639), so ties are real. A plant dropping the sort (probe.rs:721) or the tie clause
  (`last != Some(t)`, probe.rs:725) passes all three leg_series tests. No mark flips (each direction still keeps
  >= packets), the E set shifts by a few boundary records. Fix: one case in FP-1's test with a Lost down fate logged
  before an earlier Delivered one at the boundary, and two up fates sharing the last kept t_in; about 20 lines.
- C-U6-5 test (packets 1,002) keeps the same 1,002 up fates as [10 s, 20 s) (1,000 + Oversize at 11 s + Lost at 12 s);
  leg_series test (packets 4) keeps all 5 down (4 outside + 1 in burst) and the 4 peer ups, one second: the same fates.
  Both pass at any packets >= their fate counts, so they pin "the same fates", not the cut (FP-1's test does). OK.
- FP-2: `edge_events(_bound: &ImpairSocket, …)` (server probe.rs:179), calls :465 and :553 pass `&sock`; after
  `drop(sock)` (:571) nothing at :572-657 reads the kernel (header, params, seconds, summary are values taken before).
  plant-p8c.log: E0382 at :571 "value borrowed here after move". Notes line 278-279 word it as the ruling does. OK.
