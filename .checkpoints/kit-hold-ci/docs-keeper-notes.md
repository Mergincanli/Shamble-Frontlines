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
- B0 done (18:33:59 UTC, date -u): ls-remote main = 9b4b518f520aa2360a8007dc857ba74b343c8a40 = origin/main =
  merge-base(HEAD, origin/main). Remote branch head db445f6; local HEAD 8dedfcc (the main session's save, 18:33:57).
  The four record files: worktree = HEAD = main (STATUS f69be90a, archive 39792023, ROADMAP f944af37, REPO_MAP
  50ccc76b). DECISIONS.md: modified in the worktree (the main session's script; not mine to touch). Next B1.
- B1 done: read the card (steps 1-26), report-draft.md (373 lines), DECISIONS (15:38:13, 16:39:23, 18:01:08 at
  DECISIONS.md:130-213; RECORD-FILES :286; OLD-LINES :239; the main session committed DECISIONS as a18a62b at 18:34:30),
  infra-engineer-review1.md (L1 :252, L5 :266, Q7 :228-237), security-auditor-review1.md (F1-F5 :143-181; Not covered
  :208-226), #58's report (binding ruling :36-40; follow-ups :233-268; the two doc lines :244-248).
- B1 finding: the two doc lines that claim more than the code (#58 report :244-248): "never writes outside the
  repository" in crates/protocol/src/ts_export.rs:76 and crates/client-endpoint/src/layout_ts.rs:58; "consecutive" in
  crates/protocol/src/messages.rs:56 (Input.commands). #60 touches none of the three (its diff: .claude/CHECKPOINTS.md,
  .claude/scripts/checkpoint.mjs, .claude/scripts/checkpoint.test.mjs, .github/workflows/toolchain.yml, docs/DECISIONS.md).
- B2 finding (blame on main 9b4b518): STATUS own lines (this lane = #56 c94c81b..cd61b28, #57 d7dad97..f26ac5b, #58
  22c6afa..1bfe462): 5, 13, 29, 36-37, 40-45, 104-107, 120 (500757c, 95b67cc, 1aaa067, 07b58fc). Others: #52 (3f98c0b,
  554face, bfd719b), #55 (9660197).
- B2 finding: ROADMAP Recent work = 12 entries: #58 144-150 (own, 95b67cc), #57 152-160 (own), #56 162-168 (own),
  #55 170-178, #54 180-185, #53 187-193, #52 195-203, #48 205-220, #51 222-231, #50 233-248, #49 250-257, #46 259-270
  (3f98c0b, #52's). Adding #60 -> 13: fold #46 (259-270, merged 2d22b69) byte for byte into Build history after #44's
  folded entry (:92-106), blank separators, as cs1's pass did.
- B2 finding: REPO_MAP own lines: 19-22, 26-30, 79-80, 87-92. The kit and CI rows are other lanes': :97
  CHECKPOINTS.md (9660197, #55, the netcode proof test's session), :135 checkpoint.mjs, :136 its tests, :165
  toolchain.yml (550856a, #48, the foundation chat). RECORD-FILES: not reworded; precedent: Part 15's report :59 "Left as
  found, because they are another lane's lines", cs1's pass left REPO_MAP :16/:18/:21. They go in the answer as left.
- B2 finding: archive "In flight" newest first at :159 (#57's row, own). Archive "Follow-ups" ends :929 with "From
  change-set 0 (PR #56) and Part 15 (PR #57)" (own, 1aaa067); NO block yet for #58's follow-ups (its STATUS row says
  "for Follow-ups once merged (their text: the report's "Follow-ups")"): add one now that #58 is merged.
- B3 done (verified against the code at HEAD): checkpoint.mjs:17-24 the "Dependency hold" header paragraph, :26 exit 1
  "a dependency change is held", :37 DEP_FILE (the five names, any folder), :120-141 heldDependencyFiles (remote
  branch, else merge-base with origin/main; -z --no-renames; record at <rev>:.checkpoints/<slug>/dependency-check.md;
  link/submodule held; equal blob on origin/main at the literal path not held), :219 called in save. Tests: 33 test()
  calls in checkpoint.test.mjs (new ones :443, :492, :527, :540, :570, :591). CHECKPOINTS.md section 3 (:53-74): the
  bullet "A dependency change is held" at :60-69. toolchain.yml diff vs main: +12 -0 (cache-mode: none after
  timeout-minutes: 15; SHAMBLE_WRITE_TYPES=1 cargo test + git status compare after the first cargo test; strict tsc on
  packages/types/src shipped files, then with tests and --skipLibCheck, after tsc --version).
- B3 facts (gh api, read-only, 18:36 UTC): PR #58 merged_at 2026-11-18T15:39:29Z (commit date of 9b4b518: 15:39:28
  UTC, 16:39:28+01:00), merge 9b4b518, head 1bfe462, by Mergincanli. Main's CI on 9b4b518 (push, head_branch main):
  gate run 884 (id 44338178450; jobs gate and smoke success, 15:39:37-15:40:26) and toolchain run 182 (id
  44338178455, success). Runs 885/183 = this branch's push of the same commit. PR #60: open, draft, created
  2026-11-18T17:18:51Z, head 2dff689, no label yet. Issue #59 "Lane requests": open, created 2026-11-18T15:41:35Z by
  Mergincanli. Code commits on the branch: 8bf4bee (kit, 17:18:10), 94c197f (workflow, 17:18:10), 0c0e271 (plant,
  17:20:38), 415bd73 (take-back, 17:21:54), 4877599 (KIT-FIX, 18:03:10 committer date).
