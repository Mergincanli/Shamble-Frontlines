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
  "wrong" or "unsupported" goes to a skeptic. Save the 1.99.1 `f64` page's §Precision lines for `div_euclid` and
  `rem_euclid` too: d0 saved only the `f32` lines (verifier-fixcheck-d0-scratch/excerpts.md:118 names the `f64` lines,
  "txt 313-339", without quoting them), so d3b's row 1 rests on that heading for `f64`. If the session ends before that, add fact 2 to the appendix §7.3 bullet
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
  449356c..HEAD: none); the last commit touching any of them is 8fa7173 (2026-11-02 10:36 UTC), before the final
  round's 604a497 (19:46 UTC), and ADR 0001's ledger last changed in 3fc7087 (2026-11-01). So the final round's and the
  touch-ups' claims (1.97.0's div_euclid 3, the Molotov's `pellets` 0, the LOOK_V2 Expected lines, the PM-0004-22
  clause, PM-0003-30's median rule) are in no ledger, and one ledger row now contradicts its draft:
  netcode-architect-adr0001-claims.md:340, row 307, "the 30 Hz marks may use a stand-in, leaving 7.49 ms", where
  ADR 0001:1047 now says 4.69 ms (fixcheck-d1 F1). No d1, d2 or d3 unit checked rule 9 (grep "ledger"
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
  placeholders. Items found by reading it:
  - report:26 "(112 open)" (D3C-4); report:38, 43, 44 hold {TOUCHN}, {D3}, {HEAD}; appendix:561, §7.1's d3 row, is {D3}.
  - report:34 and appendix:563-564 "five rounds: 30 upheld, 9 partly, 2 refuted": the five results files' own count
    lines give 9/2/1, 6/6/1, 4/1/0 and 5/0/0 for rounds 1-4 and three upheld parts in round 5 (27 upheld, not 30;
    not reconciled here), and skeptic-6 (PARTLY on D3A-1) is a sixth round not counted.
  - report:33-34 "A skeptic re-checked every "wrong" or "unsupported" verdict first" and the deviation at report:66-68
    ("the owner checked four "wrong" verdicts … (fixcheck-d0 row 9; fixcheck-d1 F1, F2, F10)"): d1's three
    "unsupported" rows (N12, I21, I24: F13 twice, F14) and d2's nine wrong or unsupported rows (32, 34, 51, 73, 77,
    92, 96, 111, S12) also went to their owners with no fresh skeptic (verifier-gdcheck dry-ran the game-designer's
    four). d3a and d3b then judged every fixed text against its source, so the substance is checked; the report's
    wording undercounts. d3a C15 (unsupported, labelled) had no skeptic either (G4).
  - appendix:612-613 "Not read at their sources" names PM-0003-2's WebGPU model and `write_length_prefix`, not rustc
    1.99.1's `div_euclid` (G1).
- How to close: when the session fills the placeholders, a short verifier pass checks each number and claim in the
  report's Evidence and Deviations against the verdict files: skeptic rounds recounted with skeptic-6, the deviation
  widened to "every wrong or unsupported verdict of fixcheck-d0, -d1 and -d2", {TOUCHN} = 53 FIX blocks and 3 NOTEs
  (this critic's rebuild), and the not-read list completed.
END

## Checks this critic ran itself (no gap)
| Check | Method and result |
|---|---|
| Every changed line came from a judged block | rebuild_all.py (scratch): the 53 FIX blocks of the nine touch-up files, applied to 449356c in commit order, give HEAD 88fec25 byte for byte for all 8 changed files (SPIKE_PLAN, 0001-0006, README); ENGINE_OVERVIEW unchanged; every BEFORE once. d3a judged 14 (netcode 3, infra 7, engine 0002 1, test-engineer's d1 three), d3b 39 (test-engineer's d2 eight, game-designer 5, synthesis 1, rust-reviewer 1, PM22 1, re-pins 23) = 53. d3b's "42 FIX blocks" is its script's count, which also took the test-engineer's three d1 blocks; its breakdown "8 test-engineer" sums to 39. Record slip only. |
| Every NOTE judged | 3 NOTE blocks exist (rust-reviewer D3A-1-PM22, D3A-1-EVIDENCE; test-engineer PM22-EVIDENCE); d3b judged all 3. The other files' reasoning text (touchup-test-engineer-fixes.md lines 3-11, 30-32, 42-46, 56-60, 80-85, 95-99, 147-148, 166-167, 177-179; the netcode, infra and engine-architect notes) was judged by d3a or d3b, and gdcheck dry-ran the game-designer's five. |
| d1 findings | F1, F2, F6, F7 (in the key), F9, F10, F11, F12, F13, F14 fixed and judged by d3a; F3, F8 in §7.3 with reasons; F15 done by the synthesis and judged by d2 (S2-S8, consistency rows). No F4 or F5 appears in d1's verdicts or notes (numbering gap, no lost finding). d0's optional #5, #7, #8b are marked "not findings" by d1. |
| d2 findings | F1a, F1b, F2, F4, F5, F8, F9, F10, F12, SF1 fixed and judged by d3b; F6, F7, F14 in §7.3; F13 fixed for PM-0003-17 only, its other places not in §7.3 (G2). No F3 appears in d2's verdicts or notes; F11 became F1b (d2 notes line 9). |
| Rule 4 (trace) and rule 11 (template fields) | Each hunk mapped to its section: the Decision hunks are 0001 part 4 (wording), 0002 part 8 (retention), 0004 part 7 (allow-list policy), 0005 AB-1, AB-3, AB-5 (restorations and the pool's size), 0006 parts 6 and 15 (pointers): no new engine feature, so no new need to trace. No hunk lies in a "Hard constraints", "Migration plan" or "Rollback" section. Neither d3a nor d3b named rules 4 and 11; both hold. |
| Rule 7 (open questions) | No open question added or removed (114 numbered at 449356c, d2 S16, and at HEAD, d3c counts.py); d3c's labels.py finds no label lost in any touched OQ. |
| Line length on all added lines | 107 added lines, 0 non-table lines over 120 characters (d3a had not stated it for 0001, 0002, 0006). |
| Pointers into ADR 0005 outside README | 0005 grew by 2 lines; no other doc in docs/adr, SPIKE_PLAN, ENGINE_OVERVIEW, STATUS, ROADMAP or the report cites an ADR 0005 line by number (grep), so README's 23 re-pins were the only ones needed. |
| F9's "(at least 1)" | legacy throws one Molotov per press (shamble-game.js:1469, `throwMolotov()`; the pellets loop at :1474 is for guns), so the counted 1 matches legacy. |
| D3C-2 and D3C-4 spot-checks | 0005:106-107 cites S5, S9, S10; README:328-333 holds S1, S3, S4, S6, S11, S12 only. OQ-0003-14 (0003:885-886) reads "Answered by its decider"; README:23 still says 112 open. Both stand. |
| The four owner-checked verdicts | d3a section 3 re-judged all four from the sources (d0 row 9; d1 F1, F2, F10); skeptic-6 then tested D3A-1 (PARTLY) and the rust-reviewer and test-engineer touch-ups took its point (a); d3b judged both. |

## Not verified by this critic
- No page fetched (as asked): G1's three facts and the 1.99.1 `f64` lines stay unread.
- No Wasm run of the Euclidean cases (no wasm32 target; nothing installed).
- The skeptic tally in G5 was recounted from each results file's own count line, not from every row.

## Notes (one line per unit of work)
- created file; next: read the four unit verdict files, skeptic files and touch-up fix files.
- read d3a verdicts (19 claims, 14 blocks on 0001/0002/0006, D3A-1, D3A-2) and d3b verdicts (33 claims, 42 FIX + 3 NOTE, D3B-1, D3B-2); next: d3c, d3d, skeptic files.
- read d3c verdicts (sweeps; D3C-1..4), d3d verdicts (0 of 3 facts read, before 08:30), skeptic-6 (D3A-1 only: PARTLY); next: list touch-up blocks vs judged blocks.
- listed d1 findings (F1-F3, F6-F15; no F4/F5 assigned) and d2 findings (F1a, F1b, F2, F4-F10, F12-F14, SF1; F11 renamed F1b); appendix §7.3 read (lines 584-613). Next: touch-up blocks vs units a/b.
- rebuild_all.py (scratch): all 53 FIX blocks of the 9 touch-up files applied to 449356c give HEAD exactly for all 8 changed files (SPIKE_PLAN, 0001-0006, README; ENGINE_OVERVIEW unchanged); every BEFORE once. d3a judged 14, d3b 39 (its "42" counts the 3 d1 test-engineer blocks d3a judged; its breakdown says 8 test-engineer): no changed line unjudged. Next: reasoning text and NOTEs inside the touch-up files.
- d1 F1-F3, F6-F15 and d2 F1a-F14, SF1 mapped: all fixed or in §7.3 except d2 F13's other places (D3B-1, still absent from §7.3 at HEAD). d1 F15's SPIKE_PLAN and event-rule halves were judged by d2's synthesis section (S7, S8, consistency rows). F9's "(at least 1)": legacy throws one Molotov per press (shamble-game.js:1469, throwMolotov), checked. Next: rules 4, 7, 9, 11 on changed text; ledgers; line length; report diff.
- rule 9 (claims ledger per draft): no *-claims.md changed since 449356c (last commit 8fa7173, 11-02 10:36 UTC, before 604a497); no d1, d2 or d3 unit checked ledgers; the report records no deviation. Gap G3. Next: rules 4, 7, 11 and line length on the 53 hunks.
- gap blocks G1-G5 written. Next: verify 'line 99' reference in G5, then checks done list and DONE.
- G5 rewritten with exact report lines (26, 34, 38, 43, 44, 66-68; appendix 561, 563-564, 612-613). Next: closed-by-critic checks list, then DONE.
- checks-ran and not-verified sections written; G1 extended with the f64 lines. Next: final read-through, DONE.
- G3 sharpened: ledger row 307 (netcode-architect-adr0001-claims.md:340) still says 7.49 ms, ADR 0001:1047 says 4.69. All blocks complete. DONE

DONE
