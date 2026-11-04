# Verifier review: B3 final diff (PR #36)

## Plan
1. Gate (`node tools/gate.mjs --no-smoke`, expect PASS +0 -0) and repo-map-check.
2. E1 (his words, incl. B3-Q answers, vs mustafa-2026-11-04.md and the user-relayed answers) to E10.
3. Advisor conditions rev1 (1-11) and rev2 (1-10): met or not, file:line.
4. R0's ids one at a time (condition 7); CD5/AB5 clauses; no register row lost words; no ADR file touched.
5. Spot-check 8 facts against sources.
6. Verdict.

## Notes as I go
- Gate: `node tools/gate.mjs --no-smoke` in the worktree (head = working tree, base 4893ab6): GATE PASS, [1]-[4], [5b]-[5d]
  PASS, symbols core 138 / game 714 / editor 353, +0 -0; 5a skipped. Output: b3/.gate/b3-gate.txt.
- repo-map-check: "repo map OK: 113 entries cover 100 required paths", exit 0.
- Diff (working tree vs origin/main, .checkpoints excluded): 7 files (DESIGN_NOTES, LOOP_V2, REPO_MAP, ROADMAP, STATUS,
  UNITS, adr/README). Edits uncommitted (HEAD 73d546f = main + checkpoint commit).
- E1/E2 (own script b3/.gate/vcheck.mjs + items.json, output b3/.gate/vcheck-out.txt): 45 checks (17 items split into
  sentences, Q1/Q2/Q3i/Q3ii, 5 carried 2026-10-31 + 4 carried 2026-11-03 lines) ALL PASS, each in its planned section
  and under its planned tag. His answers in mustafa-2026-11-04.md:28-31 are byte-identical to the relayed request.
  STATUS's quoted copy of B3-Q1..Q3 also verbatim. NOT run against the report appendix: the report does not exist yet.
- E3 PASS: the 9 replaced strings (S1-S5, S7, S8 + rev2's line 105 and " and specialists") occur in LOOP_V2 only in
  the §12 bullet (LOOP_V2:204-210), once each.
- E4 PASS: "Open since" 0 in every file under docs/; no "See ... open question" pointer left in LOOP_V2/UNITS/DESIGN_NOTES.
  `[OPEN]` LOOP_V2 6 (main 4: +AB14 sub-bullet :26, +war :146), UNITS 0, DESIGN_NOTES 1 (unchanged) = rev2 cond 7.
- E5 PASS: `[IDEA]` LOOP_V2 2 (:8 tag line, :125), UNITS 4 (:17, :49, :51, :54); none on a Decided item. `[LATER]` 6=6.
  Bullet-level `[DEFAULT]` 20=20: -1 (old "All three are shared") +1 (the Q-LBD reading, :230). The raw `[DEFAULT]`
  count 24 vs 22 is the two quoted old lines inside §12 (:204, :205), not tags.
- E6 PASS: LOOP_V2 §1-§13 in order (§13 at :221), UNITS §0-§5. Citations LOOP_V2 §2-§13 (and 3-4, 4-5) all resolve.
- E7 PASS: 7 docs files, no docs/adr/000*, no code; plus .checkpoints. The plan's E7 named 4 + docs-keeper's 3 = 7.
- E10 PASS: every relative link in UNITS, LOOP_V2, DESIGN_NOTES, adr/README resolves (script: none broken).
- Register (README): 12 rows changed, each keeps its old question text as a prefix and its Decider/Where cells
  unchanged (script on the -U0 diff); R0 keeps "war around the matches maps to no id." = 13 edits (rev2 cond 8 met).
  No docs/adr/000* file in the diff (no ADR decision text moved).
- R0 ids one at a time (rev1 cond 7): keep OQ-0001-4 (Molotov), OQ-0001-7 (reload tap), OQ-0001-10 (pickups),
  OQ-0004-11 (pools: effects, dropped guns), OQ-0005-2 (weapon type), AB9 (drops), AB11 (Molotov/Peacekeeper),
  CD8 (speed: weight idea, "lighter, faster"), R1-ABL-OQ6 (active reload, seats), R2-SYNC-OQ7 (seats), R2-INT-OQ1/
  OQ4/R4-LOS-OQ2 (the KGB reveal "on the map"), R2-INT-OQ8 (Spy's rules), R2-INT-OQ3 (steal money/tech: weak, kept),
  R2-ROOM-OQ3/OQ4 (rejoin, bots: weak, kept). NOT plainly touched: OQ-0001-9 (lag comp for cover and melee: the walk
  names neither), R4-OBST-OQ1 (vehicle in path: block/push/crush; the walker's leap is not "in its path"),
  R2-BUD-OQ5 (caps on vehicles and projectiles: the walk sets no cap), OQ-0005-AB15 (who sees picks: the walk says
  nothing on visibility). The builder kept all 21; condition 7's filtering was left to me: these 4 are my drops.
- B3-Q3 vs register (rev2 cond 8): only R2-INT-OQ8 (Spy) and the sight rows are touched, all already in R0; no row is
  about downed/revive/HQ/power; no new clause needed.
- Spot-checks (all confirmed): ADR 0005:1263 (AB4 quotes "sidegrade variants"); :1279, :1282 (AB5 options: "sidegrade
  variants", "grows in classes", "as unlocking a class"); :985 (Molotov "area blast", "no fire area"); :936 (AB-3
  "active reload's short damage boost"); ADR 0001:541 ("per unit type"); ADR 0005:1398-1401 (AB14 (a)/(b) and "first
  shot or ability press"); :1316-1317 (AB8 "always different", "fewer left than the offer's size (3)"); #35 merged_at
  15:50:40Z (gh) and commit 16:50:39+01:00 (git); README:23 118/98; advisor-final-35/notes.md:27 14:03:10; LOOP_V2:42
  (power plant prices, not added by L8); LOOP_V2:64/:80 Javelin Tech 2; appendix:195-201 (2026-11-03 words);
  r1-simulation-core.md:1386 ("sidegrade variants").
- Conditions: rev1 1,2,3,4,6,8,9 met; 7 done here (4 drops, not yet applied); 5 pending (report); 10 moot/met; 11
  replaced. rev2 4,5(L16),6,7,8 met; 1 doc half met (LOOP_V2:86-87), report half pending; 2 E1/E2 met by me, the
  deviations list/appendix/§8 pending; 3 pending (report §8.5); 9 STATUS/ROADMAP met, queue-file bookkeeping not done.
- Findings: R0 4 ids (README:53-55); STATUS:395 "answered in part" vs register "bears on"; STATUS:422-424 misses the 2
  new [OPEN]s; STATUS superseded lines :212/:222/:224/:261 unmarked (plan §5.4); STATUS:3 missing ", "; README:51-52
  "at most in part" vs AB1 (nit).
- Verdict: PASS WITH NOTES.
