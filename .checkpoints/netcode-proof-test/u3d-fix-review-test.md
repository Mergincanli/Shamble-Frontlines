# U3d fix-pass review (test-engineer), 2026-11-09

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_2c76ec82-8fc-1 on dae4f0f (uncommitted). Scope: ONLY the
lines changed since my build review (u3d-build-review-test.md), per the fix agent's "## Fix pass" (items 1-6). I am the
only one touching the worktree. Build rules: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target on
every cargo call (check.mjs too); touch every .rs under spikes/netcode-proof/crates before each build (mtime only);
df -h /home/user before each build (stop under 2 GB); each run's test count vs the one before. No iptables/tc/network;
git without -C/-c; files via Write only (no Edit tool here); commit nothing.

## Plan
1. Baseline: git status, sha256 of changed files (expect fixed socket_tests.rs ca17a9f6...b2b9, SERIES.md a902d10d...0bd1,
   run.sh f4a73a5b...dfe3, socket.rs/stats.rs/impair.rs/Cargo.lock as before).
2. Read the fix diff (vs my reviewed state: the build's socket_tests.rs 31294953...0625): items 1-6 lines.
3. Per item, a revert-in-place (the old code at that spot) and/or a plant in my own form, one at a time, run its test,
   record fail, restore, sha256 (script u3d-fix-review-bites.py, this folder):
   F1-R slack assert reverted + chrt-after (must go INVALID); F1-P release thread raises its slack to 50,000 after the
   start read (must fail on "now").
   F2-A one DOWN entry left in the log (S5b, my form: must fail "down: the fate log holds"); F2-R item 2 reverted + the
   same plant (informative: the old code passes it).
   F3-P pre-touch stops at cap/2 (must fail on touched).
   F4-E \xff on the fixed binary (INVALID at once); F4-R old Err(_) arm + \xff (informative: runs the default).
   F5-X1, F5-X2 (T6 must fail now).
   F6-R samples() reverted to the panic (T6 must fail); F6-P early dropped uncounted (T6 must fail); F6-S delays under
   25 ms read as early (self-test must fail on early); F6-SH run.sh's touch: every non-.rs file under crates newer than
   T0 (fixed) / none (reverted).
4. Re-run chrt-after (u3d-test-run-chrt-after.txt) against the fix; chrt-born too.
5. Fresh touch, check.mjs 9/9 once; counts vs 267/281.
6. git status + sha256 unchanged; findings, verdict, DONE.

## Progress
- Baseline (03:38 UTC): git status = M socket_tests.rs, M SERIES.md, ?? results/u3d-timing/ (run.sh only), ?? the
  builder's and fixer's notes under the worktree's .checkpoints/. sha256 socket_tests.rs ca17a9f6...b2b9, SERIES.md
  a902d10d...0bd1, run.sh f4a73a5b...dfe3 (= the fixer's final), socket.rs 7ad7cb22...76f9, stats.rs 27dfd326...9e22,
  impair.rs f79b5084...ffca, Cargo.lock d0158c8a...6397 (unchanged since my review). No .gate in the worktree; no cargo
  or npt_net process; df 14 GB free.
- The reviewed state rebuilt in my scratchpad from HEAD + wip/u3d-build.patch (recon.py): socket_tests.rs 31294953...0625,
  SERIES.md b06b67a0...fc45, run.sh 32babfb1...602e: each equals my review's baseline, so the diff is exactly the fix
  pass. Fix diff: socket_tests.rs (import; Samples.early + samples() match; T6 down early fate, agree x2, loss case,
  doc; NotUnicode arm; slack_ok/slack before prints, slack print, new assert; touched assert; fate_log_full moved before
  the loop; fates == n and early == 0 before the loss band; old fate_log_full + start-only slack asserts removed);
  SERIES.md:396-397 wording; run.sh:29-30 comment, :52 `-name '*.rs'` dropped. Nothing else.
- Read: value() panics on a missing key, but the moved slack reads (:1487-1490) name keys header_numbers always writes
  (socket.rs:319-322), and value(&r, ...) was already called before the prints: no new pre-print panic path.
