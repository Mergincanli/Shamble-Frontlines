# docs-keeper: review-tracks docs pass (worktree .gate/rt-sync, branch claude/confident-gates-tbnq38, head 6a630c25)

## Plan
1. DECISIONS: five entries by script (docs-keeper-decisions.mjs), saved bytes from `git show 6a630c25:<path>`; one-line
   headers; chat's words pulled by line range from answers-context.md / chat-2026-11-26-option-d-offered.md; check with
   docs-keeper-words.mjs (wordsCheck + otherAdded; five results, the five uuids, all ok; 0 deletions).
2. STATUS: move rows MAIN-WALL (:34), #75 (:35), #78 (:36) and #75's waiting bullet (:49) to the archive's "In flight"
   end by script (docs-keeper-move.mjs, prefix asserts) first; then edits: Phase (T10 done, wall), In flight row for
   review-tracks, Next 4, Waiting on, Follow-ups (From review-tracks), Open risks. <= 120 lines.
3. Archive: moved block; "From review-tracks" follow-ups block; risks block; health row (size 1029, suites [assumed:
   card] unless re-run); MAIN-WALL T10 _corrected_ notes.
4. ROADMAP: one entry; fold oldest if > 12.
5. REPO_MAP: track.mjs, track.test.mjs; guard/advisor rows checked; repo-map-check.
6. Drift: TECHNICAL §8c PR Advisor half; START_HERE, MODULE_MAP grep.

## Notes (one line per finding)
- Step 1 DONE: DECISIONS +30/-0 (lines 183-191 the 13:55:39 entry; 234-263 the four 2026-11-26 20:xx entries); words
  check PASS (5 ok, ids in order, 0 removed, no other added text). (d)'s bold line = the offered option's terms; the
  as-built details under "Records, not his words". No pointer added, "Queued" untouched.
- T-MARK: a measurement in plan.md:434-441 (app mark on comments); not in the card's steps: [assumed: not run].
