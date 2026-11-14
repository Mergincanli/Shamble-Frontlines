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
