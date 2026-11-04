# verifier fix-check, B3 (PR #36), 2026-11-04
Base of fixes: bfaf142 (uncommitted fixes in worktree). Upheld findings to check: GD-1, F1 (narrowed: drop R2-BUD-OQ5,
R4-OBST-OQ1, AB15; keep OQ-0001-9), F2, F3, F4, F5, F6, F7, CC-1 (+ADR 0005:991), CC-3, CC-4, CC-5, CC-6, CC-7, CC-8.
Refuted (not checked): GD-2, GD-3, CC-2.

## Verdicts (as I go)
- Gate --no-smoke (worktree): GATE PASS, [1]-[4],[5b]-[5d] PASS, symbols +0 -0, 5a SKIP. repo-map-check: "repo map OK: 113 entries cover 100 required paths".
- Appendix: walk block report:86-108 == mustafa-2026-11-04.md:2-24 (cmp identical); answers report:147-150 == :28-31 (identical);
  questions report:112-143 == advisor-plan-rev1.md:139-170 (diff identical).
- Report numbers checked vs 4893ab6: numstat matches all 7 files (LOOP +109-16, UNITS +51-3, DN +12-2, README +18-13, STATUS +64-23,
  ROADMAP +5-4, REPO_MAP +2-2). Tags: OPEN LOOP 4->6, UNITS 0, DN 1; IDEA 2/4; LATER unchanged; raw `[DEFAULT]` 22->26; DEFAULT bullets 20->20.
- NOTE: b3-check.mjs's base() is `git show HEAD:` (b3-check.mjs:201), i.e. bfaf142 now, not main: its rerun "PASS" for base-relative
  checks (E3 raw, E5 LATER/DEFAULT) compares against the build commit (prints raw 24->26, "new [DEFAULT] text: none"). Report's
  numbers are still right vs main (my counts above), so evidence holds; the script's rerun is weaker than the report implies. Low.
- b3-check rerun: 90 PASS, 0 FAIL. e1-appendix rerun: 43 fragments, 0 failing.
- ADR files: 0 changed vs 4893ab6 (diff --stat docs/adr/0*.md empty). CC-1 line cites verified: 0004:502 in part 17 (489-514),
  0004:518 in part 18 (515-), 0005:796-797 triangle `[DEFAULT]` + Spy, 0005:991 spawn row, 0005:994 Spy row.
- vcheck.mjs rerun on docs (.gate/vcheck-fixcheck.txt): E1/E2 ALL PASS (his words byte for byte in LOOP_V2/UNITS).
- Register rows: 12 changed vs 4893ab6, 0 lost words (word-order script); only the R0 paragraph lost text (F1/F6, intended).

## Per finding (working tree)
- GD-1 RESOLVED: LOOP_V2.md:104-105, STATUS.md:431-432. Source: his "stealthy and fast, and sabotages" (walk); open, not decided.
- F1 RESOLVED (deviates from skeptic, recorded): README:53-55, 14 ids; report:58-66 lists 7 drops with reasons. Primary
  sources: OQ-0001-9 (ADR 0001:1159-1164 melee/short-range at attacker view time; his leaps are [IDEA] sketches: drop is
  defensible), R2-BUD-OQ5 (README:172), R4-OBST-OQ1 (:195), AB15 (:132), OQ-0004-11 (:113), R2-INT-OQ3 (:175), R2-ROOM-OQ3 (:178):
  none plainly touched by his words. No dropped id referenced elsewhere in B3's text.
- F2 RESOLVED: STATUS.md:156 "faction asymmetry: his walk and B3-Q1 bear on it (OQ-0005-CD5)"; matches README:53, :119.
- F3/CC-3 RESOLVED: STATUS.md:431-435 now lists AB14 coverage + ability press (LOOP_V2:26-27) and the war (LOOP_V2:148, §8).
- F4 RESOLVED: STATUS.md:218, :228, :231, :277 (after the blockquote, outside his words); target header STATUS.md:133.
- F5/CC-7 RESOLVED: STATUS.md:3 "10:21 UTC), part-b-test".
- F6 RESOLVED: README:51-52 AB1 exception; matches advisor rev1 cond 6 and README:66.
- F7 RESOLVED (report written; queue marks done; plan-text edits replaced by the report, disclosed report:56-57).
- CC-1 RESOLVED: LOOP_V2.md:219-223; all 5 ADR line cites checked (incl. 991; 518 is part 18).
- CC-4 RESOLVED: STATUS.md:27-29, :86; ROADMAP.md:71, :91. VISION_V2:36-37/59/86 contents match the description.
- CC-5 RESOLVED: README:119, :124, :125.
- CC-6 RESOLVED: queue/docs-pass-items.md:76,78,80,84,87 (outside repo).
- CC-8 RESOLVED: STATUS.md:134-136, :86; ROADMAP.md:91.

## New defects (all nit)
- N1 report body 81 lines (SKILL: at most ~60).
- N2 report:49 "README row 86" is line 91 at the working tree (OQ-0002-9 row; 86 on main).
- N3 report:29 "builder's script rerun ... PASS": b3-check.mjs:201 base() = HEAD (bfaf142), so base-relative checks ran
  against the build commit; numbers in report verified vs main by me, so only the wording overstates.
- N4 report:14 "Revision 2 ... all met": plan-text items (cond 2, 4, 6) met via the report instead; disclosed at :56-57.
- N5 report:21 128 chars (wrap ~120); :1 121, :27 122 fine.
Consistency LOOP_V2/UNITS/DESIGN_NOTES/STATUS/ROADMAP/README: no contradiction found. No ADR file changed.
VERDICT: PASS WITH NOTES
