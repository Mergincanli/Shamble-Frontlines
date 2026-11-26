# docs-keeper: docs pass for ci-preview-handover (worktree .gate/ci4, branch cs/preview-handover, head d12e3a8, base main 2483e9a)

## Plan
1. DECISIONS.md by script (`docs-keeper-decisions.mjs` in this folder, adapted from 57fca475's copy): four entries
   above "Taken by Mustafa, 2026-11-25, 20:48:11 UTC" (all four are newer), newest first: 2026-11-26 14:08:56,
   13:04:35, 12:21:41 UTC; 2026-11-25 20:50:19 UTC. AT = readable at commit d12e3a8. Records per entry; --check.
2. STATUS.md (<= 120 lines): last updated; Last merged + #70 (2483e9a, merged_at 2026-11-26T12:22:01Z, REST read);
   main's CI line on 2483e9a (gate 45818982040, toolchain 45818982052, success, REST read); In flight: #70's row moved
   byte for byte to the archive's "In flight" (OLD-LINES), this PR's row in its place ("CI: to be read"); Waiting: a
   bullet for this PR; #70's bullet corrected for the merge and FIREFOX-REPLAY (answered by his 13:20 block);
   Follow-ups: the ci-after-3a bullet (merged #71) moved byte for byte to the archive (OLD-LINES), line 91's pointer
   widened; a ci-preview-handover bullet (TOKEN-WORDS done, 4A-FIREFOX-REPLAY done, CUT-ESCAPE accepted + CUT-ESCAPE-FIX,
   SA-2, IE-4, STEP23-NAMES, PREVIEW-MERGE-REF, the deletions at the merge).
3. STATUS_ARCHIVE.md: In flight: #70's row + a moved-header; TOKEN-WORDS (:1344 and :1731) dated pointers "done";
   FIREFOX lines (:1261 group, :1370, 3A-OTHER-BROWSERS :1484, 4A-FIREFOX-REPLAY :1794) dated pointers; CUT-ESCAPE
   (:803) dated pointer (his 15:05 point 6, SA-2); new block "From ci-preview-handover" after the 4a block (before
   :1804); the ci-after-3a STATUS bullet moved under a header at the end of Follow-ups; Health row for main 2483e9a.
4. REPO_MAP.md: toolchain.yml row (the hand-over steps; "last steps with the network" corrected); playwright.config.ts
   row (firefox: rules + replay); tests/replay row ("Runs in the chromium project" -> both); tests/rules row unchanged;
   report row docs/reports/2026-11-26-ci-preview-handover.md. Run repo-map-check.
5. ROADMAP.md: entry on top of "Recent work"; fold the oldest (Part 15) into Build history by script; dated pointers
   on ci-after-3a's (merged #71) and 4a's (merged #70) entries.
Old and new text of every corrected or moved line: docs-keeper-old-new.md.

## Notes (one line per file, as done)
- DECISIONS.md: done by `docs-keeper-decisions.mjs` (this folder) on the pristine file (sha256 75fddbe8...e12b ->
  f1c868d8...f09e), 4 entries at lines 130-235 above 2026-11-25 20:48:11 UTC, insertions only (106 lines; 3578 -> 3684);
  --check OK for all four. Workflow and config at d12e3a8 hash to his two values (git show).
- STATUS.md: done, 120 lines. Moves by `docs-keeper-moves.py` (#70's row; #69's and #71's follow-ups bullets); Edits:
  last updated, Last merged (+#70, git), main's CI on 2483e9a (gate 45818982040 with gate+smoke jobs, toolchain
  45818982052, success, gh REST), this PR's row (CI: to be read), Next 2 (+#70 merged), Waiting (+this PR; #70's bullet:
  merged, FIREFOX-REPLAY answered), Follow-ups (pointer widened; 4A-FIREFOX-REPLAY done on the branch; new bullet with
  PH- ids), Open risks CI (+ upload after the cut). Lane 2's #67 waiting line left as is (stale: #67 merged c1da1bd).
- STATUS_ARCHIVE.md: done, 2278 -> 2376 lines. Moves (script): #70's row to "In flight"; #69's and #71's STATUS
  bullets to the end of "Follow-ups" under a header. Edits: dated pointers on CUT-ESCAPE (his 15:05 point 6, PH-SA-2,
  CUT-ESCAPE-FIX), FIREFOX (his words of 2026-11-19 group), TOKEN-WORDS (#64 block and "From ci-after-3a": done),
  "Firefox in CI", 3A-OTHER-BROWSERS, 4A-FIREFOX-REPLAY (done); new block "From ci-preview-handover" after the 4a block
  (hand-over, TOKEN-WORDS, FIREFOX-REPLAY, CUT-ESCAPE-FIX, PH-SA-2, PH-IE-4, PH-KEEP-NAMES, PH-MERGE-REF, PH-TIME,
  PH-ACTIONS-SETTING, PH-DELETE); Health: a new latest row for 2483e9a, the old one's label "CI on main (2026-11-25)".
  [assumed] marks: PH-SA-2's read of lane 2's files (security-auditor's, not mine), PH-MERGE-REF.
- REPO_MAP.md: done, 244 -> 245 lines. Rows: playwright.config.ts (firefox: rules + replay), tests/replay (chromium and
  firefox), toolchain.yml ("before the cut"; the firefox tests; the check and the upload), + the report row (date order).
  repo-map-check: exit 1, one line only: "in the map but not in the repository:
  docs/reports/2026-11-26-ci-preview-handover.md" (expected until the main session writes the report; no stub made).
