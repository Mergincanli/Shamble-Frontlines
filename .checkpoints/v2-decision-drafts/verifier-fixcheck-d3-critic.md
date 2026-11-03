# fixcheck-d3 completeness critic

Branch claude/laughing-lovelace-ox61se. Started 2026-11-03. Nothing fetched, no doc edited, no commit.
Scratch: .checkpoints/v2-decision-drafts/verifier-fixcheck-d3-critic-scratch/

## Gaps (each block ends with END)

### G1 (high for completeness): unit d judged none of its three facts
- What is missing: verifier-fixcheck-d3d-verdicts.md ran 08:07-08:10 Amsterdam, fetched nothing (clock rule) and
  judged 0 of 3: (1) the WebGPU error model behind PM-0003-2 (docs/adr/0003-rendering.md:1136); (2) rustc 1.99.1's
  `div_euclid` behind ADR 0004 part 7 (0004:270-278) and PM-0004-22's bite; (3) std `Hasher` (`write_length_prefix`,
  `write_usize` defaults) behind ADR 0004 part 9 (0004:334-341) and PM-0004-21. Its "PASS WITH NOTES" means only "no
  fault found"; the unit file says so itself ("all three still have to be read on a later run").
- Local evidence that exists (not a source read): fact 2, rustc 1.97.0 gives 3 (d3a, d3b, skeptic-6 runs; the IEEE
  steps give 3 on any target); fact 3, a logging Hasher on 1.97.0 gets write_usize(len) before slice elements
  (d3d scratch hasher_probe.out).
- How to close: re-run unit d after 08:30 Amsterdam (the clock read 08:17 during this critic), fetching only the
  allow-listed hosts its prep names (www.w3.org or gpuweb.github.io, developer.mozilla.org, doc.rust-lang.org; all in
  .claude/settings.json), filling each fact's verdict under its prep block, with excerpts and a fetch audit; any
  "wrong" or "unsupported" goes to a skeptic. If the session ends before that, add fact 2 to the appendix §7.3 bullet
  "Not read at their sources" (it names facts 1 and 3 only) and state in the report that unit d did not run.
END

### G2 (medium): d2 F13's other places are neither fixed nor listed in §7.3 (D3B-1, still open at HEAD)
- What is missing: docs/reports/2026-11-01-v2-decision-drafts-appendix.md §7.3 (lines 584-613) names d2 F6, F7 and
  F14 as deferred but not F13; grep "F13" in the appendix: 0 hits. ADR 0003:959-960, 993, 1367 and SPIKE_PLAN 247,
  255, 267 keep "legacy screenshot(s)"; the test-engineer's block reason (touchup-test-engineer-fixes.md:177-179) left
  0003:960 and 1367 "to their author, graphics-engineer", and no block followed.
- How to close: the main session adds D3B-1's sentence to the §7.3 "From the final re-checks" bullet (d3b verdicts
  line 103 gives the words), or graphics-engineer fixes the three ADR 0003 lines; then a fresh check that the bullet
  or the lines are at HEAD.
END

### G3 (medium): rule 9, the claims ledgers, was not checked and the ledgers are stale
- What is missing: appendix §4 rule 9 says each draft keeps a claims ledger and "the verifier works from it and checks
  that no claim in the draft is missing from it". None of the 16 *-claims.md files changed after 449356c (git log
  449356c..HEAD: none), and the newest is 2026-11-02 10:36 UTC, before 604a497 and 449356c, so the final round's and
  the touch-ups' claims (4.69 ms, 1.97.0's div_euclid 3, the Molotov's `pellets` 0, the LOOK_V2 Expected lines, the
  PM-0004-22 clause, PM-0003-30's median rule) are in no ledger. No d1, d2 or d3 unit checked rule 9 (grep "ledger"
  in their verdict files: 0 hits). The report's "Decisions and deviations" does not record it.
- How to close: either the owners append the touch-up and final-round claims to their ledgers and a verifier checks
  coverage, or the report and appendix state the deviation: from the final round on, the verifiers' claim tables
  (fixcheck-d1 sections 1a-4c, d2's claims table, d3a section 1, d3b claims) stand in for the ledgers. The second is
  one sentence and matches what was done.
END

### G4 (low): the new findings of this round have no skeptic and no disposition yet
- What is missing: D3B-1, D3B-2, D3C-1, D3C-2 (medium), D3C-3 and D3C-4 are open at HEAD 88fec25 (0005:106-107 still
  cites seams S5, S9, S10 that README's Seams table, lines 328-333, lacks; README:23 still says "112 of them open"
  while OQ-0003-14, 0003:885-886, is "Answered"; 0004:275 still says "an exact reference"; §7.3 still lacks F13).
  Only D3A-1 had a skeptic (skeptic-6, PARTLY), and D3A-2 is fixed (appendix line 600, 5c1dfec). The critic
  spot-checked D3C-2 and D3C-4: both stand as written.
- The one "unsupported" claim verdict of the round, d3a C15 (0006:953, the TCP 443 listener), had no skeptic. It is
  labelled "(no source read)" inside OQ-0006-16, whose options are "not researched", so rule 2's "becomes an open
  question" is met; a skeptic is optional.
- How to close: fix or defer each of the six with its reason (§7.3), the medium D3C-2 first (three rows in README's
  Seams table from the decision map's lines 207, 211, 212, or drop the codes at 0005:106); send any finding the
  session disputes to a skeptic; then re-check only the changed lines.
END

### G5 (low): the report's own changes since 449356c were judged by no unit
- What is missing: docs/reports/2026-11-01-v2-decision-drafts.md and its appendix changed in c6e581a, 08534a2,
  4b76636, f9ec6c8, 5c1dfec and 54d3fa0; d3b lists the report as not read, and d3c only checked line 26 and the
  placeholders. Concretely: line 26 "(112 open)" (D3C-4); lines 38, 43, 44 hold {TOUCHN}, {D3}, {HEAD}; line 35
  "five rounds: 30 upheld, 9 partly, 2 refuted" sums rounds 1-5 (skeptic-1..5 results: 9/2/1, 6/6/1, 4/1/0, 5/0/0
  and round 5) and leaves out skeptic-6 (PARTLY on D3A-1), and its "A skeptic re-checked every 'wrong' or
  'unsupported' verdict first" does not hold for d3a C15 unless labelled claims are excluded; line 99 of the
  appendix-facing "Not read" list misses fact 2 (G1).
- How to close: when the session fills the placeholders, a short verifier pass (or the next fixcheck unit) checks
  each number and claim in the report's Evidence and Deviations against the verdict files, with skeptic-6 counted
  (six rounds) and the touch-up count = 53 FIX blocks and 3 NOTEs (this critic's rebuild).
END

## Notes (one line per unit of work)
- created file; next: read the four unit verdict files, skeptic files and touch-up fix files.
- read d3a verdicts (19 claims, 14 blocks on 0001/0002/0006, D3A-1, D3A-2) and d3b verdicts (33 claims, 42 FIX + 3 NOTE, D3B-1, D3B-2); next: d3c, d3d, skeptic files.
- read d3c verdicts (sweeps; D3C-1..4), d3d verdicts (0 of 3 facts read, before 08:30), skeptic-6 (D3A-1 only: PARTLY); next: list touch-up blocks vs judged blocks.
- listed d1 findings (F1-F3, F6-F15; no F4/F5 assigned) and d2 findings (F1a, F1b, F2, F4-F10, F12-F14, SF1; F11 renamed F1b); appendix §7.3 read (lines 584-613). Next: touch-up blocks vs units a/b.
- rebuild_all.py (scratch): all 53 FIX blocks of the 9 touch-up files applied to 449356c give HEAD exactly for all 8 changed files (SPIKE_PLAN, 0001-0006, README; ENGINE_OVERVIEW unchanged); every BEFORE once. d3a judged 14, d3b 39 (its "42" counts the 3 d1 test-engineer blocks d3a judged; its breakdown says 8 test-engineer): no changed line unjudged. Next: reasoning text and NOTEs inside the touch-up files.
- d1 F1-F3, F6-F15 and d2 F1a-F14, SF1 mapped: all fixed or in §7.3 except d2 F13's other places (D3B-1, still absent from §7.3 at HEAD). d1 F15's SPIKE_PLAN and event-rule halves were judged by d2's synthesis section (S7, S8, consistency rows). F9's "(at least 1)": legacy throws one Molotov per press (shamble-game.js:1469, throwMolotov), checked. Next: rules 4, 7, 9, 11 on changed text; ledgers; line length; report diff.
- rule 9 (claims ledger per draft): no *-claims.md changed since 449356c (newest mtime 11-02 10:36 UTC, before 47eadd9's final round was even verified); no d1, d2 or d3 unit checked ledgers; the report records no deviation. Gap G-LEDGER. Next: rules 4, 7, 11 and line length on the 53 hunks.
- gap blocks G1-G5 written. Next: verify 'line 99' reference in G5, then checks done list and DONE.
