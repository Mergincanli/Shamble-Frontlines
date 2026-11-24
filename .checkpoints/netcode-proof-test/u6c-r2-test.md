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
