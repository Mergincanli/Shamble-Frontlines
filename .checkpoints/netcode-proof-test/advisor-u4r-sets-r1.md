# Advisor: U4r judged sets, ruling r1 (Q1 P3 INVALID, Q2 P0 judge memory, Q3 disk projection)

## Plan
1. Read advisor-u4r-r2-full.md (step 17), the disk plan, the card's run procedure.
2. Q1: read P3 verdict.json, runs.txt, kept log; the judge's PM-0001-2 check and -17's pre: rule; the profile code
   (P3 = 150 ms, 30 ms jitter, 5 % loss?); the client view/interest code; why "everyone in view" fails under loss.
   Decide expected or not; -17 L1 P3 and L1 P2 fate; P2 runs or not; report; Mustafa.
3. Q2: check the judge-apart log, the judge's read path (main.rs:99, lib.rs:37-46), memory projection, free -m. Rule.
4. Q3: check du/df, runner projection (c-u4r-runs.mjs:106-121, :250-255, :392, :416), worktree list, disk plan s5,
   advisor-u4d2-m9 Block 2, CHECKPOINTS s10. Rule order of freeing; runner rates fix or manual margin.
5. Findings, next steps, Mustafa.

## Checks (a line each)
- Read advisor.md from main; r2-full step 17 (INVALID named only for B1; B1 -17/-18 INVALID recorded, never counted);
  disk plan sections 1-7.
- P3 verdict.json: -2 SS part FAIL on 3 runs (0.352778, 0.085444, 0.203556), 10 other checks pass; -17 L1 P3 INVALID
  pre:PM-0001-2 on 3; -57 L1 P3 PASS (6984, 2750, 88063 on all 3). Facts hold.
- r2 (e59ece9) p3 suite, 20 s measured, seed 1: everyone in view 1 (pass). So the short run never showed it.
- Per-tick flag sums (room.jsonl, measured): s1 48 on every tick to 70,335, first below 48 at 70,336 (all 48 slots drop
  within 70,336-70,354); s2 first below at 65,536 (all 48 within 65,536-65,550); s3 at 67,696. After: window means
  8-29 of 48, never back to 48. A global event, not per-client loss. Overflow resyncs 0; datagrams 996-1,000 B every
  tick before and after (the cap is always full at P3).
- carry 49 in the same runs: very large units held in 0.9865 / 0.9673 / 0.9628 of sets, so the frames are NOT rebuilt
  from the defaults (B1's cause gave 0.2811): specific soldiers are missing, not whole sets.
- D4's 15 runs (r5, 50 s measured to tick 65,535): every SS P0/P2/P3 run 1.0; B1 0.62-0.69. Consistent with onsets.
- One slot per run is 0 on every tick from the onset on (s1 slot 44, s2 slot 10, s3 slot 2); the other slots' zeros
  are a different random-looking set each tick (~30 % held after onset). Not geometric (that would be a stable set).
- Code: a soldier leaves a frame only by a Remove (not in c.relevant: radius 6,400 cm AND flood.reaches, room.rs
  906-935) or a defaults-coded snapshot. Large units held 0.96-0.99 rules out frequent defaults coding. So soldier W
  drops out of the room's interest for everyone, flickering; per-client lags (baseline age, cap placement) scatter it.
- Steps without a command vs per-slot share: Spearman ~0 (dominated by W); W's own steps_without high in s1/s2.
- THE CAUSE (fingerprint): the others' flag count, post-onset, has autocorrelation peaks at lags ~355 and 1,080 ticks
  (0.88-0.92); phase-folded on (t-63,736) mod 360 it is a square wave: ~180 ticks at ~0 holding, ~180 ticks rising
  (s1 closed from phase ~120 to ~300 = door 4's toggles; s2 and s3 ~0 to ~180-195 = door 0; corrected on re-read). world.rs:1203-1208:
  door (second mod 6) toggles each second, so each door is shut 6 s of every 12 s (360 ticks); greybox.rs:89-99 six
  buildings 22 m out, doors facing the centre, joins [0, a]; room.rs:932 relevance needs flood.reaches. So soldier W
  is inside a building: while its door is shut the room drops W from every field client's set and the field from W's.
  The room's interest is right; the scene's precondition (everyone in view) stops holding.
- Why at P3 and not short runs/P0: bots/src/lib.rs:31-51, :206-215: the brawl walks a 16-step circle (~2 m radius),
  turning every 4 commands GIVEN (no time in it, FP-7 item 1); room/src/input.rs take(): a tick with no command re-runs
  the last one. At P3 16-27 % of ticks re-run (client_counts steps_without 1,026-5,172 of ~18,300 per slot), so the
  circle does not close and each soldier random-walks; onsets 60-210 s after start (r2 30 s, D4 60 s: both too short).
  r2 P0 (same npt-lab sha b3b4a60d): 43,152 commands = 48 x 899, steps without 0, late 0: at P0 the circle closes.
- Not U4r's runner, the judge or the mirror (0 breaches); it is npt-bots' brawl (U4r's crate) in the stress scene.
- Q2: judge-apart log read: one run 178,922,504 B -> 641.1 MiB, 2.7 s; read alone 426.2 MiB, 1.9 s (70 %).
  main.rs:99 + lib.rs:37-46: read_logs reads every .jsonl whole before judge(); --only does not cut reading.
  The whole-set 1,507.9 MiB is peak.py around the whole set call (cargo builds incl. rustc for npt-judge, npt-lab runs,
  the judge): not shown to be the judge's, so the 216 + 2.49x line has one judge point only (slope an upper bound if
  linear). Read-alone scaling 2.5x bytes: P0 at 3,073 MiB -> ~7.7 GiB; at 1.5x bytes (4,610) -> ~11.5 GiB. free -m:
  16,087 total, 14,529 available (B1 running).
- Q3: du -sm P3 folder 519; du -sb 543,998,603 B = 518.8 MiB; runner printed 360 MiB: ratio 1.44 (not 1.42).
  B1 s1 285.6 MB, s2 283.1 MB vs 190.6 MB projected: 1.49-1.50x. lab.jsonl per tick grows over a run (P3 s1:
  3,949 B in minute 1 to ~7,800 by minute 7; room/client flat). D4 P0 first 50 s: 24.3-25.4 kB lab per tick, flat,
  matching the runner's P0 rate (27.8 kB/tick with room+client); P0's growth over 10 min unknown.
  df now 9,017 MiB (B1's s3 running). Copies: .claude/worktrees 413 MiB + .gate copies 299 MiB = ~712 MiB.
  shared-target: debug 7,875, release 282, wasm32 176, doc 24 MiB. W1-TARGET item 3 as amended (advisor-u4d2-m9.md
  :252-257) allows --release/--profile dev/--doc part-cleans between waves with no lane running. CHECKPOINTS s10
  read. LEFTOVER-DELETE (history.md:404-410) read: deletes past approved methods = a narrow kit rule, his.
  Runner: floor() (:250-255) per run with runBytes (low rates); a mid-set stop throws, leaving a partial set.
- Bites (eval.rs:1385-1500): a bite run is judged on its own checks with its parts' validity; reading is still whole.

## Ruling (summary; full text in the StructuredOutput)
- Q1: not as expected; cause = brawl drift into a building (door fingerprint). -17 L1 P3 INVALID recorded, never
  counted, no re-run at this commit; moves with BRAWL-1 (next npt-lab lane) to its re-run. P2 runs as planned; -17
  L1 P2 INVALID with the same fingerprint is recorded and moved without a stop; anything else to me. Nothing to
  Mustafa to decide; heads-up in PR #40's report.
- Q2: P0 does not start until the judge is measured on whole set folders (P3, B1, P2) apart; P0 runs if the fit at
  F x 3,073 MiB is <= 10 GiB; else a plan first (judge in parts or shorter bites).
- Q3: A (copies, s10) between B1 and P2; after P2, R = 4,096 + F x 3,073 + 500 MiB; B then C only if short; manual
  margin suffices for P0; runner rates corrected at the next c-u4r-runs.mjs change (RATES-1). Heads-up to Mustafa:
  after U4r the disk sits near the floor; the next judged runs will need his narrow-rule decision.
