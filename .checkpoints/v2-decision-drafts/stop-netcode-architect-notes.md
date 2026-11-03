# stop-netcode-architect notes

- step 1: started; output file created. Next: read inputs, verdict, map rows, plan.
- step 2: read inputs, verdict, plan, map rows (mine: RC1-3,5 H1-6,10,11 S1-5,8,9 N1-3 RW1-9,13 O15-17 M14,15 X1,2,5) and every ADR 0001 place. Docs unchanged since 4374a1c. Next: read apply_fixes.py + glossary check, then write blocks.
- step 3: FIX O15 written (header split into the template lines; Status stays Proposed). Next: H3+S3 summary line 11.
- step 4: FIX H3,S3 (summary) and H4 (Requirements tick) written. Next: H2,S2 part 2 (lines 119-129).
- step 5: FIX H2,S2 (part 2 rule: goal budget + early signal + slice 6 decides; readings marked) written. Next: RW4 part 5, RW2/RW3 part 8.
- step 6: FIX RW4 (part 5 buffer aim), RW2 (part 8 cap rule), RW3 (part 8 arithmetic: would clip; cap from measured rewind) written. Next: M14 part 10 certs (430-438), RC2 part 11 (461-463).
- step 7: FIX M14 (part 10 certs), RC2 (part 11 crash), N3 (part 13 profiles: split decided, loss per direction as reading) written. Next: O16 part 14 (548-550), H5,S4 budgets row 694, RC3 713, RW8 731.
- step 8: FIX O16 (part 14), H5,S4 (budgets row), RC3 (failure row -> PM-0001-59), NOTE RW8 (verdict keeps anti-cheat text), FIX RW6 (trade-off), RW7 (revisit) written. Next: open questions H1, S1, N1, RW1, RW5, X1, N2, RC1.
- step 9: FIX H1 (OQ-0001-1), S1 (OQ-0001-2), N1 (OQ-0001-3) written. Next: RW1 (OQ-0001-5, partial), RW5 (OQ-0001-6), X1, N2, RC1.
- step 10: FIX RW1 (OQ-0001-5 in part), RW5 (OQ-0001-6 aim), X1 (OQ-0001-14) written. Next: N2 (OQ-0001-16), RC1 (OQ-0001-18), O16-2 (1176), M15 (1197-1200).
- step 11: FIX N2 (OQ-0001-16), RC1 (OQ-0001-18), O16-2 (spike intro), M15 (item 1 certs; lines 1199-1201) written; N3 regenerated in place (loss wording). Next: H6,S5 item 3; RW9 items 4,5; H10,S8,O17 step 1; S8-2 step 8; RW1-2 step 6; H11.
- step 12: FIX H6,S5 (item 3), RW9 (item 4), RW9-2 (item 5) written. Next: migration step 1 (H10,S8,O17), step 6 (RW1-2), step 8 (S8-2), rollback H11, appendix S9, RW13, RC5, X2; NOTE X5.
- step 13: FIX H10,S8,O17 (step 1), RW1-2 (step 6 Feel Lab list drops -5: unmapped place, same answer), S8-2 (step 8), H11 (rollback) written. Next: appendix S9 (1641,1644), RW13 x5, RC5, X2; NOTE X5.
- step 14: Appendix A blocks S9, S9-2, RW13, RC5, RW13-2..5, X2 written (RW13-2/-5 reworded: answered in part). Next: NOTE X5, then remaining NOTEs check, apply dry-run, glossary, diff review.
- step 15: NOTE X5 written; FIX H2-2 (Takes list: ADR 0006 goal budget, unmapped place for H2) written. Sweep of my sections done (492 OQ-0006-15 = production domain, still open, no change). Next: apply dry-run, glossary, line lengths, cell counts, word diff review.
- step 16: apply dry-run: 42 blocks all ok. Glossary: fixed 3 exact "whole tick" hits (now "whole number of ticks") and dropped 2 "always"; re-running checks.
- step 17: FOUND and fixed: RW2 BEFORE was off by one (350-351 -> 349-350); word-multiset check caught it. All other blocks checked first/last lines join correctly.
- step 18: final checks: 42 FIX + 2 NOTE; every BEFORE once, whole lines; dry-run all applied; glossary exact 0 (in context 138->151: "the goal budget"/"goal modes", "N = 48"); no new prose line >120; cells unchanged; no line under a table; Pass marks section untouched. Next: return result.
- step 19: header block matched to engine-architect shape (" ·" kept); preface added to fixes file; final dry-run on a fresh copy: 42 applied. DONE
- finished 13:20 Amsterdam (the step labels replace clock times I had guessed wrong).
DONE
