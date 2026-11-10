# U4a + U4b review, round 2 (test-engineer), on what changed only

Started 2026-11-10 (after a usage-limit reset; no earlier round-2 test notes existed, so a fresh start).
U4a: .claude/worktrees/agent-a9297644e7c0354f3 vs .gate/u4a-prefix. U4b: .gate/u4b-wt vs .gate/u4b-prefix.
Only plants in place (one at a time, restored, sha256-checked). No commit. Build rules: plan §7.1 (shared target,
offline, touch crates/ before and every 2 s during a counted run, wait for no other cargo/rustc, df before each build).
No Edit tool in this run: plants by Write of the whole file, restored by Write (or cp from a scratch copy), sha256.

## Plan
1. sha256 of every file under spikes/netcode-proof/crates (+ check.mjs, check.test.mjs) in both copies, before anything.
2. diff -ru both pairs; read each changed hunk; map to round-1 ids (T-A1, T-A6, T-A7, nit, N2; T-B1, T-B2, T-B3).
3. Judge each closure from the test's power: which tests are new, which lack a plant (fixer planted PF2, PF4; main
   session planted the T-B1 "- 1"); plant one per new test without a plant, show it failing by name, restore, sha256.
   Candidates: U4a msg a3 send half (send_limit -> receive_limit), full-state 65,536 test, wire debug test (drop the
   redaction), wire a2's as_sent; U4b carry-6 test via due_ticks (literal cap).
4. Look for defects added by the fix (tests that can't fail, negative cases missing, the rust-reviewer's R2-L1).
5. check.mjs once per copy, counted runs (U4b expected measured 286 = 275 + 11, bites 309 = 292 + 17; U4a: record).
6. Final sha256 of both copies = step 1; git status file sets unchanged.

## Progress
