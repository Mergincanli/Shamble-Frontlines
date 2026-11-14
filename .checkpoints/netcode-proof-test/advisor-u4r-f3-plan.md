# Advisor notes: plan review of U4r-F3 (the one runner change)

Started 2026-11-14 (late UTC). Read advisor.md from origin/main first. Plan: u4r-f3-plan.md (344 lines).

## My plan
1. Read the rulings: sets-r2 (next step 3), f2-review (INF-F2-1/2), f2-head (ADV-Q), f1-review (INF-F1-3/5),
   sets-r1 (RATES-1, RUNS-LINE), r2 (INF-R2-5, item 18), f2-plan (condition 8).
2. Read the scripts as committed: c-u4r-runs.mjs (791bc55), c-u4r-r2.mjs (f1f131b), check.mjs run() :1909-1922.
3. Check each item's place and words: INF-F2-1 sentence; INF-F2-2 three wordings (+ Q1 flag); INF-F1-5 compare kept.
4. RUNS-LINE: copies fields only; its rule on all five committed verdict files gives each set's own parts and bites.
5. RATES-1: du -sb each kept run folder; check the figures and the projections; fit to the fix's words.
6. Six-point fit: reproduce from judge-apart logs (least squares), residuals; where it prints.
7. INF-F1-3 deferral vs INF-R2-5 home; r2 scope; evidence enough.
8. Q1-Q6; verdict; size; Mustafa.

## Checks (a line per check as I go)
- C1 Rulings read: sets-r2 step 3 list; f2-review INF-F2-1/2 (words); f2-head ADV-Q (after INF-F1-1's sentence, infra's
  words, at the next change); f1-review INF-F1-3 (no signal), INF-F1-5 (trim; "No code change" then, moved here by
  f2-plan condition 8 / Q4 "its fix lives in both scripts' sh()"); sets-r1 RATES-1 (per-profile rates from measured
  whole-run bytes, P0's from P0, source in comment), RUNS-LINE (print each listed mark's verdict for the set's own
  profile part); r2 INF-R2-5 (run() returns signal + self-test, home = check.mjs follow-up, item 18).
- C2 Scripts: main folder copies = committed (runs 18c1dcf6 at 791bc55; r2 94564f8c at f1f131b). All plan line refs
  hold: :10-14, :35-36, :36/:444, :66-69, :53-56, :78-79, :256 trim, :323 reading, :313/:315 start reads, :452/:454
  "by a signal"; r2 :114-118 sh, :170 reading, :157 step. check.mjs:1921 returns no signal (confirmed).
- C3 INF-F1-5: reading() feeds both start (:324) and unchanged (:387), so keepLead applies to both sides alike; start
  spike reading always "" (start refuses tracked changes :313-314); rev-parse has no leading space; porcelain output is
  never whitespace-only, so the compare's result is unchanged. Display gains the first line's leading space. OK.
- C4 RUNS-LINE rule emulated (python3 -I) on all five verdict files: P3 -> -2 SS runs, -17 L1 P3, -57 L1 P3; B1 -> -2 SS,
  -17 L1 B1, -57 L1 B1, -18 L1 B1; P2 -> -2 SS, -17 L1 P2, -57 L1 P2; P0 verdict.json -> -2 SS, -17 L1 P0, -57 L1 P0 (no
  bites: all runs 0); P0 verdict-bites.json -> no parts, the six bites. Each set's own parts and bites exactly. Plan's
  sample (P3 block, P0 bites block) matches the files word for word. Bites have no why field; invalid runs carry
  invalid_why (not copied: fine, the file is the record).
  Counts for the helper test: P3 3 parts / 6 why; B1 4 / 9; P2 3 / 4; P0 verdict.json 3 / 2, 0 bites; P0 bites 0 / 6 bites.
- C5 RATES-1, du -sb today (.gate/netcode-proof-test/runs/): P3 178,922,504 / 181,429,340 / 183,256,573 (folder
  543,998,603); B1 285,578,840 / 283,081,680 / 291,324,004 (860,377,134); P2 271,216,673 / 272,819,713 / 272,351,303
  (816,778,012); P0 measured 514,005,562 / 514,868,190 / 520,269,430, bites pos-4cm 603,630,452, yaw 567,497,932, pitch
  518,771,189, -17 bites 55,602,928 / 55,840,142 / 55,602,244 (folder 3,406,486,626). All plan figures hold. Rounded-up
  rates 183.3 / 291.4 / 272.9 / 603.7 MB correct. Projections 524.4 / 833.7 / 780.8 / 3,614.6 MiB and old 359.9 / 545.3 /
  628.9 / 3,073.3 correct; ratios correct. Each new >= measured (P2 margin thin: 780.8 vs 778.9). P0's one rate is set by
  bite pos-4cm and applied to measured runs too: +366 MiB over measured at P0's start check: conservative, named in risks.
- C6 Pre-check sentence: folder = SS-P0-s4.rec 16,950,444 (= entry.bytes) + marks.json 372,637 = 17,323,081; projection
  17,025,936 = entry + 3 x 25,164 (pairs go to results/runs.txt, not the folder). Difference 297,145, not "only the
  marks.json copy, 372,637": Low wording; conclusion (immaterial) holds.
- C7 r2: record run r2-e59ece9ecafc du -sb 119,159,321 (113.6 MiB) vs r2Projected 169,813,722 B (161.9 MiB) ✓; largest
  suite 24,813,880 B -> 302.2 + 2.3403 x 23.66 = 357.6 MiB ✓.
- C8 Six-point fit, my own least squares on the judge-apart logs' .jsonl bytes and ru_maxrss: a = 302.21, b = 2.34031;
  residuals -60.4, -7.7, +39.0, +49.1, +70.1, -90.0 MiB ✓. Projections 1,529.5 / 2,253.3 / 2,129.5 / 4,344.4 / 4,719.3
  MiB ✓; the line at 4,246.4 MiB of logs ✓.
- C9 ERROR in plan 2.7: "at P0's measured bytes, the six-point fit gives about 8.2 GiB". P0 logs 3,406,088,069 B =
  3,248.3 MiB -> six-point 7,904 MiB = 7.7 GiB; four-point (212.3 + 2.5062x) 8,353 MiB = 8.2 GiB. My sets-r2 sentence
  "about 8.2 GiB" was the four-point fit's (ambiguous where I put it). Low; fix the clause.
- C10 Memory: MemTotal 16,473,776 kB (15.7 GiB), MemAvailable 15.2 GiB now, swap 0. "about 3.4 times" is of total
  (3.2 of available): fine, no finding.
- C11 Gaps: no cmp-precheck though `head` and sh() are shared (F2 cond 4(a) had it); no fail-P3 (single-call path
  changes); parse guard covers JSON.parse only: setParts on `{}` throws TypeError (for..of on object) -> would lose
  runs.txt after a 70-minute set (F2-plan Q4's worry) -> Medium; precheck must print no judge line; runs.txt order of
  the new lines not fixed (SPLIT_LINE no longer "directly above" both judge lines); judgeRoom's line form not given;
  evidence agent's model/effort not stated (CLAUDE.md).
- C12 INF-F1-3: run() has no signal; INF-R2-5's home is the check.mjs follow-up (item 18), whose own text already names
  r2's step(); adding runs' judgeCall there fits. Report :193/:306 already say so. Q4 (a).
- C13 Q answers: Q1 (b), Q2 (a), Q3 (b) (it is sets-r1 Q2's own rule, before anything is made), Q4 (a), Q5 (a), Q6 (a).
- C14 F2's harness keeps cmp-precheck (9/9) and fail-P3 (3/3) stagings (.gate/u4r-f2/logs/summary.log): adding them
  to F3's list is cheap.
- Verdict: APPROVE WITH CONDITIONS (no High). Size expected 100-130 insertions, 20-30 deletions. Nothing to Mustafa.
