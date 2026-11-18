docs-keeper-cs1: the docs pass of change-set 1 "Types" (foundation step 2.4), PR #58, lane 1; edits only STATUS,
STATUS_ARCHIVE, ROADMAP, REPO_MAP; never commits.

## Plan (blocks, in order)
B0. Main check: origin/main must be 22c6afa (else stop, edit nothing).
B1. Read DECISIONS: RECORD-FILES (2026-11-17 22:10:34 UTC) and OLD-LINES (2026-11-18 11:08:25 UTC), word for word.
B2. Read the report docs/reports/2026-11-18-cs1-types.md (Follow-ups, Two misses, decisions asked).
B3. Read STATUS, STATUS_ARCHIVE, ROADMAP, REPO_MAP; blame each; list this lane's commits (#56, #57, #58).
B4. Moves under OLD-LINES (merged PRs only): pick lines, write the byte-copy script, run it, list every moved line in
    docs-keeper-moved-lines.md (file, old line, new place, first words). Script beside it.
B5. STATUS edits in this lane's own lines: Last merged (#57 + pointer to ROADMAP's list), Next ("the netcode proof
    test's result"; then change-set 2), In flight (#58), Waiting on Mustafa, Follow-ups, Open risks.
B6. STATUS_ARCHIVE edits: #57's merged row, follow-up texts, health if it changed.
B7. ROADMAP: entry for #57 (merged) and #58 as the repo's style has it; keep 8-12 entries (fold oldest by script).
B8. REPO_MAP: new paths, changed descriptions, the regenerate command, the target-folder rule; stale lines of other
    lanes listed, not touched.
B9. Checks: repo-map-check, wc -l STATUS (<= 120), line widths (<= 120), RECORD-FILES blame proof (command + result).
B10. DONE and the answer.

## Notes
- B0 done: origin/main = 22c6afa0341c36c40479f671736a569adfdb2663, merge-base(HEAD, origin/main) = 22c6afa. Next B1.
