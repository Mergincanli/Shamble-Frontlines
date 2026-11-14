# rust-reviewer: U3c fix pass, part C1 (npt-net), re-review

Scope: `git diff 95597ef..81203da -- spikes/` (C1's files: delay.rs, delay_clock.rs, delay_names.rs, delay_tests.rs,
stats.rs, ws_frame.rs, wt_tests.rs, Cargo.toml, SERIES.md). Build dir CARGO_TARGET_DIR=.gate/u3c-review-target.
No source edited; scratch copies only under .gate/ or the session scratchpad.

## Plan
1. Read the diff of delay.rs (Pump rewrite: F3/F2/F4/F5/F10; add_burst F1/S7/NC-U3c-8; gate_inputs F-L; NC-U3c-4).
2. Read ws_frame.rs (S2) against httparse 1.10.1's line rules (registry source).
3. Read stats.rs (F7 Running).
4. Read delay_tests.rs changes (each finding's test).
5. Build, clippy, fmt, run the l2 tests.
6. Bites (revert in a scratch copy) for the high-value ones: F1, F3/F4 pump, S2, F7, F-K.
7. Race analysis of the pump's lock change (lost wakeups, double commits, partial take, ordering vs current()/add_burst).

## Findings log (one line each as found)
- Baseline at 81203da (source equals HEAD e317067): `cargo test -p npt-net --lib l2::` 81 passed, 2 ignored, 18.2 s
  (.gate/u3c-fixrev-c1-l2-run1.txt); fmt --check clean; clippy -D warnings, all targets, with and without
  --all-features: clean.
- F6 clock change read (delay_clock.rs:191-203): /proc read with the state lock dropped, re-lock then `continue`; a
  schedule() during the read may notify nobody (wait_until stale) but the loop re-checks the schedule under the lock
  before any wait, so no lost wake. OK.
- S2 scanner (ws_frame.rs:180-191) matches httparse 1.10.1 skip_empty_lines (lib.rs:558-581), newline! (macros.rs:55-67)
  and the head end (lib.rs:1105-1116): CR only before LF, lone CR = httparse error. OK.
- F7 Running (stats.rs): mark logic correct (go_dense sets touched buckets back to their marks; the triggering bucket is
  still pre-window); 2 x 142,240 + 8,192 + 16,384 + 527 = 309,583 B as claimed.
- SUSPECT REGRESSION (to prove): Pump::poll's up loop steps the model again (delay.rs:1470-1471) and keeps only
  got[UP]; a down record released by that step is not written this poll and no wake is owed for it (due() has no
  entry for "released, not written"), so it waits for the next model event. Pre-fix pump stepped once (95597ef).
- REGRESSION R1 PROVEN (new, medium): the up loop's step (delay.rs:1470-1471) releases down records and drops
  got[DOWN]; nothing wakes the pump for them (current() wakes it only for its own new releases, delay.rs:861-865;
  due() has no "released, unwritten" term, :457-469). Scratch probes in .gate/u3c-fixrev-c1 (scratch copy, plus the
  pre-fix tree from `git archive 95597ef` in .gate/u3c-fixrev-c1/pre), CARGO_TARGET_DIR=.gate/u3c-review-target:
  * probe 1 (hand-polled pump, S's write takes 30 ms, fixed(200), records 20 ms apart): after the poll at t0+135 ms,
    1 record released and unwritten, next due in 85 ms, no wake owed; first wake at t0+221 ms.
  * probe 2 (wrap, spawned pump; B written after its release, in us): S write 30 ms / gap 20 ms: post 130,412 vs pre
    39,528; S write 2 ms / gap 1 ms: post 102,485 vs pre 2,440; S write 0 / gap 0: post 197 vs pre 170.
  * probe 3 (a caller reads outstanding_bytes() every 1 ms): post 1 of 3 runs held B about 100 ms more (S took both
    after 202 ms vs 102-103 ms), pre 0 of 3.
  Frequency at real S speeds (rustls on loopback, tens of us per write) not measured; the A/B lateness files show no
  consistent down p99.9 shift (pre 2.900/2.493, post 2.608/2.955 ms), so it is rare there; post's max 35.3 ms is
  consistent with it but not attributed. Fix: keep got[DOWN] from every step in the up loop and, if set, wake the
  pump's own waker (cx.waker().wake_by_ref()) or loop back to the down block. Test: probe 1's assertion (after a
  pump poll, no released down record without a wake owed).
- R1 also breaks binding condition C3 (u3c-plan.md:68-69: "Released down records go into S at once").
- Bites re-run on the FINAL code (C1's bites-run1.txt was run at 11:24 on an earlier draft: its line numbers do not
  match 81203da's tests). Script: scratchpad fixrev_bites.py; output .gate/u3c-fixrev-c1/bites-final.txt (running).
  First 14: F1, NC-U3c-8, F2 x2, F3 write/read/flush/shutdown under the lock, F4, F-R2, F5, S2 x2, S2 through
  DelayStream (each_limit hits its 30 s hang guard): all BITE.
- Pre-existing (not in this diff, not assigned): the up room's reopening by ACKs at end 1 that a current() caller
  steps past wakes nobody (current() wakes the pump only on got[DOWN], delay.rs:861-865). Same family as R1; the lost
  first-run rust F7 (checkpoint netcode-proof-test.md, "first-run findings"). Low: bounded by the next pump wake.
- NC-U3c-8 test's hidden margin: delay_tests.rs:1057 steps the model 10 ms ahead of the clock; "at_now + 1 accepted"
  (:1062) fails if 10 ms pass before that call (a test-thread stall under check.mjs's load). Nit; use 10 s.
- Plan text carries (NC-U3c-1's 1.4 (c) and R13 wording, NC-U3c-2's 1.2, NC-U3c-3's 1.5) are NOT yet in
  u3c-plan.md (grep finds none); listed in C1's notes and the checkpoint's "after it" list.
- Bites on the final code, rest (bites-final.txt): F-B burst test alone, F-B in (c) (at_ns, :2517), NC-U3c-10
  (a gate one snapshot more cautious fails the vacuity guard, :2529), F-A, F-G, F7 x3 (incl. go_dense not restoring
  marks), F-M (slack never set), F-H up (read floor 8,000): all BITE. MISSED: F-H down (Pending at 20 KB free: the
  bound 64 KB - 16,384 B is met exactly, 49,152 B; the doc's own derivation gives about 65,316 B); NC-U3c-4
  (Return.owed forced to 0: no test reads its value); NC-U3c-3 (fresh now reverted: printed only, by design);
  F10 (move_up's bounds shift dropped: no l2::delay test reaches it).
- F10 gap proven: probe 4 (scratch; fixed(20); request + 6,000 frames of 10 B, nobody reads until the up room is
  full, then drain to EOF) passes on the final code (96,148 of 96,148 B; move_up leaves bounds pending 10 times while
  draining) and fails with the shift dropped: the pump panics (slice index, core slice/index.rs:1020) and the reader
  sees a clean EOF after 27,652 of 96,148 B (a pump panic read as EOF: the lost first-run F9, still open).
- R1 fix checked in scratch: keep got[DOWN] from the up loop's step and wake_by_ref the pump's own waker before
  Pending. Probe 2: B written 39,860 us (S 30 ms) / 2,606 us (S 2 ms) / 340 us (S 0) after its release, the pre-fix
  numbers; l2::delay suite 24 passed, 1 ignored.
- CAUTION (method): a scratch copy of the workspace shares unit hashes with the branch in one CARGO_TARGET_DIR (cargo
  hashes path packages relative to the workspace root), so a run in a tree whose files are older than the last
  artifact reuses ANOTHER tree's binary. Seen: a branch run after the scratch builds ran 159 tests incl. my probes
  (.gate/u3c-fixrev-c1-net-lib.txt: INVALID). Valid by construction: every scratch run whose files were just modified
  (each bite, probe 1, probe 2 both trees (test counts differ: 155 vs 148 filtered), probe 4, the R1 fix, the F-K phase
  runs) and the first branch run (before any scratch build). Re-run: the branch head from a pristine archive with fresh
  mtimes, and probe 3's PRE runs.
- Valid re-runs (fresh mtimes): pristine 81203da archive (.gate/u3c-fixrev-c1/head): npt-net lib 144 passed, 11
  ignored, 0 failed (.gate/u3c-fixrev-c1-net-lib-head.txt). Probe 2 (S write 2 ms, records 1 ms apart), 5 runs each:
  PRE B written 2,141-3,257 us after its release (5/5); POST 102,241-102,410 us (5/5). Probe 3 (racy) 0/5 held in
  both: dropped as evidence; probe 2 is the proof.
- F-K under phase: (c) with every write moved 5/12/20/25 ms in its tick, 2 runs each: 8/8 pass, first byte after
  the second return up to 2,569 ms (the old fixed loop ended about 2.1 s after it) (.gate/u3c-fixrev-c1/fk-phase.txt).
- F-P: P(X >= 21), Binomial(200, 0.02) = 9.33e-10 (computed): the comment's 9.3e-10 holds.

## Verdicts per finding
- F1 + S7 fixed (bite :1029). F-K + NC-U3c-1 + NC-U3c-10 fixed (bites :2517, :2529; phase runs 8/8). F-L/NC-U3c-5/F11
  fixed (by reading). F3 fixed (4 bites), F2 fixed (2 bites), F4 fixed (bite) - but R1 regression in the same rewrite.
  F7 fixed (3 bites + header). F-B, F-A, F-G, F-M, S2, F5, F-R2, NC-U3c-8 fixed (bites). F-F, F-C, F-O, F-I, F-P,
  NC-U3c-9, NC-U3c-11, F6 fixed (doc/by reading). Partial: NC-U3c-4 (value untested), NC-U3c-2 (no lossy test, plan
  text), F10 (compaction untested), F-H (down bound 16 KB loose). NC-U3c-3 fixed (print only, by design) and the
  source of R1.

DONE
