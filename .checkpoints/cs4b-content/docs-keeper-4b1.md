# docs-keeper, 4b-1 docs pass (first pass, before the reviews' outcomes)
Started 2026-11-26 16:10 UTC (date -u). Worktree `.gate/lane1-takeover`, branch `cs/4b-content`.

## Plan
1. docs/adr/README.md: VISION-RULE v2 (register 2026-11-26 15:51:38 UTC, point 4b): R2-INT-OQ1, R4-LOS-OQ2 answered;
   R4-LOS-OQ1, R4-LOS-OQ4 partly (point 2f answers sight; point 3 stays open). Every place naming them.
2. docs/STATUS.md: In flight row 4b-1; 4a merged (#70, 2483e9a); follow-ups 4B-AIM-REACH (fixed text), VISION-RULE
   line (point 4c), 4B-FEATURE-POINTER, 4B-DEPART-3, 4B-PM5-3, 4B-CI-CONTENT, 4B-OLD-TIMERS; 4A-S3-READ closed (moved
   per STATUS's rule); Waiting on Mustafa: STEP-60 changed, answered ones removed, SIGHT-FLAG and DUR-1 added.
3. docs/ROADMAP.md: 4b-1 entry (built, in review), 4b-2 next.
4. docs/REPO_MAP.md: new crate, files, vectors; run repo-map-check.
5. Drift: START_HERE, MODULE_MAP, TECHNICAL for crate lists / content not built.

## Done
- 1 DONE (16:11 UTC): docs/adr/README.md lines 57-59 (one sentence after the 2026-11-04 list), rows R2-INT-OQ1,
  R4-LOS-OQ1, R4-LOS-OQ2, R4-LOS-OQ4, G3. Every quote checked by node `includes` against DECISIONS.md: all true.
- 4 DONE: docs/REPO_MAP.md: `crates/` row (content added; devtools' dropped deps), new rows sim/src/time.rs,
  crates/content/, devtools content_proofs.rs, time_report.rs, bin/time_report.rs, five new tests rows, vectors row
  extended, files.rs, wasm.rs, recordings.rs rows extended; client scripts legacy-extract.mjs, corpus-texts.mjs; boundary
  content.test.mjs, plain-file.ts, plain-file.test.mjs; e2e rules row (content.spec.ts), node-names, determinism. Facts
  from file headers at ef2edec. repo-map-check: OK, 234 entries cover 109 required paths.
- Found: #73 (guard-wrappers, 4a62873) and #74 (ci-preview-handover, 3b949c1) are merged on main but STATUS still shows
  them in flight. OLD-LINES (a) lets a docs pass move lines about merged PRs by script, byte for byte, listing each.
- 2 DONE: STATUS by script `docs-keeper-4b1-move.mjs` (moved byte for byte, each line checked present in the archive):
  STATUS lines 5-6 (top lines of #74's and #73's docs passes), 33-34 (#74's and #73's In flight rows), 39-44 (lane 1's
  Next item 2), 52-53 (#74's and #73's waiting bullets), 54-61 (lane 1's 4a-report bullet and RELEASE-COUNT) -> archive,
  end of "In flight", one dated block. Then new lines: header, Last merged, 4b-1 row, Next item 2, Waiting (4b-1 go +
  SIGHT-FLAG + DUR-1; answered list with pointers; STEP-60), Follow-ups VISION-RULE (his 4c words) and From 4b-1
  (4B-AIM-REACH exact text + ids), Open risks v2 (4A-S3-READ closed at 4b-1's merge). STATUS = 134 lines (was 125
  before this pass): OVER 120; not fixed (would need other lanes' lines moved).
- Archive: new block "From change-set 4b-1" at the end of "Follow-ups" (texts of all 4B- ids; quotes checked by node);
  dated pointers on 4A-S3-READ (follow-up) and on 4a's risk 1.
- 3 DONE: ROADMAP by script `docs-keeper-4b1-roadmap.mjs`: 4b-1 entry at the top of Recent work (lines 320-325),
  merge pointers under #74's and #73's entries (335, 345), Next/open pointer (251-255), four oldest entries (cs1,
  kit-hold-ci, cs2, ci-wasm) folded byte for byte into Build history (220-263); Recent work now 10 entries.
- 5 DONE: drift: START_HERE, MODULE_MAP, TECHNICAL name no v2 crate (TECHNICAL:71's "crates" is legacy walls): nothing
  made false by 4b-1, nothing changed.
- Note 16:23 UTC: the main session's uncommitted fix pass changes crates/ (content-corpus.txt header now 20360, was
  13067 at ef2edec): REPO_MAP's numbers are tagged "at ef2edec" and need the main session's update after the fix pass.
## Status: DONE (first pass), 2026-11-26 16:23 UTC.
