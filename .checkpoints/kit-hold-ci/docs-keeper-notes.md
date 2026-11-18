docs-keeper-khc: the docs pass of change-set "kit-hold-ci", PR #60, lane 1 (branch claude/wonderful-hamilton-jdmlnf,
base main 9b4b518). Edits only docs/STATUS.md, docs/STATUS_ARCHIVE.md, docs/ROADMAP.md, docs/REPO_MAP.md (and a drifted
line of this lane's elsewhere in docs/ if found); never DECISIONS.md, never docs/reports/; never commits or pushes;
never runs a copy of the checkpoint script.

## Plan (blocks, in order; a restart continues after the last block marked done)
B0. Main check: origin/main must still be 9b4b518 (ls-remote, read-only); else stop, edit nothing.
B1. Read: the card (.checkpoints/kit-hold-ci.md, steps 1-26), report-draft.md, DECISIONS (his words of 15:38:13,
    16:39:23, 18:01:08; RECORD-FILES, OLD-LINES), infra-engineer-review1.md (findings 1, 5, question 7),
    security-auditor-review1.md ("Findings (final, ranked...)" F1-F5, "Not covered"), #58's report (two misses,
    binding ruling, follow-ups, the two doc lines).
B2. Read STATUS, STATUS_ARCHIVE (In flight, Follow-ups, Open bugs and risks, Health), ROADMAP, REPO_MAP; blame each;
    list this lane's commits (#56, #57, #58, #60).
B3. Verify against the code: checkpoint.mjs (the hold: heldDependencyFiles, exit codes), checkpoint.test.mjs (33
    tests), CHECKPOINTS.md section 3 (the rule line), toolchain.yml (cache-mode: none; regenerate-and-compare; tsc).
B4. Moves under OLD-LINES (lines about merged PRs only): #58's row STATUS -> archive "In flight" (own line, then its
    cells edited); ROADMAP folds if Recent work > 12. Script: docs-keeper-move.mjs (copy of cs1's, new anchors);
    list every moved line in docs-keeper-moved-lines.md.
B5. STATUS own-line edits: Last updated; Last merged (#58); In flight (#60's row); Next; Waiting on Mustafa; Follow-ups
    (gate v2 list, F1, F3, carried from #58); the DEP-HOLD binding line reworded. Keep <= 120 lines.
B6. STATUS_ARCHIVE: #58's merged row cells; Follow-ups texts (gate v2 list incl. reviewers' lines; F1; F3; #58's
    follow-ups that STATUS row named "for Follow-ups once merged"); Health if it changed.
B7. ROADMAP: #58's entry -> merged; a new kit-hold-ci (#60) entry; keep 8-12 entries (fold oldest by script).
B8. REPO_MAP: the new report's line; checkpoint.mjs / checkpoint.test.mjs / CHECKPOINTS.md / toolchain.yml lines if
    less than true; issue #59 if issues are listed. Run repo-map-check.
B9. Drift grep in docs/ (not reports/, DECISIONS.md, research/): the checkpoint script, the toolchain workflow's steps,
    #58's state. Fix this lane's lines; list others'.
B10. Checks: wc -l STATUS <= 120; widths <= 120 (own lines); blame proof (docs-keeper-blame-check.mjs); repo-map-check.
B11. DONE and the answer.

## Notes
