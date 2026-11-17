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
- B3 DONE (ROADMAP.md 239 lines; Recent work l.117-239 holds 12 entries: #55 (l.120-127, "[2026-11-17, PR #55: in
  review]", ends "Next: the advisor agent's review of the final head, then his go."), #54, #53, #52, #48, #51, #50, #49,
  #46, #44, #43 (l.237, one line), #42 (l.239, one long line) -> adding cs0 makes 13: fold #42 into Build history (its
  newest entries l.81-87; form "- **Name (date):** slug (PR #n, sha, ...)"). Next / open l.100 says "step 2.3's ... plan
  waits for his go; wave 1 runs one heavy spike at a time" (changed by his decision: add a dated pointer clause).
  #55 merge facts (merge commit c94c81b msg): by merge commit, his go typed 17:08:17 UTC for head aea6f5b, after the
  advisor agent's APPROVE WITH FINDINGS of aea6f5b and the verifier PASS; merged_at 17:09:33 UTC. CI on c94c81b not
  checked (out of his decision's reach: leave STATUS l.23-25). Branch has not touched my 4 files (diff empty). Next: B4.
- B4 DONE. REPO_MAP (148 lines): docs/ table rows l.61-75; research files one row each (l.62-69 after the folder row
  l.61); reports: folder row l.71 + four individual rows (v2-research + appendix l.72-73, decision drafts + appendix
  l.74-75). repo-map-check: entries must exist; docs/reports/ and docs/research/ required as folders only. FINDING:
  docs/reports/README.md is 2 lines (naming rule + "paste the latest one"), NO index of reports (46 files in the
  folder) -> "in the index's own form" has no form; adding 3 lines would make a partial index. Decision: leave it
  unchanged, tell the caller. FINDING: docs/research/README.md has a topic table + "other files" list (GLOSSARY,
  UNREAD_SOURCES, l.33-34) the two new research files would join: outside my 4 files, tell the caller. Next: B5.
- B5 DONE. Plan report l.17-141 "Since the plan": carriers: ADR 0002 "one change-set per step" -> change-set 2's docs
  pass (l.23); machine lines in ADR 0001, ADR 0003, SPIKE_PLAN -> change-set 6's docs pass (l.26); O1 draw cap "a
  question for Mustafa with change-set M's plan" (l.116-120); O2 fifth material class two names (l.121-122); coat motion
  "Mustafa's call at change-set 10's pictures" (l.125-126); first real Blender export proves the rest (l.134-136).
  Verified: ADR 0005 l.2365-2366 PM-0005-54's "fail on the rifle file as reported (6 draws per pass: AER part 7, O1)";
  register docs/adr/README.md l.23 "119 numbered open questions, 96 of them open" (was 98 on main); OQ-0002-6,
  OQ-0006-18, OQ-0003-1, PM-0004-34 rows touched in the branch diff. Part 15 at ea851a8 (20:42:04 UTC):
  `.checkpoints/part-15/part-15-the-second-lane.md`, title "Part 15. The second lane (proposed 2026-11-17, revision 2;
  it waits for Mustafa's answers)", all six codes present; HEAD copy identical. Docs commits on branch: 3747bb3,
  4a1b0d7, 212f55b, 5f356ef, 65c8fe8. Next: B6 (STATUS edits).
- Budget plan (added lines by numstat, max 30): STATUS 18 (l.5:1; Phase "The build" bullet:4; Last merged:2 keeping
  l.15; #56 row replacing #55 row:1; Next item 2:3; Waiting #56 go:1 + Part 15:2; Follow-ups "From change-set 0":4);
  ROADMAP 7 (cs0 entry:3; #55 entry l.120+l.127 -> merged:2; fold #42 l.239 -> after l.87 by script, "- " prefix:1;
  Next/open l.100 dated pointer:1); REPO_MAP 4 (2 research rows; plan report row naming its appendix; cs0 report row);
  reports/README 0 (no index). Total 29. Left alone on purpose: STATUS l.23-25 CI on 82c605e; l.108 asset-pipeline
  risk (still true in the main: only the export part is on paper); Routines.
- FINDING (B6, STATUS cap): main's STATUS is exactly 120 lines = STATUS-DIET's cap ("at most 120 lines"); my first
  pass made it 127 (+7: build bullet 4, Next +1, Part 15 +2). The archive is outside my 4 files and his words limit
  cs0 to lines his decision changed, so no unrelated line may be cut to make room. Neither STATUS nor ROADMAP says who
  records the 2026-11-14 decision or names builder chats (grep): those are new facts, not untrue lines. Rework: net 0
  in STATUS: Last merged -> 1 line (-2); build carried in Next item 2 (no Phase bullet); Part 15 compact. Left out
  (report to caller): the Phase bullet, the second builder chat line, the five follow-ups (their text is in the rules
  file O1/O2 and the plan report "Since the plan"); the archive's "In flight" row for #55.
- B6 DONE (STATUS: 120 lines, numstat +8 -8; only wide non-table line is l.23, unchanged from main). Final: l.5 Last
  updated (cs0, #56, main c94c81b); Last merged -> 1 line (DOCS-54, PR #55, c94c81b, 17:09:33 UTC merged_at, his go;
  every merge: ROADMAP; dropped "the archive's In flight" since #55 has no archive row); #55 row -> #56 row (table,
  carries: the build + plan path, "No game code is built before change-set 0 is on main.", CHANGE 2 quote + second
  chat facts, rules/joint list, answers 98->96, and the five open items "for Follow-ups once merged"); Next item 2 (3
  lines: 2.3 quote, build plan's change-sets #56 first, Slice 3 quote; "one heavy spike" clause gone); Waiting: #56 go
  + Part 15 (2 lines, ea851a8 + path, six codes, change-set 1 waits). No Phase bullet (cap). Next: B7 (ROADMAP).
- B7 DONE (ROADMAP numstat +13 -4; Recent work 12 entries). Fold by script docs-keeper-fold.mjs: #42's entry (old
  l.239) moved verbatim as "- " line after the #39 fold (now l.88); its blank line before dropped. New cs0 entry (4
  lines + blank) on top; #55 entry: header "[2026-11-17, MERGED]" + closing sentence (advisor agent's APPROVE WITH
  FINDINGS of aea6f5b, verifier's PASS; merged PR #55, c94c81b, 17:09:33 UTC merged_at, merge commit, his go); dated
  pointer bullet (4 lines) under "v2, next" (his decision changes "wave 1 runs one heavy spike at a time"; his go's
  2.3 quote). Only added line >120 cols: the folded #42 entry (moved, not written). Branch = docs only (checked).
  Next: B8 (REPO_MAP).
- B8 DONE (REPO_MAP numstat +5 -0): rows for docs/research/asset-export-rules.md and soldier-joint-list.md (after
  UNREAD_SOURCES), and for the three reports (after the decision drafts appendix). repo-map-check: "in the map but not
  in the repository: docs/reports/2026-11-17-cs0-model-rules.md" (exit 1) - only the report written last. The research
  files are already committed (cacea55, 21:38:23 UTC) and docs/research/README.md already lists both (l.29-30): my
  B4 finding about it is resolved. Review round 1 under way (f61f43c 21:46:56; checkpoint 4fe1cfa): #56 row's state
  updated (cacea55; round 1 under way; report next). Next: B9 (reports/README: no index -> unchanged) then B10.
- B9 DONE: docs/reports/README.md left unchanged (2 lines, no index; 46 files in the folder; a 3-entry list would be a
  partial index). Report to caller as their call.
- B10 DONE: numstat STATUS +8 -8, ROADMAP +13 -4, REPO_MAP +5 -0, README 0 -> TOTAL added 26 (<= 30), deleted 12.
  STATUS 120 lines. Added lines > 120 cols outside tables: only ROADMAP's folded #42 entry (moved byte for byte).
  ROADMAP cs0 entry's last sentence now "Review round 1 under way; then his go." (118 cols). repo-map-check last line:
  "in the map but not in the repository: docs/reports/2026-11-17-cs0-model-rules.md". Nothing opened under
  /root/.claude/uploads/. Not checked: CI on main c94c81b (STATUS still says "CI green on 82c605e", left as is).
DONE
