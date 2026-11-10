# U4a + U4b review, round 2 (test-engineer), on what changed only

Started 2026-11-10 (after a usage-limit reset; no earlier round-2 test notes existed, so a fresh start).
U4a: .claude/worktrees/agent-a9297644e7c0354f3 vs .gate/u4a-prefix. U4b: .gate/u4b-wt vs .gate/u4b-prefix.
Only plants in place (one at a time, restored, sha256-checked). No commit. Build rules: plan §7.1 (shared target,
offline, touch crates/ before and every 2 s during a counted run, wait for no other cargo/rustc, df before each build).
No Edit tool in this run: plants by Write of the whole file, restored by Write, sha256-checked.
Runner: scratchpad te-run.sh (round 1's: waits 8 s idle, logs df, touches, toucher every 2 s, stops it).

## Plan
1. sha256 of every file under spikes/netcode-proof/crates (+ check.mjs, check.test.mjs) in both copies, before anything.
2. diff -ru both pairs; read each changed hunk; map to round-1 ids (T-A1, T-A6, T-A7, nit, N2; T-B1, T-B2, T-B3).
3. Judge each closure from the test's power: which tests are new, which lack a plant (fixer planted PF2, PF4; main
   session planted the T-B1 "- 1"); plant one per new test without a plant, show it failing by name, restore, sha256.
4. Look for defects added by the fix (tests that can't fail, negative cases missing, the rust-reviewer's R2-L1).
5. check.mjs once per copy, counted runs (U4b expected measured 286 = 275 + 11, bites 309 = 292 + 17; U4a: record).
6. Final sha256 of both copies = step 1; git status file sets unchanged.

## Progress
- Step 1 done (before any build or plant). Hash lists: scratchpad r2/u4a-sha-start.txt (126 files), r2/u4b-sha-start.txt
  (116 files); git status: r2/u4a-git-start.txt, r2/u4b-git-start.txt. U4a's 8 changed files = the fixer's sha256
  (msg 8a170d9b, input a94e669a, control 881c6eae, hash d3a9f95b, error e72bcc4e, limits 82c7c74e, wire 8c09389a,
  common 254ffc7a); the other 7 proto files = round 1's baseline (bits 4a390bdd, lib a8603264, quant 0d41e906, snapshot
  ef9e6013, wrap 9f2613c7, bites 9d7cb2f8, catalogue 14313b7b). U4b: tests/schedule.rs 05e2068b; schedule.rs f9076ff8,
  lib.rs 2b62afca, clippy.toml 5a363a03 = round 1's.
- Step 2 done. diff u4a: 8 files (scratchpad r2/u4a.diff, 629 lines); diff u4b: tests/schedule.rs only (r2/u4b.diff).
  Fixer's own evidence (its notes "Fix pass, round 1"): before-fix FAILs for the snapshot send half, a6's decoder half,
  the Join half of the Debug test; plants PF2 (encoder ack refusal) and PF4 (finish guard); check.mjs x2 300/325/52.
- Tests with no failure shown yet: msg a3 full-state test (passed before and after: no plant); msg a3 snapshot test's
  receive half (held before); the Debug test's SessionControl half (the before-run stopped at Join); wire a2's as_sent
  branch; U4b carry 6's new due_ticks lines. Plants planned: P1 send check limited to Snapshot (msg.rs), P2
  receive_limit's Snapshot branch -> Limits::SNAPSHOT (msg.rs), P3 RecordStep's cap unredacted (control.rs), P4 as_sent
  -> Some(b) (wire.rs; reach of R2-L1), P5 a cap plant in U4b's schedule.rs for carry 6's new lines.
- 15:46 UTC P1 (msg.rs:312, the send check limited to Snapshot: `row(t).0 == "Snapshot" && ...`; diff vs the scratch
  copy = that one line). `cargo test --locked -p npt-proto --no-fail-fast`, measured build, npt-proto recompiled from
  this copy (log scratchpad r2/p1.log; idle 8 s, df 12 G): lib 12 passed, 1 FAILED:
  msg::tests::a3_a_full_state_is_sent_and_taken_up_to_65_536_b_after_its_length at msg.rs:678 (left Ok(()), right
  Err(TooLarge)); catalogue 7, wire 8 passed. So the new full-state test bites, and it is the only test that pins the
  send check for a type other than Snapshot. Restore next.
