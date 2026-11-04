# B3 builder: edit plan and notes (2026-11-04)

Worktree: scratchpad/b3, branch b3-design-walk, base 73d546f (main 4893ab6 + checkpoint). No commits by the builder.
Sources: plan-rev1-folded.md (L*, U*, N*, R*), b3-design-walk.md "Plan revision 2", advisor-plan-rev2.md conditions
1-10, mustafa-2026-11-04.md (his words, byte for byte).

## Plan (applied bottom-up in each file; each edit anchored by its old text)
LOOP_V2.md (21 edit sites): L17, L16 (+cond 4 line 105 and " and specialists"; +cond 5 ADR 0005 1263, 1278-1283),
L15b, L15a (cond 6: "tech building (the HQ)"), L14, L13, L12-112 (cond 4 exact text), L12-105 (rev2: replaced by two
lines), L11, L10, L9 (rev2 untagged pointer), L8 (rev2 two [DECIDED] sub-bullets), L7 (rev2 + cond 1 note), L6, L5,
L4, L3b, L3a, L2, L1, L0.
UNITS.md (5): U4, U3, U2, U1, U0 (cond 6: Factions and Infiltrators pointers replaced; tag line drops [OPEN]).
DESIGN_NOTES.md (7): N7, N6, N5 (cond 6: "(the HQ)"), N4, N3 (cond 6 text), N2, N1.
adr/README.md (13): R11, R10, R9, R8, R7, R6 (+cond 8 CD5 clause), R5, R4, R3, R2, R1, AB5 row 60 (cond 8), R0 (AB5
dropped: 21 ids).
Not mine (main session / docs-keeper): the plan text's §3 deviations list, §7 risk, §8 "Answered", §8.5 HQ tension,
counts and "4 open questions" wording; the report appendix; STATUS/ROADMAP/REPO_MAP; queue/docs-pass-items.md.

## Notes (one line per finished edit)
- LOOP_V2 done (21 sites): L17, L16 (+line 105, " and specialists", ADR 0005 1263/1278-1283), L15b, L15a, L14, L13,
  L12-112, L12-105, L11, L10, L9, L8, L7 (+cond 1), L6, L5, L4, L3b, L3a, L2, L1, L0.
  Deviation: L8's (i) keeps his ';' and lowercase '"cuts their power"' (rev2's draft had '. "Cuts'); byte for byte wins.
  Note: ADR 0005 line 1263 is in OQ-0005-AB4, not AB5; L16 names both.
- UNITS done (5): U4, U3, U2, U1 (inside the status blockquote, before the Empire elite paragraph), U0 (tag line without
  [OPEN]; cond 6 pointers; the Factions pointer wrapped at 120, whitespace only).
- DESIGN_NOTES done (7): N7 (wrapped), N6, N5 ("tech building (the HQ)"), N4, N3 (cond 6 text), N2, N1.
- README done (13): R11 AB14, R10 AB10, R9 AB8, R8 AB7, R7 AB4, R6 CD5 (+B3-Q1 clause), R5 OQ-0004-12, R4 OQ-0004-10,
  R3 AB3, R2 AB2, R1 AB1, AB5 row (cond 8), R0 (21 ids, AB5 dropped). All edits applied; next: E-checks.
- Fix after E1: L8 (i) had rev2's "no new building;"; his words have "no new building:" — corrected to his colon.
- Checks: scratchpad/b3-check.mjs (outside the worktree) PASS, 0 failing (E1-E6, E10); E7 4 files; gate --no-smoke PASS
  (symbols +0 -0); repo-map-check OK (113 entries, 100 paths). Builder done; nothing committed.
