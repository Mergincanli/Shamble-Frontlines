# verifier-3d-critic: completeness critic of step 3d (plan-amendments-2.md), all six topics, read-only (task: prompts/verifier-3d-critic.txt)

Started 2026-10-31. Head under test: 4cf8633 (diff base 7c6e0d7).

## Check 1 (approved ids and shared questions at 4cf8633): DONE, result none
- My own script (scratchpad/critic/chk_after.py, chk_oq.py) on 4cf8633: 143 change ids parsed (same set as apply-3d-all.tsv); 6 (C17.b1-b3, C23.b1-b3) absent, their befores present (r1:2958, r6:683-688): right.
- 13 "NOT found verbatim" all explained: C1.3 (r14 fix item 1, r1:2050), C1.9/C11.9/C11.10/C17.3/C17.5/C9.4/C9.8 (glossary renames, verifier-r2-3d.md:12, r14 and r356 reports), C1.10 (C11.8 + R2 fix item 2, r2:2411-2414), C1.10b ("at p99" r2:2415), C10.17 (R5 fix item 2), C23.2 (OBJECT-1 nit), C11.12 (session-script parse artefact: inline "core"->"vCPU" greedy match; applied at r2:1725 and r2:3436).
- No replaced "before" text survives anywhere at head.
- 72 XT-OQ texts and 18 shared-question copies: all word for word at head (links stripped); one subsection each per file; R1 18, R2 15, R3 8, R4 8, R5 14, R6 9 headings. Rank 41: r5:1629 + part-7 r5:2256.
- Verdict-revised texts (C1.5, C1.9, C1.10, C1.10b, C1.21, C19.3, C19.8, C21.1, C21.2, C21.4, C21.8, C29.2, rank 9) are folded in resolutions.md.
## Check 2 (condition-4 recommendation changes no verifier checked): DONE, result none
- R1 items (C1.1, C1.5, C5.1-4, C6.1-2, C9.1, C9.3, C9.7, C29.1-3) and R4 (C1.15-18, C9.18-20): verifier-r14-3d-result.md check 2, K1-K11. R2 (C1.8-C1.11 incl. C1.10b, C9.9; INT-REC1/2/4 p99): verifier-r2-3d-result.md check 2 + table recompute; C1.8 also K3 (r14). C1.8, C1.11, C9.9 only under R2's blanket "each holds" (no line of its own); I re-read them at head: C1.11 5 % = 10 % x 50 % (0.83/1.67 ms) ok; C9.9 1 MB vs r2:2139-2140 "about 0.1 MB at 60 Hz for 60 movers", 54+15 = 69 movers still far under: holds; C1.8 r2:2292-2296 holds.
- R3 C9.14, R5 C19.1/C21.1/C22.1/C22.3, R6 C1.21/C19.5/C19.6/C21.4/C21.8/C22.5: verifier-r356-3d.md "20:55 Check 2" lines. R3 p99 marks: r356 final check item 1.
- R5/R6 time marks with no statistic (r5:1263, 1281, 1320, 1429, 1473, 1545, 1691, 1732; r6:669, 838) are single-run limits (GLOSSARY "Time statistics" one-off clause, 96a4c29): the rule changes none of them.
