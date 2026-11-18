# advisor: plan review of kit-hold-ci (started 2026-11-18 15:46 UTC, date -u)

## Plan of this review
1. Read his words (mustafa-2026-11-18-1624-merge-58.md), plan.md, kit-edit.diff, ci-edit.diff.
2. Read the files they change on main: checkpoint.mjs, checkpoint.test.mjs, CHECKPOINTS.md s3/s5, toolchain.yml.
3. Read the report's "exact texts" at 1bfe462, DEP-HOLD / CI-TYPES / CI-CUT, A1 + security notes at 76df3f3.
4. Read the rules: CLAUDE.md, advisor.md, DECISIONS (11-14 13:58, 11-17 19:10, 11-17 22:10), STATUS "The kit".
5. Run the three test commands. Diff CI text byte for byte. Check 566d1d1 case.
6. Findings, one line each, below. Verdict at the end. DONE.

## Findings (one line each)
- Block 1 done (read his words, plan, both diffs, the four files on main). Kit diff file == scratch copies vs main; worktree kit files == origin/main.
- CI text: rebuilt from report@1bfe462 blocks 1-3 onto main's toolchain.yml by script: byte for byte equal to .gate copy (5665 B); anchors unique (L82, L90 last line, L31); ci-edit.diff == scratch. Nothing else changes. PASS.
- Tests reproduced (node v22.23.4, git 2.43.0): edited 30/30 pass; edited tests vs today's script 27 pass, 3 fail (28,29,30); existing 27 vs edited script 27 pass.
- Block 2 done (rules read: DECISIONS 11-14 13:58:05 "No kit, gate or CI file is edited in this chat without asking me in chat with its exact text"; 11-13 "written only after my yes, with the chat in Accept edits. A guard prompt alone is not my yes"; GATE-FOLLOW (b)(i) sets "CI work stays with Opus 5.5" aside for CI edits asked with exact text; Part 15 "the kit stays frozen"; DEP-HOLD = his exception "for this one change").
- Replay of miss 1 (scratch clone, origin = scratch bare at 55397c8; 566d1d1's tree committed on top, tree 0339b08 equal; da22f96's notes; foreground save): edited script HELD, exit 1, names Cargo.lock, Cargo.toml, crates/protocol/Cargo.toml, origin unchanged. Today's script on the same state: "saved and pushed", exit 0. The hold would have stopped the miss. PASS.
- Plants (scratchpad/plants): hold removed -> 3 new fail; any line releases -> 2 fail; list emptied -> 3 fail; release ignores the hash -> 1 fail; base always main -> 1 fail; record read from the working tree instead of the commit -> 0 fail (Low: "read from the commit to push" is not pinned by a test).
- E1 Low/known cost: a branch already on the remote that takes main in, where main gained a reviewed manifest/lockfile change from another PR, is HELD (false hold). Not named in the plan's known costs.
- E2 Low: a symlink named Cargo.lock is released by the hash of the link text; later edits of the target are not held (mode 120000 visible in the diff).
- E3 Low: a gitlink (submodule entry) named Cargo.lock: cat-file fails, sum 'deleted', so a "deleted  <path>" line releases a file that is present. "What cannot be read is held" is not fully true.
- E4 ok: a rename lists both paths (--no-renames); odd name (u-umlaut, quotes, space) released by real sha256sum output; CRLF record ok; "./" prefix stays held.
- E5 ok: new branch released by a record and pushed; new branch with no manifest change not held; origin/main missing -> held "could not be read".
- E6 Low: a manifest-named file inside .checkpoints/ is held when code is ahead, pushed when only notes are ahead (inert either way: workspaces are packages/* and crates/*).
- E7 Low (pre-existing quirk the hold inherits): onlyCheckpointsAhead has rename detection on; Cargo.lock moved into .checkpoints/ counts as notes-only, so the hold is skipped and the push carries [skip ci]. A removal only; --locked/npm ci would fail; required checks block a merge.
- Kit tests on main: settings.test.mjs 127/127, routine-guard.test.mjs 1107/1107, checkpoint.test.mjs 27/27.
- Export tests read (ts_export.rs, layout_ts.rs): plain mode already fails on content, missing, extra and non-plain entries; so the only drift that reaches the compare step without a planted test defect is a mode change. The plan's plant is the only non-code plant. Honest, if the report says what it proves and what not.
- cache-mode: security-auditor SRC5 (docs.github.com workflow-syntax, jobs.<job_id>.cache-mode, values read/write/write-only/none); run #168's job log :27 already prints "Cache mode: write". Not re-read by me (no web in this seat).
- M1 Medium (plan.md:26-27): "the list of the report's option (a)" is wrong: the report (1bfe462, line 355) says only "a manifest or lockfile"; the five names are the plan's reading for Cargo and npm. Say so in what he is shown.
- M2 Medium (plan.md:37-38): "its record then simply says so" reads as a self-release without the security check, against the bullet ("written after security-auditor's answer") and his "until the security check of that change has answered". Reword: a short security-auditor answer first; or ask him.
- M3 Medium (plan.md:65-67): the stop rule covers cache-mode only; git inside the cut and the tsc lines never ran on a runner (report "Not tried: git inside the cut"). Extend: any new line failing on the clean run = stop, three lines, no CI text changed without his word.
- L (plan.md:57-62): concurrency cancel-in-progress (toolchain.yml:16-18): push the take-back commit only after the plant's run has ended at the compare step, or the plant run is cancelled. chmod +x + git add is the same plant if update-index is refused.
- L (kit-edit.diff:76-81): bullet wording: "while what it would push changes" (net, tree to tree), "compared with the remote branch (main for a branch not yet pushed)", "naming that answer's notes file", "from the repository root"; while held, notes do not reach the remote either (known cost).
- L: report must carry the tier-2 before/after table of changed rule sentences (all new or stricter; none looser), the kit test counts (guard 1107, settings 127 = main; checkpoint 27 -> 30), the replay of miss 1, the model note (Fable chat: kit edit under his DEP-HOLD to this chat; CI under GATE-FOLLOW; no guard change).
- L: engine-architect counts toward the 3-agent cap; the cs2 draft fetches, builds and commits no library, holds no manifest-named file.
- Size counted: +150 -1 (script 36/1, tests 96, rules 6, workflow 12). Under 300.
- Verdict: APPROVE with conditions (no Critical/High; the Mediums are wording in what goes to him, fixed before the ask).
DONE

# Revised kit text, one verdict (2026-11-18 16:18 UTC, date -u)
- Verified: kit-edit.diff == scratch copies; kit/CI files == main; 31/31, 27 pass + 4 new fail on today's script, 27/27 existing on edited. Plants: hold removed 4 fail; gated on codeAhead 1; any mode 1; any line 2; hold skipped in background 0 (Low); --no-renames dropped 0 (Low); record read from working tree 0 (the defect below). Edge harness: link and gitlink held whatever the record; lockfile moved into notes held; E1/E4/E5 unchanged. No change for saves without a watched file except: a branch not yet pushed in a clone without origin/main is held as unreadable (fail-safe corner; name it). Within his words: "the checkpoint script" = save and watch; same five names; no guard change.
- MUST CHANGE (High on the scale: evidence that does not hold): new test 1, step 3 (kit-edit.diff:138-142) never reaches the hold: the flagged record is the only change, so save stops at "nothing else to save". It passes with the record read from the working tree. Fix: add note(repo, 'test.md', 'step 3\n') after that depCheck; assert /no recorded check: crates\/a\/Cargo\.toml\. / (the commit still holds step 2's record); fix the comment. Checked in scratchpad/step3: fixed test holds on the real script, FAILS on the working-tree plant.
- Low: no test pins the hold in front of the saver's push (q3) or --no-renames (q7): add short tests or say "by code reading" in the report. Size: 168 non-blank added lines = 181 by git diff --numstat; state the method.
- Verdict: REVISE (one line + comment in the test file); with that fix and the plant shown failing, APPROVE without another pass from me.
DONE

# The one change after his KIT-TEXT answer (2026-11-18)
Started 16:51 UTC (date -u). One read of kit-change.diff (52 added lines) under main's advisor.md (9b4b518).

## Plan of this read
1. Read his words (mustafa-2026-11-18-1730-kit-text.md), kit-change.diff, the resume card steps 8-9.
2. Rebuild the three diffs from the scratch copies; check only lines were added.
3. Read the code in place; probe the edge cases of question 5 in scratch clones (temp folder).
4. Run the three test commands; check each broken copy fails at its own step.
5. Rule on readings (a)-(d); verdict.

## Findings (one line each)
- Block 1 done: his words read whole (1542 B file; core lines 10-12); card steps 8-9; kit-change.diff read.
- Diffs rebuilt by diff -u (header timestamps aside): kit-edit.diff == main(9b4b518) to scratch; kit-edit-as-shown.diff == main to as-shown/ == kit-edit.diff at 3ebe654a (sha256 70756f3d...6ce5, 262 lines); kit-change.diff == as-shown/ to scratch: 52 added, 0 removed. toolchain.yml scratch == as-shown. Kit files in the worktree == origin/main. broken-main-rule-missing.mjs == as-shown/checkpoint.mjs byte for byte; the other two differ from the scratch script in one line each, as described.
