# U3c fix pass (c1): adversarial verification of the re-reviewer's gaps (R1, G-F10, G-NC4, G-NC2, G-PLAN, G-FH, G-NC8, G-PRE-WAKE, G-PRE-PANIC, G-METHOD) and regression R1

Scope: `git diff 95597ef..81203da -- spikes/`; claims from u3c-fix-review-c1.md.
Rules: edit nothing but this file; scratch in the session scratchpad (own copies from `git archive`);
CARGO_TARGET_DIR=.gate/u3c-review-target (fresh mtimes before each run, check test counts); no iptables, no #[ignore]d tests.

## Plan
1. Read delay.rs at 81203da: Pump::poll (up loop, down block), current(), due(), move_up, Pump::drop, poll_read.
2. Read pre-fix pump at 95597ef for R1's comparison.
3. Read delay_tests.rs: (c) Return.owed, quiet P2 held bounds, NC-U3c-8 check.
4. Grep u3c-plan.md for the carry corrections (G-PLAN, G-NC2), C3 text.
5. Reproduce R1 (probe 1 style) and F10 cheaply in a scratch copy if time allows.
6. Verdict per item.

## Progress
- Read (81203da = HEAD for crates/): R1 mechanism holds by code. Pump::poll's first step (delay.rs:1376-1377) keeps
  only [UP] but the down loop is state-driven (c.released[DOWN], :1387), so those releases are written. The up loop's
  step (:1470-1471) keeps only [UP]; a DOWN release there sets released[DOWN] after the down loop ended; the final
  block (:1526-1534) wakes only reader/writer/shutter; due() (:457-469) = next model event, burst end, shutdown. The
  clock entry is removed when it fires (delay_clock.rs:8, :209), so during the pump's run nothing is armed for a
  delivery that falls inside it. Pre-fix pump (95597ef delay.rs:1197-1336) steps once, holds the lock for the whole
  run: no second step. C3 text confirmed at u3c-plan.md:68-69.
- G-METHOD reproduced: my pristine `git archive 81203da` copy (mtimes 15:08) in .gate/u3c-review-target: `cargo test
  -p npt-net --lib --no-run` "Finished in 0.05s" with no compile, and pointed at npt_net-088534cbcaf56b90, whose
  --list has 159 tests incl. review_probe{,2,3,4} (the re-reviewer's scratch probes). C1 notes say the target was
  shared with C2, but C2's notes (u3c-fix-c2.md:6) and the checkpoint (netcode-proof-test.md, fix pass) say C2 used
  u3v3-model-target: that sub-claim is wrong; the hazard itself is real.
- G-NC4: grep `\.owed\b` outside c./self. in crates/: only ws_gate.rs:123,130 (Inputs.owed) and delay.rs:897; the
  Return row test (delay_tests.rs:1967-1972) constructs owed: 6 by hand. No test reads record_return's value (:448).
- G-NC2: SERIES.md:533/:562 and delay.rs:43-45 carry the corrected bound; the only held-bytes test (delay_tests.rs:
  547-625) is quiet P2 and still asserts the old bound max[DOWN] <= BUDGET (:586). No lossy case.
- G-PLAN: u3c-plan.md:122 "Down: at most 64 KB", :123 up bound unchanged, :176 "N(srtt then)", 1.5 (:204-208)
  unchanged; checkpoint lists only the 1.4 (c) wording explicitly (netcode-proof-test.md:540, :563).
- G-FH: mutation "Pending at <= 20 KB free" with 8,192 B writes: 6 records x 8,192 = 49,152 = BUDGET - RECORD_MAX
  (delay.rs:81) exactly, so `>=` (:597-601) passes: MISSED by arithmetic.
- G-NC8: delay_tests.rs:1057 steps to clock + 10 ms; add_burst steps to the clock's now (delay.rs:806-807); if > 10 ms
  pass before :1062 the model's now passes at_now and +1 is refused (start > now, :811). Hidden 10 ms wall margin.
- G-PRE-PANIC: Pump::drop (:1327-1341) sets pump_done only; poll_read (:1214) returns Ready(Ok) on pump_done after
  closed/failed checks (:1208-1213): a panic reads as EOF. Same at 95597ef (:1177-1191, :1075). lock() is
  poison-tolerant (:611-613). tokio drops a panicked future inside its panic guard, so panicking() is true in drop.
- G-PRE-WAKE: current() wakes the pump only on got[DOWN] (:861-865), same at 95597ef (:750-755). But the clock entry
  is always at the next event after any reschedule; current() can step past an ACK event only in [event time, clock
  fire) or during a pump run after its :1477 check: narrow windows (contrast R1: the whole S write).
- Own probes (independent of the re-reviewer's; source in scratchpad v1r/src/{common,post_only}.rs, appended to a
  scratch copy of 81203da; touched; "Compiling npt-net" seen; 158 tests = 155 + my 3; output scratchpad
  v1r/post-probes.txt):
  * VPROBE1 (hand-polled, S write 25 ms, fixed(200), A then B 10 ms later, run at t0+103): run 103.2..128.6 ms; B
    released at 110.2 ms (inside the run); after it: S took 1, released-unwritten 1, next due 210.2 ms, first wake
    210.3 ms. R1 REPRODUCED (about 100 ms hold, no wake owed).
  * VPROBE2 (spawned pump) post: S 2 ms/gap 1 ms: B written 102,337 / 102,455 / 102,287 us after release; S 5 ms/gap
    2 ms: 105,339 / 105,318 / 105,538 us.
  * VPROBE4 (F10 shape, 7 B frames, fixed(10)): post passes, up room full with 1 bound pending, 65,148 of 65,148 B.
- Post copy, all l2::delay: 26 passed (24 + vprobe2, vprobe4), vprobe1 FAILED, 1 ignored.
- F10 bite (post copy, `b.0 -= start` dropped, delay.rs:527): 25 passed (the 24 originals + vprobe2), vprobe4 FAILED:
  pump panicked on a tokio worker at core slice/index.rs:1020, and the reader saw Ok(0) after 24,419 of 65,148 B
  (the loop's `.expect("read")` did not fire; the `got == sent` assert did). G-F10 CONFIRMED; G-PRE-PANIC REPRODUCED
  (a pump panic reads as a clean EOF).
- R1 fix (scratch: owe_self |= got[DOWN] in the up loop's step; cx.waker().wake_by_ref() before Pending): VPROBE1 first
  wake 128.4 ms (= end of run, owed); VPROBE2 2,432/2,287/2,209 us (S 2 ms) and 7,324/7,345/7,534 us (S 5 ms); the
  l2::delay suite 27 passed (24 + my 3), 1 ignored. The re-reviewer's fix works.
- Pre copy (git archive 95597ef + common.rs; touched; 149 tests = 148 + 1): VPROBE2 2,181/2,386/2,257 us (S 2 ms),
  7,234/8,383/7,302 us (S 5 ms). Post 102,287-102,455 and 105,318-105,538 us. R1 REGRESSION CONFIRMED (5 of 5 x2).
- Three mutations in one build (pristine 81203da delay.rs restored first; 158 tests): (1) poll_write Pending at
  free <= 20 KB (G-FH), (2) record_return owed: 0 (G-NC4), (3) a 15 ms sleep before the "+1 accepted" assert (G-NC8).
  Failed: a_burst_past_the_clocks_range (:1063 = the "+1" assert after the stall: G-NC8 CONFIRMED);
  one_record_per_write_never_beyond_the_budget (:528, w(23_616) == Some(13_572) gets Pending: the G-FH mutation IS
  caught by the suite, so "MISSED" holds only for the held-bytes test alone, which passed); vprobe1 (R1). Test (c) and
  everything else passed: G-NC4 CONFIRMED (owed forced to 0 caught by nothing).

## Verdicts
- R1 (gap and regression): REAL, medium (reproduced independently, pre vs post, fix verified).
- G-F10: REAL, low (bite reproduced; new compaction code, a dropped shift panics the pump and truncates up to an EOF).
- G-NC4: REAL, nit. G-NC2: REAL, low (no lossy case; test still asserts the old bound :586). G-PLAN: REAL, low.
- G-FH: REAL as stated for the held-bytes test (bound 16 KB under its own derivation), but the suite catches the
  20 KB mutation at :528: nit, no hidden defect.
- G-NC8: REAL, nit (15 ms stall fails it). G-PRE-WAKE: REAL, low (pre-existing; narrow windows; bounded).
- G-PRE-PANIC: REAL, low (reproduced via the F10 bite). G-METHOD: REAL, nit (reproduced; "shared with C2" sub-claim
  contradicted by u3c-fix-c2.md:6).

DONE
- Cleanup: restored my scratch copy to pristine 81203da and rebuilt, so .gate/u3c-review-target's npt_net test binary
  is 81203da again (155 tests), not one of my mutants.
