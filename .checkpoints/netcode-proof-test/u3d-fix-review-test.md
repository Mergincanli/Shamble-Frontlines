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
   record fail, restore, sha256 (scripts u3d-fix-review-bites.py and -bites2.py, this folder; output
   u3d-fix-review-bites-output.txt, logs u3d-fix-review-<id>.log, run.sh records u3d-fix-review-runsh-*.txt).
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
- Clean group (03:43-03:45 UTC; release binary npt_net-f72f21a42e60b4c9 built after a .rs touch, newer than it, sha256
  c236b449...; lists 172 tests, T1-T7 by name, the self-test among the ignored; df 14.2 GB):
  - clean 3 s: exit 0, 39 prints, slack "now 1, at start 1 ... SCHED_OTHER 0: accepts [1]".
  - chrt-after re-run (6 s, `chrt -f -p 1` on npt-l2-release 1 s in; before SCHED_OTHER 0, after SCHED_FIFO 1, slack
    from outside 0): exit 0, 39 prints, "now 0 ns, at start 1 ns ... SCHED_FIFO 1: accepts [0, 1]". My earlier run of
    the same procedure on the reviewed code (u3d-test-run-chrt-after.txt) was INVALID: item 1 settled by a run.
  - chrt-born (3 s): exit 0, "now 0, at start 0 ... SCHED_FIFO 1".
  - env: \xff exit 101 after 0.06 s, "INVALID: NPT_L2_SELFTEST_SECS="\xFF" is not UTF-8", no self-test print (the 1
    counted line is libtest's indented failures list); unset still running at 3 s, no INVALID; 0, 601, abc each exit
    101 in 0.05-0.06 s with their INVALID (no regression).
- Reverts and plants (each alone, built after a .rs touch, binary newer than the touch, 1 test run, every release
  binary listing 172 like the one before, every self-test failure after all 39 prints, every restore sha256 == fixed):
  - Item 1. F1-R (the old start-only assert put back) + the chrt-after procedure: INVALID at :1688 "the slack read back
    at start under SCHED_FIFO 1" left 1 right 0 = the fix is what makes that run valid. F1-P (mine: the release thread
    writes 50,000 to its own slack right after its start read, socket.rs after `started.send`): INVALID "1 ns at start
    and 50000 ns now under SCHED_OTHER 0, outside [1]" (the "now" read bites, another form than the fixer's S9).
  - Item 2. F2-A (mine: one DOWN entry left after the drain): INVALID "down: the fate log holds 30001 fates of the 30000
    sent". F2-R (item 2 reverted: fate_log_full back after the loop, fates == n removed) + the same entry: PASSES (exit
    0) = the old code missed it; the new assert is what bites. F2-S4 (my review's S4, cap = n) now "the fate log
    filled" 30,000 (was the loss band); F2-S5 (my review's S5, drain removed) now "the fate log filled" 60,000 (was the
    loss band): finding 2's cause labels fixed.
  - Item 3. F3-P (mine: pre-touch stops at cap / 2): INVALID "pre-touched with 30500 of its 61000 entries" (my review's
    S6, the same file without this assert, passed).
  - Item 4. F4-R (the old `Err(_) => 100u64` arm) + \xff: still running at 5.01 s, no INVALID = the new arm is what
    makes it INVALID (the script's verdict word for this line read "INVALID (informative)" because its "no INVALID"
    text contains the word; the data line is right; bites2.py fixes the word).
  - Item 5. F5-X1 (agree always "disagree"): T6 fails :1241 "E MISS and p99.9 FAIL"; F5-X2 (loss over n128): T6 fails
    :1256 "106 lost of 10,106" pass[1] false. Both passed the reviewed T6 (u3d-test-bites-extra.txt).
  - Item 6. F6-R (samples() back to the old panic): T6 panics :749 "a fate released before it arrived: Fate {dir: 1,
    t_in 5,000,000, out 4,000,000}" = T6 now needs the counting form. F6-P (mine: early dropped uncounted): T6 fails
    :1244 "down: the one fate released before it arrived is counted" 0 vs 1. F6-S (mine: delays under 25 ms read as
    early): self-test INVALID :1667 "up: Delivered fates released before they arrived" 14,519 vs 0. run.sh (check mode,
    U3D_CHECK=1, 2 s + 2 s, 1 round; T0 taken 1.2 s after a .rs touch): fixed: 0 of 43 non-.rs files under crates older
    than T0 (all touched), exit 0; reverted (`-name '*.rs'`): 43 of 43 older than T0, exit 0. Records moved out of
    results/ (u3d-fix-review-runsh-SH-*.txt); results/u3d-timing holds run.sh only; run.sh restored == fixed.
    SERIES.md:396-397 and the FATE_BYTES import: by reading (doc text; compile-only), correct.
- check.mjs (fresh .rs touch 03:49:39.7, shared folder, df 14 GB, no other cargo): 9/9 PASS. fmt 0.3 s; clippy 2.2 s;
  clippy-wasm32 0.3 s; test 76.7 s, measured 267 passed 0 failed 0 skips, bites 281 passed 0 failed 0 skips (= my
  review's and the fixer's counts); deps 0.4 s; refs 481 tracked files; marks 81; self 43/43; redact 10/10, 150 items 0
  findings. Both npt_net debug test binaries newer than the touch (03:49:44, 03:50:20).
- End state (03:51 UTC): git status identical to the baseline; sha256 of socket_tests.rs, SERIES.md, run.sh, socket.rs,
  stats.rs, impair.rs, Cargo.lock all equal the baseline; results/u3d-timing holds run.sh only; no .gate in the
  worktree; no cargo or npt_net process left. The worktree is unchanged by me (mtimes aside, from the touches).

## Findings
- Item 1's deviation (0 accepted at start under SCHED_FIFO/RR) is sound and I accept it: writing 0 to timerslack_ns
  resets it to the default (thread.rs:37-38 "0: leaves the default"), so a 0 can only be read on a kernel that forces a
  real-time task's slack to 0, where 0 is the true value; the chrt-born run (start 0, FIFO) shows that form is real here.
  Residual (INFO, no fix): under a real-time policy on such a kernel the condition cannot tell whether the thread ever
  set its slack, which is harmless there; on a kernel without the commit a missing set reads 50,000 and is INVALID.
- No gap: every item 1-6 has a test or run condition that fails (or a run that changes outcome) when reverted.
- No regression: counts 267/281 and 172 listed as before; 0/601/abc still INVALID; unset still the default; clean,
  chrt-after and chrt-born runs exit 0 with all 39 prints.
VERDICT: complete. Gaps: none. Regressions: none.
DONE (2026-11-09 03:52 UTC).
