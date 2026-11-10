# U4a + U4b review, round 1 (test-engineer)

Started 2026-11-10. U4a: .claude/worktrees/agent-a9297644e7c0354f3 (base dffef0e); U4b: .gate/u4b-wt (base 4b5812a).
Only plants in place (one at a time, restored, sha256-checked). No commit. Build rules: plan §7.1 (shared target,
offline, touch crates/ before and every 2 s during a counted run, wait for no other cargo/rustc, df before each build).
Run helper: scratchpad te-run.sh (waits 8 s idle, df, touches, toucher every 2 s inside the same command, stops it).

## Plan
1. sha256 of every changed file in both copies (U4b's four given; U4a's against its notes' baseline).
2. Read each plan test and bite (§5 U4a a1-a9, catalogue items 1/2/3/6, carry 4; §5 U4b's six scenarios + 3 bites)
   and check each tests what it claims. For doubtful power: plant in place, show the named test failing, restore,
   sha256. Findings a probe can settle without touching a copy (N1, SEC-2, SEC-5's tally, SEC-6/N6, S1's width) go
   through one scratch probe crate under .gate/te-probe (a path dependency on U4a's copy; nothing written in the copy).
3. Carries U4a 1-5, 34; U4b 6, 35 against code and tests.
4. check.mjs once per copy at the end, fresh touch; record counts vs deltas (U4a 296/321/52, U4b 285/308/48).
5. Final sha256 of both copies = step 1.

## Progress
- Step 1 done (both copies, before any plant). U4a, 18 files, every one = the builder's baseline (first 16 hex):
  bits 4a390bdd0679dd1c, control 390c1fa059c7d336, error ce66d7b58375f789, hash 5a24907f63b6340f, input
  75b167fd93d08bf0, lib a8603264db0cf0c3, limits 08e80997cab3809e, msg 63677d4f00e3e151, quant 0d41e906a41f1c4d,
  snapshot ef9e601309e0a88d, wrap 9f2613c747b35c9c, tests/bites 9d7cb2f874485439, catalogue 14313b7ba5bb1f38, wire
  8675723a2b32b636, common 17bee55e5d501f55, clippy.toml 30ab9b2103d6b4e0, check.mjs 7eddc285abef4580,
  check.test.mjs da63983fc622b546. U4b, 4 files = given: clippy.toml 5a363a03b44ef029, lib.rs 2b62afca56b2c964,
  schedule.rs f9076ff81b453e4d, tests/schedule.rs dfb7da2cbac98c07. Full hashes: scratchpad u4a-sha-before.txt,
  u4b-sha-before.txt.
- Step 2-3 reading done (both copies, every src and test file; check.mjs/check.test.mjs diffs; seam.rs at base;
  catalogue §1-§3). Read the three reviewers' notes (netcode N1-N8/L1-L2, security SEC-1..7, rust F-A1..8/F-B1..3:
  the rust notes are complete, "DONE").
- Process note (honest record): the Edit tool is not available in this run (as the rust-reviewer found). Two progress
  lines above were first appended to this file with a shell heredoc before I noticed; the file is rewritten whole
  with Write from here on, and plants are made by Write of the whole file, restored by Write, sha256-checked.
- 10:20 UTC U4b: my first plant (last_due without "- 1") was NOT applied (the Edit call failed); the run that followed
  was therefore a clean bites-build run: 16 passed, 0 failed (matches u4b-build.md's 16). sha256 of all four U4b
  files re-checked: unchanged. Not counted as a plant.
