Task: cs0-docs-keeper (docs-keeper's pass for change-set 0 of the build plan; brief docs-keeper-brief.md)

## Plan (blocks, in order)
B1. Read DECISIONS.md's five entries (2026-11-14 13:58:05 decision; its addition; go 2026-11-17 19:10:44; 20:03:57; 20:39:23).
B2. Read docs/STATUS.md fully; list the lines the facts make untrue.
B3. Read docs/ROADMAP.md "Recent work" (count entries) and the fold target "Build history".
B4. Read docs/REPO_MAP.md research and reports sections; docs/reports/README.md form.
B5. Read the plan report's "Since the plan" (carriers) and git log origin/main..HEAD -- docs.
B6. Edit STATUS.md.
B7. Edit ROADMAP.md (one entry; fold oldest if over 12).
B8. Edit REPO_MAP.md; run node .claude/scripts/repo-map-check.mjs.
B9. Edit docs/reports/README.md.
B10. Checks: numstat (added <= 30 across the four), no line > 120 cols outside tables.
B11. DONE + answer.

## Notes
- B1 DONE (DECISIONS.md lines 129-248 and 434-493). Entry titles: "Taken by Mustafa, 2026-11-14, 13:58:05 UTC: the game
  is built now" (l.453); addition "2026-11-14, 14:12:13 UTC" (l.434, label 15:03); "2026-11-17, 19:10:44 UTC: his go
  on the build plan" (l.183); "2026-11-17, 20:03:57 UTC: four small things" (l.157); "2026-11-17, 20:39:23 UTC:
  SIZE-0 (a)" (l.130). Quotable (exact): CHANGE 1 "No game code is built before change-set 0 is on main." and "The 2.3
  chat stays stopped for now: I pass it nothing until its plan has been checked."; go: "build change-set 0 now, then
  the change-sets in the plan's order, each as its own pull request that stops for my go on the merge. This go covers
  no merge."; CHANGE 2: "That chat exists since 2026-11-15: branch claude/optimistic-albattani-mdoonz, main model Opus
  5.5, Fable 5.2 on its advisor tool. It runs the preview hosting check in a separate test repository of mine and
  touches nothing of yours."; "Before change-set 1 starts, add a short Part 15 to your plan, "The second lane"."; 6.
  PREVIEW-START "(a). It has started, in the second builder chat"; SIZE-0 "(a). It goes on as one pull request." and
  "No game code before change-set 0 is on main: that stands."; 2026-11-14 decision changes SPIKE-ORDER (b) "wave 1 runs
  one heavy spike at a time" for steps 2.4, 2.5, Slice 1. 2026-11-14 block: record "by step 2.3's first docs
  change-set" -> changed by CHANGE 1 to change-set 0. Next: B2.
- B2 DONE (STATUS.md, 120 lines; branch claude/wonderful-hamilton-jdmlnf, tree clean; HEAD 323db1b). Facts: #55
  merged_at 2026-11-17T17:09:33Z, merge c94c81b (gh REST); #56 open draft, head claude/wonderful-hamilton-jdmlnf,
  base c94c81b. Archive has no #55/#56 row (archive outside my 4 files: tell caller). Untrue lines: l.5 Last updated
  (DOCS-54, main 82c605e); l.13-15 Last merged #54 -> #55; l.31 In-flight #55 row -> #56 row; l.37-39 Next item 2
  (2.3 plan waits for go; "Wave 1: one heavy spike at a time"); l.42 Waiting "His go for #55" -> #56 + Part 15's six
  questions; Phase l.8-12 lacks the build (2.4, 2.5, Slice 1 now); second builder chat nowhere; l.43-44 "name the
  laptop and desktop" still true (classes named, machines before cs 6) - optional precision. Follow-ups section
  exists (ids here, text in archive; I can't edit archive -> short text + pointer to rules file / plan report).
  Routines: nothing his words change (leave). Next: B3.
