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
- Tests (node v22.23.4, git 2.43.0): scratch tests vs scratch script 33/33 pass. New test vs broken-main-rule-missing: fails at :610 (step 1, status 1 not 0); vs broken-main-any-content: fails at :620 (step 2, status 0 not 1); vs broken-main-before-plain: fails at :632 (step 3, the list lacks sub/Cargo.lock). Kit's own 27 vs scratch script: 27/27. Scratch tests vs today's kit script: 27 pass, 6 fail (28-33). Each broken copy fails at the step meant for it.
- Probes (scratchpad/probe/probe.mjs, temp repos; final scratch script vs as-shown): S1 stale origin/main, push back to old main's X: final RELEASED (as-shown held); same after a fetch (main Y): held. S2 main's content at another path: held. S3 link equal on main: held; plain file whose blob equals main's link: held. S4 mode 644->755, main's blob: final released (reading (b)). S6 origin/main ref missing: held. S7 main removed the lockfile, taken in: held (reading (c)). S8 an unpushed local main taken in: held (origin/main is read, not local main).
- FINDING (new): S5 a lockfile under a root folder named ':' (path ':/Cargo.lock', also ':/package.json') with content NOT on main is RELEASED with no record by the final script: `git ls-tree <ref> -- :/Cargo.lock` reads ':/' as pathspec magic (top) and returns the ROOT Cargo.lock on both sides; the root file equals main's, so the new line releases the other file. As-shown script: held. A release beyond his words ("its content ... equals its content on main": that path has no content on main).
- S5 widened: final script releases ':/Cargo.lock', ':/package.json', ':/crates/a/Cargo.toml', ':(literal)x/Cargo.lock', ':(top)x/Cargo.lock' with no record when the file git resolves them to equals main's (':(top)/Cargo.lock' held: ls-tree errors). Fix A (scratchpad/probe/fixA.mjs): the new line reads main with git('--literal-pathspecs', 'ls-tree', 'refs/remotes/origin/main', '--', f): all six held; every other probe unchanged; 33/33 scratch tests and the kit's 27/27 pass on it.
- Drafted test step 4 (scratchpad/probe/checkpoint.test.step4.mjs, +21 lines at the end of test 33, own fixture): vs fixA 33/33 pass; vs the current scratch script test 33 FAILS at :652 (status 0, not 1); vs the three broken copies still fails at :610 / :620 / :632.
- A copy reading the LOCAL main (scratchpad/probe/broken-local-main.mjs) passes all 33: no test pins origin/main over local main (the code is right: probe S8). A step for it (drafted, fails on that copy at the list match) would take the size over the 300 stop line, so: say "by code reading and the advisor's probe" in the report.
- Size by git diff --no-index --numstat: script 46, tests 202, rules 10, workflow 12 = 270 (the card says 203 and 13 = 272). With step 4: 291 (stop line 300).
- His reason's premise: every manifest on main today came in through #2, #5, #48, #58 (git log --first-parent; each go not re-checked). mergeAllowed (routine-guard.mjs:89-100) would let the PR Advisor merge a manifest under docs/ (any of the five names) or a Cargo file under tools/ without his go; none exists. Not this PR (no guard change: his words); one line in the ready report.
- Known cost: a manifest-named link on main stops every branch that takes main in, with no release but its removal (step 3 asserts it). Safe (the link's target can change unseen, test 31's real.lock); none on main (all 100644).

## Required change 1 (Medium, security): the new code line, read main literally
In checkpoint.mjs (scratch line 133; kit-change.diff line 24), the new line becomes:
    const onMain = /^100(?:644|755) blob ([0-9a-f]{40,64})\t/.exec(git('--literal-pathspecs', 'ls-tree', 'refs/remotes/origin/main', '--', f).out);

## Required change 2: test 33 gains a fourth step (21 lines, inside test 33, before its closing line)

  // A lockfile in a root folder named ':' is read at its own path, never as git's ':/' (the root Cargo.lock, whose
  // content equals main's here): held.
  const magic = fixture(t);
  const first = sh(magic.origin, 'rev-parse', 'refs/heads/cs/test');
  sh(magic.repo, 'switch', '--quiet', 'main');
  writeFileSync(join(magic.repo, 'Cargo.lock'), 'merged\n');
  sh(magic.repo, 'add', 'Cargo.lock');
  sh(magic.repo, 'commit', '--quiet', '-m', 'main gains Cargo.lock');
  sh(magic.repo, 'push', '--quiet', 'origin', 'main');
  sh(magic.repo, 'switch', '--quiet', 'cs/test');
  sh(magic.repo, 'merge', '--quiet', '--no-edit', 'main');
  mkdirSync(join(magic.repo, ':'));
  writeFileSync(join(magic.repo, ':', 'Cargo.lock'), 'not on main\n');
  sh(magic.repo, 'add', '--', ':(literal):/Cargo.lock');
  sh(magic.repo, 'commit', '--quiet', '-m', 'a lockfile in a folder named :');
  note(magic.repo);
  r = run(magic.repo, 'save', 'test', '-m', 'a folder named :');
  assert.equal(r.status, 1, r.stderr + r.stdout);
  assert.match(r.stdout, /no recorded check: :\/Cargo\.lock\. /);
  assert.equal(sh(magic.origin, 'rev-parse', 'refs/heads/cs/test'), first);

Show: edited tests vs edited script 33 pass; test 33 vs today's scratch copy (keep it as broken-main-pathspec.mjs) fails at step 4's status line; vs the three broken copies still at steps 1, 2, 3; the kit's 27 vs the edited script pass.

## Rulings on readings
- (a) inside his words read with his reason; keep; one line in the ready report: a stale clone releases content main had at its last fetch (probe S1).
- (b) inside; keep (probe S4 mode-only released; S3a/S3b held).
- (c) inside his literal words (strict); right to ask him as its own decision; recommend for that entry: also pass a file main removed too (absent on main as last fetched and in the push); held until he says so.
- (d) inside; keep (his sentence, "my" -> "Mustafa's").

## Lows (no push on their own; ride along or report lines)
- Pre-existing in the text he answered on: the entry read (scratch line 128) has the same pathspec reading, so a record for a ':' path is checked against the resolved file's sha256 (a true sha256sum line never matches: fail-safe). "Everything else stays exactly": not here; STATUS follow-up + one line in the ready report (a security finding left open: needs-mustafa, which tier 2 is anyway).
- Rule line and header say "a manifest or lockfile"; the code releases plain files only (a link equal to main's stays held). Stricter than the words; optional "(a plain file on both sides)".
- No test for origin/main over local main, S3b, S4: by code reading and probe, say so.
- Size count in the card (272 vs 270 by numstat): state the method.

## Verdict
REVISE: required changes 1 and 2 above (one token in the new line; one test step shown failing on today's scratch copy). Once done and shown, APPROVE without another read from me.
DONE
