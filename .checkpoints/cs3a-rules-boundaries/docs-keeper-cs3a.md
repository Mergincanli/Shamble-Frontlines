# docs-keeper, change-set 3a docs pass (and #64's merge)

Worktree: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt, branch cs/3a-rules-boundaries. Code head 8619d539; main 1ec6f6c.
Edit only docs/ (not DECISIONS, adr/, research/, VISION/LOOP/LOOK/EDITOR_V2, the 3a report).

## Plan
1. Read sources: card (steps 1-12, open findings, key numbers, next steps 5-6), review1/2 notes, plan s4/s9/s12,
   advisor-mul-add, mustafa-2026-11-2*.md, files-changed.txt, the diff 1ec6f6c..8619d539.
2. STATUS: In flight (#64/#63/#62 out; 3a line), Phase/Next/Waiting, Follow-ups (3A-* ids), closed marks.
3. STATUS_ARCHIVE: #64 merge block (like #61's), "From change-set 3a" follow-ups, closed marks.
4. Corrections to #64's records (card Next steps item 5), with a dated line.
5. ROADMAP: one entry; keep 8-12, fold the oldest.
6. REPO_MAP: new paths + .gate/rules-module/ + packages/sim-worker/pkg/; run repo-map-check.
7. Drift: MODULE_MAP, START_HERE, TECHNICAL only if false.

## Progress
- (step 0) notes file written.
- Read: card (all 670 lines), STATUS (120 lines, at the cap), DECISIONS diff (6 entries, 23:44:10 to 15:58:25; not touched).
- Checked: archive :161 "byte for byte" is false for the "Phase" block: f5030a3 STATUS :14-16 reworded into archive
  :162-163 ("what he meant by it (FREEZE-FDN)" dropped, wrap changed); his quoted words :17-19 = :164-166 (diff empty).
  The "Binding this chat" block (:158-159 = f5030a3 :106-107) is byte for byte. Correction owed holds.
- Source of the #64 corrections: `.gate/ci-wasm/advisor-final.md` (untracked; B4d, B5a, B6b, B8): F-SHOWN, F-EFFORT,
  F-MOVE (archive :161 AND report :498-499), F-NOTES-SHA (archive :1235; report :25 plan.md no SHA), Lows (report
  :480-482 heads, :465 "Four Lows", :35-38 "acceptEdits" (not in my brief: report only), STATUS :25 verdicts, :33-34 40 %).
- F-SHOWN checked in the transcript (assistant text, by `scratchpad/find_shown.py`): d1dbcb53 full 17:04:48 (before
  write 1, 17:23:43), abbreviated 19:49:01, full 23:17:21; 56d37fc6 full after write 2 at 20:17:08; 84b6b3fd and the
  take-back's 56d37fc6 and the planted runs' gh line (27c24e42) after their writes only at 23:17:21 UTC.
- F-EFFORT: advisor B6b: 12 launches, claude-opus-5-5; 11 at max, docs-keeper at high (its agent file).
- "Four Lows": 5911b56 test-engineer-review-1.md :47-50: Lows 4 (timeout record: deviations), 5 TE-104, 6 TE-MONO,
  7 TE-TSC-LINES: three deferred, one in "Decisions and deviations".
- 5911b56 holds `.checkpoints/ci-wasm/plan.md` and the reviewers' notes (ls-tree checked).
- GitHub (GET, 2026-11-24 ~16:3x UTC): main = 1ec6f6c; on it gate+smoke run 45373057216, toolchain run 45373056953,
  success. #64 merged 10:15:31Z by Mergincanli (merge commit's message: his tap 10:14:23 UTC in his main Advisor's
  thread, expected head 8aadfb0). #63 and #62: merged=true, closed 10:15:33Z by GitHub through 1ec6f6c (their heads
  21b6ea0, d064e17 are ancestors of 1ec6f6c: checked). 8619d539: gate+smoke run 45435601291, toolchain run
  45435601223, all success.
- Size 3,561 added (fixtures, lockfiles out) reproduced by numstat 1ec6f6c...8619d539. 56 paths, 21 new = git.
- Closed follow-ups, evidence: TE-L, TE-NT3-5, TE-MONO, TE-104: test-engineer-review1 "Proven as claimed" (a1c59e8).
  EA-F8, EA-N3, EA-N6+VF-L7: engine-architect-review1 "closed"; EA-F10 TS part only (Rust `newest_input`: 3b);
  EA-F13 (1)(3)(4) (4 in 0412814), (2) the anchor/one-writer: 3b. F-CHANNEL: 5c92311, SA r1 env probe "refused as
  meant". SA-TREE: 5c92311, its M-TREE fixed 79666ad9, SA r2 "M1 confirmed fixed". RR-F5: 5c92311, CI runner bytes =
  container's, "no registry path"; RR r1 reproduced e138cae8. `__pycache__/`: a1c59e8 (.gitignore:18 at 8619d539).
  Also in a1c59e8's message, beyond my brief: VF-L4, FAVICON (SA r1 read it: "Not a finding"), SA-ENV (TE r1 R20: no
  test can see it), FPS-LINE, NEG-ZERO. TE-NT7 NOT closed (nobody says so).
- Running, no verdict to state: security-auditor round 3 (review3.md), the verifier (verifier-final.md).
- The 3a notes (review1/2 of all five, advisor-mul-add.md) are byte-equal at f153ddf4 (pushed): cite that commit.
- Found false, NOT mine to fix: the 3a report draft :186 "His words for #64 were "The guard, the gate and CI work
  stays with Opus 5.5."": they are his words of 2026-11-12 (DECISIONS :1631), set aside for #64's CI edits by his block
  of 2026-11-23 16:30:46 UTC point 6 (:457). Tell the main session.
- Drift: START_HERE, MODULE_MAP, TECHNICAL, ART_DIRECTION, ANIMATION_PLAN hold no v2 claim (grep); nothing false.
  REPO_MAP :18 "devtools ... still an empty stub with one test" is false now.
- cs2 report names 9a09c88 as the notes' last commit: #61's block pointer is fine.
- DONE in STATUS_ARCHIVE (2026-11-24 ~16:50 UTC): :161 header corrected (F-MOVE); #64 merged row at the top of "In
  flight" (with #63/#62 closed by GitHub, the STATUS-row and 40 % corrections, his go's accepted list summarized, not
  copied); #61 block: RR-F5, SA-TREE, __pycache__ (CI group), EA-F8, EA-F10 (TS), EA-F13 (but one-writer), EA-N3,
  EA-N6, VF-L7, VF-L4, TE-NT3-5, TE-L, SA-ENV, FAVICON, FPS-LINE, NEG-ZERO marked done in 3a + a dated group line;
  #64 block: header (merged, 5911b56, corrected line), TE-104, TE-MONO, F-CHANNEL done; GH-TOOLS answered for 3a.
- DONE in STATUS_ARCHIVE too: the "From change-set 3a" block (23 ids, each with its source; the table of build options
  recorded inside 3A-NAME-SECTION: build plan :330, plan s8); Health rows (main 1ec6f6c; change-set 3a; #64 merged);
  Open bugs and risks: change-set 3a's three risks. NOTE: the archive's "3a, ranked" is the LEGACY 3a (PR #15).
- VF-L4, FPS-LINE, NEG-ZERO verified in code at 8619d539 (serve.test.mjs:73-78; frame-times mean; report.ts zero).
- REPO_MAP: map check fails on a listed entry missing on disk; not in CI. Build outputs go in rows the checker does
  not take as entries (first cell not starting with a backtick), so a clean clone still passes.
