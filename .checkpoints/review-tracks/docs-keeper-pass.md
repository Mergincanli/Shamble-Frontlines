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
- Step 2 DONE: STATUS rows :34-:36 and bullet :49 moved by docs-keeper-move.mjs (archive In flight end, checked byte for
  byte), old Last updated line by docs-keeper-move2.mjs; new text: Phase T10, Last merged, In flight row, Next 2 and 4,
  Waiting (PASS-FORGE marker "PASS-FORGE: pending his answer"), follow-ups bullet, risks line; 120 lines.
- RC3-L1: card step 31 says SA-RC-9's fix closes it; step 36 lists it as a follow-up; caller omits it: left out.
- SA-PA-5's route name kept out of STATUS (class words): "an entry to drop from the no-prompt list".
- Step 3 DONE: archive In flight moved block :351-:366; MAIN-WALL T10 _corrected_ notes (:2057-:2060, and in its risk
  bullet); follow-ups block "From review-tracks" :2248-:2270; risks block "review-tracks" :2439-:2455; health row :2753.
  Suites re-run: track 85/85, guard 1581/1581 [verified].
- Step 4 DONE: ROADMAP entry :337-:344, three merged pointers (#78, #75, #76/#77); 3a and ci-wasm folded into Build
  history by docs-keeper-fold.mjs (:254-:281; every removed line re-added); Recent work 12.
- Step 5 DONE: REPO_MAP :239-:240 (track.mjs, track.test.mjs), :203 advisor.md, :230-:231 guard rows; check OK (239
  entries, 111 paths).
- Step 6 DONE: TECHNICAL :241 (T10's PR Advisor half). START_HERE, MODULE_MAP: no merge or reviewer claim made wrong.
- Final greps: no merge-check line in added docs lines; no model name outside his quotes; STATUS 120 lines.
- After the advisor tool's check: DECISIONS regenerated (20:43:30 Records: the plan by the card's step 11, no path or
  line count; 20:58:55 Records: security-auditor APPROVE, the advisor agent "ready to build", as the card says); 30/0,
  words check PASS again. Archive risks block: his inline quote replaced by a pointer.
- PASS-FORGE markers to update when he answers: STATUS :34, :47, :110; archive :2443-:2446 (and :2266); ROADMAP :343.
- Not recorded (not in the caller's list): SA-RC-5's remainder (card step 31, "stays for him, as on main"); RC3-L1.
- Untracked docs/reports/2026-11-27-review-tracks.md seen (not mine; untouched).
- T-MARK: a measurement in plan.md:434-441 (app mark on comments); not in the card's steps: [assumed: not run].
