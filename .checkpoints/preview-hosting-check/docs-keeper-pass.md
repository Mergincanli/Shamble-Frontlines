# docs-keeper: the docs pass for the preview hosting check (lane 2), 2026-11-24
Branch claude/optimistic-albattani-mdoonz (main 28254a7 merged in). Nothing committed by me.

## Plan
1. STATUS.md is at 120 of 120 lines. Lane 2 needs about 12. Room only by OLD-LINES (whole lines about merged pull
   requests, moved by script, byte for byte): the four Follow-ups bullets for the gate step (2.3) from merged PRs
   #49 (status-diet), #50 (2.1a), #51 (2.1b), #48 (2.2): STATUS lines 78-93 (16 lines) to the archive's Follow-ups, a
   new block "Moved from STATUS's Follow-ups" at its end; one pointer line of mine in STATUS in their place.
2. STATUS, lane 2's own new lines: In flight row; Next item 3; a Waiting on Mustafa bullet; a Follow-ups bullet (ids);
   an Open risks bullet. Not touched: lane 1's stale lines (3a's row and Waiting bullet, "Last merged", "Last
   updated", ROADMAP's 3a "[in flight]"): flagged to the caller.
3. STATUS_ARCHIVE.md: the follow-ups text block "From the preview hosting check" (ids PHC-*) after 3a's block; the
   risk block after 3a's v2 risk block in "Open bugs and risks". Nothing in the archive's "In flight" (merged only).
4. ROADMAP.md: one entry at the top of Recent work; 12 -> 13, so the oldest (foundation 2.2, merged #48) moves by
   script, byte for byte, to the end of Build history (after foundation 2.1b).
5. REPO_MAP.md: one row for the report after 3a's browser check row; run repo-map-check.
6. SPIKE_PLAN.md entry 6: one dated pointer line appended at the entry's end.

## Edits, as made
### STATUS.md (120 -> 119 lines; script edit-status.py in the session scratchpad, line-number asserts)
- MOVED out (OLD-LINES, byte for byte, checked: the 16 removed lines are one contiguous block among the archive's added
  lines): STATUS lines 78-93 at main 28254a7, the Follow-ups bullets "From status-diet (PR #49)", "From foundation
  2.1a (PR #50)", "From foundation 2.1b (PR #51)", "From foundation 2.2 (PR #48)". Why: STATUS was at 120 of 120.
- ADDED, In flight: row "| PR to open | the preview hosting check ... | his three decisions (ONE-OFF-HEADER,
  STEP4-COUNT, DEPARTURES); the advisor agent's review of its head; his go |" after 3a's row.
- ADDED, Next: item 3 (4 lines): lane 2's step 2.6 plan after his three decisions, unless ONE-OFF-HEADER makes it wait
  for ADR 0006's revision; his question of 2026-11-25 on change-set 6 and M before change-set 5: answered read-only in
  the chat, awaiting his yes; nothing built.
- ADDED, Waiting on Mustafa (5 lines, after 3a's bullet): ONE-OFF-HEADER, STEP4-COUNT, DEPARTURES with plain words; his
  go; the hourly checker off before 2 December; after the merge, archive the test repository, delete the test project
  and its Access application.
- ADDED, Follow-ups (2 lines) in the moved bullets' place: the pointer "For the gate step (2.3), from status-diet (#49),
  foundation 2.1a (#50), 2.1b (#51) and 2.2 (#48): moved by script ... the archive's "Follow-ups", "Moved from STATUS
  (2026-11-24)"."
- ADDED, Follow-ups (2 lines, after 3a's): "From the preview hosting check" with ids PHC-CLICKLESS, PHC-LOCKFILE,
  PHC-TOKEN-SETUP, PHC-PROBES, PHC-DELETE, PHC-ACTIONS-READ, PHC-ERROR-PATH, PHC-TR-END, PHC-SCRIPT-LOWS.
- ADDED, Open risks (1 line, last): "Preview hosting (lane 2): PHC-HEADER, the header mark not met as written (back to
  the draft); PHC-PROBE-MAIL."
- NOT touched (lane 1's, stale since #65 merged at 28254a7): 3a's In flight row and Waiting bullet, "Last merged",
  "Last updated", Next item 2's "After change-set 3a"; the Part 15 Waiting bullet ("give lane 2 Part 15").
### STATUS_ARCHIVE.md (+51 lines, additions only)
- Follow-ups, after 3a's block: "From the preview hosting check" (the nine PHC ids with the report's text, 19 lines);
  then an italic note "Moved from STATUS (2026-11-24)" (4 lines) and the 16 moved lines.
- Open bugs and risks, after 3a's v2 block: "The preview hosting check" with PHC-HEADER and PHC-PROBE-MAIL (12 lines).
### ROADMAP.md (+29 -17)
- ADDED at the top of "Recent work" (11 lines): "**The preview hosting check, ADR 0006's spike 1 [2026-11-15 to 11-24,
  in flight]** — preview-hosting-check ...": 56 runs; deletion through the cleanup job; the wall; no token or
  production for a pull request; the deleted token refused; not passed by the draft's rule (header mark; build count
  and step 4 open); two review rounds, the verifier PASS WITH NOTES; the report; waits for his three decisions, the
  advisor agent's review and his go.
- MOVED (OLD-LINES, by script, byte for byte, checked): the oldest entry, "**The foundation, 2.2: the toolchain
  [2026-11-10 to 11-13, MERGED]**" (16 lines, PR #48), from the end of "Recent work" to the end of "Build history",
  after the 2.1b entry. Recent work stays at 12 entries.
### REPO_MAP.md (+1)
- ADDED after 3a's browser-check row: "| `docs/reports/2026-11-24-preview-hosting-check.md` | The report of the preview
  hosting check (ADR 0006's spike 1, SPIKE_PLAN entry 6; lane 2, ...): the verdict per mark (...), his three
  decisions; appendices: the results log, the workflow texts as run, the advisor tool's consultations |".
- `node .claude/scripts/repo-map-check.mjs` output, verbatim: `repo map OK: 178 entries cover 109 required paths` (exit 0).
### SPIKE_PLAN.md (+4), entry 6
- ADDED at the entry's end, the file's dated-pointer form: "_Pointer (2026-11-24, the preview hosting check's report):
  the runs are done; ... By ADR 0006's rule ("A spike passes when every mark that applies passes", ADR 0006 :1404) the
  spike has not passed yet: the header mark (PM-0006-2) is not met as written (back to the draft), and the build-count
  mark (PM-0006-7) and his verdict on checklist step 4 (inside PM-0006-9) are open._" The quote checked against
  `docs/adr/0006-*.md:1404`. No word of the entry changed.
### Not done, for the caller
- His message of 2026-11-25 is in neither DECISIONS.md nor the report; only in
  `.checkpoints/preview-hosting-check/mustafa-2026-11-25-lane2-build.md`, deleted before ready. STATUS names it
  without a file pointer.
- Lane 1's stale lines (since #65 merged at 28254a7) left for lane 1: STATUS's 3a row, its Waiting bullet, "Last
  merged", "Last updated", Next item 2's "After change-set 3a"; ROADMAP's 3a "[in flight]".
- The report's DOCSKEEPERLINE is the caller's (the report is not mine to edit here).
