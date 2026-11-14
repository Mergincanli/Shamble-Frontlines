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
