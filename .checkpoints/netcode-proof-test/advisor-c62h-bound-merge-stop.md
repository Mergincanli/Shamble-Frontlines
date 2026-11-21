# Advisor: C62H-BOUND merge-step stop (main-run.sh check.mjs test failure)

Role: main's .claude/agents/advisor.md (read via git show origin/main). Read only; this file is my only write.

## Plan
1. Read the brief's sections 6, 7, 10 (merge step, P0 re-judge, stops) and the card's NOW block.
2. Read the merge log and the failed test step log (:1407, :1476-1532, :510).
3. Read the failing test (delay_tests.rs ~2200-2310) and Conn::check_held_bound (delay.rs :382, :538); G-NC2 notes.
4. Check the lane touched nothing under crates/net (git diff --stat), nor shared crates net depends on; Cargo.lock / workspace.
5. Check the 96 "ok" logs claim (grep) and whether any earlier flake of this test exists.
6. Judge: flake vs defect; lane-caused or not; severity.
7. Rule (b) next run, (c) recording, (d) Lows deferral + script fixes, (e) NA-CBB-2 pre-flight, (f) order.

## Notes (a line after each point)
- 1 DONE: brief sha256 7936074c… (663 lines) confirmed; s6 item 2 commands, s7, s10 ("a check.mjs step fails that no
  edit of this lane explains": stop, never retried or cleaned around) read; card NOW block and build notes :380-439 read.
- 2 DONE: merge.log: header free 6131, HEAD 561bf42, diff/untracked e3b0…; test FAIL 532.4 s; all other steps PASS;
  "other cargo or rustc now" empty; lock held exclusively. test.log :1407 FAILED, :1476-1532: "100 writes of 16 KB at
  P2: largest down excess over 64 KB 14326 B; samples above the bound [1, 0]; first losses 28, stalls 28"; panic at
  delay_tests.rs:2300. NOTE: the bites build stopped at npt-net's lib (429+1 = 430 of 632 run): cargo's fail-fast, so the
  crates after npt-net in the bites build did not run their tests. delay.rs DOWN = 0 (delay.rs:121), so [1, 0] = one
  DOWN sample. Second assert (excess 14,326 < 16,406) would have held.
- 4 DONE: `git diff --stat 044fab1 561bf42 -- spikes/netcode-proof/crates/net` prints nothing (exit 0). npt-net
  depends on npt-l1 (crates/net/Cargo.toml:19-20, for Profile/ReorderStep). The lane's l1 lib.rs diff is +33 -0: a new
  struct DatagramCounts and a new read-only method FateLog::server_datagrams_since; Profile, ReorderStep, P2 untouched.
  No Cargo.lock/Cargo.toml change. So npt-net's code under test is the same; only the binary was rebuilt.
- 5 DONE: grep over .gate/**/*.log: FAILED in .gate/netcode-proof-test/failed-steps/test.log only; "... ok" in 96 files
  (96 lines). nproc 4; load now 0.02 / 0.82 / 0.92 (16:21).
- 3 (code read): check_held_bound (delay.rs:528-543) samples held = bytes[d].len() (ALL down plaintext not yet taken by
  S, released-but-untaken included) against budget + prefix of recs[d].get(released[d]) (the front record NOT yet
  released). A release (harvest, delay.rs:675-686) moves a record from unreleased to released without changing held;
  if that record carried the acked prefix and held was above 64 KB, the bound drops by that prefix at the release while
  held stays, until the pump drains it into S (delay.rs:1727-1761). The writer cannot add in that window (down_free =
  free_budget - released_model, delay.rs:473-477), so no extra byte is admitted. The pump's closing block samples
  held_now() (delay.rs:1879) exactly in the state its own comment R1 names (:1868-1872: "a down record released after
  the down loop ended (by the up loop's steps, or by another task's step while the lock was free) is still unwritten
  here"). The test's loop calls ds.snapshot() -> current() -> app_step (delay.rs:1163-1170, 1251-1252) from the test
  thread every iteration, so another task's release can land between the pump's down loop and its closing sample. That
  is a real-time scheduling window (the model runs on DelayStream's clock), wider on a loaded or just-booted machine.
  Reading: the check's formula does not exempt a just-released, not-yet-taken record; DelayStream itself held nothing
  it should not. Not proven by a run (I run nothing); the follow-up's first step shows it.
- 3b (the pump): its up loop steps the model to a fresh now on every pass (delay.rs:1813-1814, By::Mid), after the
  down loop; its comment (:1800): "A down record this step releases is written by the pump's next pass, which the
  closing block wakes at once (R1)". So the window is the pump's own down-loop time (real time), every pass.
- 3c (why the bites build only): held grows only at poll_write (bytes[DOWN].extend, delay.rs:1574, the one extend);
  between writes it only falls (drain, :1748). At a write, outstanding + released_model <= 64 KB (down_free), so held
  <= 64 KB + the prefix at that write: the bound holds at every point where held can grow. The second assert (excess
  14,326 < 16,406) held. The bites build's npt-net lib has 169 tests (wt-wtransport and wt-wtq features: QUIC sessions
  at P2 running beside it) against the measured build's 160: more load in the same binary, a wider window.
- 3d (the lane's reach into that binary): only npt-l1's additive code (unused by npt-net: grep finds no caller outside
  l1 and lab) and SERIES.md's bytes via include_str! in three other test modules (names.rs:143, delay_names.rs:449,
  socket_tests.rs:390), which passed. cargo runs test binaries one at a time, so the lane's new tests never ran beside
  it. Shared target: lock exclusive, "other cargo or rustc now" empty. Not the lane's in any way I can find.
- Precedent read: advisor-u4j-r2.md:161-171 (B2.3: never retried into green unnamed), advisor-u4d2-r1.md:294-316 and
  :514-517 (M6: the merge goes ahead only on a whole counted run with all 11 steps passing, the earlier failure named;
  the cause a planned change of its own; "Never a retry, and never a looser bound without its cause"); te2-5-record.md.
- 6 (a) RULED: a false positive of U3c's test-only check (G-NC2), timing-dependent; not a DelayStream defect shown; not
  the lane's. Medium, U3c's code. No finding against C62H-BOUND. Plus a Low: check.mjs's bites build has no
  --no-fail-fast, so the failure hid 202 of 632 bites tests (430 ran).
- 7 (b) RULED: no loop before (the code decides lane-independence; a 1-in-97 rate needs hundreds of runs to compare;
  FX-10's file already gives name and panic). ONE second counted main-run.sh call on 561bf42, s6 item 2's commands in
  order, log merge-2.log; pass whole. A second failure of this test: stop, no third call; the U3c fix first. Any other
  failure: stop whole, the advisor.
- 7 (c) RULED: notes + card on disk now; a follow-up entry (Medium, main session, before the next lane's counted run);
  check.mjs Low follow-up; the report names both calls. To Mustafa in the report and his next chat update, no ask.
- 7 (d) RULED: deferral of the ten Lows accepted (TE-CBH-7's reading confirmed; TE-CBH-5 = NA-CBB-1 with an
  operating condition for P2's two judge calls). Script fixes accepted: diff read (bite_rule's `rest`, bite_lines and
  the self-test's `late` bump); sha256 e9eb9b23…; dry-2: 12 PASS, 29 of 29 FAIL; commit it with the card.
- 7 (e) RULED: do the pre-flight once, after the P0 re-judge's SAME ANSWERS, before 8.1; commands in the notes first,
  read in the head check; its readings meet the same stop rules as any clean run.
- 7 (f) RULED: order written in the return.
- Extra check: crates/lab/Cargo.toml names no npt-net and crates/lab/src has no DelayStream use (grep empty), so the
  failing code is outside every C62H-BOUND evidence path (lab, l1, judge; the L1 sets).
- DONE: structured return sent (verdict: CONTINUE WITH CONDITIONS; second counted call; nothing to Mustafa now).
